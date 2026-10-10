#!/usr/bin/env python3
"""Owned target rotation and provenance for same-path CI builds. No subprocesses."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import sys

COMMAND = 'cargo build --release --locked --bin tekai --no-default-features'
SETTINGS = {'codegen_units': 1, 'lto': 'fat', 'panic': 'abort'}
BUILD_ENV = ('RUSTUP_TOOLCHAIN', 'CARGO_INCREMENTAL', 'CARGO_BUILD_JOBS',
             'CARGO_PROFILE_RELEASE_CODEGEN_UNITS', 'CARGO_PROFILE_RELEASE_LTO',
             'CARGO_PROFILE_RELEASE_PANIC', 'RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS')
REVISION = re.compile(r'[0-9a-f]{40}\Z')
HASH = re.compile(r'[0-9a-f]{64}\Z')


def require(condition, message):
    if not condition:
        raise ValueError(message)


def read(path):
    require(not path.is_symlink() and path.is_file() and path.stat().st_size <= 1024 * 1024,
            'Missing, aliased or oversized build record: ' + str(path))
    value = json.loads(path.read_text(encoding='utf-8'))
    require(isinstance(value, dict), 'Build record must be an object')
    return value


def write(path, value):
    with path.open('x', encoding='utf-8') as handle:
        handle.write(json.dumps(value, indent=2, allow_nan=False) + '\n')


def sha256(path):
    result = hashlib.sha256()
    with path.open('rb') as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b''):
            result.update(chunk)
    return result.hexdigest()


def absent(path):
    return not path.exists() and not path.is_symlink()


class Session:
    def __init__(self, report, config):
        self.report = Path(report)
        self.config = config
        self.workspace = Path(config['build_workspace'])
        self.target = Path(config['build_target_dir'])
        self.frozen = Path(config['frozen_target_dir'])
        self.root = Path(config['runner_temp'])
        require(self.report == self.root / 'tekai-performance-reports'
                and self.target == self.root / 'tekai-performance-target'
                and self.frozen == self.root / 'tekai-performance-baseline-target',
                'Targets/reports must be the exact owned runner-temp siblings')
        require(self.root.is_dir() and self.root.resolve() == self.root
                and self.report.is_dir() and self.report.resolve() == self.report
                and self.workspace.is_dir() and self.workspace.resolve() == self.workspace,
                'Build directories must be canonical existing directories')
        require(all(isinstance(config.get(label), str) and REVISION.fullmatch(config[label])
                    for label in ('baseline', 'candidate')) and config['baseline'] != config['candidate'],
                'Expected build revisions must be distinct exact SHAs')

    @classmethod
    def prepare(cls, report, root, workspace, target, frozen, baseline, candidate, current,
                runner_label, baseline_policy):
        require(current == candidate, 'Initial checkout is not the exact candidate')
        config = dict(runner_temp=str(Path(root).resolve()), build_workspace=str(Path(workspace).resolve()),
                      build_target_dir=str(Path(target)), frozen_target_dir=str(Path(frozen)),
                      baseline=baseline, candidate=candidate, runner_label=runner_label,
                      baseline_policy=baseline_policy)
        result = cls(Path(report), config)
        require(absent(result.target) and absent(result.frozen), 'Build targets are not fresh')
        write(result.report / 'build-session.json', config)
        return result

    @classmethod
    def load(cls, report):
        report = Path(report)
        return cls(report, read(report / 'build-session.json'))

    def record(self, role, suffix):
        require(role in ('baseline', 'candidate'), 'Unknown build role')
        return self.report / (role + '-' + suffix + '.json')

    def check_cwd(self):
        require(Path.cwd().resolve() == self.workspace, 'Build workspace changed')

    def executable_hash(self, target):
        path = target / 'release' / 'tekai'
        require(path.resolve() == path and path.is_file() and os.access(path, os.X_OK),
                'Build executable is missing, aliased or not executable')
        return sha256(path)

    def start(self, role, revision, toolchain, environment, source_clean=True):
        self.check_cwd()
        require(revision == self.config[role], 'Restored checkout revision differs')
        require(source_clean is True, 'Checkout contains source changes or untracked residue')
        require(absent(self.target), 'Canonical target is not fresh')
        if role == 'baseline':
            require(absent(self.frozen), 'Frozen target destination already exists')
        else:
            require(read(self.report / 'target-isolation.json').get('candidate_target_fresh') is True,
                    'Baseline target isolation did not complete')
        require(isinstance(toolchain, str) and bool(toolchain.strip()), 'Missing actual toolchain')
        require(environment.get('CARGO_PROFILE_RELEASE_CODEGEN_UNITS') == '1'
                and environment.get('CARGO_PROFILE_RELEASE_LTO') == 'fat'
                and environment.get('CARGO_PROFILE_RELEASE_PANIC') == 'abort'
                and environment.get('CARGO_INCREMENTAL') == '0', 'Release settings differ')
        record = dict(revision=revision, build_command=COMMAND, build_workspace=str(self.workspace),
                      build_target_dir=str(self.target), toolchain=toolchain.strip(), release_settings=SETTINGS,
                      source_clean_before_build=True,
                      build_environment={key: environment.get(key, '') for key in BUILD_ENV})
        write(self.record(role, 'start'), record)
        return record

    def finish(self, role, revision, cargo_exit, log_exit, source_clean=True):
        self.check_cwd()
        record = read(self.record(role, 'start'))
        require(all(type(value) is int and value >= 0 for value in (cargo_exit, log_exit)),
                'Invalid pipeline exit status')
        record.update(cargo_exit=cargo_exit, log_exit=log_exit,
                      command_finished=cargo_exit < 128 and log_exit < 128,
                      checkout_revision_after_build=revision, source_clean_after_build=source_clean, successful=False)
        if cargo_exit == log_exit == 0 and revision == record['revision'] == self.config[role] and source_clean is True:
            try:
                record['artifact_sha256'] = self.executable_hash(self.target)
                record['successful'] = True
            except (OSError, ValueError) as error:
                record['error'] = str(error)
        elif revision != record['revision']:
            record['error'] = 'Checkout changed during build'
        write(self.record(role, 'finished'), record)
        return record

    def isolate(self):
        """Rotate a normally finished attempt, even if Cargo failed. Never delete."""
        self.check_cwd()
        state = {'candidate_target_fresh': False, 'baseline_verified': False, 'target_moved': False}
        try:
            require(absent(self.frozen), 'Frozen target destination already exists or is a symlink')
            start_path, finish_path = self.record('baseline', 'start'), self.record('baseline', 'finished')
            if absent(start_path):
                require(absent(finish_path) and absent(self.target), 'Unowned target or finish record without a start')
            else:
                start, finished = read(start_path), read(finish_path)
                require(finished.get('command_finished') is True
                        and all(type(finished.get(key)) is int and 0 <= finished[key] < 128
                                for key in ('cargo_exit', 'log_exit')),
                        'Baseline timeout/signal or missing normal completion prevents target rotation')
                require(all(start.get(key) == finished.get(key) for key in start)
                        and start.get('revision') == self.config['baseline']
                        and start.get('build_workspace') == str(self.workspace)
                        and start.get('build_target_dir') == str(self.target), 'Baseline start/finish provenance differs')
                require(absent(self.target) or (self.target.is_dir() and self.target.resolve() == self.target),
                        'Canonical target is not an owned regular directory')
                if self.target.exists():
                    self.target.rename(self.frozen)
                    state['target_moved'] = True
                if finished.get('successful') is True:
                    try:
                        expected = finished.get('artifact_sha256')
                        require(isinstance(expected, str) and HASH.fullmatch(expected)
                                and self.executable_hash(self.frozen) == expected, 'Frozen baseline executable hash differs')
                        state['baseline_verified'] = True
                    except (OSError, ValueError) as error:
                        # A bad baseline cannot pass metadata, but the target was
                        # removed normally and candidate correctness can still run.
                        state['baseline_error'] = str(error)
            require(absent(self.target), 'Canonical target still exists after rotation')
            state['candidate_target_fresh'] = True
        except (OSError, ValueError) as error:
            state['error'] = str(error)
            write(self.report / 'target-isolation.json', state)
            raise
        write(self.report / 'target-isolation.json', state)
        return state

    def metadata(self):
        self.check_cwd()
        baseline, candidate = (read(self.record(role, 'finished')) for role in ('baseline', 'candidate'))
        isolation = read(self.report / 'target-isolation.json')
        require(isolation.get('candidate_target_fresh') is True and isolation.get('baseline_verified') is True
                and isolation.get('target_moved') is True, 'A verified frozen baseline is unavailable')
        for role, record, target in (('baseline', baseline, self.frozen), ('candidate', candidate, self.target)):
            require(record.get('successful') is True and record.get('command_finished') is True
                    and all(type(record.get(key)) is int and record[key] == 0 for key in ('cargo_exit', 'log_exit'))
                    and record.get('revision') == record.get('checkout_revision_after_build') == self.config[role]
                    and record.get('source_clean_before_build') is True and record.get('source_clean_after_build') is True
                    and record.get('build_command') == COMMAND and record.get('release_settings') == SETTINGS
                    and record.get('build_workspace') == str(self.workspace)
                    and record.get('build_target_dir') == str(self.target)
                    and self.executable_hash(target) == record.get('artifact_sha256'),
                    'Current ' + role + ' artifact/provenance differs')
            record['build_artifact_path'] = str(self.target / 'release' / 'tekai')
            record['artifact_path'] = str(target / 'release' / 'tekai')
        require(baseline['toolchain'] == candidate['toolchain']
                and baseline['build_environment'] == candidate['build_environment'], 'Build toolchain/environment differs')
        return dict(runner_label=self.config['runner_label'], toolchain=candidate['toolchain'],
                    baseline_policy=self.config['baseline_policy'], release_settings=SETTINGS,
                    build_path_policy='same-canonical-workspace-and-target-fresh-per-side',
                    baseline=baseline, candidate=candidate)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--report-dir', type=Path, required=True)
    sub = parser.add_subparsers(dest='operation', required=True)
    prepare = sub.add_parser('prepare')
    prepare.add_argument('--revision', required=True)
    for operation in ('start', 'finish'):
        command = sub.add_parser(operation)
        command.add_argument('--role', choices=('baseline', 'candidate'), required=True)
        command.add_argument('--revision', required=True)
        command.add_argument('--source-clean', choices=('true', 'false'), required=True)
        if operation == 'start':
            command.add_argument('--toolchain-file', type=Path, required=True)
        else:
            command.add_argument('--cargo-exit', type=int, required=True)
            command.add_argument('--log-exit', type=int, required=True)
    sub.add_parser('isolate')
    sub.add_parser('metadata')
    args = parser.parse_args(argv)
    try:
        if args.operation == 'prepare':
            Session.prepare(args.report_dir, os.environ['RUNNER_TEMP'], os.environ['GITHUB_WORKSPACE'],
                            os.environ['CARGO_TARGET_DIR'], os.environ['BASELINE_TARGET_DIR'],
                            os.environ['BASELINE_SHA'], os.environ['CANDIDATE_SHA'], args.revision,
                            os.environ['RUNNER_LABEL'], os.environ['BASELINE_POLICY'])
        else:
            session = Session.load(args.report_dir)
            if args.operation == 'start':
                require(args.toolchain_file.stat().st_size <= 1024 * 1024, 'Toolchain record is oversized')
                session.start(args.role, args.revision, args.toolchain_file.read_text(encoding='utf-8'), os.environ,
                              args.source_clean == 'true')
            elif args.operation == 'finish':
                result = session.finish(args.role, args.revision, args.cargo_exit, args.log_exit, args.source_clean == 'true')
                require(result['successful'], 'Build did not complete successfully; normal-exit evidence retained')
            elif args.operation == 'isolate':
                session.isolate()
            else:
                write(args.report_dir / 'comparison-metadata.json', session.metadata())
        return 0
    except (OSError, ValueError, KeyError, TypeError) as error:
        print('Build isolation/provenance failed: ' + str(error), file=sys.stderr)
        return 1


if __name__ == '__main__':
    sys.exit(main())
