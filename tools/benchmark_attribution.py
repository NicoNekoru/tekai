#!/usr/bin/env python3
"""Failure-only, non-gating controls using the original paired executables."""

import argparse
import contextlib
import copy
import json
import math
import os
from pathlib import Path
import re
import shutil
import signal
import sys

import benchmark_runtime as benchmark

POLICY = {
    'pairs_per_case': 40, 'warmups_per_side': 2, 'min_sample_seconds': 1.0,
    'warm_cache_min_sample_seconds': 1.0, 'max_iterations': 256,
    'per_command_deadline_seconds': 30.0, 'whole_run_deadline_seconds': 900.0,
    'relative_threshold': 0.10, 'familywise_confidence': 0.95,
    'calibration_headroom': 2.0, 'noise_relative_mad_limit': 0.10,
    'order_bias_limit': 0.05,
}
BUILD_COMMAND = 'cargo build --release --locked --bin tekai --no-default-features'
SHA256 = re.compile(r'[0-9a-f]{64}\Z')
REVISION = re.compile(r'[0-9a-f]{40}\Z')
MAX_REPORT_BYTES = 64 * 1024 * 1024


def read_json(path):
    if path.stat().st_size > MAX_REPORT_BYTES:
        raise ValueError('Report exceeds the finite input limit')
    with path.open(encoding='utf-8') as handle:
        report = json.load(handle)
    require(isinstance(report, dict), 'Report must be a JSON object')
    return report


def digest(value):
    return isinstance(value, str) and SHA256.fullmatch(value) is not None


def positive(value):
    try:
        return type(value) in (int, float) and math.isfinite(value) and value > 0
    except OverflowError:
        return False


def records(value):
    return isinstance(value, list) and all(isinstance(row, dict) for row in value)


def verified_output(case, value):
    if not isinstance(value, dict):
        return False
    if case == 'image-compile':
        return value.get('every_decoded_pixel_verified') is True and value.get('pages') == 8 \
            and value.get('image_objects') == 16 and value.get('dimensions') == [1024, 1024] \
            and value.get('alpha') == 128 and value.get('rgb_by_page') == [[i * 7 % 256, i * 13 % 256, 90] for i in range(8)]
    expected = 'Ordinary nested lookup.' if case == 'nested-lookup' else 'Probe.'
    return value.get('text_verified') is True and value.get('expected_text') == expected


def valid_batch(batch, iterations, case):
    if not isinstance(batch, dict):
        return False
    commands = batch.get('commands')
    return records(commands) and len(commands) == iterations and type(batch.get('iterations')) is int \
        and batch['iterations'] == iterations \
        and all(positive(command.get('seconds')) and type(command.get('command_id')) is int
                and command['command_id'] > 0 for command in commands) and positive(batch.get('seconds')) \
        and verified_output(case, batch.get('output_validation')) \
        and math.isclose(batch['seconds'], sum(command['seconds'] for command in commands), rel_tol=1e-12)


def require(condition, reason):
    if not condition:
        raise ValueError(reason)


def validate_policy(report):
    require(isinstance(report.get('policy'), dict) and all(type(report['policy'].get(key)) in (int, float)
                and report['policy'][key] == value for key, value in POLICY.items()),
            'Reported policy differs from the fixed control policy')


def validate_cases(report, allow_infeasible=False):
    rows = report.get('results', [])
    require(records(rows) and [row.get('case') for row in rows] == list(benchmark.CASES),
            'Completed case inventory is incomplete')
    outcomes = []
    for case_index, row in enumerate(rows):
        start, end = row.get('fixture_sha256'), row.get('fixture_sha256_end')
        require(isinstance(start, dict) and row.get('fixture_unchanged') is True
                and set(start) == set(benchmark.LABELS) and start == end
                and all(digest(value) for value in start.values()) and len(set(start.values())) == 1,
                'Completed fixture integrity is invalid')
        samples, iterations = row.get('samples'), row.get('iterations_per_batch')
        warmups, initial = row.get('warmups'), row.get('initialization')
        require(records(samples) and records(warmups) and len(warmups) == 2
                and type(row.get('min_sample_seconds')) in (int, float) and row['min_sample_seconds'] == 1.0,
                'Completed case sampling configuration differs')
        require(isinstance(initial, dict) and set(initial) == set(benchmark.LABELS)
                and all(isinstance(value, dict) and positive(value.get('seconds'))
                        and type(value.get('command_id')) is int and value['command_id'] > 0
                        and verified_output(row['case'], value.get('output_validation')) for value in initial.values()),
                'Completed initialization is incomplete')
        for index, warmup in enumerate(warmups):
            require(warmup.get('order') == list(benchmark.paired_order(index, case_index))
                    and all(valid_batch(warmup.get(label), 1, row['case']) for label in benchmark.LABELS),
                    'Completed warmups are incomplete')
        try:
            calibrated = benchmark.calibrated_iterations(warmups, 1.0, 256)
        except benchmark.CalibrationInfeasible as error:
            require(allow_infeasible and samples == [] and iterations is None
                    and row.get('calibration_infeasible') == error.details
                    and isinstance(row.get('analysis'), dict) and row['analysis'].get('status') == 'inconclusive',
                    'Completed calibration is infeasible or inconsistent')
            outcomes.append('inconclusive')
            continue
        require(len(samples) == 40 and type(iterations) is int and iterations == calibrated
                and 'calibration_infeasible' not in row, 'Completed case batches differ from calibration')
        for index, sample in enumerate(samples):
            require(type(sample.get('pair_index')) is int and sample['pair_index'] == index
                    and type(sample.get('iterations')) is int and sample['iterations'] == iterations
                    and sample.get('order') == list(benchmark.paired_order(index, case_index)),
                    'Completed sample inventory is incomplete')
            require(all(valid_batch(sample.get(label), iterations, row['case']) for label in benchmark.LABELS),
                    'Completed paired batches are incomplete')
        analysis = benchmark.analyze_pairs(samples, 0.10, 1.0)
        require(isinstance(row.get('analysis'), dict) and row['analysis'].get('status') == analysis['status'],
                'Completed sample analysis differs')
        outcomes.append(analysis['status'])
    overall = 'fail' if 'fail' in outcomes else ('inconclusive' if 'inconclusive' in outcomes else 'pass')
    require(report.get('status') == overall, 'Completed overall outcome differs')


def eligible(primary, metadata, sources, revisions, profile):
    """Validate completed evidence, without changing its verdict or statistics."""
    require(isinstance(primary, dict), 'Report must be a JSON object')
    require(profile == 'full', 'Attribution is limited to the full profile')
    require(primary.get('schema_version') == 2 and primary.get('gate') is True,
            'Requires a primary statistical gate report')
    status = primary.get('status')
    require(status in ('fail', 'inconclusive') and type(primary.get('exit_code')) is int
            and primary['exit_code'] == {'fail': 1, 'inconclusive': 2}.get(status),
            'Requires a completed non-green primary gate')
    require(primary.get('errors') == [] and positive(primary.get('elapsed_seconds')),
            'Primary execution errors or incomplete timing prevent attribution')
    require(primary.get('metadata') == metadata, 'Primary provenance does not match its metadata file')
    require(metadata.get('release_settings') == {'codegen_units': 1, 'lto': 'fat', 'panic': 'abort'}
            and type(metadata['release_settings']['codegen_units']) is int,
            'Primary release settings differ')
    validate_policy(primary)
    for label in benchmark.LABELS:
        side = metadata.get(label, {})
        require(isinstance(revisions[label], str) and REVISION.fullmatch(revisions[label]) is not None
                and side.get('revision') == revisions[label]
                and side.get('build_command') == BUILD_COMMAND, 'Primary revision/build provenance differs')
    require(revisions['baseline'] != revisions['candidate'], 'The primary must compare distinct revisions')
    protected = {}
    executables = primary.get('executables', [])
    require(records(executables) and len(executables) == 2
            and {row.get('label') for row in executables} == set(benchmark.LABELS),
            'Primary executable inventory is incomplete')
    for row in executables:
        label = row['label']
        hashes = [row.get(key) for key in ('source_sha256_start', 'source_sha256_end', 'sha256_start', 'sha256_end')]
        require(row.get('unchanged') is True and all(digest(value) for value in hashes) and len(set(hashes)) == 1,
                'Primary executable integrity is invalid')
        require(Path(row.get('source_path', '')).resolve() == sources[label]
                and sources[label].is_file() and os.access(sources[label], os.X_OK),
                'Primary executable source path differs')
        protected[sources[label]] = hashes[0]
    verifiers = primary.get('output_verifiers', {})
    require(isinstance(verifiers, dict) and set(verifiers) == {'pdftotext', 'pdfinfo', 'pdfimages'}
            and all(isinstance(row, dict) for row in verifiers.values()), 'Primary Poppler inventory is incomplete')
    for name, row in verifiers.items():
        start, end = row.get('sha256_start'), row.get('sha256_end')
        current = shutil.which(name)
        require(row.get('unchanged') is True and digest(start) and start == end and current is not None
                and Path(current).resolve() == Path(row.get('path', '')).resolve(),
                'Primary Poppler integrity/path differs')
        protected[Path(current).resolve()] = start
    validate_cases(primary)
    return protected


def verify_current(protected, verifiers):
    for path, expected in protected.items():
        require(benchmark.executable_sha256(path) == expected, 'Protected source/report/verifier changed: ' + str(path))
    for name, row in verifiers.items():
        current = shutil.which(name)
        require(current is not None and Path(current).resolve() == Path(row['path']).resolve(),
                'Poppler selection changed: ' + name)


def control_metadata(metadata, kind, primary_path):
    result = copy.deepcopy(metadata)
    if kind == 'reversed':
        result['baseline'], result['candidate'] = copy.deepcopy(metadata['candidate']), copy.deepcopy(metadata['baseline'])
    else:
        result['baseline'] = copy.deepcopy(metadata['candidate'])
        result['candidate'] = copy.deepcopy(metadata['candidate'])
    result.update(baseline_policy='diagnostic-' + kind, diagnostic_only=True, used_for_primary_gate=False,
                  diagnostic_kind=kind, primary_report=str(primary_path), original_provenance=copy.deepcopy(metadata))
    return result


def stop_reason(report, selected, expected):
    errors = report.get('errors', [])
    require(isinstance(errors, list) and all(isinstance(error, str) for error in errors), 'Control errors are malformed')
    for error in errors:
        if any(text in error for text in ('Run interrupted', 'Cleanup failed:', 'owned-group cleanup', 'TimeoutExpired',
                                         'changed', 'disappeared', 'removed benchmark fixture',
                                         'fixtures do not have equal content')):
            return error
    executables, verifiers, fixtures = report.get('executables', []), report.get('output_verifiers', {}), report.get('results', [])
    require(records(executables) and isinstance(verifiers, dict)
            and all(isinstance(row, dict) for row in verifiers.values()) and records(fixtures),
            'Control integrity inventories are malformed')
    if report.get('status') != 'error':
        require(len(executables) == 2 and {row.get('label') for row in executables} == set(benchmark.LABELS)
                and set(verifiers) == {'pdftotext', 'pdfinfo', 'pdfimages'}, 'Control integrity inventories are incomplete')
        require(errors == [] and positive(report.get('elapsed_seconds')), 'Completed control has errors or no elapsed time')
        validate_cases(report, allow_infeasible=True)
    for row in executables:
        hashes = [row.get(key) for key in ('source_sha256_start', 'source_sha256_end', 'sha256_start', 'sha256_end')]
        label = row.get('label')
        if label not in selected or row.get('unchanged') is not True or not all(digest(value) for value in hashes) \
                or len(set(hashes)) != 1 or hashes[0] != expected[selected[label]] \
                or Path(row.get('source_path', '')).resolve() != selected[label]:
            return 'Control executable integrity failed'
    for name, row in verifiers.items():
        path = Path(row.get('path', '')).resolve()
        if name not in ('pdftotext', 'pdfinfo', 'pdfimages') or row.get('unchanged') is not True \
                or not digest(row.get('sha256_start')) or row['sha256_start'] != row.get('sha256_end') \
                or row['sha256_start'] != expected.get(path) or shutil.which(name) is None \
                or Path(shutil.which(name)).resolve() != path:
            return 'Control Poppler integrity failed'
    for row in fixtures:
        start, end = row.get('fixture_sha256'), row.get('fixture_sha256_end')
        if not isinstance(start, dict) or set(start) != set(benchmark.LABELS) \
                or not all(digest(value) for value in start.values()) or len(set(start.values())) != 1 \
                or ('fixture_sha256_end' in row and (start != end or row.get('fixture_unchanged') is not True)) \
                or row.get('fixture_unchanged') is False:
            return 'Control fixture integrity failed'
    return None


def persist(summary, directory):
    (directory / 'attribution.json').write_text(json.dumps(summary, indent=2, allow_nan=False) + '\n', encoding='utf-8')
    lines = ['# Non-gating attribution controls', '',
             'Primary failure remains authoritative. These controls cannot replace or pass the primary gate.', '',
             'Attribution status `' + summary['status'] + '`.', '', summary.get('reason', ''), '',
             '| Control | Semantic outcome | Exit code |', '| --- | --- | ---: |']
    for row in summary['controls']:
        lines.append('| ' + row['kind'] + ' | ' + row['status'] + ' | ' + str(row['exit_code']) + ' |')
    (directory / 'attribution.md').write_text('\n'.join(lines) + '\n', encoding='utf-8')


def run_attribution(primary_path, metadata_path, sources, revisions, directory, profile='full', runner=None):
    """Run two advisory controls. The benchmark owns all native processes."""
    directory = Path(directory)
    if directory.exists() or directory.is_symlink():
        raise FileExistsError('Attribution output directory already exists or is a symlink')
    primary_path, metadata_path, directory = (Path(path).resolve() for path in (primary_path, metadata_path, directory))
    sources = {label: Path(path).resolve() for label, path in sources.items()}
    # Claim a new directory exclusively. Existing files, directories and aliases
    # are never reused, including any primary report or selected executable.
    directory.mkdir(exist_ok=False)
    summary = {'schema_version': 1, 'diagnostic_only': True, 'used_for_primary_gate': False,
               'status': 'skipped', 'controls': [], 'primary_report': str(primary_path)}
    try:
        primary, metadata = read_json(primary_path), benchmark.load_metadata(metadata_path)
        summary['primary_outcome'] = {
            'status': primary.get('status') if isinstance(primary.get('status'), str) else 'invalid',
            'exit_code': primary.get('exit_code') if type(primary.get('exit_code')) is int else None,
        }
        protected = eligible(primary, metadata, sources, revisions, profile)
        input_paths = [primary_path, metadata_path]
        if primary_path.with_suffix('.md').exists():
            input_paths.append(primary_path.with_suffix('.md'))
        protected.update({path: benchmark.executable_sha256(path) for path in input_paths})
        verify_current(protected, primary['output_verifiers'])
    except (OSError, ValueError, KeyError, TypeError, OverflowError) as error:
        summary['reason'] = str(error)
        persist(summary, directory)
        return 0
    summary['status'] = 'running'
    persist(summary, directory)
    previous_handler = signal.getsignal(signal.SIGTERM)

    def terminate(_number, _frame):
        raise KeyboardInterrupt

    signal.signal(signal.SIGTERM, terminate)
    try:
        for kind in ('reversed', 'candidate-self'):
            try:
                verify_current(protected, primary['output_verifiers'])
                metadata_file = directory / (kind + '-metadata.json')
                provenance = control_metadata(metadata, kind, primary_path)
                selected = {'baseline': sources['candidate'], 'candidate': sources['baseline']} if kind == 'reversed' \
                    else {'baseline': sources['candidate'], 'candidate': sources['candidate']}
                output = directory / (kind + '.json')
                log_file = directory / (kind + '.log')
                require(all(not path.exists() and not path.is_symlink()
                            for path in (metadata_file, output, output.with_suffix('.md'), log_file)),
                        'Control destinations already exist or alias inputs')
                with metadata_file.open('x', encoding='utf-8') as handle:
                    handle.write(json.dumps(provenance, indent=2) + '\n')
                protected[metadata_file] = benchmark.executable_sha256(metadata_file)
                argv = ['--baseline', str(selected['baseline']), '--candidate', str(selected['candidate']),
                        '--metadata', str(metadata_file), '--output', str(output), '--pairs', '40', '--warmups', '2',
                        '--timeout', '30', '--budget', '900', '--threshold', '0.10', '--min-sample-seconds', '1.0',
                        '--warm-cache-min-sample-seconds', '1.0', '--max-iterations', '256']
                row = {'kind': kind, 'status': 'error', 'exit_code': None}
                summary['controls'].append(row)
                with log_file.open('x', encoding='utf-8') as log, \
                        contextlib.redirect_stdout(log), contextlib.redirect_stderr(log):
                    try:
                        row['exit_code'] = (runner or benchmark.main)(argv)
                    except SystemExit as error:
                        row['exit_code'] = error.code if type(error.code) is int else 1
                        print('Benchmark invocation exited: ' + str(error))
                    except Exception as error:
                        row['exit_code'] = 1
                        row['error'] = type(error).__name__ + ': ' + str(error)
                        print(row['error'])
                if type(row['exit_code']) is not int:
                    row['invalid_exit_code'] = repr(row['exit_code'])
                    row['exit_code'] = None
                    raise ValueError('Control runner returned a non-integer exit code')
                control = read_json(output) if output.exists() else {'status': 'error', 'errors': [row.get('error', 'Control report missing')]}
                require(control.get('status') in ('pass', 'fail', 'inconclusive', 'error')
                        and isinstance(control.get('errors', []), list)
                        and all(isinstance(error, str) for error in control.get('errors', [])),
                        'Control outcome is malformed')
                row.update(status=control['status'], reported_status=control['status'], errors=list(control.get('errors', [])))
                if row.get('error') and row['error'] not in row['errors']:
                    row['errors'].append(row['error'])
                    row['status'] = 'error'
                persist(summary, directory)
                verify_current(protected, primary['output_verifiers'])
                if output.exists():
                    require(control.get('schema_version') == 2 and control.get('gate') is False
                            and control.get('metadata') == provenance, 'Control report provenance/gate differs')
                    validate_policy(control)
                    require(positive(control.get('elapsed_seconds'))
                            and type(control.get('exit_code')) is int and type(row['exit_code']) is int
                            and control['exit_code'] == row['exit_code']
                            and row['exit_code'] == benchmark.comparison_exit(control['status'], False)
                            and bool(control.get('errors')) == (control['status'] == 'error'),
                            'Control completion or exit evidence differs')
                reason = stop_reason(dict(control, errors=row['errors']), selected, protected)
                if reason:
                    raise ValueError(reason)
                for path in (metadata_file, output, output.with_suffix('.md'), directory / (kind + '.log')):
                    if path.exists():
                        protected[path] = benchmark.executable_sha256(path)
            except (OSError, ValueError, KeyError, TypeError, OverflowError) as error:
                summary.update(status='stopped', reason=str(error))
                if summary['controls'] and summary['controls'][-1]['kind'] == kind:
                    summary['controls'][-1].update(status='invalid', reason=str(error))
                break
        else:
            summary['status'] = 'complete'
    except KeyboardInterrupt:
        summary.update(status='stopped', reason='Run interrupted; no further control launched')
        if summary['controls'] and summary['controls'][-1]['exit_code'] is None:
            summary['controls'][-1].update(status='interrupted', exit_code=130)
    finally:
        signal.signal(signal.SIGTERM, previous_handler)
        persist(summary, directory)
    return int(summary['status'] == 'stopped' or any(row['exit_code'] != 0 or row['status'] == 'error' for row in summary['controls']))


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('primary-report', 'primary-metadata', 'baseline', 'candidate', 'output-dir'):
        parser.add_argument('--' + name, type=Path, required=True)
    for name in ('baseline-revision', 'candidate-revision', 'profile'):
        parser.add_argument('--' + name, required=True)
    args = parser.parse_args(argv)
    try:
        return run_attribution(args.primary_report, args.primary_metadata,
                               {label: getattr(args, label) for label in benchmark.LABELS},
                               {label: getattr(args, label + '_revision') for label in benchmark.LABELS},
                               args.output_dir, args.profile)
    except (OSError, ValueError) as error:
        print('Attribution setup failed: ' + str(error), file=sys.stderr)
        return 1


if __name__ == '__main__':
    sys.exit(main())
