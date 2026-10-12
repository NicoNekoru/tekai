//! Rust-owned web2c file opening and configuration shims.

pub(crate) use crate::kpathsea::{kpse_absolute_p, kpse_find_file, kpse_var_value};
pub use crate::support::{dir_p, xdirname, xgetcwd};

use crate::stdio::Stream as FILE;
use crate::support::xfopen;
use libc::{c_char, c_int, c_uint, c_void, size_t};
use std::ffi::{CStr, CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr;

const TRUE: c_int = 1;
const FALSE: c_int = 0;
const KPSE_TFM_FORMAT: c_int = 3;
const KPSE_OCP_FORMAT: c_int = 19;
const KPSE_OFM_FORMAT: c_int = 20;
const KPSE_TEX_FORMAT: c_int = 26;
const KPSE_VF_FORMAT: c_int = 33;

#[inline]
unsafe fn fast_fgetc(stream: *mut FILE) -> c_int {
    unsafe { crate::stdio::fgetc(stream) }
}

fn c_strlen(s: *const c_char) -> usize {
    unsafe { crate::byte_string::strlen(s) }
}

fn c_bytes(s: *const c_char) -> &'static [u8] {
    unsafe { CStr::from_ptr(s).to_bytes() }
}

unsafe fn alloc_copy(bytes: &[u8]) -> *mut c_char {
    let ptr = unsafe { xmalloc(bytes.len() + 1) as *mut c_char };
    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), ptr as *mut u8, bytes.len());
        *ptr.add(bytes.len()) = 0;
    }
    ptr
}

unsafe fn xstrdup_ptr(s: *const c_char) -> *mut c_char {
    unsafe { alloc_copy(c_bytes(s)) }
}

unsafe fn concat3_ptr(a: *const c_char, b: *const c_char, c: *const c_char) -> *mut c_char {
    let a_bytes = c_bytes(a);
    let b_bytes = if b.is_null() { &[][..] } else { c_bytes(b) };
    let c_bytes = if c.is_null() { &[][..] } else { c_bytes(c) };
    let len = a_bytes.len() + b_bytes.len() + c_bytes.len();
    let out = unsafe { xmalloc(len + 1) as *mut u8 };
    unsafe {
        ptr::copy_nonoverlapping(a_bytes.as_ptr(), out, a_bytes.len());
        ptr::copy_nonoverlapping(b_bytes.as_ptr(), out.add(a_bytes.len()), b_bytes.len());
        ptr::copy_nonoverlapping(
            c_bytes.as_ptr(),
            out.add(a_bytes.len() + b_bytes.len()),
            c_bytes.len(),
        );
        *out.add(len) = 0;
    }
    out as *mut c_char
}

unsafe fn set_nameoffile_from_c_str(src: *const c_char) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let len = c_strlen(src);
    unsafe {
        crate::allocation::free((*__tekai_state_generated_pdftexextra).nameoffile as *mut c_void);
        (*__tekai_state_generated_pdftexextra).nameoffile = xmalloc(len + 2) as *mut u8;
        ptr::copy_nonoverlapping(
            src as *const u8,
            (*__tekai_state_generated_pdftexextra).nameoffile.add(1),
            len + 1,
        );
        (*__tekai_state_generated_pdftexextra).namelength = len as c_int;
    }
}

unsafe fn nameoffile_body() -> *const c_char {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe { (*__tekai_state_generated_pdftexextra).nameoffile.add(1) as *const c_char }
}

unsafe fn is_name_prefixed_dot_slash() -> bool {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        *(*__tekai_state_generated_pdftexextra).nameoffile.add(1) == b'.'
            && *(*__tekai_state_generated_pdftexextra).nameoffile.add(2) == b'/'
    }
}

unsafe fn strip_leading_dot_slash_in_place(fname: *mut c_char) {
    let mut i = 0usize;
    unsafe {
        while *fname.add(i + 2) != 0 {
            *fname.add(i) = *fname.add(i + 2);
            i += 1;
        }
        *fname.add(i) = 0;
    }
}

unsafe fn record_name(prefix: *const c_char, name: *const c_char) {
    let __tekai_state_openclose: *mut crate::openclose::State =
        unsafe { crate::openclose::state() };

    unsafe {
        if (*__tekai_state_openclose).recorder_enabled == 0 {
            return;
        }
        if (*__tekai_state_openclose).RECORDER_FILE.is_null() {
            recorder_start();
        }
        if !(*__tekai_state_openclose).RECORDER_FILE.is_null() {
            crate::stdio::fprintf_args(
                (*__tekai_state_openclose).RECORDER_FILE,
                c"%s %s\n".as_ptr(),
                &[
                    crate::utils::PrintfArg::from(prefix),
                    crate::utils::PrintfArg::from(name),
                ],
            );
        }
    }
}

unsafe fn recorder_start() {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    unsafe { recorder_start_with_nonce(nonce) };
}

fn open_recorder_temporary(output_directory: Option<&OsStr>, nonce: u128) -> (CString, *mut FILE) {
    let mut attempt = 0;
    loop {
        let basename = format!("pdftex{}-{nonce}-{attempt}.fls", std::process::id());
        let path = match output_directory {
            Some(directory) => Path::new(directory).join(basename),
            None => std::path::PathBuf::from(basename),
        };
        // Both inputs originate from NUL-terminated names. New components
        // contain only decimal digits and fixed ASCII punctuation.
        let name =
            CString::new(path.as_os_str().as_bytes()).expect("recorder name contains no NUL");
        let file = unsafe { crate::stdio::fopen(name.as_ptr(), c"wbx".as_ptr()) };
        // Exclusive creation is the authority. Even identical nonces on
        // independent threads cannot truncate another job's recorder.
        if !file.is_null() || crate::stdio::errno() != libc::EEXIST || attempt == 99 {
            return (name, file);
        }
        attempt += 1;
    }
}

unsafe fn recorder_start_with_nonce(nonce: u128) {
    let __tekai_state_openclose: *mut crate::openclose::State =
        unsafe { crate::openclose::state() };

    unsafe {
        let directory = if (*__tekai_state_openclose).output_directory.is_null() {
            None
        } else {
            Some(OsStr::from_bytes(c_bytes(
                (*__tekai_state_openclose).output_directory,
            )))
        };
        let (name, file) = open_recorder_temporary(directory, nonce);
        if file.is_null() {
            crate::stdio::perror(name.as_ptr());
            crate::stdio::exit(libc::EXIT_FAILURE);
        }
        (*__tekai_state_openclose).RECORDER_NAME = xstrdup_ptr(name.as_ptr());
        (*__tekai_state_openclose).RECORDER_FILE = file;
        let cwd = xgetcwd();
        if !cwd.is_null() {
            crate::stdio::fprintf_args(
                (*__tekai_state_openclose).RECORDER_FILE,
                c"PWD %s\n".as_ptr(),
                &[crate::utils::PrintfArg::from(cwd)],
            );
            crate::allocation::free(cwd as *mut c_void);
        }
    }
}

pub unsafe fn recorder_change_filename(new_name: *mut c_char) {
    let __tekai_state_openclose: *mut crate::openclose::State =
        unsafe { crate::openclose::state() };

    unsafe {
        if (*__tekai_state_openclose).RECORDER_FILE.is_null() {
            return;
        }

        let mut final_name = new_name;
        let mut allocated = false;
        if !(*__tekai_state_openclose).output_directory.is_null() {
            final_name = concat3_ptr(
                (*__tekai_state_openclose).output_directory,
                c"/".as_ptr(),
                new_name,
            );
            allocated = true;
        }

        let source = Path::new(OsStr::from_bytes(c_bytes(
            (*__tekai_state_openclose).RECORDER_NAME,
        )));
        let destination = Path::new(OsStr::from_bytes(c_bytes(final_name)));
        let _ = crate::working_directory::with_resolved(source, |source| {
            crate::working_directory::with_resolved(destination, |destination| {
                std::fs::rename(source, destination)
            })
        });
        crate::allocation::free((*__tekai_state_openclose).RECORDER_NAME as *mut c_void);
        (*__tekai_state_openclose).RECORDER_NAME = xstrdup_ptr(final_name);

        if allocated {
            crate::allocation::free(final_name as *mut c_void);
        }
    }
}

pub unsafe fn recorder_record_input(name: *const c_char) {
    unsafe { record_name(c"INPUT".as_ptr(), name) }
}

pub unsafe fn recorder_record_output(name: *const c_char) {
    unsafe { record_name(c"OUTPUT".as_ptr(), name) }
}

pub(crate) unsafe fn open_input(
    f_ptr: *mut *mut FILE,
    filefmt: c_int,
    fopen_mode: *const c_char,
) -> c_int {
    let __tekai_state_openclose: *mut crate::openclose::State =
        unsafe { crate::openclose::state() };

    unsafe {
        *f_ptr = ptr::null_mut();
        if !(*__tekai_state_openclose).fullnameoffile.is_null() {
            crate::allocation::free((*__tekai_state_openclose).fullnameoffile as *mut c_void);
            (*__tekai_state_openclose).fullnameoffile = ptr::null_mut();
        }

        let input_name = nameoffile_body();
        if !(*__tekai_state_openclose).output_directory.is_null()
            && kpse_absolute_p(input_name, FALSE) == 0
        {
            let fname = concat3_ptr(
                (*__tekai_state_openclose).output_directory,
                c"/".as_ptr(),
                input_name,
            );
            *f_ptr = crate::stdio::fopen(fname, fopen_mode);
            if !(*f_ptr).is_null() && dir_p(fname) != 0 {
                crate::stdio::fclose(*f_ptr);
                *f_ptr = ptr::null_mut();
            }
            if !(*f_ptr).is_null() {
                set_nameoffile_from_c_str(fname);
                (*__tekai_state_openclose).fullnameoffile = fname;
            } else {
                crate::allocation::free(fname as *mut c_void);
            }
        }

        if (*f_ptr).is_null() {
            if filefmt < 0 {
                *f_ptr = crate::stdio::fopen(nameoffile_body(), fopen_mode);
            } else {
                let must_exist = ((filefmt != KPSE_TEX_FORMAT
                    || (*__tekai_state_openclose).texinputtype != 0)
                    && filefmt != KPSE_VF_FORMAT) as c_int;
                let fname = kpse_find_file(nameoffile_body(), filefmt as c_uint, must_exist);
                if !fname.is_null() {
                    (*__tekai_state_openclose).fullnameoffile = xstrdup_ptr(fname);
                    if *fname == b'.' as c_char
                        && *fname.add(1) == b'/' as c_char
                        && !is_name_prefixed_dot_slash()
                    {
                        strip_leading_dot_slash_in_place(fname);
                    }
                    *f_ptr = xfopen(fname, fopen_mode);
                    set_nameoffile_from_c_str(fname);
                    crate::allocation::free(fname as *mut c_void);
                }
            }
        }

        if !(*f_ptr).is_null() {
            if !crate::lookup::record_input(std::path::Path::new(OsStr::from_bytes(
                CStr::from_ptr(nameoffile_body()).to_bytes(),
            ))) {
                eprintln!("tekai: current-pass generated-input tracking exceeds the 4 MiB limit");
                crate::stdio::fclose(*f_ptr);
                *f_ptr = ptr::null_mut();
                return FALSE;
            }
            recorder_record_input(nameoffile_body());
            if filefmt == KPSE_TFM_FORMAT {
                (*__tekai_state_openclose).tfmtemp = fast_fgetc(*f_ptr);
            } else if filefmt == KPSE_OCP_FORMAT {
                (*__tekai_state_openclose).ocptemp = fast_fgetc(*f_ptr);
            } else if filefmt == KPSE_OFM_FORMAT {
                (*__tekai_state_openclose).tfmtemp = fast_fgetc(*f_ptr);
            }
        }

        (!(*f_ptr).is_null()) as c_int
    }
}

pub(crate) unsafe fn open_input_with_dirname(
    f_ptr: *mut *mut FILE,
    filefmt: c_int,
    fname: *const c_char,
) -> c_int {
    unsafe {
        let mut ret = FALSE;
        let top_dir = xdirname(fname);
        if !top_dir.is_null()
            && *top_dir != 0
            && crate::byte_string::strcmp(top_dir, c".".as_ptr()) != 0
            && kpse_absolute_p(nameoffile_body(), TRUE) == 0
        {
            let newname = concat3_ptr(top_dir, c"/".as_ptr(), nameoffile_body());
            set_nameoffile_from_c_str(newname);
            ret = open_input(f_ptr, filefmt, c"rb".as_ptr());
            crate::allocation::free(newname as *mut c_void);
        }
        crate::allocation::free(top_dir as *mut c_void);
        ret
    }
}

pub(crate) unsafe fn open_output(f_ptr: *mut *mut FILE, fopen_mode: *const c_char) -> c_int {
    let __tekai_state_openclose: *mut crate::openclose::State =
        unsafe { crate::openclose::state() };

    unsafe {
        let original = nameoffile_body();
        let absolute = kpse_absolute_p(original, FALSE) != 0;
        let mut fname = original as *mut c_char;
        let mut allocated = false;

        if !(*__tekai_state_openclose).output_directory.is_null() && !absolute {
            fname = concat3_ptr(
                (*__tekai_state_openclose).output_directory,
                c"/".as_ptr(),
                original,
            );
            allocated = true;
        }

        if !crate::lookup::can_record_output(std::path::Path::new(OsStr::from_bytes(
            CStr::from_ptr(fname).to_bytes(),
        ))) {
            eprintln!("tekai: current-pass generated-output tracking exceeds the 4 MiB limit; output was not opened");
            if allocated {
                crate::allocation::free(fname as *mut c_void);
            }
            *f_ptr = ptr::null_mut();
            return FALSE;
        }
        *f_ptr = crate::stdio::fopen(fname, fopen_mode);
        if (*f_ptr).is_null() {
            let texmfoutput = kpse_var_value(c"TEXMFOUTPUT".as_ptr());
            if !texmfoutput.is_null() && *texmfoutput != 0 && !absolute {
                if allocated {
                    crate::allocation::free(fname as *mut c_void);
                }
                fname = concat3_ptr(texmfoutput, c"/".as_ptr(), original);
                allocated = true;
                if crate::lookup::can_record_output(std::path::Path::new(OsStr::from_bytes(
                    CStr::from_ptr(fname).to_bytes(),
                ))) {
                    *f_ptr = crate::stdio::fopen(fname, fopen_mode);
                } else {
                    eprintln!("tekai: current-pass generated-output tracking exceeds the 4 MiB limit; fallback output was not opened");
                }
            }
            crate::allocation::free(texmfoutput as *mut c_void);
        }

        if !(*f_ptr).is_null() {
            if allocated {
                set_nameoffile_from_c_str(fname);
            }
            if !crate::lookup::record_output(std::path::Path::new(OsStr::from_bytes(
                CStr::from_ptr(fname).to_bytes(),
            ))) {
                eprintln!("tekai: current-pass generated-output tracking exceeds the 4 MiB limit");
                crate::stdio::fclose(*f_ptr);
                *f_ptr = ptr::null_mut();
            } else {
                recorder_record_output(fname);
            }
        }
        if allocated {
            crate::allocation::free(fname as *mut c_void);
        }
        (!(*f_ptr).is_null()) as c_int
    }
}

pub(crate) unsafe fn close_file(f: *mut FILE) {
    unsafe {
        if f.is_null() {
            return;
        }
        if crate::stdio::fclose(f) == libc::EOF {
            crate::stdio::perror(c"fclose".as_ptr());
        }
    }
}

pub unsafe fn setupboundvariable(var: *mut c_int, var_name: *const c_char, dflt: c_int) {
    unsafe {
        *var = dflt;
        let expansion = kpse_var_value(var_name);
        if expansion.is_null() {
            return;
        }

        let conf_val =
            crate::integer_parse::decimal_i32_legacy(CStr::from_ptr(expansion).to_bytes()).value;
        if conf_val >= 0 && (conf_val != 0 || dflt <= 0) {
            *var = conf_val;
        } else {
            crate::stdio::fprintf_args(
                crate::stdio::stderr(),
                c"pdftex: Bad value (%ld) in environment or texmf.cnf for %s, keeping %ld.\n"
                    .as_ptr(),
                &[
                    crate::utils::PrintfArg::from(conf_val as libc::c_long),
                    crate::utils::PrintfArg::from(var_name),
                    crate::utils::PrintfArg::from(dflt as libc::c_long),
                ],
            );
        }
        crate::allocation::free(expansion as *mut c_void);
    }
}

/// Allocate an engine-owned buffer, terminating with the existing diagnostic on
/// overflow or allocation failure.
///
/// # Safety
/// Initialize bytes before reading them. Release storage through
/// `crate::allocation::free`, never a foreign deallocator.
pub(crate) unsafe fn xmalloc(size: size_t) -> *mut c_void {
    unsafe {
        let mem = crate::allocation::malloc(size);
        if mem.is_null() {
            let msg = format!("fatal: memory exhausted (xmalloc of {size} bytes).\n");
            crate::stdio::fwrite(msg.as_ptr().cast(), 1, msg.len(), crate::stdio::stderr());
            crate::stdio::exit(libc::EXIT_FAILURE);
        }
        mem
    }
}

/// Resize engine-owned storage, terminating on overflow or allocation failure.
///
/// # Safety
/// A non-null pointer must be the exact live pointer produced by the engine's
/// Rust allocator. No references may be used during the resize.
pub(crate) unsafe fn xrealloc(old_ptr: *mut c_void, size: size_t) -> *mut c_void {
    unsafe {
        if old_ptr.is_null() {
            return xmalloc(size);
        }
        let mem = crate::allocation::realloc(old_ptr, size);
        if mem.is_null() {
            let msg = format!("fatal: memory exhausted (realloc of {size} bytes).\n");
            crate::stdio::fwrite(msg.as_ptr().cast(), 1, msg.len(), crate::stdio::stderr());
            crate::stdio::exit(libc::EXIT_FAILURE);
        }
        mem
    }
}

/// Copy a NUL-terminated string into engine-owned Rust storage.
///
/// # Safety
/// The input must remain valid through the copy. Release the returned pointer
/// through `crate::allocation::free`.
pub(crate) unsafe fn xstrdup(s: *const c_char) -> *mut c_char {
    unsafe { xstrdup_ptr(s) }
}

#[cfg(test)]
mod tests {
    use super::{xmalloc, xrealloc, xstrdup};
    use libc::c_void;
    use proptest::prelude::*;
    use std::ffi::{CStr, CString};

    #[test]
    fn recorder_temporary_creation_retries_only_collisions_and_never_truncates() {
        let fixture = std::env::temp_dir().join(format!(
            "tekai-recorder-collision-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&fixture).unwrap();
        let occupied =
            |attempt| fixture.join(format!("pdftex{}-0-{attempt}.fls", std::process::id()));
        std::fs::write(occupied(0), b"foreign recorder").unwrap();
        let mut directory =
            crate::working_directory::WorkingDirectory::new(Some(fixture.clone())).unwrap();
        directory
            .as_mut()
            .scope(|| {
                let mut streams = crate::stdio::StreamSession::new();
                streams.as_mut().scope(|| {
                    let (name, file) = super::open_recorder_temporary(None, 0);
                    assert!(!file.is_null());
                    assert_eq!(
                        name.to_bytes(),
                        format!("pdftex{}-0-1.fls", std::process::id()).as_bytes()
                    );
                    assert_eq!(unsafe { crate::stdio::fclose(file) }, 0);
                    assert_eq!(std::fs::read(occupied(0)).unwrap(), b"foreign recorder");
                    for attempt in 1..100 {
                        std::fs::write(occupied(attempt), b"foreign recorder").unwrap();
                    }
                    let (_, file) = super::open_recorder_temporary(None, 0);
                    assert!(file.is_null());
                    assert_eq!(crate::stdio::errno(), libc::EEXIST);
                    for attempt in 0..100 {
                        assert_eq!(
                            std::fs::read(occupied(attempt)).unwrap(),
                            b"foreign recorder"
                        );
                    }
                    let (name, file) =
                        super::open_recorder_temporary(Some(std::ffi::OsStr::new("missing")), 0);
                    assert!(file.is_null());
                    assert_eq!(crate::stdio::errno(), libc::ENOENT);
                    assert_eq!(
                        name.to_bytes(),
                        format!("missing/pdftex{}-0-0.fls", std::process::id()).as_bytes()
                    );
                });
                streams.as_mut().finish().unwrap();
            })
            .unwrap();
        std::fs::remove_dir_all(fixture).unwrap();
    }

    #[test]
    fn concurrent_distinct_jobs_share_a_directory_without_recorder_collisions() {
        let fixture = std::env::temp_dir().join(format!(
            "tekai-concurrent-recorder-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&fixture).unwrap();
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(8));
        let workers = (0..8)
            .map(|job| {
                let fixture = fixture.clone();
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    let mut directory =
                        crate::working_directory::WorkingDirectory::new(Some(fixture.clone()))
                            .unwrap();
                    directory
                        .as_mut()
                        .scope(|| {
                            let mut streams = crate::stdio::StreamSession::new();
                            streams.as_mut().scope(|| {
                                crate::invocation_state::with_fixture(|| unsafe {
                                    let state = super::state();
                                    (*state).recorder_enabled = 1;
                                    // Force every thread into the exact same
                                    // initial namespace, independently of clock
                                    // resolution or thread scheduling.
                                    super::recorder_start_with_nonce(0);
                                    let marker = CString::new(format!("job-{job}.pdf")).unwrap();
                                    super::recorder_record_output(marker.as_ptr());
                                    barrier.wait();
                                    let final_name =
                                        CString::new(format!("job-{job}.fls")).unwrap();
                                    super::recorder_change_filename(final_name.as_ptr().cast_mut());
                                    super::recorder_record_output(marker.as_ptr());
                                    assert_eq!(crate::stdio::fclose((*state).RECORDER_FILE), 0);
                                    (*state).RECORDER_FILE = std::ptr::null_mut();
                                });
                            });
                            streams.as_mut().finish().unwrap();
                        })
                        .unwrap();
                    let recorded = std::fs::read(fixture.join(format!("job-{job}.fls"))).unwrap();
                    let marker = format!("OUTPUT job-{job}.pdf\n");
                    assert!(recorded.ends_with(format!("{marker}{marker}").as_bytes()));
                })
            })
            .collect::<Vec<_>>();
        for worker in workers {
            worker.join().unwrap();
        }
        assert_eq!(std::fs::read_dir(&fixture).unwrap().count(), 8);
        std::fs::remove_dir_all(fixture).unwrap();
    }

    #[test]
    fn allocation_overflow_uses_owned_diagnostics_and_does_not_abort() {
        let mut streams = crate::stdio::StreamSession::new();
        streams.as_mut().scope(|| {
            crate::invocation_state::with_fixture(|| {
                assert_eq!(
                    crate::termination::catch(|| unsafe { xmalloc(usize::MAX) }),
                    Err(crate::EngineCompletion::Errors)
                );
                let retained = unsafe { xmalloc(8) };
                assert_eq!(
                    crate::termination::catch(|| unsafe { xrealloc(retained, usize::MAX) }),
                    Err(crate::EngineCompletion::Errors)
                );
                // The failed resize leaves the original allocation owned.
                unsafe { crate::allocation::free(retained) };
            });
        });
        streams.as_mut().finish().unwrap();
        assert!(!crate::invocation_state::is_active());
    }

    #[test]
    fn recorder_rename_uses_owned_directory_for_both_byte_paths() {
        use super::{recorder_change_filename, recorder_record_output, state};
        use std::os::unix::ffi::OsStrExt;
        use std::time::{SystemTime, UNIX_EPOCH};

        let parent = std::env::current_dir().unwrap();
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let fixture = std::env::temp_dir().join(format!(
            "tekai-owned-recorder-{}-{nonce}",
            std::process::id()
        ));
        std::fs::create_dir_all(fixture.join("output-é")).unwrap();
        let mut directory =
            crate::working_directory::WorkingDirectory::new(Some(fixture.clone())).unwrap();
        directory
            .as_mut()
            .scope(|| {
                for output_directory in [None, Some(c"output-é")] {
                    let mut streams = crate::stdio::StreamSession::new();
                    streams.as_mut().scope(|| {
                        crate::invocation_state::with_fixture(|| unsafe {
                            let state = state();
                            (*state).recorder_enabled = 1;
                            if let Some(output_directory) = output_directory {
                                (*state).output_directory = xstrdup(output_directory.as_ptr());
                            }
                            recorder_record_output(c"owned-output.pdf".as_ptr());
                            let old_name = std::ffi::OsStr::from_bytes(
                                CStr::from_ptr((*state).RECORDER_NAME).to_bytes(),
                            );
                            let old_path = fixture.join(old_name);
                            assert!(old_path.is_file());
                            let final_name = CString::new("result-é.fls").unwrap();
                            recorder_change_filename(final_name.as_ptr().cast_mut());
                            recorder_record_output(c"after-rename.pdf".as_ptr());
                            assert_eq!(crate::stdio::fclose((*state).RECORDER_FILE), 0);
                            (*state).RECORDER_FILE = std::ptr::null_mut();
                            let final_path = match output_directory {
                                Some(_) => fixture.join("output-é/result-é.fls"),
                                None => fixture.join("result-é.fls"),
                            };
                            assert!(!old_path.exists());
                            let recorded = std::fs::read(final_path).unwrap();
                            assert!(recorded.ends_with(b"OUTPUT after-rename.pdf\n"));
                            assert_eq!(std::env::current_dir().unwrap(), parent);
                        });
                    });
                    streams.as_mut().finish().unwrap();
                }
            })
            .unwrap();
        std::fs::remove_dir_all(fixture).unwrap();
    }

    #[test]
    fn xstrdup_returns_rust_allocated_copy() {
        let source = CString::new("hello").unwrap();
        let dup = unsafe { xstrdup(source.as_ptr()) };
        assert_eq!(unsafe { CStr::from_ptr(dup) }.to_str().unwrap(), "hello");
        unsafe { crate::allocation::free(dup as *mut c_void) };
    }

    #[test]
    fn zero_sized_allocations_still_return_storage() {
        let ptr = unsafe { xmalloc(0) };
        assert!(!ptr.is_null());
        let ptr = unsafe { xrealloc(ptr, 0) };
        assert!(!ptr.is_null());
        unsafe { crate::allocation::free(ptr) };
    }

    proptest! {
        #[test]
        fn duplicated_strings_resize_and_release_with_rust_allocator(
            bytes in prop::collection::vec(1u8..=255, 0..4096),
            extra in 1usize..129,
        ) {
            let source = CString::new(bytes.clone()).unwrap();
            let duplicate = unsafe { xstrdup(source.as_ptr()) };
            let resized = unsafe { xrealloc(duplicate.cast(), bytes.len() + extra + 1) };
            assert!(!resized.is_null());
            let preserved = unsafe { CStr::from_ptr(resized.cast()) }.to_bytes();
            let matches = preserved == bytes.as_slice();
            unsafe { crate::allocation::free(resized) };
            prop_assert!(matches);
        }
    }
}

pub(crate) struct State {
    pub(crate) fullnameoffile: *mut c_char,
    pub(crate) output_directory: *mut c_char,
    pub(crate) recorder_enabled: c_int,
    pub(crate) tfmtemp: c_int,
    ocptemp: c_int,
    pub(crate) texinputtype: c_int,
    RECORDER_NAME: *mut c_char,
    RECORDER_FILE: *mut FILE,
}

#[inline(always)]
fn __tekai_require_copy<T: Copy>() {}

pub(crate) struct StateInitialization {
    destination: *mut State,
    initialized: [bool; 8],
}
#[inline]
unsafe fn drop_initialized<T>(pointer: *mut T) {
    if core::mem::needs_drop::<T>() {
        unsafe { core::ptr::drop_in_place(pointer) };
    }
}

impl Drop for StateInitialization {
    fn drop(&mut self) {
        unsafe {
            if self.initialized[7] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).RECORDER_FILE));
            }
            if self.initialized[6] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).RECORDER_NAME));
            }
            if self.initialized[5] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).texinputtype));
            }
            if self.initialized[4] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).ocptemp));
            }
            if self.initialized[3] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).tfmtemp));
            }
            if self.initialized[2] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).recorder_enabled
                ));
            }
            if self.initialized[1] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).output_directory
                ));
            }
            if self.initialized[0] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).fullnameoffile));
            }
        }
    }
}
impl StateInitialization {
    pub(crate) unsafe fn begin(destination: *mut State) -> Self {
        Self {
            destination,
            initialized: [false; 8],
        }
    }
    pub(crate) fn disarm(&mut self) {
        self.initialized.fill(false);
    }
    pub(crate) unsafe fn initialize_fields(
        &mut self,
        invocation: *const crate::invocation_state::Storage,
    ) {
        let destination = self.destination;
        let _ = invocation;
        const {
            assert!(core::mem::size_of::<*mut c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).fullnameoffile).write(ptr::null_mut());
        self.initialized[0] = true;
        const {
            assert!(core::mem::size_of::<*mut c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).output_directory).write(ptr::null_mut());
        self.initialized[1] = true;
        const {
            assert!(core::mem::size_of::<c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).recorder_enabled).write(0);
        self.initialized[2] = true;
        const {
            assert!(core::mem::size_of::<c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).tfmtemp).write(0);
        self.initialized[3] = true;
        const {
            assert!(core::mem::size_of::<c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).ocptemp).write(0);
        self.initialized[4] = true;
        const {
            assert!(core::mem::size_of::<c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).texinputtype).write(0);
        self.initialized[5] = true;
        const {
            assert!(core::mem::size_of::<*mut c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).RECORDER_NAME).write(ptr::null_mut());
        self.initialized[6] = true;
        const {
            assert!(core::mem::size_of::<*mut FILE>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).RECORDER_FILE).write(ptr::null_mut());
        self.initialized[7] = true;
    }
    pub(crate) unsafe fn initialize_fixups(
        &mut self,
        invocation: *const crate::invocation_state::Storage,
    ) {
        let destination = self.destination;
        let _ = invocation;
    }
}
pub(crate) unsafe fn state() -> *mut State {
    core::cell::UnsafeCell::raw_get(core::ptr::addr_of!(
        (*crate::invocation_state::current()).openclose
    ))
}
#[cfg(test)]
pub(crate) unsafe fn check_state_construction(phase: u8) {
    let invocation = crate::invocation_state::current();
    let destination = state();
    assert!(!destination.is_null());
    let _ = (invocation, phase);
}
