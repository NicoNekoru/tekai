"""Correctness policy, isolation and subprocess supervision, without TeX."""

import contextlib
import hashlib
import io
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch
import zlib

import performance_ci as ci

NORMAL_ENGINE_ERROR = 'Error: TeX engine failed with status exit status: 1\nTeX log: /fixture/build/main.log\n'
MISSING_INPUT_ERROR = 'Error: TeX input missing.sty was not found; use --report-json to inspect its search paths\n'


def options(work, **values):
    defaults = dict(engine=Path(sys.executable), output=work / 'report.json', profile='quick',
                    timeout=1, expansion_engine=work / 'examples/audit_expansion',
                    strict_known_failures=False, fail_on_skip=False)
    defaults.update(values)
    return SimpleNamespace(**defaults)


def cache_evidence(**values):
    result = dict(next_build={'skipped': False, 'tex_runs': 1}, text='NEW-CONTENT-LONGER',
                  forced_text='NEW-CONTENT-LONGER', stale_cache_hit=False,
                  mtime_preserved=True, size_changed=True)
    result.update(values)
    return result


def jpeg_evidence(**values):
    result = dict(code=0, build_report={'skipped': False, 'tex_runs': 1}, pdf_complete=True,
                  embedded_image_dimensions=[1, 1], jpeg_bytes_embedded=True,
                  expected_dpi=[144, 72], natural_dimensions_sp=[32891, 65782])
    result.update(values)
    return result


class VerdictTests(unittest.TestCase):
    def status(self, case, **values):
        return ci.verdict(case, values)['status']

    def test_fixed_source_identity_is_an_assertion(self):
        self.assertEqual(self.status('input-identity', **cache_evidence()), 'passed')
        stale = cache_evidence(next_build={'skipped': True}, text='OLD-CONTENT', stale_cache_hit=True)
        self.assertEqual(self.status('input-identity', **stale), 'failed')

    def test_fixed_source_identity_checks_the_fixture(self):
        for key in ('mtime_preserved', 'size_changed'):
            with self.subTest(key=key):
                self.assertEqual(self.status('input-identity', **cache_evidence(**{key: False})), 'failed')

    def test_new_text_without_rebuild_does_not_pass(self):
        evidence = cache_evidence(next_build={'skipped': True, 'tex_runs': 0})
        self.assertEqual(self.status('input-identity', **evidence), 'failed')

    def test_database_replacement_is_a_fixed_gate(self):
        evidence = dict(next_build={'skipped': False}, text='NEW-CHOICE', forced_text='NEW-CHOICE',
                        located={'code': 0}, stale_cache_hit=False)
        self.assertEqual(self.status('database-identity', **evidence), 'passed')
        evidence.update(next_build={'skipped': True}, text='OLD-CHOICE', stale_cache_hit=True)
        self.assertEqual(self.status('database-identity', **evidence), 'failed')

    def test_known_lookup_failure_requires_a_working_forced_control(self):
        evidence = dict(next_build={'skipped': True}, text='OLD-CHOICE', forced_text='NEW-CHOICE',
                        located={'code': 0}, stale_cache_hit=True)
        self.assertEqual(self.status('lookup-precedence', **evidence), 'known_failure')
        evidence['forced_text'] = 'OLD-CHOICE'
        self.assertEqual(self.status('lookup-precedence', **evidence), 'failed')

    def test_known_failure_cannot_hide_a_resolver_failure(self):
        evidence = dict(next_build={'skipped': True}, text='OLD-CHOICE', forced_text='NEW-CHOICE',
                        located={'code': 1}, stale_cache_hit=True)
        self.assertEqual(self.status('lookup-precedence', **evidence), 'failed')

    def test_known_lookup_fix_is_an_unexpected_pass(self):
        evidence = dict(next_build={'skipped': False}, text='NEW-CHOICE', forced_text='NEW-CHOICE',
                        located={'code': 0}, stale_cache_hit=False)
        self.assertEqual(self.status('lookup-precedence', **evidence), 'unexpected_pass')

    def test_known_fixture_infrastructure_error_fails(self):
        for case in ci.KNOWN_ISSUES:
            with self.subTest(case=case):
                self.assertEqual(self.status(case, exception='read failed'), 'failed')

    def test_source_boundaries_require_distinct_mtime_and_forced_output(self):
        evidence = dict(next_build={'skipped': True}, text='OLD-CONTENT', forced_text='NEW-CONTENT',
                        mtime_changed=True, stale_cache_hit=True)
        self.assertEqual(self.status('source-boundaries', **evidence), 'known_failure')
        evidence['mtime_changed'] = False
        self.assertEqual(self.status('source-boundaries', **evidence), 'failed')

    def test_edit_race_requires_proof_of_input_consumption(self):
        evidence = dict(first_text='OLD-CONTENT', next_text='OLD-CONTENT', stale_cache_hit=True)
        self.assertEqual(self.status('edit-race', **evidence), 'known_failure')
        evidence['first_text'] = 'NEW-CONTENT'
        self.assertEqual(self.status('edit-race', **evidence), 'failed')

    def test_format_cache_requires_fresh_companion_control(self):
        evidence = dict(first_text='OLD-FORMAT', second_text='OLD-FORMAT', refreshed_text='NEW-FORMAT',
                        stale_raw_companion=True)
        self.assertEqual(self.status('format-cache', **evidence), 'known_failure')
        evidence['refreshed_text'] = 'OLD-FORMAT'
        self.assertEqual(self.status('format-cache', **evidence), 'failed')

    def test_cancellation_requires_child_and_parent_control(self):
        evidence = dict(child_started=True, parent_exited=True, child_survived=True)
        self.assertEqual(self.status('cancel', **evidence), 'known_failure')
        evidence['child_started'] = False
        self.assertEqual(self.status('cancel', **evidence), 'failed')

    def test_pdf_cycle_requires_a_controlled_parent_chain_rejection(self):
        evidence = dict(code=1, engine_log='! pdfTeX error: xpdf: cyclic PDF page Parent chain\n',
                        stderr_tail=NORMAL_ENGINE_ERROR)
        self.assertEqual(self.status('pdf-parent-cycle', **evidence), 'passed')
        self.assertNotIn('pdf-parent-cycle', ci.KNOWN_ISSUES)
        for changed in ({'timeout': True}, {'code': 0}, {'code': -6},
                        {'engine_log': 'xpdf: unrelated PDF error'}, {'engine_log': ''},
                        {'stderr_tail': 'Error: permission denied\n'},
                        {'stderr_tail': ''}):
            with self.subTest(changed=changed):
                self.assertEqual(self.status('pdf-parent-cycle', **{**evidence, **changed}), 'failed')

    def test_pdf_cycle_diagnostic_does_not_hide_wrapped_signals_or_panics(self):
        for status in ('signal: 6 (SIGABRT)', 'signal: 11 (SIGSEGV)', 'exit status: 101'):
            with self.subTest(status=status):
                stderr = f'Error: TeX engine failed with status {status}\n'
                self.assertEqual(self.status('pdf-parent-cycle', code=1,
                                            engine_log='xpdf: cyclic PDF page Parent chain',
                                            stderr_tail=stderr), 'failed')

    def test_pdf_cycle_diagnostic_must_be_in_the_engine_log(self):
        self.assertEqual(self.status('pdf-parent-cycle', code=1, engine_log='',
                                    stderr_tail=NORMAL_ENGINE_ERROR + 'xpdf: cyclic PDF page Parent chain\n'),
                         'failed')

    def test_pdf_cycle_diagnostic_accepts_tex_print_width_line_breaks(self):
        for diagnostic in ('xpdf: cyclic\n PDF page Parent chain',
                           'xpdf: cyclic \nPDF page Parent chain',
                           'xpdf: cyc\nlic PDF page Par\r\nent chain',
                           'xpd\r\nf: cyclic PDF page Parent chain'):
            with self.subTest(diagnostic=diagnostic):
                self.assertEqual(self.status('pdf-parent-cycle', code=1,
                                            engine_log='! pdfTeX error: ' + diagnostic + '\n ==> Fatal error\n',
                                            stderr_tail=NORMAL_ENGINE_ERROR), 'passed')

    def test_pdf_cycle_line_break_matching_preserves_exact_phrase_and_spaces(self):
        for diagnostic in ('xpdf: acyclic\n PDF page Parent chain',
                           'xpdf: cyclic\n unrelated PDF page Parent chain',
                           'xpdf: cyclic\nPDF page Parent chain',
                           'xpdf: cyclic  PDF page Parent chain',
                           'xpdf: cyclic PDF page Child chain'):
            with self.subTest(diagnostic=diagnostic):
                self.assertEqual(self.status('pdf-parent-cycle', code=1,
                                            engine_log=diagnostic, stderr_tail=NORMAL_ENGINE_ERROR), 'failed')

    def test_png_rejection_requires_normal_exit_and_specific_diagnostic(self):
        evidence = dict(code=1, expected_error='invalid PNG PLTE length', engine_log='invalid PNG PLTE length',
                        stderr_tail=NORMAL_ENGINE_ERROR)
        self.assertEqual(self.status('png-invalid-PLTE-769', **evidence), 'passed')
        for changed in ({'code': -6}, {'code': 0}, {'timeout': True}, {'engine_log': 'generic failure'}):
            with self.subTest(changed=changed):
                self.assertEqual(self.status('png-invalid-PLTE-769', **{**evidence, **changed}), 'failed')

    def test_large_palette_observation_still_asserts_rejection(self):
        self.assertEqual(self.status('png-palette-25165824', code=1, engine_log='invalid PNG PLTE length',
                                    stderr_tail=NORMAL_ENGINE_ERROR), 'passed')
        self.assertEqual(self.status('png-palette-25165824', code=0), 'failed')
        self.assertEqual(self.status('png-palette-3', code=0), 'passed')

    def test_tex_print_text_removes_only_line_breaks(self):
        self.assertEqual(ci._tex_print_text('one\n \r\ntwo\r\t  three'), 'one two\t  three')

    def test_png_expected_phrases_accept_print_width_breaks_at_every_character(self):
        cases = [('png-invalid-PLTE-769', 'invalid PNG PLTE length'),
                 ('png-invalid-tRNS-25165824', 'invalid or duplicate PNG tRNS length'),
                 ('png-palette-25165824', 'invalid PNG PLTE length')]
        for case, expected in cases:
            for newline in ('\n', '\r\n', '\r'):
                for index in range(len(expected) + 1):
                    with self.subTest(case=case, newline=newline, index=index):
                        diagnostic = expected[:index] + newline + expected[index:]
                        self.assertEqual(self.status(case, code=1, expected_error=expected,
                                                    engine_log='! pdfTeX error: ' + diagnostic,
                                                    stderr_tail=NORMAL_ENGINE_ERROR), 'passed')

    def test_png_print_width_matching_requires_the_exact_diagnostic(self):
        for diagnostic in ('invalid\n PNG tRNS length', 'invalid  PNG PLTE length',
                           'invalid\nPNG PLTE length', 'invalid PNG unrelated\n PLTE length',
                           'invalid PNG PLTE\n count'):
            with self.subTest(diagnostic=diagnostic):
                for case in ('png-invalid-PLTE-769', 'png-palette-25165824'):
                    self.assertEqual(self.status(case, code=1, expected_error='invalid PNG PLTE length',
                                                engine_log=diagnostic, stderr_tail=NORMAL_ENGINE_ERROR), 'failed')

    def test_png_framing_requires_normal_exit_and_the_exact_diagnostic_path(self):
        for fixture in ci._png_framing_fixtures():
            if 'expected_error' not in fixture:
                continue
            expected = fixture['expected_error']
            evidence = dict(code=1, stderr_tail=NORMAL_ENGINE_ERROR, expected_error=expected, engine_log=expected,
                            pdf_exists_after_failure=False)
            with self.subTest(case=fixture['name']):
                self.assertEqual(self.status(fixture['name'], **evidence), 'passed')
                for change in ({'code': 0}, {'code': -6}, {'timeout': True}, {'engine_log': 'unrelated PNG error'},
                               {'expected_error': ''}, {'stderr_tail': ''}, {'pdf_exists_after_failure': True},
                               {'engine_log': expected.replace('writepng: ', 'invalid PNG metadata: ')
                                if expected.startswith('writepng: ') else expected.replace('invalid PNG metadata: ', 'writepng: ')}):
                    self.assertEqual(self.status(fixture['name'], **{**evidence, **change}), 'failed')
                for child_status in ('signal: 6 (SIGABRT)', 'signal: 11 (SIGSEGV)', 'exit status: 101'):
                    self.assertEqual(self.status(fixture['name'], **{**evidence,
                        'stderr_tail': f'Error: TeX engine failed with status {child_status}\n'}), 'failed')
                for index in range(len(expected) + 1):
                    self.assertEqual(self.status(fixture['name'], **{**evidence,
                        'engine_log': expected[:index] + '\r\n' + expected[index:]}), 'passed')

    def test_png_multi_idat_control_requires_a_fresh_exact_copy(self):
        evidence = dict(code=0, build_report={'skipped': False, 'tex_runs': 1}, engine_log='(PNG copy)',
                        pdf_complete=True, embedded_image_dimensions=[1, 1], png_idat_stream_copied=True)
        self.assertEqual(self.status('png-copy-multiple-idat', **evidence), 'passed')
        for change in ({'code': 1}, {'code': -6}, {'timeout': True}, {'stdout_truncated': True},
                       {'pdf_truncated': True}, {'build_report': {'skipped': True, 'tex_runs': 0}},
                       {'build_report': {'skipped': False, 'tex_runs': True}},
                       {'build_report': {'skipped': False, 'tex_runs': 0}}, {'engine_log': '(PNG decoded)'},
                       {'pdf_complete': False}, {'embedded_image_dimensions': [2, 1]},
                       {'png_idat_stream_copied': False}):
            with self.subTest(change=change):
                self.assertEqual(self.status('png-copy-multiple-idat', **{**evidence, **change}), 'failed')

    def test_png_decode_rejections_require_the_exact_decoder_error_and_normal_exit(self):
        for fixture in ci._png_decode_fixtures():
            if 'expected_error' not in fixture:
                continue
            expected = fixture['expected_error']
            evidence = dict(code=1, stderr_tail=NORMAL_ENGINE_ERROR, expected_error=expected, engine_log=expected,
                            pdf_exists_after_failure=False)
            self.assertEqual(self.status(fixture['name'], **evidence), 'passed')
            for change in ({'code': 0}, {'code': -6}, {'timeout': True}, {'expected_error': ''},
                           {'engine_log': 'invalid PNG image data: unrelated error'}, {'pdf_exists_after_failure': True},
                           {'stderr_tail': 'Error: TeX engine failed with status exit status: 101\n'}):
                self.assertEqual(self.status(fixture['name'], **{**evidence, **change}), 'failed')
            for index in range(len(expected) + 1):
                self.assertEqual(self.status(fixture['name'], **{**evidence,
                    'engine_log': expected[:index] + '\r\n' + expected[index:]}), 'passed')

    def test_png_decode_controls_require_exact_pixels_and_a_fresh_complete_pdf(self):
        evidence = dict(code=0, build_report={'skipped': False, 'tex_runs': 1},
                        pdf_complete=True, decoded_pixels_exact=True)
        self.assertEqual(self.status('png-decode-valid-0', **evidence), 'passed')
        for change in ({'code': 1}, {'timeout': True}, {'pdf_truncated': True}, {'stdout_truncated': True},
                       {'build_report': {'skipped': True, 'tex_runs': 0}}, {'decoded_pixels_exact': False},
                       {'pdf_complete': False}):
            self.assertEqual(self.status('png-decode-valid-0', **{**evidence, **change}), 'failed')

    def test_jpeg_framing_requires_a_normal_exit_and_exact_wrappable_diagnostic(self):
        expected = 'reading JPEG image failed (premature file end)'
        evidence = dict(code=1, stderr_tail=NORMAL_ENGINE_ERROR, expected_error=expected,
                        engine_log='reading JPEG image failed (prema\nture file end)')
        self.assertEqual(self.status('jpeg-framing-signature-only', **evidence), 'passed')
        for changed in ({'code': 0}, {'code': -6}, {'timeout': True}, {'engine_log': 'unrelated error'},
                        {'engine_log': ''}, {'expected_error': ''}, {'stderr_tail': ''}):
            with self.subTest(changed=changed):
                self.assertEqual(self.status('jpeg-framing-signature-only', **{**evidence, **changed}), 'failed')
        for status in ('signal: 6 (SIGABRT)', 'signal: 11 (SIGSEGV)', 'exit status: 101'):
            with self.subTest(status=status):
                self.assertEqual(self.status('jpeg-framing-signature-only', **{
                    **evidence, 'stderr_tail': f'Error: TeX engine failed with status {status}\n'}), 'failed')

    def test_jpeg_embedding_requires_pixels_geometry_and_a_fresh_complete_pdf(self):
        self.assertEqual(self.status('jpeg-valid-dpi-little', **jpeg_evidence()), 'passed')
        for changed in ({'code': 1}, {'code': -6}, {'timeout': True}, {'stdout_truncated': True},
                        {'pdf_truncated': True}, {'pdf_complete': False}, {'jpeg_bytes_embedded': False},
                        {'embedded_image_dimensions': [2, 1]}, {'natural_dimensions_sp': [65782, 65782]},
                        {'natural_dimensions_sp': None}, {'build_report': {'skipped': True, 'tex_runs': 0}},
                        {'build_report': {'skipped': False, 'tex_runs': 0}}, {'expected_dpi': []}):
            with self.subTest(changed=changed):
                self.assertEqual(self.status('jpeg-valid-dpi-little', **jpeg_evidence(**changed)), 'failed')

    def test_jpeg_invalid_optional_metadata_has_a_strict_fallback_geometry_gate(self):
        evidence = jpeg_evidence(expected_dpi=[72, 72], natural_dimensions_sp=[65782, 65782])
        self.assertEqual(self.status('jpeg-invalid-ifd-offset', **evidence), 'passed')
        self.assertEqual(self.status('jpeg-invalid-ifd-offset', **{**evidence, 'code': 1,
                         'stderr_tail': NORMAL_ENGINE_ERROR}), 'failed')

    def test_preview_must_complete_prewarming_and_stay_alive(self):
        self.assertEqual(self.status('unicode-preview', prewarmed=True, alive=True), 'passed')
        self.assertEqual(self.status('unicode-preview', prewarmed=False, alive=True), 'failed')
        self.assertEqual(self.status('unicode-preview', prewarmed=True, alive=False), 'failed')

    def test_invalid_expansion_must_return_an_error_without_abort(self):
        self.assertEqual(self.status('expansion-invalid-input', code=0, stdout='expansion_error=overflow'), 'passed')
        self.assertEqual(self.status('expansion-invalid-input', code=-6, stdout='expansion_error=overflow'), 'failed')
        self.assertEqual(self.status('expansion-invalid-input', code=0, stdout='output=[]'), 'failed')

    def test_scope_output_is_deterministic_and_timing_is_ignored(self):
        for mode in ('read-only', 'local'):
            self.assertEqual(self.status('expansion-scopes', depth=64, mode=mode, code=0,
                                        stdout='output_tokens=129 expansion_ms=99999999'), 'observation')
        self.assertEqual(self.status('expansion-scopes', depth=64, code=0, stdout='output_tokens=1'), 'failed')

    def test_lookup_not_found_and_alias_semantics_are_assertions(self):
        self.assertEqual(self.status('symlink-dag', code=1, depth=16, stderr_tail=MISSING_INPUT_ERROR), 'passed')
        self.assertEqual(self.status('symlink-dag', timeout=True, depth=16), 'failed')
        self.assertEqual(self.status('symlink-dag', code=0), 'failed')
        self.assertEqual(self.status('symlink-alias-semantics', code=0, alias_found=True), 'passed')
        self.assertEqual(self.status('symlink-alias-semantics', code=0, alias_found=False), 'failed')

    def test_lint_checks_preserve_bytes_and_expect_correct_exit(self):
        self.assertEqual(self.status('format-many', code=1, source_unchanged=True), 'observation')
        self.assertEqual(self.status('format-many', code=0, source_unchanged=True), 'failed')
        self.assertEqual(self.status('lint-slashes', code=0, source_unchanged=False), 'failed')

    def test_small_formatter_oracle_checks_count_and_no_writes(self):
        evidence = dict(lines=4, fixes_available=8, warning_count=8, fixes_applied=0,
                        files_changed=[], source_unchanged=True, code=1)
        self.assertEqual(self.status('lint-correctness', **evidence), 'passed')
        self.assertEqual(self.status('lint-correctness', **{**evidence, 'fixes_available': 7}), 'failed')

    def test_cache_hit_requires_zero_tex_work(self):
        evidence = dict(first={'skipped': False}, second={'skipped': True, 'tex_runs': 0})
        self.assertEqual(self.status('runtime-cache-hit', **evidence), 'passed')
        evidence['second']['tex_runs'] = 1
        self.assertEqual(self.status('runtime-cache-hit', **evidence), 'failed')

    def test_open_scaling_costs_are_disclosed_without_fake_time_gates(self):
        result = ci.verdict('pdf-shared-resources-256', {'code': 0, 'seconds': 100000, 'peak_mib': 100000})
        self.assertEqual(result['status'], 'observation')
        self.assertIn('open_scaling_issue', result)

    def test_pdf_dictionary_is_a_fixed_gate_without_a_time_threshold(self):
        result = ci.verdict('pdf-dictionary-8192', {'code': 0, 'seconds': 100000, 'peak_mib': 100000})
        self.assertEqual(result['status'], 'passed')
        self.assertNotIn('open_scaling_issue', result)
        self.assertNotIn('pdf-dictionary', ci.OPEN_SCALING)
        self.assertEqual(self.status('pdf-dictionary-8192', code=1), 'failed')
        self.assertEqual(self.status('pdf-dictionary-8192', code=0, timeout=True), 'failed')

    def test_deep_input_error_requires_capacity_diagnostic(self):
        self.assertEqual(self.status('deep-inputs', code=1, phase='build', stdout='TeX capacity exceeded',
                                    stderr_tail=NORMAL_ENGINE_ERROR), 'passed')
        self.assertEqual(self.status('deep-inputs', code=1, phase='build', stdout='file not found'), 'failed')
        self.assertEqual(self.status('deep-inputs', code=-6, phase='check'), 'failed')

    def test_capacity_diagnostic_accepts_print_width_breaks_in_each_output(self):
        for diagnostic in ('TeX capacity\n exceeded', 'TeX capacity \r\nexceeded',
                           'TeX capac\nity excee\rded'):
            for output in ('stdout', 'stderr_tail'):
                with self.subTest(diagnostic=diagnostic, output=output):
                    evidence = dict(code=1, stderr_tail=NORMAL_ENGINE_ERROR)
                    evidence[output] = evidence.get(output, '') + diagnostic
                    self.assertEqual(self.status('deep-inputs', **evidence), 'passed')

    def test_capacity_diagnostic_does_not_join_distinct_output_streams(self):
        self.assertEqual(self.status('deep-inputs', code=1, stdout='capacity ',
                                    stderr_tail='exceeded\n' + NORMAL_ENGINE_ERROR), 'failed')

    def test_missing_lookup_exit_one_does_not_hide_unrelated_errors(self):
        for stderr in ('', 'Error: permission denied\n', 'Error: failed to open project directory\n',
                       'Error: TeX input different.sty was not found; use --report-json to inspect its search paths\n'):
            with self.subTest(stderr=stderr):
                self.assertEqual(self.status('symlink-dag', code=1, stderr_tail=stderr), 'failed')

    def test_png_cli_exit_one_does_not_hide_engine_signals_or_panics(self):
        for status in ('signal: 6 (SIGABRT)', 'signal: 11 (SIGSEGV)', 'exit status: 101', 'exit status: 10', ''):
            stderr = f'Error: TeX engine failed with status {status}\n'
            for diagnostic in ('invalid PNG PLTE length', 'invalid\n PNG PL\r\nTE length'):
                with self.subTest(status=status, diagnostic=diagnostic):
                    self.assertEqual(self.status('png-invalid-PLTE-769', code=1,
                                                expected_error='invalid PNG PLTE length',
                                                engine_log=diagnostic, stderr_tail=stderr), 'failed')
                    self.assertEqual(self.status('png-palette-25165824', code=1,
                                                engine_log=diagnostic, stderr_tail=stderr), 'failed')

    def test_deep_capacity_diagnostic_does_not_hide_wrapped_abort(self):
        stderr = 'Error: TeX engine failed with status signal: 6 (SIGABRT)\nTeX capacity exceeded\n'
        for phase in ('build', 'check'):
            with self.subTest(phase=phase):
                self.assertEqual(self.status('deep-inputs', code=1, phase=phase, stderr_tail=stderr), 'failed')

    def test_normal_engine_error_accepts_timer_suffix_but_requires_exact_status(self):
        timer = '1048576 maximum resident set size\n'
        self.assertTrue(ci.normal_engine_input_error({'code': 1, 'stderr_tail': NORMAL_ENGINE_ERROR + timer}))
        self.assertFalse(ci.normal_engine_input_error({'code': 1, 'stderr_tail': timer}))
        self.assertFalse(ci.normal_engine_input_error({'code': 1, 'timeout': True, 'stderr_tail': NORMAL_ENGINE_ERROR}))
        self.assertFalse(ci.normal_engine_input_error({'code': 1, 'stderr_tail': NORMAL_ENGINE_ERROR
                                                     + 'TeX engine failed with status signal: 6 (SIGABRT)\n'}))
        self.assertFalse(ci.normal_engine_input_error({'code': 1,
                                                     'stderr_tail': NORMAL_ENGINE_ERROR.replace('exit status', 'exit\n status')}))

    def test_skip_is_explicit(self):
        result = ci.verdict('input-identity', {'skipped': True, 'reason': 'pdftotext missing'})
        self.assertEqual(result['status'], 'skipped')
        self.assertEqual(result['reason'], 'pdftotext missing')

    def test_report_exit_policy_distinguishes_known_and_actual_failures(self):
        args = SimpleNamespace(strict_known_failures=False, fail_on_skip=False)
        report = {'results': [{'correctness': {'status': status}} for status in ('passed', 'known_failure', 'unexpected_pass', 'skipped')]}
        self.assertFalse(ci.failed_report(report, args))
        args.strict_known_failures = True
        self.assertTrue(ci.failed_report(report, args))
        args.strict_known_failures, args.fail_on_skip = False, True
        self.assertTrue(ci.failed_report(report, args))
        args.fail_on_skip = False
        report['results'].append({'correctness': {'status': 'failed'}})
        self.assertTrue(ci.failed_report(report, args))

    def test_timing_fields_are_removed_recursively(self):
        value = {'seconds': 3, 'next_build': {'elapsed_ms': 4, 'tex_runs': 1,
                 'passes': [{'tex_elapsed_ms': 2, 'aux_elapsed_ms': 3, 'draft': False}]}, 'peak_mib': 8}
        self.assertEqual(ci.without_timings(value), {'next_build': {'tex_runs': 1, 'passes': [{'draft': False}]}})

    def test_watch_initial_edit_waits_for_prewarming_completion(self):
        initial = 'built /fixture/build/main.pdf in 1s\n'
        warm = 'built /fixture/build/.tekai-hmr-warm/main.pdf in 1s\n'
        self.assertFalse(ci.watch_ready(initial, 0, initial=True))
        self.assertTrue(ci.watch_ready(initial + warm, 0, initial=True))
        self.assertFalse(ci.watch_ready(initial + warm, 2, initial=False))
        self.assertTrue(ci.watch_ready(initial + warm + initial, 2, initial=False))

    def test_executable_integrity_is_a_fixed_assertion(self):
        unchanged = dict(unchanged=True, sha256_start='a' * 64, sha256_end='a' * 64)
        self.assertEqual(self.status('executable-integrity', **unchanged), 'passed')
        self.assertEqual(self.status('executable-integrity', **{**unchanged, 'unchanged': False}), 'failed')
        self.assertEqual(self.status('executable-integrity', unchanged=True, sha256_start=None), 'failed')


class HashTests(unittest.TestCase):
    def test_hash_reads_fixed_size_chunks(self):
        payload = b'x' * (2 * ci.HASH_CHUNK_BYTES + 17)
        handle = Mock(wraps=io.BytesIO(payload))
        with patch('performance_ci.Path.open', return_value=contextlib.nullcontext(handle)):
            digest = ci.executable_sha256(Path('/unused/fixture'))
        self.assertEqual(digest, hashlib.sha256(payload).hexdigest())
        self.assertEqual(handle.read.call_count, 4)
        self.assertTrue(all(call.args == (ci.HASH_CHUNK_BYTES,) for call in handle.read.call_args_list))


class PNGFixtureTests(unittest.TestCase):
    def test_decode_fixtures_have_complete_small_chunks_and_specific_data_errors(self):
        fixtures = ci._png_decode_fixtures()
        self.assertEqual(len(fixtures), 12)
        self.assertEqual(len({row['name'] for row in fixtures}), 12)
        self.assertLess(max(len(row['content']) for row in fixtures), 128)
        for row in fixtures:
            content, position, chunks = row['content'], 8, []
            while position < len(content):
                length = int.from_bytes(content[position:position + 4], 'big')
                kind = content[position + 4:position + 8]
                payload = content[position + 8:position + 8 + length]
                crc = int.from_bytes(content[position + 8 + length:position + 12 + length], 'big')
                actual = zlib.crc32(kind + payload)
                self.assertEqual(len(payload), length)
                self.assertEqual(crc, actual ^ (0x01000000 if kind == b'IDAT' and '-crc-' in row['name'] else 0))
                chunks.append((kind, payload))
                position += length + 12
            self.assertEqual(position, len(content))
            self.assertEqual([kind for kind, _ in chunks], [b'IHDR', b'IDAT', b'IEND'])
            color = int(row['name'].rsplit('-', 1)[1])
            self.assertEqual(chunks[0][1][9], color)
            if '-zlib-' in row['name']:
                self.assertEqual(chunks[1][1], bytes(2))
                self.assertEqual(row['expected_error'], 'invalid PNG image data: Corrupt deflate stream. BadZlibHeader')
            elif '-short-scanline-' in row['name']:
                self.assertEqual(zlib.decompress(chunks[1][1]), b'\x00')
                self.assertIn('does not have enough data for image.', row['expected_error'])
            else:
                pixels = zlib.decompress(chunks[1][1])[1:]
                self.assertEqual(pixels, {0: b'\x0a', 2: b'\x0a\x14\x1e', 6: b'\x0a\x14\x1e\x80'}[color])
                if '-crc-' in row['name']:
                    self.assertIn('CRC error: expected 0x', row['expected_error'])
                    self.assertIn('type: IDAT,', row['expected_error'])

    def test_decoded_pdf_evidence_requires_exact_uncompressed_color_and_alpha_pixels(self):
        def image(number, color, pixels):
            return (f'{number} 0 obj\n<< /Subtype /Image /Width 1 /Height 1 /BitsPerComponent 8 /ColorSpace /Device{color} >>\nstream\n'.encode()
                    + pixels + b'\nendstream\nendobj\n')
        rgb = image(1, 'RGB', b'\x0a\x14\x1e')
        alpha = image(2, 'Gray', b'\x80')
        pdf = b'%PDF-1.5\n' + rgb + alpha + b'%%EOF\n'
        expected = [('rgb', [10, 20, 30]), ('gray', [128])]
        self.assertTrue(ci._png_decoded_pdf_evidence(pdf, expected)['decoded_pixels_exact'])
        for bad in (pdf.replace(b'\x80', b'\x00'), pdf.replace(b'/Width 1', b'/Width 2'),
                    pdf.replace(b'/BitsPerComponent 8', b'/BitsPerComponent 16'),
                    pdf.replace(b'/DeviceRGB', b'/DeviceGray'),
                    pdf.replace(b'/ColorSpace', b'/Filter /FlateDecode /ColorSpace'),
                    b'%PDF-1.5\n' + rgb + b'%%EOF', b'%PDF-1.5\n' + rgb + alpha + alpha + b'%%EOF'):
            self.assertFalse(ci._png_decoded_pdf_evidence(bad, expected)['decoded_pixels_exact'])
        self.assertFalse(ci._png_decoded_pdf_evidence(pdf[:-6], expected)['pdf_complete'])

    def test_framing_cases_are_finite_unique_and_declarations_have_no_large_payload(self):
        rows = ci._png_framing_fixtures()
        self.assertEqual(len(rows), 15)
        self.assertEqual(len({row['name'] for row in rows}), 15)
        self.assertLess(max(len(row['content']) for row in rows), 128)
        for stage in ('first', 'later'):
            for length in (0xfffffff4, 0x80000000, 0x7fffffff):
                row = next(row for row in rows if row['name'] == f'png-framing-idat-{stage}-{length:08x}')
                content = row['content']
                position = 33
                if stage == 'later':
                    first_length = int.from_bytes(content[position:position + 4], 'big')
                    self.assertEqual(content[position + 4:position + 8], b'IDAT')
                    self.assertEqual(zlib.decompress(content[position + 8:position + 8 + first_length]), bytes(4))
                    position += first_length + 12
                self.assertEqual(content[position:position + 8], length.to_bytes(4, 'big') + b'IDAT')
                self.assertEqual(len(content) - position, 8)
                prefix = 'invalid PNG metadata: ' if stage == 'first' else 'writepng: '
                message = 'invalid PNG chunk length' if length > 0x7fffffff else 'PNG chunk exceeds file extent'
                self.assertEqual(row['expected_error'], prefix + message)

    def test_short_framing_and_nonzero_iend_target_first_and_later_paths(self):
        rows = {row['name']: row for row in ci._png_framing_fixtures()}
        for stage in ('first', 'later'):
            for defect in ('short-payload', 'short-crc', 'short-header', 'nonzero-iend'):
                content = rows[f'png-framing-{defect}-{stage}']['content']
                position = 33
                if stage == 'later':
                    position += int.from_bytes(content[position:position + 4], 'big') + 12
                tail = content[position:]
                if defect == 'short-header':
                    self.assertEqual(len(tail), 5)
                else:
                    declared = int.from_bytes(tail[:4], 'big')
                    self.assertEqual(tail[4:8], b'IEND' if defect == 'nonzero-iend' else b'IDAT')
                    if defect == 'short-crc':
                        self.assertEqual(len(tail), declared + 11)
                    elif defect == 'short-payload':
                        self.assertEqual(declared, 10)
                        self.assertEqual(len(tail), 10)
                    else:
                        self.assertEqual(declared, 1)
                        self.assertEqual(len(tail), 13)

    def test_multi_idat_control_has_valid_complete_crc_and_one_compressed_scanline(self):
        row = ci._png_framing_fixtures()[-1]
        content, position, chunks = row['content'], 8, []
        self.assertEqual(content[:8], b'\x89PNG\r\n\x1a\n')
        while position < len(content):
            length = int.from_bytes(content[position:position + 4], 'big')
            kind = content[position + 4:position + 8]
            payload = content[position + 8:position + 8 + length]
            self.assertEqual(len(payload), length)
            self.assertEqual(int.from_bytes(content[position + 8 + length:position + 12 + length], 'big'),
                             zlib.crc32(kind + payload))
            chunks.append((kind, payload))
            position += length + 12
        self.assertEqual(position, len(content))
        self.assertEqual([kind for kind, _ in chunks], [b'IHDR', b'IDAT', b'IDAT', b'IEND'])
        encoded = b''.join(payload for kind, payload in chunks if kind == b'IDAT')
        self.assertTrue(all(payload for kind, payload in chunks if kind == b'IDAT'))
        self.assertEqual(encoded, row['expected_stream'])
        self.assertEqual(zlib.decompress(encoded), bytes(4))
        self.assertEqual(chunks[0][1], bytes.fromhex('00 00 00 01 00 00 00 01 08 02 00 00 00'))

    def test_pdf_copy_evidence_requires_exact_stream_filter_length_and_one_image(self):
        encoded = ci._png_framing_fixtures()[-1]['expected_stream']
        obj = (f'1 0 obj\n<< /Subtype /Image /Width 1 /Height 1 /Filter /FlateDecode /Length {len(encoded)} >>\nstream\n'.encode()
               + encoded + b'\nendstream\nendobj\n')
        pdf = b'%PDF-1.5\n' + obj + b'%%EOF\n'
        self.assertEqual(ci._png_copy_pdf_evidence(pdf, encoded), {
            'pdf_complete': True, 'embedded_image_dimensions': [1, 1], 'png_idat_stream_copied': True})
        for bad in (pdf.replace(b'FlateDecode', b'DCTDecode'),
                    pdf.replace(f'/Length {len(encoded)}'.encode(), b'/Length 0'),
                    pdf.replace(encoded, b'wrong stream'), b'%PDF-1.5\n' + obj + obj + b'%%EOF'):
            self.assertFalse(ci._png_copy_pdf_evidence(bad, encoded)['png_idat_stream_copied'])
        self.assertFalse(ci._png_copy_pdf_evidence(pdf[:-6], encoded)['pdf_complete'])


@unittest.skipUnless(os.name == 'posix', 'POSIX process groups required')
class JPEGFixtureTests(unittest.TestCase):
    def test_static_jpeg_has_complete_baseline_tables_and_entropy_data(self):
        content = ci._jpeg_bytes()
        self.assertEqual(content[:2], b'\xff\xd8')
        position, segments = 2, []
        while True:
            self.assertEqual(content[position], 255)
            marker = content[position + 1]
            length = int.from_bytes(content[position + 2:position + 4], 'big')
            payload = content[position + 4:position + 2 + length]
            self.assertEqual(len(payload), length - 2)
            segments.append((marker, payload))
            position += length + 2
            if marker == 218:
                break
        self.assertEqual([marker for marker, _ in segments], [219, 192, 196, 218])
        self.assertEqual(segments[0][1], b'\x00' + b'\x01' * 64)
        self.assertEqual(segments[1][1], bytes.fromhex('08 00 01 00 01 01 01 11 00'))
        tables = segments[2][1]
        self.assertEqual(tables[:18], b'\x00\x01' + bytes(15) + b'\x00')
        self.assertEqual(tables[18:], b'\x10\x01' + bytes(15) + b'\x00')
        self.assertEqual(content[position:], b'\x3f\xff\xd9')

    def test_exif_is_the_first_app1_with_its_complete_six_byte_signature(self):
        metadata = ci._jpeg_tiff()
        content = ci._jpeg_bytes(metadata)
        self.assertEqual(content[:4], b'\xff\xd8\xff\xe1')
        self.assertEqual(int.from_bytes(content[4:6], 'big'), 8 + len(metadata))
        self.assertEqual(content[6:12], b'Exif\0\0')
        self.assertEqual(content[12:12 + len(metadata)], metadata)

    def test_tiff_byte_orders_offsets_and_resolution_values_are_exact(self):
        for order in ('little', 'big'):
            with self.subTest(order=order):
                metadata = ci._jpeg_tiff(order)
                self.assertEqual(len(metadata), 66)
                self.assertEqual(metadata[:2], b'II' if order == 'little' else b'MM')
                number = lambda start, length: int.from_bytes(metadata[start:start + length], order)
                self.assertEqual(number(2, 2), 42)
                self.assertEqual(number(4, 4), 8)
                self.assertEqual(number(8, 2), 3)
                self.assertEqual(number(10, 2), 282)
                self.assertEqual(number(18, 4), 50)
                self.assertEqual(number(30, 4), 58)
                self.assertEqual([number(start, 4) for start in (50, 54, 58, 62)], [144, 1, 72, 1])

    def test_profiles_are_finite_unique_and_full_contains_every_tiff_prefix(self):
        quick, full = ci._jpeg_fixtures('quick'), ci._jpeg_fixtures('full')
        self.assertEqual(len(quick), 12)
        self.assertEqual(len(full), 88)
        self.assertEqual(len({row['name'] for row in full}), len(full))
        self.assertLess(max(len(row['content']) for row in full), 256)
        self.assertEqual(full[:len(quick)], quick)
        by_name = {row['name']: row for row in full}
        self.assertEqual(by_name['jpeg-framing-signature-only']['content'],
                         bytes.fromhex('ff d8 ff e1 00 08 45 78 69 66 00 00'))
        control = ci._jpeg_tiff()
        for length in range(len(control)):
            row = by_name[f'jpeg-invalid-tiff-prefix-{length}']
            self.assertEqual(row['content'][12:12 + length], control[:length])
            self.assertEqual(row['expected_dpi'], [72, 72])

    def test_positive_controls_exercise_both_byte_orders_cm_and_integer_division(self):
        rows = {row['name']: row for row in ci._jpeg_fixtures('full')}
        for order in ('little', 'big'):
            self.assertEqual(rows['jpeg-valid-dpi-' + order]['expected_dpi'], [144, 72])
            self.assertEqual(rows['jpeg-valid-cm-' + order]['expected_dpi'], [365, 182])
            self.assertEqual(rows['jpeg-valid-fractional-' + order]['expected_dpi'], [72, 36])

    def test_signed_division_control_reaches_a_supported_rational_tag(self):
        rows = {row['name']: row for row in ci._jpeg_fixtures('full')}
        for name, order in [('jpeg-invalid-signed-division', 'little'),
                            ('jpeg-invalid-signed-division-little', 'little'),
                            ('jpeg-invalid-signed-division-big', 'big')]:
            content = rows[name]['content']
            self.assertEqual(int.from_bytes(content[24:26], order), 5)
            self.assertEqual(int.from_bytes(content[62:66], order), 0x80000000)
            self.assertEqual(int.from_bytes(content[66:70], order), 0xffffffff)

    def test_fixture_generators_reject_unbounded_or_unknown_arguments(self):
        with self.assertRaises(ValueError):
            ci._jpeg_segment(225, bytes(65534))
        with self.assertRaises(ValueError):
            ci._jpeg_segment(256, b'')
        with self.assertRaises(ValueError):
            ci._jpeg_tiff('unknown')
        with self.assertRaises(ValueError):
            ci._jpeg_fixtures('unknown')

    def test_pdf_evidence_requires_the_exact_image_and_one_dct_object(self):
        content = ci._jpeg_bytes()
        obj = b'1 0 obj\n<< /Subtype /Image /Width 1 /Height 1 /Filter /DCTDecode >>\nstream\n' + content + b'\nendstream\nendobj\n'
        pdf = b'%PDF-1.4\n' + obj + b'%%EOF\n'
        self.assertEqual(ci._jpeg_pdf_evidence(pdf, content), {
            'pdf_complete': True, 'embedded_image_dimensions': [1, 1], 'jpeg_bytes_embedded': True})
        self.assertFalse(ci._jpeg_pdf_evidence(pdf[:-6], content)['pdf_complete'])
        self.assertFalse(ci._jpeg_pdf_evidence(pdf.replace(b'DCTDecode', b'FlateDecode'), content)['jpeg_bytes_embedded'])
        self.assertFalse(ci._jpeg_pdf_evidence(pdf, content + b'extra')['jpeg_bytes_embedded'])
        duplicate = ci._jpeg_pdf_evidence(b'%PDF-1.4\n' + obj + obj + b'%%EOF', content)
        self.assertIsNone(duplicate['embedded_image_dimensions'])
        self.assertFalse(duplicate['jpeg_bytes_embedded'])


class RunnerTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix='tekai-performance-unit-')
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name).resolve()
        self.args = options(self.work)
        self.audit = ci.PerformanceCI(self.args, self.work)
        self.addCleanup(self.audit.close)

    def test_all_caches_and_home_paths_are_fixture_owned(self):
        for key in ('HOME', 'USERPROFILE', 'XDG_CACHE_HOME', 'XDG_CONFIG_HOME', 'XDG_DATA_HOME',
                    'TMPDIR', 'TEMP', 'TMP', 'APPDATA', 'LOCALAPPDATA', 'TEKAI_ENGINE_CACHE'):
            self.assertTrue(Path(self.audit.env[key]).is_relative_to(self.work), key)
        env = self.audit.environment(self.work / 'project')
        for key in ('TEKAI_AUX_CACHE', 'TEKAI_FORMAT_CACHE', 'TEKAI_BIBTEX_CACHE'):
            self.assertTrue(Path(env[key]).is_relative_to(self.work / 'caches/project'), key)

    def test_host_tex_settings_and_embedded_runner_are_cleared(self):
        with tempfile.TemporaryDirectory() as temporary, patch.dict(os.environ, {
            'TEKAI_EMBEDDED_ENGINE_RUNNER': '/host/engine', 'TEKAI_FORMAT_CACHE': '/host/cache',
            'TEXINPUTS_pdflatex': '/host/tree', 'WEB2C': '/host/config', 'BIBINPUTS': '/host/bib',
            'KPATHSEA_DEBUG': '1', 'INDEXSTYLE': '/host/index'}):
            audit = ci.PerformanceCI(options(Path(temporary)), Path(temporary))
            for key in ('TEKAI_EMBEDDED_ENGINE_RUNNER', 'TEKAI_FORMAT_CACHE', 'TEXINPUTS_pdflatex',
                        'WEB2C', 'BIBINPUTS', 'KPATHSEA_DEBUG', 'INDEXSTYLE'):
                self.assertNotIn(key, audit.env)
            self.assertEqual(audit.env['PATH'], '')

    def test_rss_units_for_bsd_and_gnu(self):
        self.assertEqual(self.audit.rss('1048576 maximum resident set size'), 1)
        self.assertEqual(self.audit.rss('__TEKAI_RSS_KIB__=1024'), 1)
        self.assertIsNone(self.audit.rss('timer denied'))

    def test_python_subprocess_records_status_and_separate_observation(self):
        result = self.audit.run([sys.executable, '-c', 'print("sample"); raise SystemExit(7)'], self.work)
        self.assertEqual(result['code'], 7)
        self.assertEqual(result['stdout'], 'sample\n')
        self.assertFalse(self.audit.owned)
        self.assertNotIn('seconds', result)
        observation = self.audit.report['observations'][-1]
        self.assertEqual(observation['return_code'], 7)
        self.assertIsNone(observation['peak_rss_mib'])
        self.assertFalse(observation['rss_available'])
        self.assertGreaterEqual(observation['elapsed_seconds'], 0)

    def test_timeout_kills_and_reaps_owned_group(self):
        self.args.timeout = 0.03
        result = self.audit.run([sys.executable, '-c', 'import time; time.sleep(2)'], self.work)
        self.assertTrue(result['timeout'])
        self.assertIsNone(result['code'])
        self.assertFalse(self.audit.owned)
        self.assertTrue(self.audit.report['observations'][-1]['timed_out'])

    def test_start_uses_file_capture_and_new_session(self):
        process = Mock(pid=12345)
        with patch('performance_ci.subprocess.Popen', return_value=process) as popen:
            owned = self.audit.start(['unused'], self.work, self.audit.env)
        self.assertIs(owned, process)
        self.assertTrue(popen.call_args.kwargs['start_new_session'])
        self.assertNotEqual(popen.call_args.kwargs['stdout'], subprocess.PIPE)
        self.audit.owned.clear()

    @unittest.skipUnless(os.name == 'posix', 'Owned process groups require POSIX')
    def test_interrupt_after_popen_returns_keeps_the_child_owned_and_cleaned(self):
        native_popen, native_clock = subprocess.Popen, time.monotonic
        launched = []
        interrupted = False

        def popen(*args, **kwargs):
            process = native_popen(*args, **kwargs)
            launched.append(process)
            return process

        def clock():
            nonlocal interrupted
            if launched and not interrupted:
                interrupted = True
                process = launched[0]
                self.assertIs(self.audit.owned.get(process.pid), process)
                self.assertEqual(process.invocation[0], sys.executable)
                self.assertTrue(all(path.exists() for path in process.capture_paths))
                raise KeyboardInterrupt
            return native_clock()

        with patch('performance_ci.subprocess.Popen', side_effect=popen), \
                patch('performance_ci.time.monotonic', side_effect=clock), self.assertRaises(KeyboardInterrupt):
            self.audit.start([sys.executable, '-c', 'import time; time.sleep(10)'], self.work, self.audit.env)
        process = launched[0]
        self.assertIs(self.audit.owned.get(process.pid), process)
        self.audit.close()
        self.assertIsNotNone(process.poll())
        self.assertFalse(self.audit.owned)
        with self.assertRaises(ProcessLookupError):
            os.killpg(process.pid, 0)

    def test_cleanup_signals_only_the_owned_process_group(self):
        process = Mock(pid=12345, observe=False)
        self.audit.owned[process.pid] = process
        with patch('performance_ci.os.killpg') as kill:
            self.audit.stop(process)
            self.audit.stop(process)
        kill.assert_called_once_with(12345, signal.SIGKILL)
        process.wait.assert_called_once_with(timeout=5)

    def test_cleanup_reaps_an_already_exited_group(self):
        process = Mock(pid=12345, observe=False)
        self.audit.owned[process.pid] = process
        with patch('performance_ci.os.killpg', side_effect=ProcessLookupError()):
            self.audit.stop(process)
        process.wait.assert_called_once_with(timeout=5)
        self.assertFalse(self.audit.owned)

    def test_unknown_process_cannot_be_killed_by_stop(self):
        with patch('performance_ci.os.killpg') as kill:
            self.audit.stop(Mock(pid=12345))
        kill.assert_not_called()

    def test_cleanup_reaps_even_if_signaling_is_denied(self):
        process = Mock(pid=12345, observe=False)
        self.audit.owned[process.pid] = process
        with patch('performance_ci.os.killpg', side_effect=PermissionError()):
            with self.assertRaises(PermissionError):
                self.audit.stop(process)
        process.wait.assert_called_once_with(timeout=5)
        self.audit.owned.clear()

    def test_launch_failure_is_an_explicit_failure(self):
        with patch.object(self.audit, 'start', side_effect=PermissionError('denied')):
            result = self.audit.run(['unused'], self.work)
        self.assertIsNone(result['code'])
        self.assertIn('denied', result['launch_error'])
        self.assertEqual(ci.verdict('warmup', result)['status'], 'failed')

    def test_timer_permission_failure_is_cached_and_falls_back_to_direct_status(self):
        with patch.object(self.audit, 'execute', return_value={'code': 1, 'stderr_tail': 'sysctl denied'}) as execute, \
                patch('performance_ci.Path.is_file', return_value=True):
            self.audit.probe_timer()
            self.audit.probe_timer()
        execute.assert_called_once()
        self.assertIsNone(self.audit.timer)
        result = self.audit.run([sys.executable, '-c', 'pass'], self.work, measured=True)
        self.assertEqual(result['code'], 0)
        self.assertEqual(self.audit.report['observations'][-1]['rss_unavailable_reason'], 'sysctl denied')

    def test_timer_launch_failure_is_unavailable_not_a_compiler_failure(self):
        with patch.object(self.audit, 'execute', return_value={'code': None, 'launch_error': 'launch denied'}), \
                patch('performance_ci.Path.is_file', return_value=True):
            self.audit.probe_timer()
        self.assertIsNone(self.audit.timer)
        self.assertEqual(self.audit.timer_reason, 'launch denied')

    def test_missing_timer_records_unavailable(self):
        with patch('performance_ci.Path.is_file', return_value=False):
            self.audit.probe_timer()
        self.assertIn('unavailable', self.audit.timer_reason)

    def test_stdout_capture_has_a_limit(self):
        with patch('performance_ci.MAX_STDOUT_BYTES', 32):
            result = self.audit.run([sys.executable, '-c', 'print("x" * 100)'], self.work)
        self.assertEqual(len(result['stdout']), 32)
        self.assertTrue(result['stdout_truncated'])

    def test_stderr_capture_keeps_the_tail(self):
        with patch('performance_ci.LOG_TAIL_BYTES', 32):
            result = self.audit.run([sys.executable, '-c', 'import sys; print("x" * 100 + "TAIL", file=sys.stderr)'], self.work)
        self.assertLessEqual(len(result['stderr_tail']), 32)
        self.assertTrue(result['stderr_tail'].endswith('TAIL\n'))

    def test_parsed_rejects_truncation_and_wrong_json_schema(self):
        for result in ({'code': 0, 'stdout': '{"skipped": false, "tex_runs": 1}', 'stdout_truncated': True},
                       {'code': 1, 'stdout': '{}'}, {'code': 0, 'stdout': '[]'}, {'code': 0, 'stdout': '{}'}):
            with self.subTest(result=result), self.assertRaises((ValueError, RuntimeError)):
                self.audit.parsed(result)

    def test_parsed_preserves_valid_build_report(self):
        self.assertEqual(self.audit.parsed({'code': 0, 'stdout': '{"skipped": false, "tex_runs": 1}'}),
                         {'skipped': False, 'tex_runs': 1})

    def test_pdf_text_extraction_uses_the_supervised_runner(self):
        self.audit.pdftext = '/unused/pdftotext'
        with patch.object(self.audit, 'run', return_value={'code': 0, 'stdout': 'PDF words'}) as run:
            self.assertEqual(self.audit.text(self.work), 'PDF words')
        self.assertEqual(run.call_args.args[0][0], '/unused/pdftotext')

    def test_process_probe_denial_records_a_skip(self):
        self.audit.ps = '/unused/ps'
        with patch.object(self.audit, 'run', return_value={'code': 1}), contextlib.redirect_stdout(io.StringIO()):
            self.assertFalse(self.audit.process_inspection_available('cancel'))
        self.assertEqual(self.audit.report['results'][-1]['correctness']['status'], 'skipped')

    def test_snapshot_returns_only_owned_group_members(self):
        self.audit.ps = '/unused/ps'
        text = '100 1 100 S own command\n101 100 100 S engine\n200 1 200 S unrelated secret\n'
        with patch.object(self.audit, 'run', return_value={'code': 0, 'stdout': text}):
            rows = self.audit.snapshot(Mock(pid=100))
        self.assertEqual([row['pid'] for row in rows], [100, 101])
        self.assertNotIn('secret', str(rows))

    def test_report_artifacts_distinguish_outcomes_and_separate_timings(self):
        with contextlib.redirect_stdout(io.StringIO()):
            self.audit.record('input-identity', **cache_evidence(), seconds=42, peak_mib=100)
            self.audit.record('edit-race', first_text='OLD-CONTENT', next_text='OLD-CONTENT', stale_cache_hit=True)
        report = json.loads(self.args.output.read_text())
        self.assertEqual(report['summary']['passed'], 1)
        self.assertEqual(report['summary']['known_failure'], 1)
        self.assertNotIn('seconds', report['results'][0]['evidence'])
        self.assertEqual(report['fixture_timing_observations'][0]['fields']['seconds'], 42)
        summary = self.args.output.with_suffix('.md').read_text()
        self.assertIn('known_failure', summary)
        self.assertIn('open bugs, not passes', summary)

    def test_embedded_json_and_scope_timings_move_to_observations(self):
        with contextlib.redirect_stdout(io.StringIO()):
            self.audit.record('decoded-images', code=0, stdout='{"elapsed_ms": 123, "tex_runs": 1}')
            self.audit.record('expansion-scopes', code=0, depth=1,
                              stdout='output_tokens=3 expansion_ms=1.234')
        self.assertEqual(self.audit.report['results'][0]['evidence']['stdout'], {'tex_runs': 1})
        self.assertEqual(self.audit.report['fixture_timing_observations'][0]['fields']['stdout.elapsed_ms'], 123)
        self.assertNotIn('expansion_ms', self.audit.report['results'][1]['evidence']['stdout'])
        self.assertEqual(self.audit.report['fixture_timing_observations'][1]['fields']['expansion_ms'], 1.234)

    def test_expansion_binary_path_follows_the_cli_option(self):
        self.args.expansion_engine = Path(sys.executable)
        with patch.object(self.audit, 'run', return_value={'code': 0, 'stdout': 'expansion_error=stub'}) as run, \
                patch.object(self.audit, 'record'):
            self.audit.expansion()
        self.assertTrue(all(call.args[0][0] == Path(sys.executable) for call in run.call_args_list))
        self.assertEqual(len(run.call_args_list), 7)

    def test_png_huge_declarations_do_not_allocate_huge_fixture_inputs(self):
        calls = []

        def run(command, project, **_values):
            calls.append((project, (project / 'image.png').stat().st_size))
            return {'code': 1}

        with patch.object(self.audit, 'media'), patch.object(self.audit, 'run', side_effect=run), \
                patch.object(self.audit, 'record'), patch.object(self.audit, 'png_framing', return_value=True), \
                patch.object(self.audit, 'png_decode', return_value=True):
            self.audit.png()
        self.assertEqual(len(calls), 5)
        self.assertLess(max(size for _, size in calls), 100)

    def test_png_framing_family_records_precise_errors_and_exact_copy_without_tex(self):
        fixtures = {row['name']: row for row in ci._png_framing_fixtures()}

        def run(command, project, **_values):
            row = fixtures[project.name]
            self.assertIn('--force', command)
            self.assertEqual((project / 'image.png').read_bytes(), row['content'])
            source = (project / 'main.tex').read_text()
            for setting in ('\\pdfminorversion=5', '\\pdfimageapplygamma=0', '\\pdfobjcompresslevel=0'):
                self.assertIn(setting, source)
            build = project / 'build'
            build.mkdir()
            if 'expected_error' in row:
                (build / 'main.log').write_text(row['expected_error'])
                return {'code': 1, 'stderr_tail': NORMAL_ENGINE_ERROR}
            encoded = row['expected_stream']
            (build / 'main.log').write_text('(PNG copy)')
            (build / 'main.pdf').write_bytes(
                f'%PDF-1.5\n1 0 obj\n<< /Subtype /Image /Width 1 /Height 1 /Filter /FlateDecode /Length {len(encoded)} >>\nstream\n'.encode()
                + encoded + b'\nendstream\nendobj\n%%EOF\n')
            return {'code': 0, 'stdout': '{"skipped": false, "tex_runs": 1}'}

        with patch.object(self.audit, 'run', side_effect=run) as calls, contextlib.redirect_stdout(io.StringIO()):
            self.assertTrue(self.audit.png_framing())
        self.assertEqual(calls.call_count, 15)
        self.assertEqual(self.audit.report['summary']['passed'], 15)

    def test_png_framing_timeout_stops_remaining_cases_and_remains_a_failure(self):
        with patch.object(self.audit, 'run', return_value={'code': None, 'timeout': True}) as calls, \
                contextlib.redirect_stdout(io.StringIO()):
            self.assertFalse(self.audit.png_framing())
        self.assertEqual(calls.call_count, 1)
        self.assertEqual(self.audit.report['summary']['failed'], 1)

    def test_png_decode_family_records_exact_errors_and_pixels_without_tex(self):
        fixtures = {row['name']: row for row in ci._png_decode_fixtures()}

        def run(command, project, **_values):
            row = fixtures[project.name]
            self.assertIn('--force', command)
            self.assertEqual((project / 'image.png').read_bytes(), row['content'])
            self.assertIn('\\pdfimageapplygamma=1', (project / 'main.tex').read_text())
            build = project / 'build'
            build.mkdir()
            if 'expected_error' in row:
                (build / 'main.log').write_text(row['expected_error'])
                return {'code': 1, 'stderr_tail': NORMAL_ENGINE_ERROR}
            pdf = b'%PDF-1.5\n'
            for number, (color, pixels) in enumerate(row['expected_pixels'], 1):
                color = 'Gray' if color == 'gray' else 'RGB'
                pdf += (f'{number} 0 obj\n<< /Subtype /Image /Width 1 /Height 1 /BitsPerComponent 8 /ColorSpace /Device{color} >>\nstream\n'.encode()
                        + bytes(pixels) + b'\nendstream\nendobj\n')
            (build / 'main.pdf').write_bytes(pdf + b'%%EOF\n')
            return {'code': 0, 'stdout': '{"skipped": false, "tex_runs": 1}'}

        with patch.object(self.audit, 'run', side_effect=run) as calls, contextlib.redirect_stdout(io.StringIO()):
            self.assertTrue(self.audit.png_decode())
        self.assertEqual(calls.call_count, 12)
        self.assertEqual(self.audit.report['summary']['passed'], 12)

    def test_png_decode_timeout_stops_remaining_cases_and_remains_a_failure(self):
        with patch.object(self.audit, 'run', return_value={'code': None, 'timeout': True}) as calls, \
                contextlib.redirect_stdout(io.StringIO()):
            self.assertFalse(self.audit.png_decode())
        self.assertEqual(calls.call_count, 1)
        self.assertEqual(self.audit.report['summary']['failed'], 1)

    def test_png_family_runs_framing_in_both_profiles_and_stops_after_its_timeout(self):
        with patch.object(self.audit, 'media'), patch.object(self.audit, 'run', return_value={'code': 1}), \
                patch.object(self.audit, 'record'), patch.object(self.audit, 'project', return_value=self.work), \
                patch('performance_ci.Audit.png') as legacy:
            for quick in (True, False):
                self.args.quick = quick
                with patch.object(self.audit, 'png_framing', return_value=False) as framing:
                    self.audit.png()
                    framing.assert_called_once()
            legacy.assert_not_called()

    def test_jpeg_family_records_strict_error_and_image_controls_without_tex(self):
        fixtures = {row['name']: row for row in ci._jpeg_fixtures('quick')}

        def run(command, project, **_values):
            row = fixtures[project.name]
            self.assertIn('--force', command)
            self.assertEqual((project / 'image.jpg').read_bytes(), row['content'])
            build = project / 'build'
            build.mkdir()
            if 'expected_error' in row:
                (build / 'main.log').write_text(row['expected_error'])
                return {'code': 1, 'stdout': '', 'stderr_tail': NORMAL_ENGINE_ERROR}
            dimensions = [(473628672 + 50 * dpi) // (100 * dpi) for dpi in row['expected_dpi']]
            (build / 'main.log').write_text(f'JPEG-WIDTH-SP={dimensions[0]}\nJPEG-HEIGHT-SP={dimensions[1]}\n')
            (build / 'main.pdf').write_bytes(b'%PDF-1.4\n1 0 obj\n<< /Subtype /Image /Width 1 /Height 1 /Filter /DCTDecode >>\nstream\n'
                                           + row['content'] + b'\nendstream\nendobj\n%%EOF\n')
            return {'code': 0, 'stdout': '{"skipped": false, "tex_runs": 1}'}

        with patch.object(self.audit, 'run', side_effect=run) as calls, contextlib.redirect_stdout(io.StringIO()):
            self.audit.jpeg()
        self.assertEqual(calls.call_count, 12)
        self.assertEqual(self.audit.report['summary']['passed'], 12)
        self.assertEqual(self.audit.report['summary']['failed'], 0)

    def test_jpeg_timeout_stops_the_family_and_remains_a_failure(self):
        with patch.object(self.audit, 'run', return_value={'code': None, 'timeout': True}) as calls, \
                contextlib.redirect_stdout(io.StringIO()):
            self.audit.jpeg()
        self.assertEqual(calls.call_count, 1)
        self.assertEqual(self.audit.report['summary']['failed'], 1)

    def test_alias_fixture_cannot_be_masked_by_implicit_project_search(self):
        def run(_command, project, env):
            self.assertFalse(any(project.rglob('choice.tex')))
            self.assertEqual(env['TEXINPUTS'], str(self.work / 'alias-tree') + '//alias//')
            self.assertTrue((self.work / 'alias-tree/a-decoy/choice.tex').is_file())
            return {'code': 0, 'stdout': str(self.work / 'alias-physical/choice.tex') + '\n'}

        with patch('performance_ci.Audit.lookup'), patch.object(self.audit, 'run', side_effect=run), \
                contextlib.redirect_stdout(io.StringIO()):
            self.audit.lookup()
        self.assertEqual(self.audit.report['results'][-1]['correctness']['status'], 'passed')

    def executable_fixtures(self):
        self.args.engine = self.work / 'fixture-engine'
        self.args.engine.write_bytes(b'engine-first')
        self.args.expansion_engine = self.work / 'fixture-expansion'
        self.args.expansion_engine.write_bytes(b'expansion-first')

    def test_unchanged_selected_executables_record_start_and_end_hashes(self):
        self.executable_fixtures()
        self.audit.capture_executables(['expansion'])
        with contextlib.redirect_stdout(io.StringIO()):
            self.audit.verify_executables()
        report = json.loads(self.args.output.read_text())
        self.assertEqual([row['role'] for row in report['executables']], ['engine', 'expansion'])
        self.assertTrue(all(row['sha256_start'] == row['sha256_end'] and row['unchanged']
                            for row in report['executables']))
        self.assertTrue(all(row['source_sha256_start'] == row['source_sha256_end'] == row['sha256_start']
                            and row['copy_matches_source'] for row in report['executables']))
        self.assertEqual(report['summary']['passed'], 2)

    def test_selected_executables_are_verified_copies_without_standalone_siblings(self):
        self.executable_fixtures()
        source_engine = self.args.engine
        source_engine.chmod(0o700)
        (source_engine.parent / 'tekai-engine').write_bytes(b'not part of the deployment')
        self.audit.capture_executables(['expansion'])
        self.assertEqual(self.args.engine, self.work / 'bin/tekai')
        self.assertEqual(self.args.expansion_engine, self.work / 'bin/audit_expansion')
        self.assertTrue(os.access(self.args.engine, os.X_OK))
        self.assertFalse((self.work / 'bin/tekai-engine').exists())
        self.assertEqual(self.args.engine.read_bytes(), source_engine.read_bytes())
        self.assertEqual(self.audit.report['source_binary'], str(source_engine))
        self.assertEqual(self.audit.report['binary'], str(self.args.engine))
        self.assertEqual(self.audit.report['executables'][0]['source_path'], str(source_engine))

    def test_copy_hash_mismatch_fails_before_any_fixture_can_run(self):
        self.executable_fixtures()
        with patch('performance_ci.shutil.copy2', side_effect=lambda _source, path: path.write_bytes(b'wrong copy')):
            with self.assertRaisesRegex(RuntimeError, 'copy does not match'):
                self.audit.capture_executables([])
        with contextlib.redirect_stdout(io.StringIO()):
            self.audit.verify_executables()
        self.assertFalse(self.audit.report['executables'][0]['copy_matches_source'])
        self.assertEqual(self.audit.report['summary']['failed'], 1)

    def test_changed_source_fails_even_when_the_executed_copy_is_stable(self):
        self.executable_fixtures()
        source = self.args.engine
        self.audit.capture_executables([])
        source.write_bytes(b'changed source')
        with contextlib.redirect_stdout(io.StringIO()):
            self.audit.verify_executables()
        row = self.audit.report['executables'][0]
        self.assertEqual(row['sha256_start'], row['sha256_end'])
        self.assertNotEqual(row['source_sha256_start'], row['source_sha256_end'])
        self.assertEqual(self.audit.report['summary']['failed'], 1)

    def test_source_disappearance_fails_even_when_the_executed_copy_remains(self):
        self.executable_fixtures()
        source = self.args.engine
        self.audit.capture_executables([])
        source.unlink()
        with contextlib.redirect_stdout(io.StringIO()):
            self.audit.verify_executables()
        self.assertTrue(self.args.engine.is_file())
        self.assertIsNone(self.audit.report['executables'][0]['source_sha256_end'])
        self.assertEqual(self.audit.report['summary']['failed'], 1)

    def test_expansion_disappearing_during_copy_is_not_an_optional_initial_absence(self):
        self.executable_fixtures()
        real_copy = ci.shutil.copy2

        def copy(source, destination):
            if source == self.work / 'fixture-expansion':
                raise FileNotFoundError('source changed during copy')
            return real_copy(source, destination)

        with patch('performance_ci.shutil.copy2', side_effect=copy), self.assertRaises(FileNotFoundError):
            self.audit.capture_executables(['expansion'])
        self.assertNotIn('unavailable_at_start', self.audit.report['executables'][1])

    def test_changed_engine_content_fails_with_both_hashes_recorded(self):
        self.executable_fixtures()
        self.audit.capture_executables(['lint'])
        self.args.engine.write_bytes(b'engine-next!')
        with contextlib.redirect_stdout(io.StringIO()):
            self.audit.verify_executables()
        row = self.audit.report['executables'][0]
        self.assertNotEqual(row['sha256_start'], row['sha256_end'])
        self.assertFalse(row['unchanged'])
        self.assertEqual(self.audit.report['results'][-1]['correctness']['status'], 'failed')

    def test_changed_selected_expansion_content_fails(self):
        self.executable_fixtures()
        self.audit.capture_executables(['expansion'])
        self.args.expansion_engine.write_bytes(b'expansion-next!')
        with contextlib.redirect_stdout(io.StringIO()):
            self.audit.verify_executables()
        results = self.audit.report['results']
        self.assertEqual([row['correctness']['status'] for row in results], ['passed', 'failed'])
        self.assertEqual(results[-1]['evidence']['role'], 'expansion')

    def test_unselected_expansion_is_not_hashed(self):
        self.executable_fixtures()
        self.audit.capture_executables(['lint'])
        self.args.expansion_engine.write_bytes(b'unselected replacement')
        with contextlib.redirect_stdout(io.StringIO()):
            self.audit.verify_executables()
        self.assertEqual([row['role'] for row in self.audit.report['executables']], ['engine'])
        self.assertEqual(self.audit.report['results'][-1]['correctness']['status'], 'passed')

    def test_removing_a_selected_executable_fails(self):
        self.executable_fixtures()
        self.audit.capture_executables(['lint'])
        self.args.engine.unlink()
        with contextlib.redirect_stdout(io.StringIO()):
            self.audit.verify_executables()
        self.assertIsNone(self.audit.report['executables'][0]['sha256_end'])
        self.assertIsNotNone(self.audit.report['executables'][0]['source_sha256_end'])
        self.assertEqual(self.audit.report['results'][-1]['correctness']['status'], 'failed')

    def test_missing_optional_expansion_remains_explicitly_unavailable(self):
        self.executable_fixtures()
        self.args.expansion_engine.unlink()
        self.audit.capture_executables(['expansion'])
        with contextlib.redirect_stdout(io.StringIO()):
            self.audit.verify_executables()
        row = self.audit.report['executables'][1]
        self.assertTrue(row['unavailable_at_start'])
        self.assertIsNone(row['sha256_start'])
        self.assertIsNone(row['sha256_end'])
        self.assertEqual(len(self.audit.report['results']), 1)

    def test_optional_expansion_appearing_mid_run_fails(self):
        self.executable_fixtures()
        self.args.expansion_engine.unlink()
        self.audit.capture_executables(['expansion'])
        self.args.expansion_engine.write_bytes(b'new executable')
        with contextlib.redirect_stdout(io.StringIO()):
            self.audit.verify_executables()
        self.assertEqual(self.audit.report['results'][-1]['correctness']['status'], 'failed')

    def test_final_hash_permission_error_fails_explicitly(self):
        self.executable_fixtures()
        self.audit.capture_executables(['lint'])
        with patch('performance_ci.executable_sha256', side_effect=PermissionError('denied')), \
                contextlib.redirect_stdout(io.StringIO()):
            self.audit.verify_executables()
        result = self.audit.report['results'][-1]
        self.assertEqual(result['correctness']['status'], 'failed')
        self.assertIn('Could not recheck', result['evidence']['exception'])


class CLITests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix='tekai-performance-cli-unit-')
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name).resolve()

    def command(self, *extra):
        return ['--engine', sys.executable, '--output', str(self.work / 'result.json'), *extra]

    def test_profiles_have_bounded_quick_subset(self):
        self.assertLess(len(ci.QUICK_CASES), len(ci.FULL_CASES))
        self.assertTrue(set(ci.QUICK_CASES).issubset(ci.FULL_CASES))
        self.assertNotIn('watch-retention', ci.QUICK_CASES)
        self.assertTrue(set(ci.AUDIT_CASES).issubset(ci.FULL_CASES))
        self.assertIn('jpeg', ci.QUICK_CASES)
        self.assertIn('jpeg', ci.FULL_CASES)

    def test_invalid_timeout_values_are_rejected(self):
        for timeout in ('0', '-1', 'nan', 'inf', '61'):
            with self.subTest(timeout=timeout), contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
                ci.main(self.command('--timeout', timeout))
            self.assertEqual(error.exception.code, 2)

    def test_missing_executable_is_rejected(self):
        with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as error:
            ci.main(['--engine', str(self.work / 'absent')])
        self.assertEqual(error.exception.code, 2)

    @unittest.skipUnless(os.name == 'posix', 'POSIX process groups required')
    def test_required_pdftext_gap_fails_and_writes_report(self):
        with patch('performance_ci.shutil.which', return_value=None), contextlib.redirect_stdout(io.StringIO()):
            code = ci.main(self.command('--require-pdftotext'))
        self.assertEqual(code, 1)
        report = json.loads((self.work / 'result.json').read_text())
        self.assertEqual(report['results'][0]['case'], 'required-pdftotext')

    @unittest.skipUnless(os.name == 'posix', 'POSIX process groups required')
    def test_required_expansion_gap_fails_and_writes_report(self):
        with contextlib.redirect_stdout(io.StringIO()):
            code = ci.main(self.command('--require-expansion', '--expansion-engine', str(self.work / 'absent')))
        self.assertEqual(code, 1)
        self.assertEqual(json.loads((self.work / 'result.json').read_text())['results'][0]['case'], 'required-expansion')

    @unittest.skipUnless(os.name == 'posix', 'POSIX process groups required')
    def test_family_exception_is_recorded_and_later_family_runs(self):
        with patch.object(ci.PerformanceCI, 'run', return_value={'code': 0, 'stdout': '{}'}), \
                patch.object(ci.PerformanceCI, 'lookup', side_effect=RuntimeError('fixture failed')), \
                patch.object(ci.PerformanceCI, 'lint') as lint, contextlib.redirect_stdout(io.StringIO()):
            code = ci.main(self.command('--case', 'lookup', '--case', 'lint'))
        self.assertEqual(code, 1)
        lint.assert_called_once()
        report = json.loads((self.work / 'result.json').read_text())
        self.assertEqual(report['selected_cases'], ['lookup', 'lint'])
        self.assertEqual(report['summary']['failed'], 1)

    @unittest.skipUnless(os.name == 'posix', 'POSIX process groups required')
    def test_duplicate_case_selection_is_deduplicated(self):
        with patch.object(ci.PerformanceCI, 'run', return_value={'code': 0, 'stdout': '{}'}), \
                patch.object(ci.PerformanceCI, 'lint') as lint, contextlib.redirect_stdout(io.StringIO()):
            code = ci.main(self.command('--case', 'lint', '--case', 'lint'))
        self.assertEqual(code, 0)
        lint.assert_called_once()

    @unittest.skipUnless(os.name == 'posix', 'POSIX process groups required')
    def test_interrupt_is_recorded_and_cleanup_runs(self):
        with patch.object(ci.PerformanceCI, 'run', side_effect=KeyboardInterrupt), \
                patch.object(ci.PerformanceCI, 'close') as close, contextlib.redirect_stdout(io.StringIO()):
            code = ci.main(self.command('--case', 'lint'))
        self.assertEqual(code, 130)
        close.assert_called_once()
        report = json.loads((self.work / 'result.json').read_text())
        self.assertEqual(report['results'][0]['case'], 'interrupted')

    @unittest.skipUnless(os.name == 'posix', 'POSIX process groups required')
    def test_warmup_supervision_error_fails_and_still_cleans_up(self):
        with patch.object(ci.PerformanceCI, 'run', side_effect=RuntimeError('supervision failed')), \
                patch.object(ci.PerformanceCI, 'close') as close, contextlib.redirect_stdout(io.StringIO()):
            code = ci.main(self.command('--case', 'lint'))
        self.assertEqual(code, 1)
        close.assert_called_once()
        report = json.loads((self.work / 'result.json').read_text())
        self.assertEqual(report['results'][0]['case'], 'runner')

    @unittest.skipUnless(os.name == 'posix', 'POSIX process groups required')
    def test_cleanup_error_is_reported_and_stops_new_fixture_work(self):
        with patch.object(ci.PerformanceCI, 'run', return_value={'code': 0, 'stdout': '{}'}), \
                patch.object(ci.PerformanceCI, 'lookup'), patch.object(ci.PerformanceCI, 'lint') as lint, \
                patch.object(ci.PerformanceCI, 'close', side_effect=RuntimeError('owned cleanup failed')), \
                contextlib.redirect_stdout(io.StringIO()):
            code = ci.main(self.command('--case', 'lookup', '--case', 'lint'))
        self.assertEqual(code, 1)
        lint.assert_not_called()
        report = json.loads((self.work / 'result.json').read_text())
        self.assertTrue(any(row['case'] == 'cleanup' and row['correctness']['status'] == 'failed'
                            for row in report['results']))

    @unittest.skipUnless(os.name == 'posix', 'POSIX process groups required')
    def test_cli_rechecks_a_changed_fixture_executable_at_end(self):
        engine = self.work / 'fixture-engine'
        engine.write_bytes(b'first fixture executable')
        engine.chmod(0o700)
        with patch.object(ci.PerformanceCI, 'run', return_value={'code': 0, 'stdout': '{}'}), \
                patch.object(ci.PerformanceCI, 'lint', side_effect=lambda: engine.write_bytes(b'changed fixture executable')), \
                contextlib.redirect_stdout(io.StringIO()):
            code = ci.main(self.command('--engine', str(engine), '--case', 'lint'))
        self.assertEqual(code, 1)
        report = json.loads((self.work / 'result.json').read_text())
        self.assertEqual(report['results'][-1]['case'], 'executable-integrity')
        self.assertEqual(report['results'][-1]['correctness']['status'], 'failed')

    @unittest.skipUnless(os.name == 'posix', 'POSIX process groups required')
    def test_cli_accepts_an_unchanged_fixture_executable(self):
        engine = self.work / 'fixture-engine'
        engine.write_bytes(b'stable fixture executable')
        engine.chmod(0o700)
        with patch.object(ci.PerformanceCI, 'run', return_value={'code': 0, 'stdout': '{}'}), \
                patch.object(ci.PerformanceCI, 'lint'), contextlib.redirect_stdout(io.StringIO()):
            code = ci.main(self.command('--engine', str(engine), '--case', 'lint'))
        self.assertEqual(code, 0)
        report = json.loads((self.work / 'result.json').read_text())
        self.assertEqual(report['executables'][0]['sha256_start'], hashlib.sha256(engine.read_bytes()).hexdigest())
        self.assertEqual(report['results'][-1]['correctness']['status'], 'passed')


if __name__ == '__main__':
    unittest.main()
