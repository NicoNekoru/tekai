"""SDK layout, bounded input and port-lifetime tests for unscored telemetry."""

import ctypes
import json
from pathlib import Path
import platform
import tempfile
import unittest
from unittest.mock import patch

import host_telemetry as telemetry


class FakeDarwinAPI:
    task_port = 42

    def __init__(self):
        self.released = []
        self.cpu_status = self.vm_status = self.release_status = 0
        self.vm_count = 38

    def acquire_host(self):
        return 101

    def release_host(self, task, port):
        self.released.append((task, port))
        return self.release_status

    def host_statistics(self, host, flavor, pointer, count):
        assert (host, flavor, count._obj.value) == (101, 3, 4)
        pointer._obj[:] = (11, 12, 13, 14)
        return self.cpu_status

    def host_statistics64(self, host, flavor, pointer, count):
        assert (host, flavor, count._obj.value) == (101, 4, 38)
        pointer._obj.free_count = 10
        pointer._obj.speculative_count = 3
        pointer._obj.swapins = 20
        pointer._obj.swapouts = 30
        count._obj.value = self.vm_count
        return self.vm_status

    def sysctl(self, name, pointer, size, new_value, new_size):
        assert new_value is None and new_size == 0
        if name == b'hw.memsize':
            pointer._obj.value = 8 * 1024 ** 3
        elif name == b'vm.swapusage':
            pointer._obj.total = 1000
            pointer._obj.available = 800
            pointer._obj.used = 200
            pointer._obj.page_size = 4096
            pointer._obj.encrypted = 1
        else:
            raise AssertionError('Unexpected read-only sysctl name')
        return 0


class DarwinTests(unittest.TestCase):
    def test_revision_one_abi_matches_sdk_offsets_on_intel_and_arm64(self):
        self.assertEqual(ctypes.sizeof(telemetry.DarwinVMRev1), 152)
        self.assertEqual(ctypes.alignment(telemetry.DarwinVMRev1), 8)
        self.assertEqual(telemetry.DarwinVMRev1.decompressions.offset, 96)
        self.assertEqual(telemetry.DarwinVMRev1.compressor_page_count.offset, 128)
        self.assertEqual(telemetry.DarwinVMRev1.total_uncompressed_pages_in_compressor.offset, 144)
        self.assertEqual(ctypes.sizeof(telemetry.DarwinSwapUsage), 32)

    def test_observation_releases_host_port_and_does_not_double_count_speculation(self):
        api = FakeDarwinAPI()
        with patch.object(telemetry, '_darwin_api', api), patch.object(telemetry.os, 'sysconf', return_value=16384):
            row = telemetry.darwin_counters()
        self.assertEqual(api.released, [(42, 101)])
        self.assertEqual(row['cpu_ticks'], {'user': 11, 'system': 12, 'idle': 13, 'nice': 14})
        self.assertEqual(row['memory_bytes']['free'], 10 * 16384)
        self.assertEqual(row['memory_bytes']['speculative'], 3 * 16384)
        self.assertEqual(row['vm_cumulative_counters']['swapins'], 20)
        self.assertEqual(row['swap_bytes']['used'], 200)
        self.assertEqual(row['mach_port_cleanup_status'], 'released')
        json.dumps(row, allow_nan=False)

    def test_failed_mach_queries_are_partial_unavailable_and_still_release_port(self):
        api = FakeDarwinAPI()
        api.cpu_status = api.vm_status = 5
        with patch.object(telemetry, '_darwin_api', api):
            row = telemetry.darwin_counters()
        self.assertEqual(api.released, [(42, 101)])
        self.assertEqual(row['cpu_status'], 'unavailable')
        self.assertEqual(row['memory_status'], 'unavailable')
        self.assertIn('status 5', row['memory_unavailable_reason'])
        self.assertEqual(row['swap_status'], 'reported')

    def test_incompatible_vm_layout_is_unavailable_instead_of_zero(self):
        api = FakeDarwinAPI()
        api.vm_count = 36
        with patch.object(telemetry, '_darwin_api', api):
            row = telemetry.darwin_counters()
        self.assertEqual(row['memory_status'], 'unavailable')
        self.assertNotIn('memory_bytes', row)
        self.assertIn('count 36', row['memory_unavailable_reason'])
        self.assertEqual(api.released, [(42, 101)])

    def test_query_exception_and_failed_cleanup_are_reported(self):
        api = FakeDarwinAPI()
        api.release_status = 3
        with patch.object(telemetry, '_darwin_api', api), \
                patch.object(api, 'host_statistics64', side_effect=OSError('sandbox denied')):
            row = telemetry.darwin_counters()
        self.assertEqual(api.released, [(42, 101)])
        self.assertIn('sandbox denied', row['memory_unavailable_reason'])
        self.assertEqual(row['mach_port_cleanup_status'], 'unavailable')
        self.assertIn('status 3', row['mach_port_cleanup_unavailable_reason'])

    def test_loader_failure_is_explicit_for_every_category(self):
        with patch.object(telemetry, '_darwin_api', None), \
                patch.object(telemetry, 'DarwinAPI', side_effect=OSError('library absent')):
            row = telemetry.darwin_counters()
        for category in ('cpu', 'memory', 'swap'):
            self.assertEqual(row[category + '_status'], 'unavailable')
            self.assertIn('library absent', row[category + '_unavailable_reason'])


class LinuxTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix='test-host-counters-')
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.paths = {name: self.root / name for name in ('stat', 'meminfo', 'vmstat')}
        self.paths['stat'].write_text('cpu 11 12 13 14 15 16 17 18 999 999\n')
        self.paths['meminfo'].write_text('MemTotal: 1000 kB\nMemFree: 100 kB\nMemAvailable: 200 kB\n'
                                       'SwapTotal: 50 kB\nSwapFree: 30 kB\n')
        self.paths['vmstat'].write_text('pswpin 4\npswpout 5\npgfault 6\npgmajfault 7\n')
        self.path_patch = patch.object(telemetry, 'Path', side_effect=lambda path: self.paths[Path(path).name])
        self.path_patch.start()
        self.addCleanup(self.path_patch.stop)

    def test_bounded_records_report_host_bytes_and_cumulative_counters(self):
        row = telemetry.linux_counters()
        self.assertEqual(row['cpu_ticks']['steal'], 18)
        self.assertEqual(len(row['cpu_ticks']), 8)
        self.assertNotIn('guest', row['cpu_ticks'])
        self.assertEqual(row['memory_bytes']['available'], 200 * 1024)
        self.assertEqual(row['swap_bytes']['used'], 20 * 1024)
        self.assertEqual(row['vm_cumulative_counters']['pswpout'], 5)
        json.dumps(row, allow_nan=False)

    def test_invalid_units_and_oversized_record_are_unavailable(self):
        self.paths['meminfo'].write_text('MemTotal: 1000 MB\n')
        self.paths['vmstat'].write_text('x' * (telemetry.MAX_PROC_BYTES + 1))
        row = telemetry.linux_counters()
        self.assertEqual(row['memory_status'], 'unavailable')
        self.assertEqual(row['swap_status'], 'unavailable')
        self.assertIn('units', row['memory_unavailable_reason'])
        self.assertIn('bounded', row['vm_counters_unavailable_reason'])

    def test_absent_cpu_file_and_truncated_first_line_are_not_zero_pressure(self):
        self.paths['stat'].unlink()
        row = telemetry.linux_counters()
        self.assertEqual(row['cpu_status'], 'unavailable')
        self.assertNotIn('cpu_ticks', row)
        self.paths['stat'].write_text('cpu ' + '1' * 5000)
        row = telemetry.linux_counters()
        self.assertIn('bounded', row['cpu_unavailable_reason'])


class IntervalTests(unittest.TestCase):
    def cpu(self, user, idle, bits=32):
        return {'cpu_status': 'reported', 'cpu_source': 'fixture', 'cpu_counter_bits': bits,
                'cpu_ticks': {'user': user, 'system': 0, 'idle': idle, 'nice': 0}}

    def test_intervals_include_gaps_and_never_enter_gate(self):
        sampler = telemetry.HostSampler()
        with patch.object(telemetry.platform, 'system', return_value='Darwin'), \
                patch.object(telemetry, 'darwin_counters', side_effect=[self.cpu(10, 20), self.cpu(20, 30)]), \
                patch.object(telemetry.time, 'monotonic', side_effect=[1, 1.001, 4, 4.002]):
            first, second = sampler.snapshot(), sampler.snapshot()
        self.assertEqual(first['cpu_utilization_status'], 'unavailable')
        self.assertEqual(second['cpu_busy_fraction'], 0.5)
        self.assertEqual(second['cpu_interval_seconds'], 3)
        self.assertIn('checkpoint gaps', second['cpu_interval_scope'])
        self.assertFalse(second['used_for_gate'])
        self.assertAlmostEqual(second['observer_seconds'], 0.002)

    def test_32_bit_wrap_is_explicit_and_64_bit_decrease_is_unavailable(self):
        for bits in (32, 64):
            sampler = telemetry.HostSampler()
            rows = [self.cpu((1 << 32) - 5, 10, bits), self.cpu(5, 20, bits)]
            with patch.object(telemetry.platform, 'system', return_value='Darwin'), \
                    patch.object(telemetry, 'darwin_counters', side_effect=rows), \
                    patch.object(telemetry.time, 'monotonic', side_effect=[1, 1, 2, 2]):
                sampler.snapshot()
                row = sampler.snapshot()
            if bits == 32:
                self.assertEqual(row['cpu_busy_fraction'], 0.5)
                self.assertEqual(row['cpu_counter_wraps'], ['user'])
            else:
                self.assertEqual(row['cpu_utilization_status'], 'unavailable')
                self.assertIn('decreased', row['cpu_utilization_unavailable_reason'])

    def test_failed_or_incompatible_sample_cannot_bridge_cpu_intervals(self):
        sampler = telemetry.HostSampler()
        rows = [self.cpu(10, 10), {'cpu_status': 'unavailable'}, self.cpu(30, 30)]
        with patch.object(telemetry.platform, 'system', return_value='Darwin'), \
                patch.object(telemetry, 'darwin_counters', side_effect=rows):
            for _ in rows:
                self.assertEqual(sampler.snapshot()['cpu_utilization_status'], 'unavailable')

    def test_unknown_platform_and_observer_cost_are_explicit(self):
        with patch.object(telemetry.platform, 'system', return_value='unknown'):
            row = telemetry.HostSampler().snapshot()
        self.assertFalse(row['used_for_gate'])
        self.assertGreaterEqual(row['observer_seconds'], 0)
        for category in ('cpu', 'memory', 'swap'):
            self.assertIn('unknown', row[category + '_unavailable_reason'])


@unittest.skipUnless(platform.system() == 'Darwin', 'native Mach telemetry requires Darwin')
class NativeTests(unittest.TestCase):
    def test_native_counters_have_valid_sdk_counts_and_released_host_port(self):
        row = telemetry.HostSampler().snapshot()
        # A denied query is legitimate evidence, never fabricated pressure.
        for category in ('cpu', 'memory', 'swap'):
            self.assertIn(row[category + '_status'], ('reported', 'unavailable'))
            if row[category + '_status'] == 'unavailable':
                self.assertIn(category + '_unavailable_reason', row)
        if row['cpu_status'] == 'reported' or row['memory_status'] == 'reported':
            self.assertEqual(row.get('mach_port_cleanup_status'), 'released')
        self.assertFalse(row['used_for_gate'])
        json.dumps(row, allow_nan=False)

    def test_repeated_native_observations_do_not_accumulate_host_send_references(self):
        try:
            api = telemetry.DarwinAPI()
        except (OSError, AttributeError, ValueError) as error:
            self.skipTest(telemetry.error_text(error))
        references = api.library.mach_port_get_refs
        references.argtypes = [ctypes.c_uint32, ctypes.c_uint32, ctypes.c_uint32,
                               ctypes.POINTER(ctypes.c_uint32)]
        references.restype = ctypes.c_int
        port = api.acquire_host()
        self.assertTrue(port)
        try:
            def count():
                value = ctypes.c_uint32()
                # MACH_PORT_RIGHT_SEND is zero in mach/port.h.
                self.assertEqual(references(api.task_port, port, 0, ctypes.byref(value)), 0)
                return value.value

            before = count()
            with patch.object(telemetry, '_darwin_api', api):
                for _ in range(128):
                    self.assertEqual(telemetry.darwin_counters()['mach_port_cleanup_status'], 'released')
            self.assertEqual(count(), before)
        finally:
            self.assertEqual(api.release_host(api.task_port, port), 0)


if __name__ == '__main__':
    unittest.main()
