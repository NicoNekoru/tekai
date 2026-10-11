"""Bounded host counters for diagnostics, with no subprocesses or scored data.

Darwin layouts use the public SDK's mach/host_info.h, mach/vm_statistics.h
and sys/sysctl.h. HOST_VM_INFO64_REV1_COUNT requests the stable 152-byte
prefix instead of assuming the latest kernel structure. Linux reads bounded
/proc records. CPU intervals include observer/checkpoint gaps between samples.
"""

import ctypes
import math
import os
from pathlib import Path
import platform
import time

CPU_STATES = ('user', 'system', 'idle', 'nice')
PROC_CPU_STATES = ('user', 'nice', 'system', 'idle', 'iowait', 'irq', 'softirq', 'steal')
MAX_PROC_BYTES = 64 * 1024
ERROR_LIMIT = 512


class DarwinVMRev1(ctypes.Structure):
    # SDK offsetof(vm_statistics64_data_t, swapped_count) is 152 bytes.
    _fields_ = [
        *[(name, ctypes.c_uint32) for name in ('free_count', 'active_count', 'inactive_count', 'wire_count')],
        *[(name, ctypes.c_uint64) for name in ('zero_fill_count', 'reactivations', 'pageins', 'pageouts',
                                            'faults', 'cow_faults', 'lookups', 'hits', 'purges')],
        ('purgeable_count', ctypes.c_uint32), ('speculative_count', ctypes.c_uint32),
        *[(name, ctypes.c_uint64) for name in ('decompressions', 'compressions', 'swapins', 'swapouts')],
        *[(name, ctypes.c_uint32) for name in ('compressor_page_count', 'throttled_count',
                                            'external_page_count', 'internal_page_count')],
        ('total_uncompressed_pages_in_compressor', ctypes.c_uint64),
    ]


class DarwinSwapUsage(ctypes.Structure):
    _fields_ = [('total', ctypes.c_uint64), ('available', ctypes.c_uint64), ('used', ctypes.c_uint64),
                ('page_size', ctypes.c_uint32), ('encrypted', ctypes.c_int32)]


class DarwinAPI:
    def __init__(self):
        library = ctypes.CDLL('/usr/lib/libSystem.B.dylib', use_errno=True)
        self.library = library
        self.task_port = ctypes.c_uint32.in_dll(library, 'mach_task_self_').value
        self.acquire_host = library.mach_host_self
        self.acquire_host.argtypes, self.acquire_host.restype = [], ctypes.c_uint32
        self.release_host = library.mach_port_deallocate
        self.release_host.argtypes, self.release_host.restype = [ctypes.c_uint32, ctypes.c_uint32], ctypes.c_int
        for name in ('host_statistics', 'host_statistics64'):
            function = getattr(library, name)
            function.argtypes = [ctypes.c_uint32, ctypes.c_int, ctypes.c_void_p, ctypes.POINTER(ctypes.c_uint32)]
            function.restype = ctypes.c_int
            setattr(self, name, function)
        self.sysctl = library.sysctlbyname
        self.sysctl.argtypes = [ctypes.c_char_p, ctypes.c_void_p, ctypes.POINTER(ctypes.c_size_t),
                               ctypes.c_void_p, ctypes.c_size_t]
        self.sysctl.restype = ctypes.c_int


_darwin_api = None


def error_text(error):
    return f'{type(error).__name__}: {error}'[:ERROR_LIMIT]


def unavailable(row, category, error):
    row[category + '_status'] = 'unavailable'
    row[category + '_unavailable_reason'] = error_text(error)


def sysctl_value(api, name, value):
    size = ctypes.c_size_t(ctypes.sizeof(value))
    if api.sysctl(name, ctypes.byref(value), ctypes.byref(size), None, 0) != 0:
        raise OSError(ctypes.get_errno(), name.decode('ascii') + ' sysctl failed')
    if size.value != ctypes.sizeof(value):
        raise ValueError(name.decode('ascii') + ' returned an incompatible structure size')
    return value


def darwin_counters():
    global _darwin_api
    row = {}
    try:
        if _darwin_api is None:
            _darwin_api = DarwinAPI()
        api = _darwin_api
    except (OSError, AttributeError, ValueError) as error:
        for category in ('cpu', 'memory', 'swap'):
            unavailable(row, category, error)
        return row
    host = None
    try:
        host = api.acquire_host()
        if not host:
            raise OSError('mach_host_self returned no host port')
        try:
            ticks = (ctypes.c_uint32 * 4)()
            count = ctypes.c_uint32(4)
            status = api.host_statistics(host, 3, ctypes.byref(ticks), ctypes.byref(count))
            if status or count.value != 4:
                raise OSError(f'HOST_CPU_LOAD_INFO returned status {status}, count {count.value}')
            row.update(cpu_status='reported', cpu_source='Mach HOST_CPU_LOAD_INFO',
                       cpu_ticks=dict(zip(CPU_STATES, ticks)), cpu_counter_bits=32)
        except (OSError, ValueError) as error:
            unavailable(row, 'cpu', error)
        try:
            memory = DarwinVMRev1()
            count = ctypes.c_uint32(ctypes.sizeof(memory) // ctypes.sizeof(ctypes.c_int32))
            status = api.host_statistics64(host, 4, ctypes.byref(memory), ctypes.byref(count))
            if status or count.value != 38:
                raise OSError(f'HOST_VM_INFO64_REV1 returned status {status}, count {count.value}')
            page_size = os.sysconf('SC_PAGE_SIZE')
            if type(page_size) is not int or page_size <= 0:
                raise ValueError('System page size is not a positive integer')
            row.update(memory_status='reported', memory_source='Mach HOST_VM_INFO64_REV1',
                       page_size_bytes=page_size,
                       memory_bytes={name: getattr(memory, field) * page_size for name, field in (
                           ('free', 'free_count'), ('active', 'active_count'), ('inactive', 'inactive_count'),
                           ('wired', 'wire_count'), ('purgeable', 'purgeable_count'),
                           ('speculative', 'speculative_count'), ('compressor', 'compressor_page_count'),
                           ('file_backed', 'external_page_count'), ('anonymous', 'internal_page_count'))},
                       vm_cumulative_counters={name: getattr(memory, name) for name in (
                           'pageins', 'pageouts', 'faults', 'compressions', 'decompressions', 'swapins', 'swapouts')},
                       memory_scope='Host page categories overlap. Free already includes speculative pages.')
            try:
                row['physical_memory_bytes'] = sysctl_value(api, b'hw.memsize', ctypes.c_uint64()).value
            except (OSError, ValueError) as error:
                row['physical_memory_unavailable_reason'] = error_text(error)
        except (OSError, ValueError) as error:
            unavailable(row, 'memory', error)
    except (OSError, ValueError) as error:
        for category in ('cpu', 'memory'):
            unavailable(row, category, error)
    finally:
        if host:
            try:
                status = api.release_host(api.task_port, host)
                if status:
                    raise OSError(f'mach_port_deallocate returned status {status}')
                row['mach_port_cleanup_status'] = 'released'
            except (OSError, ValueError) as error:
                unavailable(row, 'mach_port_cleanup', error)
    try:
        swap = sysctl_value(api, b'vm.swapusage', DarwinSwapUsage())
        row.update(swap_status='reported', swap_source='sysctl vm.swapusage xsw_usage',
                   swap_bytes={'total': swap.total, 'available': swap.available, 'used': swap.used},
                   swap_page_size_bytes=swap.page_size, swap_encrypted=bool(swap.encrypted))
    except (OSError, ValueError) as error:
        unavailable(row, 'swap', error)
    return row


def proc_text(path):
    with path.open('r', encoding='ascii') as handle:
        source = handle.read(MAX_PROC_BYTES + 1)
    if len(source) > MAX_PROC_BYTES:
        raise ValueError(str(path) + ' exceeds the bounded telemetry input size')
    return source


def linux_counters():
    row = {}
    try:
        # Only the aggregate first line is needed, independent of CPU count.
        with Path('/proc/stat').open('r', encoding='ascii') as handle:
            source = handle.readline(4097)
        if len(source) > 4096:
            raise ValueError('Aggregate CPU record exceeds the bounded telemetry input size')
        fields = source.split()
        if not 5 <= len(fields) <= 11 or fields[0] != 'cpu':
            raise ValueError('Missing or incompatible aggregate CPU record')
        values = [int(field) for field in fields[1:]]
        if any(value < 0 for value in values):
            raise ValueError('Negative cumulative CPU counter')
        # guest and guest_nice are already included in user and nice.
        row.update(cpu_status='reported', cpu_source='/proc/stat aggregate cpu',
                   cpu_ticks=dict(zip(PROC_CPU_STATES, values[:8])), cpu_counter_bits=64)
    except (OSError, ValueError, UnicodeError) as error:
        unavailable(row, 'cpu', error)
    try:
        memory = {}
        for line in proc_text(Path('/proc/meminfo')).splitlines():
            fields = line.split()
            if fields and fields[0] in ('MemTotal:', 'MemFree:', 'MemAvailable:', 'SwapTotal:', 'SwapFree:'):
                if len(fields) != 3 or fields[2] != 'kB' or int(fields[1]) < 0:
                    raise ValueError('Incompatible memory counter units')
                memory[fields[0][:-1]] = int(fields[1]) * 1024
        if not {'MemTotal', 'MemFree', 'MemAvailable'} <= memory.keys():
            raise ValueError('Required host memory counters are absent')
        row.update(memory_status='reported', memory_source='/proc/meminfo',
                   physical_memory_bytes=memory['MemTotal'],
                   memory_bytes={'free': memory['MemFree'], 'available': memory['MemAvailable']})
        if not {'SwapTotal', 'SwapFree'} <= memory.keys() or memory['SwapFree'] > memory['SwapTotal']:
            raise ValueError('Required swap counters are absent or inconsistent')
        row.update(swap_status='reported', swap_source='/proc/meminfo',
                   swap_bytes={'total': memory['SwapTotal'], 'available': memory['SwapFree'],
                               'used': memory['SwapTotal'] - memory['SwapFree']})
    except (OSError, ValueError, UnicodeError) as error:
        for category in ('memory', 'swap'):
            if row.get(category + '_status') != 'reported':
                unavailable(row, category, error)
    try:
        counters = {}
        for line in proc_text(Path('/proc/vmstat')).splitlines():
            fields = line.split()
            if fields and fields[0] in ('pswpin', 'pswpout', 'pgfault', 'pgmajfault'):
                if len(fields) != 2 or int(fields[1]) < 0:
                    raise ValueError('Incompatible cumulative VM counter')
                counters[fields[0]] = int(fields[1])
        if len(counters) != 4:
            raise ValueError('Required cumulative VM counters are absent')
        row['vm_cumulative_counters'] = counters
        row['vm_counter_source'] = '/proc/vmstat'
    except (OSError, ValueError, UnicodeError) as error:
        row['vm_counters_unavailable_reason'] = error_text(error)
    return row


class HostSampler:
    """Keep one previous CPU observation. Telemetry never selects scored samples."""

    def __init__(self):
        self.previous_cpu = None

    def snapshot(self):
        start = time.monotonic()
        system = platform.system()
        row = {'used_for_gate': False, 'cpu_utilization_status': 'unavailable',
               'cpu_utilization_unavailable_reason': 'No prior compatible CPU observation',
               'scope': 'Host-wide OS counters, not per-process or proof of physical-host isolation.'}
        if system == 'Darwin':
            row.update(darwin_counters())
        elif system == 'Linux':
            row.update(linux_counters())
        else:
            for category in ('cpu', 'memory', 'swap'):
                unavailable(row, category, ValueError('No telemetry implementation for ' + system))
        current = row.get('cpu_ticks')
        if row.get('cpu_status') == 'reported' and current:
            signature = (system, row['cpu_source'], row['cpu_counter_bits'], tuple(current))
            if self.previous_cpu is not None:
                previous_time, previous_signature, previous = self.previous_cpu
                if signature == previous_signature and start > previous_time:
                    deltas = {name: current[name] - previous[name] for name in current}
                    wraps = [name for name, value in deltas.items() if value < 0]
                    if wraps and row['cpu_counter_bits'] == 32:
                        deltas = {name: value % (1 << 32) for name, value in deltas.items()}
                    total = sum(deltas.values())
                    if total > 0 and all(value >= 0 for value in deltas.values()):
                        idle = deltas['idle'] + deltas.get('iowait', 0)
                        row.update(cpu_utilization_status='reported',
                                   cpu_busy_fraction=(total - idle) / total,
                                   cpu_interval_seconds=start - previous_time, cpu_tick_deltas=deltas,
                                   cpu_counter_wraps=wraps,
                                   cpu_interval_scope='Between host observations, including observer/checkpoint gaps.')
                        row.pop('cpu_utilization_unavailable_reason')
                        if 'steal' in deltas:
                            row['cpu_steal_fraction'] = deltas['steal'] / total
                    else:
                        row['cpu_utilization_unavailable_reason'] = 'CPU counters did not advance or decreased'
            self.previous_cpu = (start, signature, current)
        else:
            self.previous_cpu = None
        row['observer_seconds'] = max(0.0, time.monotonic() - start)
        if not math.isfinite(row['observer_seconds']):
            row['observer_seconds'] = None
            row['observer_unavailable_reason'] = 'Nonfinite observer duration'
        return row
