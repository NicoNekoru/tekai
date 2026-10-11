#!/usr/bin/env python3
"""Bounded same-binary and rotated-copy startup controls, never a gate.

Every block uses the two A/A controls and all six 2+2 A/B assignments.
Copied executables, file identities and commands remain diagnostic evidence.
"""

import argparse
from collections import Counter
import hashlib
import itertools
import json
import math
import os
from pathlib import Path
import platform
import shutil
import signal
import stat
import subprocess
import sys
import tempfile
import time

import benchmark_runtime as benchmark
import profile_runtime as profile

MAX_OUTPUT_BYTES = 32 * 1024 * 1024
MAX_XATTR_NAMES = 64
MAX_XATTR_BYTES = 128 * 1024
PATH_IDS = ('s0/r0', 's0/r1', 's1/r0', 's1/r1')
MIXED = tuple(tuple('baseline' if index in selected else 'candidate' for index in range(4))
              for selected in itertools.combinations(range(4), 2))
ASSIGNMENTS = (('baseline-aa', ('baseline',) * 4), ('candidate-aa', ('candidate',) * 4),
               *((f'ab-{index}', roles) for index, roles in enumerate(MIXED)))
WILLIAMS_BASE = (0, 1, 7, 2, 6, 3, 5, 4)
ORDER_CYCLE = tuple(order for offset in range(4)
                    for order in (PATH_IDS[offset:] + PATH_IDS[:offset],
                                  tuple(reversed(PATH_IDS[offset:] + PATH_IDS[:offset]))))
STAGES = ('parent_seconds', 'capture_setup_seconds', 'popen_seconds', 'launch_setup_seconds',
          'waiter_dispatch_seconds', 'wait_after_acceptance_seconds', 'cleanup_seconds')
FIRST_PHASE = 'first_after_copy_hash_verification'
IO_FIELDS = ('copy_bytes_read', 'copy_bytes_written', 'destination_start_hash_bytes_read',
             'destination_end_hash_bytes_read', 'source_start_hash_bytes_read', 'source_end_hash_bytes_read')


def exact_index(value, maximum, name):
    if type(value) is not int or not 0 <= value < maximum:
        raise ValueError(f'{name} must be an exact integer in 0..{maximum - 1}')


def block_schedule(block):
    exact_index(block, 8, 'Block index')
    rows = []
    for ordinal, base_index in enumerate(WILLIAMS_BASE):
        index = (base_index + block) % len(ASSIGNMENTS)
        name, roles = ASSIGNMENTS[index]
        first = ORDER_CYCLE[(block + index) % len(ORDER_CYCLE)]
        rows.append({'block_index': block, 'epoch_ordinal': ordinal, 'assignment_index': index,
                     'assignment_id': name, 'kind': 'same-binary' if index < 2 else 'mixed',
                     'assignment': dict(zip(PATH_IDS, roles)), 'copy_order': list(reversed(first)),
                     'first_launch_order': list(first)})
    return rows


def later_order(block, assignment_index, repetition):
    exact_index(block, 8, 'Block index')
    exact_index(assignment_index, 8, 'Assignment index')
    exact_index(repetition, 16, 'Later repetition index')
    # Each adjacent pair reverses the same cyclic order. Eight repetitions
    # place every fixed path at every launch ordinal exactly twice.
    return ORDER_CYCLE[(2 * ((block + assignment_index) % 4) + repetition) % len(ORDER_CYCLE)]


def schedule_balance(blocks, repeats):
    if type(blocks) is not int or not 1 <= blocks <= 8 \
            or type(repeats) is not int or not 1 <= repeats <= 16:
        raise ValueError('Schedule needs blocks 1..8 and repeats 1..16')
    assignment_positions = {name: Counter() for name, _ in ASSIGNMENTS}
    positions = {phase: {path: Counter() for path in PATH_IDS}
                 for phase in ('copy', 'first_after_copy_hash_verification', 'later')}
    artifact_paths = {path: Counter() for path in PATH_IDS}
    predecessors = Counter()
    for block in range(blocks):
        scheduled = block_schedule(block)
        predecessors.update((left['assignment_id'], right['assignment_id'])
                            for left, right in zip(scheduled, scheduled[1:]))
        for epoch in scheduled:
            assignment_positions[epoch['assignment_id']][epoch['epoch_ordinal']] += 1
            for path, label in epoch['assignment'].items():
                artifact_paths[path][label] += 1
            orders = [('copy', epoch['copy_order']),
                      ('first_after_copy_hash_verification', epoch['first_launch_order'])]
            orders.extend(('later', later_order(block, epoch['assignment_index'], repetition))
                          for repetition in range(repeats))
            for phase, order in orders:
                for ordinal, path in enumerate(order):
                    positions[phase][path][ordinal] += 1

    def balanced(rows, width):
        return all(len(row) == width and len(set(row.values())) == 1 for row in rows.values())

    return {'blocks': blocks, 'repeats': repeats,
            'artifact_counts_per_path': {path: dict(counts) for path, counts in artifact_paths.items()},
            'assignment_epoch_ordinal_counts': {name: dict(counts) for name, counts in assignment_positions.items()},
            'path_ordinal_counts': {phase: {path: dict(counts) for path, counts in rows.items()}
                                    for phase, rows in positions.items()},
            'artifact_exposure_balanced': all(counts['baseline'] == counts['candidate']
                                               for counts in artifact_paths.values()),
            'assignment_epoch_positions_balanced': balanced(assignment_positions, 8),
            'within_block_ordered_predecessor_counts': [{'previous': previous, 'next': following, 'count': count}
                                                       for (previous, following), count in sorted(predecessors.items())],
            'within_block_predecessors_balanced': len(predecessors) == 56 and set(predecessors.values()) == {1},
            'carryover_scope': 'Williams rows balance the 56 ordered distinct within-block predecessor pairs over eight blocks. Cross-block transitions are additional and unbalanced.',
            'path_positions_balanced': {phase: balanced(rows, 4) for phase, rows in positions.items()},
            'within_epoch_later_positions_balanced': repeats % 8 == 0,
            'within_epoch_later_reversal_pairs_complete': repeats % 2 == 0,
            'note': 'Reduced counts retain all assignments and equal artifact exposure, but can leave epoch or within-epoch launch positions incompletely balanced.'}


def observed_progress(report):
    """Count evidence that exists, rather than the schedule we intended to run."""
    blocks, repeats = report['policy']['blocks'], report['policy']['repeats']
    epochs, commands = report['epochs'], report['commands']
    verified = [row for row in commands if row['status'] == 'verified']
    positions = {phase: {path: Counter() for path in PATH_IDS}
                 for phase in ('copy', FIRST_PHASE, 'later')}
    artifact_paths = {path: Counter() for path in PATH_IDS}
    command_artifacts = {phase: {path: Counter() for path in PATH_IDS} for phase in (FIRST_PHASE, 'later')}
    artifact_positions = {phase: {path: {label: Counter() for label in benchmark.LABELS} for path in PATH_IDS}
                          for phase in (FIRST_PHASE, 'later')}
    assignment_positions = {name: Counter() for name, _ in ASSIGNMENTS}
    predecessors = Counter()
    by_epoch = {epoch['epoch_id']: [] for epoch in epochs}
    for row in verified:
        positions[row['phase']][row['path_id']][row['launch_ordinal']] += 1
        command_artifacts[row['phase']][row['path_id']][row['label']] += 1
        artifact_positions[row['phase']][row['path_id']][row['label']][row['launch_ordinal']] += 1
        by_epoch[row['epoch_id']].append(row)
    expected_commands = {(FIRST_PHASE, 0, path) for path in PATH_IDS} | {
        ('later', index, path) for index in range(repeats) for path in PATH_IDS}
    attempted_per_epoch = Counter(row['epoch_id'] for row in commands)
    complete_epochs = [epoch for epoch in epochs if epoch['status'] == 'complete'
                       and epoch['completed_rounds'] == repeats + 1
                       and attempted_per_epoch[epoch['epoch_id']] == len(expected_commands)
                       and len(by_epoch[epoch['epoch_id']]) == len(expected_commands)
                       and {(row['phase'], row['repetition'], row['path_id']) for row in by_epoch[epoch['epoch_id']]} == expected_commands
                       and len(epoch['copies']) == 4 and {row['path_id'] for row in epoch['copies']} == set(PATH_IDS)
                       and all(row.get('unchanged') and row.get('source_unchanged') and row.get('xattrs_unchanged') is not False
                               and row.get('sha256_start') is not None and row['sha256_start'] == row['source_sha256_start']
                               for row in epoch['copies'])]
    complete_ids = {epoch['epoch_id'] for epoch in complete_epochs}
    verified_copies = 0
    for epoch in epochs:
        for row in epoch['copies']:
            if row.get('sha256_start') is None or row['sha256_start'] != row['source_sha256_start']:
                continue
            verified_copies += 1
            positions['copy'][row['path_id']][row['copy_ordinal']] += 1
            artifact_paths[row['path_id']][row['label']] += 1
        if epoch['epoch_id'] in complete_ids:
            assignment_positions[epoch['assignment_id']][epoch['epoch_ordinal']] += 1
    for left, right in zip(epochs, epochs[1:]):
        if left['epoch_id'] in complete_ids and right['epoch_id'] in complete_ids and left['block_index'] == right['block_index']:
            predecessors[left['assignment_id'], right['assignment_id']] += 1

    def balanced(rows, width):
        return all(len(row) == width and len(set(row.values())) == 1 for row in rows.values())

    epoch_later_balance, reversal_balance = [], []
    complete_rounds = 0
    for epoch in epochs:
        rows = by_epoch[epoch['epoch_id']]
        later = [row for row in rows if row['phase'] == 'later']
        local_positions = {path: Counter(row['launch_ordinal'] for row in later if row['path_id'] == path)
                           for path in PATH_IDS}
        epoch_later_balance.append(len(later) == repeats * 4 and balanced(local_positions, 4))
        rounds = {}
        for phase, repetition in [(FIRST_PHASE, 0), *(('later', index) for index in range(repeats))]:
            selected = sorted((row for row in rows if row['phase'] == phase and row['repetition'] == repetition),
                              key=lambda row: row['launch_ordinal'])
            if len(selected) == 4 and {row['path_id'] for row in selected} == set(PATH_IDS):
                complete_rounds += 1
                rounds[phase, repetition] = [row['path_id'] for row in selected]
        reversal_balance.append(repeats % 2 == 0 and all(
            ('later', index) in rounds and ('later', index + 1) in rounds
            and rounds['later', index + 1] == list(reversed(rounds['later', index]))
            for index in range(0, repeats, 2)))
    completed_blocks = [block for block in range(blocks)
                        if sum(epoch['block_index'] == block for epoch in complete_epochs) == len(ASSIGNMENTS)]
    planned_commands = blocks * len(ASSIGNMENTS) * 4 * (1 + repeats)
    completion = {'planned_epochs': blocks * len(ASSIGNMENTS), 'attempted_epochs': len(epochs),
                  'completed_epochs': len(complete_epochs), 'completed_block_indices': completed_blocks,
                  'completed_epoch_ids': sorted(complete_ids),
                  'planned_commands': planned_commands, 'attempted_commands': len(commands),
                  'verified_commands': len(verified), 'failed_commands': sum(row['status'] == 'error' for row in commands),
                  'unverified_commands': len(commands) - len(verified),
                  'planned_rounds': blocks * len(ASSIGNMENTS) * (1 + repeats),
                  'completed_verified_rounds': complete_rounds, 'verified_replacements': verified_copies,
                  'first_launch_verified_commands': sum(row['phase'] == FIRST_PHASE for row in verified),
                  'later_verified_commands': sum(row['phase'] == 'later' for row in verified),
                  'schedule_complete': len(complete_epochs) == blocks * len(ASSIGNMENTS)
                      and len(commands) == len(verified) == planned_commands
                      and complete_rounds == blocks * len(ASSIGNMENTS) * (1 + repeats)}
    balance = {'blocks': blocks, 'repeats': repeats,
               'scope': 'Observed verified copy starts and commands. Assignment positions and predecessor pairs require complete epochs. Empty or partial coverage is not full balance.',
               'artifact_counts_per_path': {path: dict(counts) for path, counts in artifact_paths.items()},
               'artifact_command_counts_per_path': {phase: {path: dict(counts) for path, counts in rows.items()}
                                                   for phase, rows in command_artifacts.items()},
               'artifact_path_ordinal_counts': {phase: {path: {label: dict(counts) for label, counts in rows.items()}
                                                       for path, rows in paths.items()}
                                                for phase, paths in artifact_positions.items()},
               'artifact_command_exposure_balanced': {phase: all(counts['baseline'] == counts['candidate'] > 0
                                                                 for counts in rows.values())
                                                       for phase, rows in command_artifacts.items()},
               'artifact_launch_positions_balanced': {phase: all(balanced(rows, 4) for rows in paths.values())
                                                       for phase, paths in artifact_positions.items()},
               'assignment_epoch_ordinal_counts': {name: dict(counts) for name, counts in assignment_positions.items()},
               'path_ordinal_counts': {phase: {path: dict(counts) for path, counts in rows.items()}
                                      for phase, rows in positions.items()},
               'artifact_exposure_balanced': all(counts['baseline'] == counts['candidate'] > 0
                                                 for counts in artifact_paths.values()),
               'assignment_epoch_positions_balanced': balanced(assignment_positions, 8),
               'within_block_ordered_predecessor_counts': [{'previous': previous, 'next': following, 'count': count}
                                                          for (previous, following), count in sorted(predecessors.items())],
               'within_block_predecessors_balanced': len(predecessors) == 56 and set(predecessors.values()) == {1},
               'carryover_scope': report['planned_balance']['carryover_scope'],
               'path_positions_balanced': {phase: balanced(rows, 4) for phase, rows in positions.items()},
               'within_epoch_later_positions_balanced': bool(epochs) and all(epoch_later_balance),
               'within_epoch_later_reversal_pairs_complete': bool(epochs) and all(reversal_balance)}
    return completion, balance


def identity(path):
    info = path.stat()
    return {'device': info.st_dev, 'inode': info.st_ino, 'size_bytes': info.st_size,
            'mode': stat.S_IMODE(info.st_mode), 'mtime_ns': info.st_mtime_ns,
            'ctime_ns': info.st_ctime_ns}


def read_input_json(path):
    value = profile.read_json(path)
    pending = [value]
    while pending:
        node = pending.pop()
        if isinstance(node, dict):
            pending.extend(node.values())
        elif isinstance(node, list):
            pending.extend(node)
        elif type(node) is float and not math.isfinite(node):
            raise ValueError('Nonfinite numeric JSON input')
    return value


def xattrs(path):
    result = {'status': 'unavailable', 'values': [],
              'scope': 'Extended-attribute name, size and content SHA-256, observed outside command clocks.'}
    if not hasattr(os, 'listxattr') or not hasattr(os, 'getxattr'):
        result['reason'] = 'Platform has no extended-attribute API'
        return result
    try:
        names = sorted(os.listxattr(path))
        if len(names) > MAX_XATTR_NAMES:
            raise ValueError('Extended-attribute inventory exceeds its finite name limit')
        total = 0
        for name in names:
            value = os.getxattr(path, name)
            total += len(value)
            if total > MAX_XATTR_BYTES or len(name.encode('utf-8')) > 1024:
                raise ValueError('Extended-attribute inventory exceeds its finite byte/name limit')
            result['values'].append({'name': name, 'bytes': len(value), 'sha256': hashlib.sha256(value).hexdigest()})
        result['status'] = 'observed'
    except (OSError, ValueError) as error:
        result['reason'] = str(error)[-1024:]
    return result


class ControlSupervisor(benchmark.Supervisor):
    """Unwrapped benchmark supervision with available failed-attempt evidence."""

    def __init__(self, *args):
        super().__init__(*args)
        self.last_process = None
        self.last_cleanup = None

    def start(self, *args, **kwargs):
        process = super().start(*args, **kwargs)
        self.last_process = process
        return process

    def stop(self, process):
        started = time.monotonic()
        try:
            return super().stop(process)
        finally:
            self.last_cleanup = {'start': started, 'end': time.monotonic()}

    def execute(self, command, cwd, env):
        self.last_process = self.last_cleanup = None
        return super().execute(command, cwd, env)

    def failed_evidence(self):
        result = {'partial_timing': {}, 'timing_availability':
                  'Failed attempt retains only observed start, launch and cleanup boundaries. Waiter acceptance/completion remain unavailable without a successful result.',
                  'unavailable_timing_fields': ['waiter_dispatch', 'waiter_accepted', 'completion']}
        process = self.last_process
        if process is not None:
            result['command_id'] = process.command_id
            if hasattr(process, 'launch_timing'):
                result['partial_timing'].update(zip(benchmark.TIMING_METADATA['command_timing_fields'][1:6],
                                                   process.launch_timing))
            result['partial_timing']['start'] = process.started_at
            try:
                stdout, stderr, truncated = self.captured(process)
                result.update(stdout=stdout, stderr_tail=stderr, capture_truncated=truncated)
            except OSError:
                result['capture_unavailable_reason'] = 'Capture files are unavailable after the failed attempt'
        if self.last_cleanup is not None:
            result['partial_timing'].update({'cleanup_start': self.last_cleanup['start'],
                                             'cleanup_end': self.last_cleanup['end']})
        return result


def timing_stages(result):
    fields = benchmark.TIMING_METADATA['command_timing_fields']
    values = result.get('command_timing')
    seconds = result.get('seconds')
    if not profile.finite_number(seconds) or seconds <= 0 \
            or not isinstance(values, (list, tuple)) or len(values) != len(fields) \
            or any(not profile.finite_number(value) for value in values) \
            or any(left > right for left, right in zip(values, values[1:])):
        raise ValueError('Startup control needs a finite positive duration and the fixed monotonic timing inventory')
    times = dict(zip(fields, values))
    parent = result.get('parent_timing')
    if not isinstance(parent, (list, tuple)) or list(parent) != [times[name] for name in ('start', 'launch_end', 'completion')] \
            or not math.isclose(seconds, times['completion'] - times['start'], rel_tol=1e-9, abs_tol=1e-12):
        raise ValueError('Startup-control timing inventory disagrees with parent boundaries or duration')
    return {'parent_seconds': seconds,
            'capture_setup_seconds': times['capture_files_open'] - times['capture_start'],
            'popen_seconds': times['popen_return'] - times['popen_start'],
            'launch_setup_seconds': times['launch_end'] - times['start'],
            'waiter_dispatch_seconds': times['waiter_accepted'] - times['waiter_dispatch'],
            'wait_after_acceptance_seconds': times['completion'] - times['waiter_accepted'],
            'cleanup_seconds': times['cleanup_end'] - times['cleanup_start']}


def validate_copies(epoch, supervisor=None):
    errors = []
    epoch.setdefault('validation_start_monotonic', time.monotonic())
    for row in epoch['copies']:
        if row.get('validated_end'):
            continue
        path = Path(row['isolated_path'])
        if supervisor is not None:
            supervisor.remaining()
        row['validation_start_monotonic'] = time.monotonic()
        try:
            row['identity_end'] = identity(path)
            row['sha256_end'] = benchmark.executable_sha256(path)
            row['xattrs_end'] = xattrs(path)
        except OSError as error:
            row.update(identity_end=None, sha256_end=None, end_error=str(error)[-1024:])
        row['unchanged'] = row.get('sha256_start') is not None and row.get('sha256_end') == row['sha256_start'] \
            and row.get('identity_end') == row.get('identity_start')
        before, after = row.get('xattrs_start', {}), row.get('xattrs_end', {})
        row['xattrs_unchanged'] = before.get('values') == after.get('values') \
            if before.get('status') == after.get('status') == 'observed' else None
        row.update(validation_end_monotonic=time.monotonic(), validated_end=True)
        row['end_hash_bytes_read'] = row['identity_end']['size_bytes'] \
            if row.get('identity_end') and row.get('sha256_end') is not None else 0
        if not row['unchanged'] or row['xattrs_unchanged'] is False:
            errors.append('Owned executable bytes, identity or observed xattrs changed or disappeared at ' + row['path_id'])
        if supervisor is not None:
            supervisor.remaining()
    return errors


def validate_epoch_sources(epoch, sources, supervisor=None):
    if epoch.get('source_validation_end_monotonic') is not None:
        return []
    errors = []
    observations = epoch.setdefault('source_observations_end', {})
    for label, source in sources.items():
        if supervisor is not None:
            supervisor.remaining()
        try:
            current_identity, current_xattrs = identity(source), xattrs(source)
        except OSError:
            current_identity, current_xattrs = None, {'status': 'unavailable'}
        expected = epoch['source_observations_start'][label]
        before = expected['xattrs']
        xattrs_unchanged = before.get('values') == current_xattrs.get('values') \
            if before.get('status') == current_xattrs.get('status') == 'observed' else None
        unchanged = current_identity == expected['identity'] and xattrs_unchanged is not False
        observations[label] = {'identity': current_identity, 'xattrs': current_xattrs,
                               'xattrs_unchanged': xattrs_unchanged, 'unchanged': unchanged}
        if not unchanged:
            errors.append('Selected source identity or observed xattrs changed or disappeared during epoch for ' + label)
        for row in epoch['copies']:
            if row['label'] == label:
                row['source_identity_end'] = current_identity
                row['source_xattrs_end'] = current_xattrs
                row['source_xattrs_unchanged'] = xattrs_unchanged
                row['source_unchanged'] = unchanged
        if supervisor is not None:
            supervisor.remaining()
    epoch['source_validation_end_monotonic'] = time.monotonic()
    return list(dict.fromkeys(errors))


def summaries(report):
    """Keep copy-level and block/path contrasts separate from gate inference."""
    cohorts = []
    aa = []
    first_later = []
    completion, _balance = observed_progress(report)
    complete_ids = set(completion['completed_epoch_ids'])
    by_epoch = {epoch['epoch_id']: [] for epoch in report['epochs']}
    for row in report['commands']:
        by_epoch[row['epoch_id']].append(row)
    for epoch in report['epochs']:
        selected = by_epoch[epoch['epoch_id']]
        for path in PATH_IDS:
            first = [row for row in selected if row['path_id'] == path and row['phase'] == FIRST_PHASE]
            later = [row for row in selected if row['path_id'] == path and row['phase'] == 'later']
            verified_first = [row for row in first if row['status'] == 'verified']
            verified_later = [row for row in later if row['status'] == 'verified']
            complete = epoch['epoch_id'] in complete_ids and len(first) == len(verified_first) == 1 \
                and len(later) == len(verified_later) == report['policy']['repeats']
            pair = {'block_index': epoch['block_index'], 'epoch_id': epoch['epoch_id'], 'path_id': path,
                    'assignment_id': epoch['assignment_id'], 'label': epoch['assignment'][path],
                    'first_verified_commands': len(verified_first), 'later_verified_commands': len(verified_later),
                    'first_launch_ordinal': first[0]['launch_ordinal'] if first else None,
                    'seconds_since_copy_verification': first[0].get('seconds_since_copy_verification') if first else None,
                    'complete': complete, 'used_for_gate': False, 'stages': {}}
            for stage in STAGES:
                first_stats = profile.describe([row.get(stage) for row in verified_first], 'seconds')
                later_stats = profile.describe([row.get(stage) for row in verified_later], 'seconds')
                numerator, denominator = first_stats.get('median'), later_stats.get('median')
                valid = complete and numerator is not None and denominator is not None and denominator > 0
                pair['stages'][stage] = {'first': first_stats, 'later': later_stats,
                                        'first_over_later': numerator / denominator if valid else None,
                                        'first_minus_later_seconds': numerator - denominator if valid else None}
            first_later.append(pair)
        for phase in (FIRST_PHASE, 'later'):
            phase_rows = [row for row in selected if row['phase'] == phase]
            for path in PATH_IDS:
                rows = [row for row in phase_rows if row['path_id'] == path]
                if not rows:
                    continue
                verified = [row for row in rows if row['status'] == 'verified']
                cohorts.append({'block_index': epoch['block_index'], 'epoch_id': epoch['epoch_id'],
                                'assignment_id': epoch['assignment_id'], 'replacement_generation': epoch['replacement_generation'],
                                'kind': epoch['kind'], 'phase': phase, 'path_id': path,
                                'slot': path.split('/')[0], 'replica': path.split('/')[1],
                                'label': epoch['assignment'][path], 'commands': len(rows),
                                'verified_commands': len(verified), 'command_ids': [row.get('command_id') for row in rows],
                                'stages': {name: profile.describe([row.get(name) for row in verified], 'seconds') for name in STAGES},
                                'used_for_gate': False})
            if epoch['kind'] != 'same-binary':
                continue
            for slot in benchmark.SLOTS:
                numerator, denominator = slot + '/r1', slot + '/r0'
                # Fixed r1/r0 ratios compare the same artifact within one
                # slot and replacement epoch. They carry no causal verdict.
                pair = {'block_index': epoch['block_index'], 'epoch_id': epoch['epoch_id'],
                        'assignment_id': epoch['assignment_id'], 'slot': slot, 'phase': phase,
                        'label': epoch['assignment'][numerator], 'numerator_path': numerator,
                        'denominator_path': denominator, 'used_for_gate': False, 'stages': {}}
                for stage in STAGES:
                    values = {path: [row[stage] for row in phase_rows if row['path_id'] == path
                                      and row['status'] == 'verified' and stage in row]
                              for path in (numerator, denominator)}
                    described = {path: profile.describe(value, 'seconds') for path, value in values.items()}
                    expected = 1 if phase == FIRST_PHASE else report['policy']['repeats']
                    complete = all(len(value) == expected for value in values.values()) and epoch['epoch_id'] in complete_ids
                    bottom = described[denominator].get('median', 0)
                    pair['stages'][stage] = {'numerator': described[numerator], 'denominator': described[denominator],
                                            'ratio_r1_over_r0': described[numerator].get('median', 0) / bottom
                                            if complete and bottom > 0 else None, 'complete': complete}
                aa.append(pair)
    mixed = []
    for block in range(report['policy']['blocks']):
        epochs = [epoch for epoch in report['epochs'] if epoch['block_index'] == block and epoch['kind'] == 'mixed']
        if not epochs:
            continue
        for path in PATH_IDS:
            for phase in (FIRST_PHASE, 'later'):
                replacements = []
                expected = 1 if phase == FIRST_PHASE else report['policy']['repeats']
                for epoch in epochs:
                    rows = [row for row in by_epoch[epoch['epoch_id']] if row['path_id'] == path and row['phase'] == phase]
                    verified = [row for row in rows if row['status'] == 'verified']
                    replacements.append({'epoch_id': epoch['epoch_id'], 'assignment_id': epoch['assignment_id'],
                        'label': epoch['assignment'][path], 'complete': epoch['epoch_id'] in complete_ids
                            and len(rows) == len(verified) == expected,
                        'attempted_commands': len(rows), 'verified_commands': len(verified),
                        'launch_ordinals': [row['launch_ordinal'] for row in verified],
                        'seconds_since_copy_verification': [row.get('seconds_since_copy_verification') for row in verified],
                        'stage_medians': {stage: profile.describe([row.get(stage) for row in verified], 'seconds').get('median')
                                          for stage in STAGES}})
                counts = Counter(row['label'] for row in replacements)
                complete = counts == {'baseline': 3, 'candidate': 3} and all(row['complete'] for row in replacements)
                pair = {'block_index': block, 'path_id': path, 'phase': phase, 'complete': complete,
                        'replacements': replacements, 'replacement_counts': dict(counts), 'stages': {}, 'used_for_gate': False}
                for stage in STAGES:
                    described = {label: profile.describe([row['stage_medians'][stage] for row in replacements
                                                          if row['label'] == label], 'seconds') for label in benchmark.LABELS}
                    numerator, denominator = described['candidate'].get('median'), described['baseline'].get('median')
                    valid = complete and numerator is not None and denominator is not None and denominator > 0
                    pair['stages'][stage] = {**described, 'candidate_over_baseline': numerator / denominator if valid else None,
                                            'candidate_minus_baseline_seconds': numerator - denominator if valid else None}
                mixed.append(pair)
    return {'scope': 'Descriptive evidence stratified by block, replacement, path, artifact and phase. Within-copy first/later ratios and mixed-assignment block/path contrasts are diagnostics, with no confidence score, primary rescoring or causal attribution.',
            'mixed_contrast_scope': 'Each replacement supplies one phase median. Three baseline and three candidate replacements are required within one block/path/phase. First-launch ordinal and copy-to-launch delay remain recorded covariates. Per-block artifact contrasts can be order-confounded. Across a complete eight-block schedule each artifact/path appears equally at all four first-launch ordinals. No pooling across blocks, paths or phases.',
            'cohorts': cohorts, 'same_binary_slot_comparisons': aa,
            'first_later_comparisons': first_later, 'mixed_path_phase_comparisons': mixed}


def persist(report, output, final=False):
    started = time.monotonic()
    report['completion'], report['balance'] = observed_progress(report)
    if final:
        report['summaries'] = summaries(report)
    payload = json.dumps(report, separators=(',', ':'), allow_nan=False) + '\n'
    if len(payload.encode('utf-8')) > MAX_OUTPUT_BYTES:
        raise ValueError('Startup-control report exceeds the finite 32-MiB output limit')
    output.parent.mkdir(parents=True, exist_ok=True)
    benchmark.atomic_text(output, payload)
    completion = report['completion']
    lines = ['# Separate startup controls', '', f"Diagnostic status `{report['status']}`, exit code `{report.get('exit_code', 'unavailable')}`.", '',
             f"Primary outcome `{report['primary'].get('status', 'unavailable')}` remains unchanged.", '',
             'These commands are unwrapped, with phase profiling disabled. No confidence score or gate is computed.', '',
             '## Actual completion', '',
             f"Completed epochs `{completion['completed_epochs']}/{completion['planned_epochs']}`, attempted `{completion['attempted_epochs']}`.",
             f"Verified commands `{completion['verified_commands']}/{completion['planned_commands']}`, attempted `{completion['attempted_commands']}`, failed `{completion['failed_commands']}`.",
             f"Verified complete rounds `{completion['completed_verified_rounds']}/{completion['planned_rounds']}`.",
             f"Verified first launches `{completion['first_launch_verified_commands']}` and later launches `{completion['later_verified_commands']}`.",
             f"Complete blocks `{completion['completed_block_indices']}`. Requested schedule complete `{completion['schedule_complete']}`.", '',
             '## Planned and achieved balance', '', report['balance']['scope'], '',
             '| Balance check | Planned | Achieved |', '| --- | --- | --- |']
    for key in ('artifact_exposure_balanced', 'assignment_epoch_positions_balanced', 'within_block_predecessors_balanced',
                'within_epoch_later_positions_balanced', 'within_epoch_later_reversal_pairs_complete'):
        lines.append(f"| {key} | {report['planned_balance'][key]} | {report['balance'][key]} |")
    for phase in ('copy', FIRST_PHASE, 'later'):
        lines.append(f"| {phase} path ordinal balance | {report['planned_balance']['path_positions_balanced'][phase]} | {report['balance']['path_positions_balanced'][phase]} |")
    lines.extend(['', '| Path | Verified baseline replacements | Verified candidate replacements |', '| --- | ---: | ---: |'])
    for path, counts in report['balance']['artifact_counts_per_path'].items():
        lines.append(f"| {path} | {counts.get('baseline', 0)} | {counts.get('candidate', 0)} |")
    lines.extend(['', f"Verified first-launch artifact exposure balance `{report['balance']['artifact_command_exposure_balanced'][FIRST_PHASE]}`.",
                  f"Verified first-launch ordinal balance within each artifact/path `{report['balance']['artifact_launch_positions_balanced'][FIRST_PHASE]}`.", '',
                  '| Path | Artifact | First ordinal 0 | First ordinal 1 | First ordinal 2 | First ordinal 3 |',
                  '| --- | --- | ---: | ---: | ---: | ---: |'])
    for path, labels in report['balance']['artifact_path_ordinal_counts'][FIRST_PHASE].items():
        for label, counts in labels.items():
            lines.append(f"| {path} | {label} | " + ' | '.join(str(counts.get(index, 0)) for index in range(4)) + ' |')
    lines.extend(['', report['balance']['carryover_scope'], '', '## Limits', '',
                  *('- ' + note for note in report['limits']), '', '## Errors', '',
                  *('- ' + error.replace('\n', ' ') for error in report['errors'])])
    if not final:
        lines.extend(['', 'Progress checkpoint. Full descriptive tables are generated once during finalization.'])
        benchmark.atomic_text(output.with_suffix('.md'), '\n'.join(lines) + '\n')
        report['checkpointing']['completed_pairs_before_snapshot'] += 1
        report['checkpointing']['seconds_before_snapshot'] += time.monotonic() - started
        return
    lines.extend(['', report['summaries']['scope'], '',
             '## Stratified commands', '',
             '| Block | Epoch | Assignment | Generation | Path | Artifact | Phase | Verified/attempted | Median command ms | Median Popen span ms |',
             '| ---: | ---: | --- | ---: | --- | --- | --- | --- | ---: | ---: |'])
    for row in report['summaries']['cohorts']:
        def milliseconds(stage):
            value = row['stages'][stage].get('median')
            return f'{value * 1000:.4f}' if value is not None else 'unavailable'
        lines.append(f"| {row['block_index']} | {row['epoch_id']} | {row['assignment_id']} | {row['replacement_generation']} | "
                     f"{row['path_id']} | {row['label']} | {row['phase']} | {row['verified_commands']}/{row['commands']} | "
                     f"{milliseconds('parent_seconds')} | {milliseconds('popen_seconds')} |")
    lines.extend(['', '## Same-binary comparisons', '',
                  'Each ratio has a fixed r1 numerator and r0 denominator within the named slot, artifact, replacement and phase.', '',
                  '| Block | Epoch | Artifact | Slot | Phase | Command median r1/r0 | Popen median r1/r0 |',
                  '| ---: | ---: | --- | --- | --- | ---: | ---: |'])
    for row in report['summaries']['same_binary_slot_comparisons']:
        def ratio(stage):
            value = row['stages'][stage]['ratio_r1_over_r0']
            return f'{value:.6f}' if value is not None else 'unavailable'
        lines.append(f"| {row['block_index']} | {row['epoch_id']} | {row['label']} | {row['slot']} | {row['phase']} | "
                     f"{ratio('parent_seconds')} | {ratio('popen_seconds')} |")
    lines.extend(['', '## First launch versus later launches of the same copy', '',
                  'Ratios require a fully validated epoch, one first launch and every requested later launch. Partial evidence remains in JSON with null ratios.', '',
                  '| Block | Epoch | Path | Artifact | First ordinal | Seconds since verification | Complete | Command first/later | Popen first/later |',
                  '| ---: | ---: | --- | --- | ---: | ---: | --- | ---: | ---: |'])
    def formatted(value):
        return f'{value:.6f}' if value is not None else 'unavailable'
    for row in report['summaries']['first_later_comparisons']:
        lines.append(f"| {row['block_index']} | {row['epoch_id']} | {row['path_id']} | {row['label']} | "
                     f"{row['first_launch_ordinal']} | {formatted(row['seconds_since_copy_verification'])} | {row['complete']} | "
                     f"{formatted(row['stages']['parent_seconds']['first_over_later'])} | {formatted(row['stages']['popen_seconds']['first_over_later'])} |")
    lines.extend(['', '## Mixed-assignment contrasts within each block and path', '', report['summaries']['mixed_contrast_scope'], '',
                  '| Block | Path | Phase | Baseline/candidate replacements | Complete | Command candidate/baseline | Popen candidate/baseline |',
                  '| ---: | --- | --- | --- | --- | ---: | ---: |'])
    for row in report['summaries']['mixed_path_phase_comparisons']:
        counts = row['replacement_counts']
        lines.append(f"| {row['block_index']} | {row['path_id']} | {row['phase']} | {counts.get('baseline', 0)}/{counts.get('candidate', 0)} | "
                     f"{row['complete']} | {formatted(row['stages']['parent_seconds']['candidate_over_baseline'])} | "
                     f"{formatted(row['stages']['popen_seconds']['candidate_over_baseline'])} |")
    benchmark.atomic_text(output.with_suffix('.md'), '\n'.join(lines) + '\n')
    report['checkpointing']['completed_pairs_before_snapshot'] += 1
    report['checkpointing']['seconds_before_snapshot'] += time.monotonic() - started


def parser():
    result = argparse.ArgumentParser(description=__doc__)
    for name in ('baseline', 'candidate', 'metadata', 'primary-report', 'output'):
        result.add_argument('--' + name, type=Path, required=True)
    result.add_argument('--blocks', type=int, default=8, help='Fixed balanced assignment blocks, 1 to 8')
    result.add_argument('--repeats', type=int, default=8, help='Fixed later repetitions per replacement, 1 to 16')
    result.add_argument('--timeout', type=float, default=30, help='Per-command bound, (0, 30] seconds')
    result.add_argument('--budget', type=float, default=600, help='Whole diagnostic bound, (0, 600] seconds')
    return result


def alias_guard(outputs, protected, argument_parser):
    for output in outputs:
        for path in protected:
            if output.resolve() == path.resolve() or output.exists() and path.exists() and output.samefile(path):
                argument_parser.error('Startup-control output cannot overwrite a source, metadata, primary or raw-journal alias')
    if outputs[0].exists() and outputs[1].exists() and outputs[0].samefile(outputs[1]):
        argument_parser.error('Startup-control JSON and Markdown outputs cannot alias each other')


def run_epochs(report, args, sources, source_hashes, metadata, supervisor, work):
    source_observations = {label: {'identity': identity(source), 'xattrs': xattrs(source)}
                           for label, source in sources.items()}
    paths = {path: work / path / 'bin/tekai' for path in PATH_IDS}
    environments, projects = {}, {}
    for slot in benchmark.SLOTS:
        environments[slot] = benchmark.isolated_environment(work / slot / 'environment')
        environments[slot].pop('TEKAI_DIAGNOSTIC_PROFILE', None)
        projects[slot] = work / slot / 'project'
        projects[slot].mkdir(parents=True)
    report['owned_cells'] = [{'path_id': path, 'isolated_path': str(paths[path]),
                             'cwd': str(projects[path.split('/')[0]]),
                             'environment': {key: environments[path.split('/')[0]].get(key)
                                             for key in benchmark.ISOLATED_ENVIRONMENT_KEYS},
                             'phase_profile_enabled': False} for path in PATH_IDS]
    for block in range(args.blocks):
        for planned in block_schedule(block):
            supervisor.remaining()
            epoch_id = len(report['epochs'])
            epoch = {**planned, 'epoch_id': epoch_id, 'replacement_generation': epoch_id + 1,
                     'status': 'incomplete', 'copies': [], 'completed_rounds': 0,
                     'source_observations_start': source_observations,
                     'prior_epoch_id': epoch_id - 1 if epoch_id else None,
                     'prior_assignment_id': report['epochs'][-1]['assignment_id'] if epoch_id else None,
                     'block_boundary': bool(epoch_id and planned['epoch_ordinal'] == 0),
                     'start_monotonic': time.monotonic()}
            report['epochs'].append(epoch)
            try:
                epoch['source_check_start_monotonic'] = time.monotonic()
                current_sources = {}
                for label, source in sources.items():
                    supervisor.remaining()
                    current_sources[label] = {'identity': identity(source), 'xattrs': xattrs(source)}
                    supervisor.remaining()
                    before, after = source_observations[label]['xattrs'], current_sources[label]['xattrs']
                    if current_sources[label]['identity'] != source_observations[label]['identity'] \
                            or before.get('status') == after.get('status') == 'observed' \
                            and before.get('values') != after.get('values'):
                        raise ValueError('Selected source identity or observed xattrs changed before replacement for ' + label)
                epoch['source_check_end_monotonic'] = time.monotonic()
                epoch['source_observations_start'] = current_sources
                epoch['copy_hash_start_monotonic'] = time.monotonic()
                for ordinal, path_id in enumerate(epoch['copy_order']):
                    supervisor.remaining()
                    label, destination = epoch['assignment'][path_id], paths[path_id]
                    destination.parent.mkdir(parents=True, exist_ok=True)
                    previous = identity(destination) if destination.exists() else None
                    row = {'path_id': path_id, 'slot': path_id.split('/')[0], 'replica': path_id.split('/')[1],
                           'label': label, 'copy_ordinal': ordinal, 'replacement_generation': epoch_id + 1,
                           'source_path': str(sources[label]), 'isolated_path': str(destination),
                           'source_sha256_start': source_hashes[label], 'source_identity': current_sources[label]['identity'],
                           'source_hash_scope': 'Full source hash bound before the run and checked in protected_inputs at run end. Epoch boundaries check identity and available xattrs, not another full source hash.',
                           'source_xattrs': current_sources[label]['xattrs'], 'identity_previous': previous,
                           'sha256_start': None, 'copy_start_monotonic': time.monotonic()}
                    epoch['copies'].append(row)
                    # Unlink and create each owned file independently. Inode
                    # reuse is possible and is recorded, never assumed away.
                    destination.unlink(missing_ok=True)
                    shutil.copy2(sources[label], destination)
                    row.update(identity_start=identity(destination), xattrs_start=xattrs(destination),
                               sha256_start=benchmark.executable_sha256(destination),
                               copy_verified_monotonic=time.monotonic())
                    row['inode_replaced'] = None if previous is None else \
                        (previous['device'], previous['inode']) != (row['identity_start']['device'], row['identity_start']['inode'])
                    if row['sha256_start'] != source_hashes[label]:
                        raise ValueError('Owned executable copy differs from the primary source')
                    supervisor.remaining()
                epoch['all_copies_verified_monotonic'] = time.monotonic()
                epoch['copy_hash_elapsed_seconds'] = epoch['all_copies_verified_monotonic'] - epoch['copy_hash_start_monotonic']
                epoch['replacement_setup_elapsed_seconds'] = epoch['all_copies_verified_monotonic'] - epoch['source_check_start_monotonic']
                copies = {row['path_id']: row for row in epoch['copies']}
                rounds = [(FIRST_PHASE, 0, epoch['first_launch_order'])]
                rounds.extend(('later', repetition, later_order(block, epoch['assignment_index'], repetition))
                              for repetition in range(args.repeats))
                for phase, repetition, order in rounds:
                    for ordinal, path_id in enumerate(order):
                        supervisor.remaining()
                        slot, replica = path_id.split('/')
                        label = epoch['assignment'][path_id]
                        command = [paths[path_id], '--version']
                        row = {'epoch_id': epoch_id, 'block_index': block, 'epoch_ordinal': epoch['epoch_ordinal'],
                               'assignment_id': epoch['assignment_id'], 'assignment_index': epoch['assignment_index'],
                               'replacement_generation': epoch_id + 1, 'kind': epoch['kind'],
                               'path_id': path_id, 'slot': slot, 'replica': replica, 'label': label,
                               'phase': phase, 'repetition': repetition, 'launch_ordinal': ordinal,
                               'path_launch_order': list(order), 'command': list(map(str, command)),
                               'cwd': str(projects[slot]), 'status': 'running', 'used_for_gate': False,
                               'sha256': copies[path_id]['sha256_start'], 'output_validation': None,
                               'launch_unix_ns': time.time_ns(), 'launch_monotonic': time.monotonic()}
                        row['seconds_since_copy_verification'] = row['launch_monotonic'] - copies[path_id]['copy_verified_monotonic']
                        report['commands'].append(row)
                        try:
                            result = supervisor.execute(command, projects[slot], environments[slot])
                            row.update(result)
                            row.update(timing_stages(result))
                            fixture = {'expected_version': metadata[label]['expected_version'],
                                       'command': command, 'project': projects[slot], 'env': environments[slot]}
                            expected = f"tekai {fixture['expected_version']}\n"
                            if result['stdout'] != expected or result['stderr_tail']:
                                raise benchmark.BenchmarkError('Startup control output differs from the selected package version or stderr is nonempty')
                            row.update(status='verified', output_validation={'expected_version': fixture['expected_version'],
                                'expected_stdout': expected, 'stdout_verified': True, 'stderr_empty': True, 'used_for_gate': False})
                        except (OSError, ValueError, RuntimeError, subprocess.SubprocessError, KeyboardInterrupt) as error:
                            row.update(status='error', error=f'{type(error).__name__}: {error}'[-8192:])
                            if 'parent_timing' not in row:
                                row.update(supervisor.failed_evidence())
                            raise
                        finally:
                            row.update(finish_unix_ns=time.time_ns(), finish_monotonic=time.monotonic())
                    epoch['completed_rounds'] += 1
                supervisor.remaining()
                errors = validate_copies(epoch, supervisor)
                errors.extend(validate_epoch_sources(epoch, sources, supervisor))
                if errors:
                    raise ValueError('; '.join(errors))
                supervisor.remaining()
                epoch['status'] = 'complete'
            finally:
                epoch['end_monotonic'] = time.monotonic()
                errors = validate_copies(epoch)
                errors.extend(validate_epoch_sources(epoch, sources))
                report['errors'].extend(error for error in errors if error not in report['errors'])
                epoch['destination_end_hash_bytes_read'] = sum(row.get('end_hash_bytes_read', 0) for row in epoch['copies'])
                completed_copies = [row for row in epoch['copies'] if row.get('identity_start')]
                epoch['copy_bytes_read'] = sum(row['identity_start']['size_bytes'] for row in completed_copies)
                epoch['copy_bytes_written'] = epoch['copy_bytes_read']
                epoch['destination_start_hash_bytes_read'] = sum(row['identity_start']['size_bytes'] for row in completed_copies
                                                                if row.get('sha256_start') is not None)
                epoch['source_start_hash_bytes_read'] = epoch['source_end_hash_bytes_read'] = 0
                epoch['validation_elapsed_seconds'] = time.monotonic() - epoch['validation_start_monotonic']
                report['io_totals'] = {name: sum(row.get(name, 0) for row in report['epochs']) for name in IO_FIELDS}
                report['io_totals']['scope'] = 'Logical sizes for completed full copies and successful hashes during epochs. Partial copy/hash attempts are not byte-counted. Run-boundary protected hashes are recorded separately. Not physical disk I/O or page-cache misses.'
                persist(report, args.output)


def main(argv=None):
    argument_parser = parser()
    args = argument_parser.parse_args(argv)
    if os.name != 'posix' or platform.system() not in ('Darwin', 'Linux'):
        argument_parser.error('Requires owned POSIX process groups on macOS or Linux')
    if not 1 <= args.blocks <= 8 or not 1 <= args.repeats <= 16 \
            or any(not math.isfinite(getattr(args, name)) or not 0 < getattr(args, name) <= bound
                   for name, bound in (('timeout', 30), ('budget', 600))):
        argument_parser.error('Requires blocks 1..8, repeats 1..16 and finite positive bounded timeout/budget')
    sources = {label: getattr(args, label).resolve() for label in benchmark.LABELS}
    args.metadata, args.primary_report, args.output = args.metadata.resolve(), args.primary_report.resolve(), args.output.resolve()
    if args.output.suffix.lower() != '.json':
        argument_parser.error('Output must have a .json extension')
    primary_paths = [args.primary_report]
    if args.primary_report.with_suffix('.md').exists():
        primary_paths.append(args.primary_report.with_suffix('.md').resolve())
    hint_error = None
    try:
        hint = read_input_json(args.primary_report)
        journal_path = profile.primary_journal_path(hint, args.primary_report)
        if journal_path is not None:
            primary_paths.append(journal_path)
    except (OSError, ValueError, RecursionError) as error:
        hint_error = str(error)
    protected_paths = [*sources.values(), args.metadata, *primary_paths]
    alias_guard((args.output, args.output.with_suffix('.md')), protected_paths, argument_parser)
    started = time.monotonic()
    report = {'schema_version': 2, 'report_kind': 'startup-controls', 'status': 'incomplete',
              'diagnostic_only': True, 'used_for_gate': False,
              'primary': {'path': str(args.primary_report), 'status': 'unavailable'},
              'policy': {'blocks': args.blocks, 'repeats': args.repeats, 'timeout_seconds': args.timeout,
                         'budget_seconds': args.budget, 'phase_profile_enabled': False, 'wrapped': False,
                         'sampling': 'Fixed complete assignment epochs. No adaptive retries, outlier removal, sample selection or primary rescoring.',
                         'first_launch': 'First after copy/hash verification, kept separate from every fixed later repetition.',
                         'replacement': 'All four owned fixed files are independently unlinked, copied with copy2 and hash-verified each epoch. End hashes and identities validate before the next replacement.',
                         'source_verification': 'Full source hashes at run boundaries, identity and available xattrs at epoch boundaries. Destination full hashes before and after every epoch remain required.'},
              'timing': dict(benchmark.TIMING_METADATA), 'planned_balance': schedule_balance(args.blocks, args.repeats),
              'checkpointing': {'completed_pairs_before_snapshot': 0, 'seconds_before_snapshot': 0,
                               'scope': 'Initial and one compact JSON/short Markdown checkpoint per attempted epoch, plus a final report. Counts and elapsed time cover completed pairs before this snapshot, excluding the write of this snapshot. Full descriptive summaries are built only at finalization.'},
              'stage_scopes': {'waiter_dispatch_seconds': 'Waiter dispatch through worker acceptance, including worker scheduling.',
                               'popen_seconds': benchmark.TIMING_METADATA['popen_scope']},
              'machine': {'system': platform.system(), 'architecture': platform.machine(),
                          'logical_cpu_count': os.cpu_count(), 'python': platform.python_version()},
              'limits': ['Copying and hash verification read executable bytes before the first launch. These are fresh-copy controls, not cold-start or cold-page-cache measurements. No OS cache eviction is attempted.',
                         'Replacements can alter inode, code-signing and kernel launch/cache history. Inode reuse is reported. Equal-length paths do not establish equal loader/cache state.',
                         'Independent per-slot environments and one common cwd per slot remain fixed. Both executable replicas within a slot share the same environment and cwd. A cross-slot difference includes slot environment and cwd, not executable path text alone.',
                         'Recorded Popen spans include mandatory ownership registration and diagnostic bookkeeping, not isolated loader time. Blocking wait includes worker/completion scheduling.',
                         'A/A ratios are fixed r1/r0 within one artifact and slot. First/later ratios compare the same replacement. Mixed block/path contrasts compare replacement phase medians across the six mixed assignments and can be launch-order-confounded. These observations give no causal verdict or speedup confidence score.',
                         'Copying, executable hashing and report checkpoints perform file I/O outside command clocks and condition caches. At most four live owned copies remain, with no retained per-epoch executable archive.',
                         'Source full hashes occur at run boundaries. Source identity and available xattrs are checked at epoch boundaries; each destination still has independent full pre-launch and post-epoch hashes. Lower overhead changes observer I/O and is not cache-equivalent to earlier control runs.',
                         'Reduced block/repetition counts can leave positional and predecessor balance incomplete. Diagnostic completion means the requested fixed schedule finished, not that all full-cycle balance flags are true.',
                         'Cyclic/reversed launch orders balance command positions. They do not cover every possible immediate predecessor path pair, so launch-order carryover remains unresolved.',
                         'Optional xattrs are bounded outside clocks. Their hashes are observations, not a code-signing validation. Shared-runner load and cache conditioning remain unresolved.'],
              'owned_cells': [], 'epochs': [], 'commands': [], 'errors': []}
    persist(report, args.output)
    protected, supervisor = {}, None
    previous_handler = signal.getsignal(signal.SIGTERM)

    def terminate(_number, _frame):
        raise KeyboardInterrupt

    signal.signal(signal.SIGTERM, terminate)
    try:
        for path in (*primary_paths, args.metadata):
            protected[path] = benchmark.executable_sha256(path)
        primary = read_input_json(args.primary_report)
        report['primary'].update(status=primary.get('status', 'unknown'),
                                 exit_code=primary.get('exit_code'), sha256_start=protected[args.primary_report])
        if hint_error is not None:
            raise ValueError(hint_error)
        if primary.get('raw_command_journal') is not None:
            journal = primary['raw_command_journal']
            journal_path = profile.primary_journal_path(primary, args.primary_report)
            if protected.get(journal_path) != journal['sha256'] or journal_path.stat().st_size != journal['bytes']:
                raise ValueError('Primary raw journal differs from its exact byte/hash binding')
            report['primary']['raw_command_journal'] = dict(journal)
        read_input_json(args.metadata)
        metadata = benchmark.load_metadata(args.metadata)
        report['metadata'] = metadata
        source_hashes = profile.bind_primary(primary, metadata, sources)
        report['source_boundary_hash_bytes_read'] = sum(source.stat().st_size for source in sources.values())
        protected.update({sources[label]: source_hashes[label] for label in benchmark.LABELS})
        if any(not path.is_file() or not os.access(path, os.X_OK) for path in sources.values()):
            raise ValueError('Selected source executables are unavailable')
        with tempfile.TemporaryDirectory(prefix='tekai-startup-controls-') as temporary:
            work = Path(temporary).resolve()
            supervisor = ControlSupervisor(work, args.timeout, started + args.budget)
            try:
                run_epochs(report, args, sources, source_hashes, metadata, supervisor, work)
            finally:
                supervisor.close()
    except (OSError, ValueError, RuntimeError, subprocess.SubprocessError, RecursionError) as error:
        report['errors'].append(f'{type(error).__name__}: {error}'[-8192:])
    except KeyboardInterrupt:
        report['errors'].append('Startup controls interrupted; primary outcome remains unchanged')
    finally:
        if supervisor is not None:
            try:
                supervisor.close()
            except (OSError, ValueError, RuntimeError, subprocess.SubprocessError) as error:
                report['errors'].append('Owned diagnostic cleanup failed: ' + str(error)[-8192:])
        for path, expected in protected.items():
            try:
                current = benchmark.executable_sha256(path)
                hash_size = path.stat().st_size
            except OSError:
                current = None
            report.setdefault('protected_inputs', []).append({'path': str(path), 'sha256_start': expected,
                                                            'sha256_end': current, 'unchanged': current == expected})
            if path in sources.values() and current is not None:
                report['source_boundary_hash_bytes_read'] = report.get('source_boundary_hash_bytes_read', 0) + hash_size
            if path == args.primary_report:
                report['primary'].update(sha256_end=current, unchanged=current == expected)
            if current != expected:
                report['errors'].append('Protected primary/metadata/source changed or disappeared at ' + str(path))
        signal.signal(signal.SIGTERM, previous_handler)
        completion, _balance = observed_progress(report)
        complete = completion['schedule_complete']
        if not complete and not report['errors']:
            report['errors'].append('Fixed startup-control schedule is incomplete')
        report.update(status='error' if report['errors'] else 'complete',
                      elapsed_seconds=time.monotonic() - started, exit_code=int(bool(report['errors'])))
        persist(report, args.output, final=True)
    return report['exit_code']


if __name__ == '__main__':
    sys.exit(main())
