//! The engine's long-only options, with no libc parser state or argv mutation.
//!
//! Arguments borrow the retained invocation buffers. Required values may use
//! `=` or the next argument, while optional values only use `=`. Parsing stops
//! at the first non-option, as the former `+` getopt optstring required.

#![warn(clippy::all)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EngineOption {
    Fmt,
    Efmt,
    CnfLine,
    Help,
    Ini,
    Interaction,
    HaltOnError,
    KpathseaDebug,
    Progname,
    Recorder,
    Version,
    Ipc,
    IpcStart,
    Mltex,
    Enc,
    Etex,
    OutputComment,
    Draftmode,
    OutputFormat,
    ShellEscape,
    NoShellEscape,
    EnableWrite18,
    DisableWrite18,
    ShellRestricted,
    DebugFormat,
    SrcSpecials,
    FileLineErrorStyle,
    NoFileLineErrorStyle,
    FileLineError,
    NoFileLineError,
    Jobname,
    OutputDirectory,
    ParseFirstLine,
    NoParseFirstLine,
    TranslateFile,
    DefaultTranslateFile,
    EightBit,
    Mktex,
    NoMktex,
    Synctex,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ArgumentRule {
    None,
    Required,
    Optional,
}

#[derive(Clone, Copy)]
struct OptionSpec {
    option: EngineOption,
    name: &'static [u8],
    argument: ArgumentRule,
}

macro_rules! options {
    ($($option:ident, $name:literal, $argument:ident;)*) => {
        &[$(OptionSpec {
            option: EngineOption::$option,
            name: $name,
            argument: ArgumentRule::$argument,
        },)*]
    };
}

// Keep the original table order for native differential tests. In long-only
// mode even flag aliases are ambiguous unless one spelling matches exactly.
const OPTIONS: &[OptionSpec] = options! {
    Fmt, b"fmt", Required;
    Efmt, b"efmt", Required;
    CnfLine, b"cnf-line", Required;
    Help, b"help", None;
    Ini, b"ini", None;
    Interaction, b"interaction", Required;
    HaltOnError, b"halt-on-error", None;
    KpathseaDebug, b"kpathsea-debug", Required;
    Progname, b"progname", Required;
    Recorder, b"recorder", None;
    Version, b"version", None;
    Ipc, b"ipc", None;
    IpcStart, b"ipc-start", None;
    Mltex, b"mltex", None;
    Enc, b"enc", None;
    Etex, b"etex", None;
    OutputComment, b"output-comment", Required;
    Draftmode, b"draftmode", None;
    OutputFormat, b"output-format", Required;
    ShellEscape, b"shell-escape", None;
    NoShellEscape, b"no-shell-escape", None;
    EnableWrite18, b"enable-write18", None;
    DisableWrite18, b"disable-write18", None;
    ShellRestricted, b"shell-restricted", None;
    DebugFormat, b"debug-format", None;
    SrcSpecials, b"src-specials", Optional;
    FileLineErrorStyle, b"file-line-error-style", None;
    NoFileLineErrorStyle, b"no-file-line-error-style", None;
    FileLineError, b"file-line-error", None;
    NoFileLineError, b"no-file-line-error", None;
    Jobname, b"jobname", Required;
    OutputDirectory, b"output-directory", Required;
    ParseFirstLine, b"parse-first-line", None;
    NoParseFirstLine, b"no-parse-first-line", None;
    TranslateFile, b"translate-file", Required;
    DefaultTranslateFile, b"default-translate-file", Required;
    EightBit, b"8bit", None;
    Mktex, b"mktex", Required;
    NoMktex, b"no-mktex", Required;
    Synctex, b"synctex", Required;
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ParsedOption<'a> {
    pub(crate) option: EngineOption,
    pub(crate) argument: Option<&'a [u8]>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ErrorKind {
    Unknown,
    Ambiguous,
    UnexpectedArgument,
    MissingArgument,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ParseError<'a> {
    pub(crate) kind: ErrorKind,
    spelling: &'a [u8],
}

impl ParseError<'_> {
    /// Keep the macOS getopt diagnostic wording and non-UTF-8 argument bytes.
    /// The caller supplies the process name and decides where to write it.
    pub(crate) fn diagnostic(self, program: &[u8]) -> Vec<u8> {
        let program = program
            .rsplit(|&byte| byte == b'/')
            .next()
            .unwrap_or(program);
        let mut message = Vec::new();
        message.extend_from_slice(program);
        message.extend_from_slice(b": ");
        message.extend_from_slice(match self.kind {
            ErrorKind::Unknown => b"unrecognized option `",
            _ => b"option `",
        });
        message.extend_from_slice(self.spelling);
        message.extend_from_slice(match self.kind {
            ErrorKind::Unknown => b"'\n",
            ErrorKind::Ambiguous => b"' is ambiguous\n",
            ErrorKind::UnexpectedArgument => b"' doesn't allow an argument\n",
            ErrorKind::MissingArgument => b"' requires an argument\n",
        });
        message
    }
}

pub(crate) struct Parser<'a> {
    arguments: &'a [&'a [u8]],
    next_index: usize,
    finished: bool,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(arguments: &'a [&'a [u8]]) -> Self {
        Self {
            arguments,
            next_index: 1.min(arguments.len()),
            finished: false,
        }
    }

    /// Index of the first argument not consumed, including after an error.
    pub(crate) fn next_index(&self) -> usize {
        self.next_index
    }

    pub(crate) fn next_option(&mut self) -> Result<Option<ParsedOption<'a>>, ParseError<'a>> {
        if self.finished {
            return Ok(None);
        }
        let Some(&token) = self.arguments.get(self.next_index) else {
            self.finished = true;
            return Ok(None);
        };
        if token == b"--" {
            self.next_index += 1;
            self.finished = true;
            return Ok(None);
        }
        if !token.starts_with(b"-") || token == b"-" {
            self.finished = true;
            return Ok(None);
        }
        self.next_index += 1;
        let prefix_len = if token.starts_with(b"--") { 2 } else { 1 };
        let name_end = token
            .iter()
            .position(|&byte| byte == b'=')
            .unwrap_or(token.len());
        let name = &token[prefix_len..name_end];
        let inline_argument = (name_end < token.len()).then(|| &token[name_end + 1..]);
        let error = |kind, include_value| ParseError {
            kind,
            spelling: if include_value {
                token
            } else {
                &token[..name_end]
            },
        };
        let spec = if let Some(exact) = OPTIONS.iter().find(|spec| spec.name == name) {
            exact
        } else {
            let mut candidates = OPTIONS.iter().filter(|spec| spec.name.starts_with(name));
            let Some(first) = candidates.next() else {
                return Err(error(ErrorKind::Unknown, true));
            };
            if candidates.next().is_some() {
                return Err(error(ErrorKind::Ambiguous, false));
            }
            first
        };
        let argument = match spec.argument {
            ArgumentRule::None if inline_argument.is_some() => {
                return Err(error(ErrorKind::UnexpectedArgument, false));
            }
            ArgumentRule::None => None,
            ArgumentRule::Optional => inline_argument,
            ArgumentRule::Required => match inline_argument {
                Some(value) => Some(value),
                None => {
                    let Some(&value) = self.arguments.get(self.next_index) else {
                        return Err(error(ErrorKind::MissingArgument, true));
                    };
                    self.next_index += 1;
                    Some(value)
                }
            },
        };
        Ok(Some(ParsedOption {
            option: spec.option,
            argument,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn parse<'a>(arguments: &'a [&'a [u8]]) -> (Vec<Result<ParsedOption<'a>, ErrorKind>>, usize) {
        let mut parser = Parser::new(arguments);
        let mut events = Vec::new();
        loop {
            match parser.next_option() {
                Ok(Some(option)) => events.push(Ok(option)),
                Err(error) => events.push(Err(error.kind)),
                Ok(None) => break,
            }
            assert!(events.len() <= arguments.len());
        }
        (events, parser.next_index())
    }

    #[test]
    fn every_supported_spelling_accepts_both_dash_forms() {
        for spec in OPTIONS {
            for prefix in [b"-".as_slice(), b"--".as_slice()] {
                let mut token = prefix.to_vec();
                token.extend_from_slice(spec.name);
                if spec.argument != ArgumentRule::None {
                    token.extend_from_slice(b"=value");
                }
                let arguments = [b"engine".as_slice(), token.as_slice(), b"input.tex"];
                let (events, index) = parse(&arguments);
                assert_eq!(
                    events,
                    vec![Ok(ParsedOption {
                        option: spec.option,
                        argument: (spec.argument != ArgumentRule::None)
                            .then_some(b"value".as_slice()),
                    })]
                );
                assert_eq!(index, 2);
            }
        }
    }

    #[test]
    fn required_arguments_consume_empty_dash_and_option_like_values() {
        for value in [b"".as_slice(), b"-", b"--", b"--help", b"a=b", b"\xff"] {
            let arguments = [b"engine".as_slice(), b"-fmt", value, b"input.tex"];
            assert_eq!(
                parse(&arguments),
                (
                    vec![Ok(ParsedOption {
                        option: EngineOption::Fmt,
                        argument: Some(value),
                    })],
                    3
                )
            );
        }
        let arguments = [b"engine".as_slice(), b"--fmt="];
        assert_eq!(
            parse(&arguments).0[0].as_ref().unwrap().argument,
            Some(b"".as_slice())
        );
    }

    #[test]
    fn optional_arguments_only_use_equals() {
        assert_eq!(
            parse(&[b"engine", b"--src-specials", b"par", b"--help"]),
            (
                vec![Ok(ParsedOption {
                    option: EngineOption::SrcSpecials,
                    argument: None
                })],
                2
            )
        );
        for value in [b"".as_slice(), b"par,math", b"\xff"] {
            let mut token = b"--src-specials=".to_vec();
            token.extend_from_slice(value);
            let arguments = [b"engine".as_slice(), token.as_slice()];
            assert_eq!(
                parse(&arguments).0[0].as_ref().unwrap().argument,
                Some(value)
            );
        }
    }

    #[test]
    fn prefixes_exact_aliases_and_errors_are_explicit() {
        let (events, index) = parse(&[
            b"engine",
            b"--file-line-error",
            b"--file-line-e",
            b"-h",
            b"-vers",
            b"--help=",
            b"--not-an-option=x",
            b"--fmt",
        ]);
        assert_eq!(
            events,
            vec![
                Ok(ParsedOption {
                    option: EngineOption::FileLineError,
                    argument: None
                }),
                Err(ErrorKind::Ambiguous),
                Err(ErrorKind::Ambiguous),
                Ok(ParsedOption {
                    option: EngineOption::Version,
                    argument: None
                }),
                Err(ErrorKind::UnexpectedArgument),
                Err(ErrorKind::Unknown),
                Err(ErrorKind::MissingArgument),
            ]
        );
        assert_eq!(index, 8);
    }

    #[test]
    fn end_marker_and_first_non_option_leave_the_remaining_arguments_untouched() {
        for token in [b"input.tex".as_slice(), b"-", b"", b"\\relax"] {
            let arguments = [b"engine".as_slice(), b"--ini", token, b"--help"];
            assert_eq!(parse(&arguments).1, 2);
        }
        assert_eq!(parse(&[b"engine", b"--ini", b"--", b"--help"]).1, 3);
        let arguments = [b"engine".as_slice(), b"input.tex", b"--help"];
        let mut parser = Parser::new(&arguments);
        for _ in 0..3 {
            assert_eq!(parser.next_option(), Ok(None));
            assert_eq!(parser.next_index(), 1);
        }
    }

    #[test]
    fn diagnostics_keep_spelling_and_non_utf8_bytes() {
        let arguments = [
            b"engine".as_slice(),
            b"--wat=\xff",
            b"--out=x",
            b"--help=",
            b"-fmt",
        ];
        let mut parser = Parser::new(&arguments);
        let expected = [
            b"engine: unrecognized option `--wat=\xff'\n".as_slice(),
            b"engine: option `--out' is ambiguous\n",
            b"engine: option `--help' doesn't allow an argument\n",
            b"engine: option `-fmt' requires an argument\n",
        ];
        for message in expected {
            assert_eq!(
                parser.next_option().unwrap_err().diagnostic(b"/tmp/engine"),
                message
            );
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(128))]

        #[test]
        fn arbitrary_arguments_make_bounded_progress(
            mut arguments in prop::collection::vec(prop::collection::vec(1u8..=255, 0..96), 0..24),
        ) {
            arguments.insert(0, b"engine".to_vec());
            let borrowed = arguments.iter().map(Vec::as_slice).collect::<Vec<_>>();
            let mut parser = Parser::new(&borrowed);
            let mut previous = parser.next_index();
            for _ in 0..=arguments.len() {
                let next = parser.next_option();
                prop_assert!(parser.next_index() <= arguments.len());
                if next == Ok(None) {
                    prop_assert_eq!(parser.next_option(), Ok(None));
                    break;
                }
                prop_assert!(parser.next_index() > previous);
                previous = parser.next_index();
            }
            prop_assert_eq!(parser.next_option(), Ok(None));
        }

        #[test]
        fn exact_option_argument_model(
            option_index in 0..OPTIONS.len(),
            double_dash in any::<bool>(),
            equals in any::<bool>(),
            value in prop::collection::vec(1u8..=255, 0..64),
        ) {
            let spec = OPTIONS[option_index];
            let mut token = if double_dash { b"--".to_vec() } else { b"-".to_vec() };
            token.extend_from_slice(spec.name);
            let mut owned = vec![b"engine".to_vec()];
            if equals {
                token.push(b'=');
                token.extend_from_slice(&value);
            }
            owned.push(token);
            if !equals && spec.argument == ArgumentRule::Required {
                owned.push(value.clone());
            }
            owned.push(b"input.tex".to_vec());
            owned.push(b"--help".to_vec());
            let borrowed = owned.iter().map(Vec::as_slice).collect::<Vec<_>>();
            let (events, index) = parse(&borrowed);
            if equals && spec.argument == ArgumentRule::None {
                prop_assert_eq!(events, vec![Err(ErrorKind::UnexpectedArgument)]);
            } else {
                let argument = match spec.argument {
                    ArgumentRule::None => None,
                    ArgumentRule::Optional if !equals => None,
                    _ => Some(value.as_slice()),
                };
                prop_assert_eq!(events, vec![Ok(ParsedOption { option: spec.option, argument })]);
            }
            prop_assert_eq!(index, if !equals && spec.argument == ArgumentRule::Required { 3 } else { 2 });
        }
    }

    #[cfg(target_os = "macos")]
    mod native_oracle {
        use super::*;
        use std::ffi::{c_char, c_int, CStr, CString};
        use std::sync::Mutex;

        #[repr(C)]
        struct NativeOption {
            name: *const c_char,
            has_arg: c_int,
            flag: *mut c_int,
            val: c_int,
        }

        extern "C" {
            fn getopt_long_only(
                argc: c_int,
                argv: *const *mut c_char,
                shortopts: *const c_char,
                longopts: *const NativeOption,
                longind: *mut c_int,
            ) -> c_int;
            static mut optarg: *mut c_char;
            static mut optind: c_int;
            static mut opterr: c_int;
            static mut optreset: c_int;
        }

        static LOCK: Mutex<()> = Mutex::new(());

        fn compare(arguments: &[Vec<u8>]) {
            let _lock = LOCK.lock().unwrap_or_else(|error| error.into_inner());
            let storage = arguments
                .iter()
                .map(|arg| CString::new(arg.as_slice()).unwrap())
                .collect::<Vec<_>>();
            let argv = storage
                .iter()
                .map(|arg| arg.as_ptr().cast_mut())
                .chain(std::iter::once(std::ptr::null_mut()))
                .collect::<Vec<_>>();
            let names = OPTIONS
                .iter()
                .map(|spec| CString::new(spec.name).unwrap())
                .collect::<Vec<_>>();
            let native_options = OPTIONS
                .iter()
                .enumerate()
                .map(|(index, spec)| NativeOption {
                    name: names[index].as_ptr(),
                    has_arg: match spec.argument {
                        ArgumentRule::None => 0,
                        ArgumentRule::Required => 1,
                        ArgumentRule::Optional => 2,
                    },
                    flag: std::ptr::null_mut(),
                    val: index as c_int + 256,
                })
                .chain(std::iter::once(NativeOption {
                    name: std::ptr::null(),
                    has_arg: 0,
                    flag: std::ptr::null_mut(),
                    val: 0,
                }))
                .collect::<Vec<_>>();
            let borrowed = arguments.iter().map(Vec::as_slice).collect::<Vec<_>>();
            let mut parser = Parser::new(&borrowed);
            // SAFETY: all native calls are serialized, argv has its null
            // sentinel, and optreset resets libc's private scanning pointer.
            unsafe {
                let saved_opterr = opterr;
                optind = 1;
                optreset = 1;
                opterr = 0;
                loop {
                    let mut index = -1;
                    let native = getopt_long_only(
                        argv.len() as c_int - 1,
                        argv.as_ptr(),
                        c"+".as_ptr(),
                        native_options.as_ptr(),
                        &mut index,
                    );
                    let rust = parser.next_option();
                    assert_eq!(
                        parser.next_index(),
                        optind as usize,
                        "arguments {arguments:?}"
                    );
                    match rust {
                        Ok(None) => {
                            assert_eq!(native, -1, "arguments {arguments:?}");
                            break;
                        }
                        Err(_) => assert_eq!(native, b'?' as c_int, "arguments {arguments:?}"),
                        Ok(Some(parsed)) => {
                            assert_eq!(native, index + 256, "arguments {arguments:?}");
                            assert_eq!(
                                parsed.option, OPTIONS[index as usize].option,
                                "arguments {arguments:?}"
                            );
                            let native_arg = if optarg.is_null() {
                                None
                            } else {
                                Some(CStr::from_ptr(optarg).to_bytes())
                            };
                            assert_eq!(parsed.argument, native_arg, "arguments {arguments:?}");
                        }
                    }
                }
                opterr = saved_opterr;
                optind = 1;
                optreset = 1;
                optarg = std::ptr::null_mut();
            }
        }

        #[test]
        fn native_boundary_cases_and_every_prefix_agree() {
            for spec in OPTIONS {
                for prefix_len in 0..=spec.name.len() {
                    for dash in [b"-".as_slice(), b"--".as_slice()] {
                        let mut token = dash.to_vec();
                        token.extend_from_slice(&spec.name[..prefix_len]);
                        for suffix in [b"".as_slice(), b"=", b"=value"] {
                            let mut option = token.clone();
                            option.extend_from_slice(suffix);
                            compare(&[
                                b"engine".to_vec(),
                                option,
                                b"value".to_vec(),
                                b"--help".to_vec(),
                            ]);
                        }
                    }
                }
            }
            for option in [
                b"--fmt".as_slice(),
                b"--unknown",
                b"---help",
                b"--=x",
                b"-=x",
                b"-+",
                b"-:",
                b"--help=",
                b"--src-specials",
                b"--file-line-e",
            ] {
                compare(&[b"engine".to_vec(), option.to_vec()]);
            }
            compare(&[
                b"engine".to_vec(),
                b"--fmt".to_vec(),
                b"--".to_vec(),
                b"--ini".to_vec(),
                b"--".to_vec(),
                b"--help".to_vec(),
            ]);
            compare(&[b"engine".to_vec(), Vec::new(), b"--help".to_vec()]);
        }

        proptest! {
            #![proptest_config(ProptestConfig::with_cases(128))]

            #[test]
            fn bounded_option_sequences_agree_with_native(
                tokens in prop::collection::vec((0..OPTIONS.len(), any::<bool>(), any::<bool>(), prop::collection::vec(1u8..=255, 0..32)), 0..16),
                trailing in prop::collection::vec(prop::collection::vec(1u8..=255, 0..48), 0..8),
            ) {
                let mut arguments = vec![b"engine".to_vec()];
                for (index, double_dash, equals, value) in tokens {
                    let mut token = if double_dash { b"--".to_vec() } else { b"-".to_vec() };
                    token.extend_from_slice(OPTIONS[index].name);
                    if equals { token.push(b'='); token.extend_from_slice(&value); }
                    arguments.push(token);
                    if !equals && OPTIONS[index].argument == ArgumentRule::Required { arguments.push(value); }
                }
                arguments.extend(trailing);
                compare(&arguments);
            }
        }
    }
}
