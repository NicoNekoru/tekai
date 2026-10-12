//! Self-contained exact typesetting engine for Tekai.
//!
//! The CLI and standalone runner link this crate as a Rust library. The
//! core and backend calls use Rust module interfaces and invocation-owned state.
//! This crate does not produce a C-linkable archive or export a C entrypoint.

#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    overflowing_literals,
    path_statements,
    unreachable_code,
    unused_assignments,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_unsafe,
    unused_variables,
    useless_ptr_null_checks
)]
#![allow(clippy::all)]

#[cfg(panic = "abort")]
compile_error!("tekai-engine requires panic=unwind for controlled exits and invocation teardown");

mod generated {
    pub(crate) mod backend {
        pub(crate) mod avl;
        pub(crate) mod avlstuff;
        pub(crate) mod epdf;
        pub(crate) mod mapfile;
        pub(crate) mod pkin;
        pub(crate) mod subfont;
        pub(crate) mod tounicode;
        pub(crate) mod vfpacket;
        pub(crate) mod writeenc;
        pub(crate) mod writefont;
        pub(crate) mod writeimg;
        pub(crate) mod writejbig2;
        pub(crate) mod writejpg;
        pub(crate) mod writepng;
        pub(crate) mod writet1;
        pub(crate) mod writet3;
        pub(crate) mod writettf;
        pub(crate) mod writezip;
    }
    pub(crate) mod pdftex0;
    pub(crate) mod pdftex_pool;
    pub(crate) mod pdftexextra;
    pub(crate) mod pdftexini;
}

mod allocation;
mod backend_grammar;
mod backend_parse;
mod backend_types;
mod byte_string;
mod cache;
mod checked_width;
#[cfg(all(feature = "cli-supervisor", unix))]
pub mod cli_supervisor;
mod control;
mod core_types;
mod directory_graph;
mod embedded;
mod engine_options;
pub mod engine_profile;
mod environment;
pub mod file_identity;
mod float_parse;
mod integer_parse;
mod invocation;
mod invocation_state;
mod ipc;
mod jpeg_exif;
pub mod kpathsea;
mod kpathsea_types;
pub mod lookup;
mod md5;
mod openclose;
mod pdftoepdf;
mod pngshim;
mod posix_regex;
mod process_group;
mod regex_session;
pub mod runtime;
pub mod search;
mod stdio;
mod support;
mod synctex;
mod termination;
mod time;
mod ttf_name;
mod ttf_post;
mod utils;
mod web2c;
mod working_directory;
mod xpdf;

pub use control::InterruptToken;
pub use invocation::{EngineCompletion, InvocationError, InvocationOptions};
#[cfg(test)]
use libz_rs_sys as zlib;
pub use stdio::PipeProcessPolicy;

/// Run the engine with owned argument bytes, including argv0.
///
/// Every call owns its core state, buffers, caches, streams and argument storage.
/// Repeated calls and independent threads do not share mutable engine state.
/// Nested calls on the same thread are rejected. Fatal and help/version exits
/// return without terminating the host. Unexpected Rust panics propagate after
/// invocation teardown. Host environment, directory and signal handlers are not
/// changed.
///
/// # Safety
///
/// The translated engine still uses unchecked pointers and format data. The
/// caller must provide trusted input and format files and must not call internal
/// engine functions outside their invocation scope. Invocation ownership is not
/// a sandbox for untrusted TeX or shell commands.
/// Recovery requires unwind-enabled builds. The host must not independently
/// reap engine pipe children or configure SIGCHLD to reap them automatically.
pub unsafe fn run_from_args(arguments: Vec<Vec<u8>>) -> Result<EngineCompletion, InvocationError> {
    unsafe { run_with_options(arguments, InvocationOptions::default()) }
}

/// Run with an invocation-local directory, environment and interrupt token.
///
/// # Safety
/// The same trusted-input requirements as [`run_from_args`] apply.
pub unsafe fn run_with_options(
    arguments: Vec<Vec<u8>>,
    options: InvocationOptions,
) -> Result<EngineCompletion, InvocationError> {
    if invocation_state::is_active() {
        return Err(InvocationError::NestedInvocation);
    }
    let invocation = Invocation::new(arguments, &options)?;
    unsafe { invocation.run(options) }
}

/// Run with fixed diagnostic observations, including complete owner teardown.
/// The ordinary entry does not allocate a collector or read profiling clocks.
/// Errors retain a snapshot of reached phases. Unexpected panics still unwind
/// and propagate, without returning a snapshot or emitting any diagnostic.
///
/// # Safety
/// The same trusted-input requirements as [`run_from_args`] apply.
pub unsafe fn run_profiled_with_options(
    arguments: Vec<Vec<u8>>,
    options: InvocationOptions,
) -> (
    Result<EngineCompletion, InvocationError>,
    engine_profile::EngineProfile,
) {
    profiled_invocation(arguments, options, |invocation, options| unsafe {
        invocation.run_borrowed(options)
    })
}

fn profiled_invocation(
    arguments: Vec<Vec<u8>>,
    options: InvocationOptions,
    operation: impl FnOnce(
        &mut Invocation,
        InvocationOptions,
    ) -> Result<EngineCompletion, InvocationError>,
) -> (
    Result<EngineCompletion, InvocationError>,
    engine_profile::EngineProfile,
) {
    use engine_profile::{Collector, Phase};
    let profile = Collector::new();
    if invocation_state::is_active() {
        profile.finish(true);
        return (
            Err(InvocationError::NestedInvocation),
            profile.snapshot("setup_error"),
        );
    }
    let mut invocation =
        match Invocation::new_with_profile(arguments, &options, Some(profile.clone())) {
            Ok(invocation) => invocation,
            Err(error) => {
                // Partially constructed fields have already dropped inside new.
                profile.finish(true);
                return (Err(error), profile.snapshot("setup_error"));
            }
        };
    profile.transition(Phase::InvocationSetup, false);
    let result = operation(&mut invocation, options);
    let status = match &result {
        Ok(EngineCompletion::Clean) => "success",
        Ok(EngineCompletion::Warnings) => "warnings",
        Ok(EngineCompletion::Errors) => "errors",
        Err(InvocationError::StreamTeardown { .. }) => "teardown_error",
        Err(_) => "setup_error",
    };
    profile.transition(Phase::OwnerDrop, result.is_err());
    drop(invocation);
    // The collector is outside every owner. No span or resource retains it
    // after teardown, and the returned snapshot contains no borrowed payload.
    debug_assert_eq!(std::rc::Rc::strong_count(&profile), 1);
    profile.finish(false);
    (result, profile.snapshot(status))
}

// Field order is teardown order. No resource may outlive the raw-buffer arena,
// and argument bytes outlive core aliases. Routing scopes always end first.
struct Invocation {
    core: std::pin::Pin<Box<invocation_state::InvocationState>>,
    compression: std::pin::Pin<Box<generated::backend::writezip::CompressionSession>>,
    streams: std::pin::Pin<Box<stdio::StreamSession>>,
    regex: std::pin::Pin<Box<regex_session::RegexSession>>,
    environment: std::pin::Pin<Box<environment::EnvironmentSession>>,
    directory: std::pin::Pin<Box<working_directory::WorkingDirectory>>,
    time: Option<std::pin::Pin<Box<time::TimeSession>>>,
    runtime: Option<std::pin::Pin<Box<runtime::RuntimeSession>>>,
    index: std::pin::Pin<Box<kpathsea::IndexSession>>,
    lookup: std::pin::Pin<Box<lookup::LookupOwner>>,
    png: std::pin::Pin<Box<pngshim::PngSession>>,
    sync: std::pin::Pin<Box<synctex::SyncTexSession>>,
    xpdf: std::pin::Pin<Box<xpdf::XpdfSession>>,
    inclusion: std::pin::Pin<Box<pdftoepdf::PdfInclusionSession>>,
    control: std::pin::Pin<Box<control::ControlSession>>,
    ipc: std::pin::Pin<Box<ipc::IpcSession>>,
    arena: std::pin::Pin<Box<allocation::AllocationArena>>,
    arguments: invocation::OwnedArguments,
}

impl Invocation {
    fn new(arguments: Vec<Vec<u8>>, options: &InvocationOptions) -> Result<Self, InvocationError> {
        Self::new_with_profile(arguments, options, None)
    }

    fn new_with_profile(
        arguments: Vec<Vec<u8>>,
        options: &InvocationOptions,
        profile: Option<std::rc::Rc<engine_profile::Collector>>,
    ) -> Result<Self, InvocationError> {
        use invocation::resource;
        let arguments = invocation::OwnedArguments::new(arguments)?;
        Ok(Self {
            core: invocation_state::InvocationState::new(),
            compression: generated::backend::writezip::CompressionSession::new(),
            streams: stdio::StreamSession::new_with_pipe_policy(options.pipe_process_policy),
            regex: regex_session::RegexSession::new(),
            environment: resource("environment", environment::EnvironmentSession::new())?,
            directory: resource(
                "working directory",
                working_directory::WorkingDirectory::new(options.working_directory.clone()),
            )?,
            time: None,
            runtime: None,
            index: kpathsea::IndexSession::new(),
            lookup: lookup::LookupOwner::new(),
            png: pngshim::PngSession::new(),
            sync: synctex::SyncTexSession::new(),
            xpdf: xpdf::XpdfSession::new(),
            inclusion: pdftoepdf::PdfInclusionSession::new(),
            control: match profile {
                Some(profile) => control::ControlSession::new_with_profile(
                    options.interrupt.clone(),
                    Some(profile),
                ),
                None => control::ControlSession::new(options.interrupt.clone()),
            },
            ipc: ipc::IpcSession::new(),
            arena: allocation::AllocationArena::new(),
            arguments,
        })
    }

    unsafe fn run(
        mut self,
        options: InvocationOptions,
    ) -> Result<EngineCompletion, InvocationError> {
        unsafe { self.run_borrowed(options) }
    }

    unsafe fn run_borrowed(
        &mut self,
        options: InvocationOptions,
    ) -> Result<EngineCompletion, InvocationError> {
        self.run_with_operation_borrowed(options, |argc, argv| unsafe {
            (*generated::pdftexextra::state()).synctexoption = ::core::ffi::c_int::MAX;
            generated::pdftexextra::maininit(argc, argv);
            generated::pdftexini::mainbody()
        })
    }

    #[cfg(test)]
    fn run_with_operation(
        mut self,
        options: InvocationOptions,
        operation: impl FnOnce(std::ffi::c_int, *mut *mut std::ffi::c_char) -> EngineCompletion,
    ) -> Result<EngineCompletion, InvocationError> {
        self.run_with_operation_borrowed(options, operation)
    }

    fn run_with_operation_borrowed(
        &mut self,
        options: InvocationOptions,
        operation: impl FnOnce(std::ffi::c_int, *mut *mut std::ffi::c_char) -> EngineCompletion,
    ) -> Result<EngineCompletion, InvocationError> {
        use invocation::resource;
        let Self {
            core,
            compression,
            streams,
            regex,
            environment,
            directory,
            time,
            runtime,
            index,
            lookup,
            png,
            sync,
            xpdf,
            inclusion,
            control,
            ipc,
            arena,
            arguments,
        } = self;
        let (argc, argv) = arguments.engine_args();
        resource("core state", core.as_mut().scope(|| arena.as_mut().scope(|| {
            resource("environment", environment::EnvironmentSession::scope(environment.as_mut(), || {
                for (name, value) in options.environment {
                    resource("environment override", match value {
                        Some(value) => environment::set(&name, &value),
                        None => environment::remove(&name),
                    })?;
                }
                resource("working directory", directory.as_mut().scope(|| {
                    *runtime = Some(runtime::RuntimeSession::new());
                    *time = Some(resource("time", time::TimeSession::new())?);
                    resource("time", time::TimeSession::scope(time.as_mut().unwrap().as_mut(), || {
                        resource("interrupt", control::ControlSession::scope(control.as_mut(), || {
                            resource("IPC", ipc.as_mut().scope(|| {
                                resource("PDF compression", generated::backend::writezip::CompressionSession::scope(compression.as_mut(), || {
                                    resource("regex", regex_session::RegexSession::scope(regex.as_mut(), || {
                                        runtime::RuntimeSession::scope(runtime.as_mut().unwrap().as_mut(), || {
                                            kpathsea::IndexSession::scope(index.as_mut(), || {
                                                lookup::LookupOwner::scope(lookup.as_mut(), || {
                                                    pngshim::PngSession::scope(png.as_mut(), || {
                                                        synctex::SyncTexSession::scope(sync.as_mut(), || {
                                                            xpdf::XpdfSession::scope(xpdf.as_mut(), || {
                                                                pdftoepdf::PdfInclusionSession::scope(inclusion.as_mut(), || {
                                                                    let completion = streams.as_mut().scope(|| {
                                                                        engine_profile::transition(engine_profile::Phase::EngineBody, false);
                                                                        let completion = termination::catch(|| operation(argc, argv)).unwrap_or_else(|completion| completion);
                                                                        engine_profile::transition(engine_profile::Phase::Cleanup, completion == EngineCompletion::Errors);
                                                                        unsafe { stdio::flush_for_exit() };
                                                                        completion
                                                                    });
                                                                    streams.as_mut().finish().map_err(|error| InvocationError::StreamTeardown { message: error.to_string() })?;
                                                                    Ok(completion)
                                                                })
                                                            })
                                                        })
                                                    })
                                                })
                                            })
                                        })
                                    }))?
                                }))?
                            }))?
                        }))?
                    }))?
                }))?
            }))?
        })))?
    }
}

#[cfg(test)]
mod entry_tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    fn profile_options() -> InvocationOptions {
        InvocationOptions {
            environment: vec![("TZ".into(), Some("UTC0".into()))],
            ..InvocationOptions::default()
        }
    }

    fn phase<'a>(
        profile: &'a engine_profile::EngineProfile,
        name: &str,
    ) -> &'a engine_profile::PhaseObservation {
        profile
            .phases()
            .iter()
            .find(|phase| phase.name == name)
            .unwrap()
    }

    #[test]
    fn profiled_public_typed_exit_returns_after_all_owners_drop_and_is_independent() {
        for _ in 0..2 {
            let (result, profile) = unsafe {
                run_profiled_with_options(
                    vec![b"pdflatex".to_vec(), b"--version".to_vec()],
                    profile_options(),
                )
            };
            assert_eq!(result, Ok(EngineCompletion::Clean));
            assert_eq!(profile.status(), "success");
            assert!(!invocation_state::is_active());
            assert!(control::profile_collector().is_none());
            for observed in &profile.phases()[..5] {
                assert_eq!(observed.calls, 1, "{}", observed.name);
                assert_eq!(observed.failed_calls, 0, "{}", observed.name);
            }
            assert_eq!(
                profile.phases()[..5]
                    .iter()
                    .map(|phase| phase.elapsed_ns.unwrap())
                    .sum::<u64>(),
                profile.elapsed_ns()
            );
            assert!(profile.counters().iter().all(|counter| counter.value == 0));
        }
    }

    #[test]
    fn profiled_setup_errors_leave_unreached_phases_unavailable() {
        let (result, profile) = unsafe { run_profiled_with_options(Vec::new(), profile_options()) };
        assert_eq!(result, Err(InvocationError::MissingProgramName));
        assert_eq!(profile.status(), "setup_error");
        assert_eq!(phase(&profile, "owner_construction").failed_calls, 1);
        assert!(phase(&profile, "engine_body").elapsed_ns.is_none());
        assert!(phase(&profile, "owner_drop").elapsed_ns.is_none());

        let mut options = profile_options();
        options
            .environment
            .push(("invalid=name".into(), Some("value".into())));
        let (result, profile) =
            unsafe { run_profiled_with_options(vec![b"pdflatex".to_vec()], options) };
        assert!(matches!(result, Err(InvocationError::ResourceSetup { .. })));
        assert_eq!(profile.status(), "setup_error");
        assert_eq!(phase(&profile, "invocation_setup").failed_calls, 1);
        assert_eq!(phase(&profile, "owner_drop").calls, 1);
        assert!(phase(&profile, "engine_body").elapsed_ns.is_none());
        assert!(phase(&profile, "cleanup").elapsed_ns.is_none());
        assert!(!invocation_state::is_active());
        assert!(control::profile_collector().is_none());
    }

    #[test]
    fn profiled_fatal_exit_and_unexpected_panic_keep_existing_cleanup_rules() {
        let run = |panic: bool| {
            profiled_invocation(
                vec![b"pdflatex".to_vec()],
                profile_options(),
                |invocation, options| {
                    invocation.run_with_operation_borrowed(options, |_, _| {
                        assert!(control::profile_collector().is_some());
                        let stream = unsafe { stdio::tmpfile() };
                        assert!(!stream.is_null());
                        assert_eq!(
                            unsafe { stdio::fwrite(c"unfinished".as_ptr().cast(), 1, 10, stream) },
                            10
                        );
                        let mut observed = engine_profile::span(engine_profile::Phase::PngDecode);
                        engine_profile::count(engine_profile::Counter::DecodeCalls, 1);
                        observed.success();
                        drop(observed);
                        if panic {
                            std::panic::resume_unwind(Box::new(0x51a7_u64));
                        }
                        termination::exit(1)
                    })
                },
            )
        };
        let (result, profile) = run(false);
        assert_eq!(result, Ok(EngineCompletion::Errors));
        assert_eq!(profile.status(), "errors");
        assert_eq!(phase(&profile, "engine_body").failed_calls, 1);
        assert_eq!(phase(&profile, "cleanup").calls, 1);
        assert_eq!(phase(&profile, "owner_drop").calls, 1);
        assert_eq!(phase(&profile, "png_decode").calls, 1);
        let payload = catch_unwind(AssertUnwindSafe(|| run(true))).unwrap_err();
        assert_eq!(payload.downcast_ref::<u64>(), Some(&0x51a7));
        assert!(!invocation_state::is_active());
        assert!(control::profile_collector().is_none());
        let invocation = Invocation::new(vec![b"pdflatex".to_vec()], &profile_options()).unwrap();
        assert_eq!(
            invocation.run_with_operation(profile_options(), |_, _| {
                assert!(control::profile_collector().is_none());
                EngineCompletion::Clean
            }),
            Ok(EngineCompletion::Clean)
        );
    }

    #[test]
    fn nested_public_entry_is_rejected_without_replacing_the_active_core() {
        invocation_state::with_fixture(|| {
            let before = unsafe { invocation_state::current() };
            assert_eq!(
                unsafe { run_from_args(vec![b"pdflatex".to_vec(), b"--help".to_vec()]) },
                Err(InvocationError::NestedInvocation)
            );
            assert_eq!(unsafe { invocation_state::current() }, before);
        });
        assert!(!invocation_state::is_active());
    }

    #[test]
    fn unexpected_panic_tears_down_the_complete_invocation_and_preserves_payload() {
        let options = InvocationOptions::default();
        let invocation = Invocation::new(vec![b"pdflatex".to_vec()], &options).unwrap();
        let payload = catch_unwind(AssertUnwindSafe(|| {
            invocation.run_with_operation(options, |argc, argv| unsafe {
                generated::pdftexextra::maininit(argc, argv);
                assert!(!allocation::malloc(73).is_null());
                let stream = stdio::tmpfile();
                assert!(!stream.is_null());
                assert_eq!(stdio::fwrite(c"unclosed".as_ptr().cast(), 1, 8, stream), 8);
                (*generated::pdftexextra::state()).history = 3;
                std::panic::resume_unwind(Box::new(0x51a7_u64));
            })
        }))
        .unwrap_err();
        assert_eq!(*payload.downcast::<u64>().unwrap(), 0x51a7);
        assert!(!invocation_state::is_active());
        let options = InvocationOptions::default();
        let next = Invocation::new(vec![b"pdflatex".to_vec()], &options).unwrap();
        assert_eq!(
            next.run_with_operation(options, |_, _| {
                assert_eq!(unsafe { (*generated::pdftexextra::state()).history }, 0);
                assert!(!stdio::stdout().is_null());
                EngineCompletion::Clean
            })
            .unwrap(),
            EngineCompletion::Clean
        );
    }
}
