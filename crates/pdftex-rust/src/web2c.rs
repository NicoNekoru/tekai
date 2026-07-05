//! Small Rust-owned web2c support shims.

use libc::{c_double, c_int, FILE};

#[no_mangle]
pub extern "C" fn zround(value: c_double) -> c_int {
    if value > 2_147_483_647.0 {
        2_147_483_647
    } else if value < -2_147_483_647.0 {
        -2_147_483_647
    } else if value >= 0.0 {
        (value + 0.5) as c_int
    } else {
        (value - 0.5) as c_int
    }
}

#[no_mangle]
pub extern "C" fn uexit(unix_code: c_int) {
    let final_code = match unix_code {
        0 => libc::EXIT_SUCCESS,
        1 => libc::EXIT_FAILURE,
        other => other,
    };
    unsafe { libc::exit(final_code) }
}

#[no_mangle]
pub unsafe extern "C" fn eof(file: *mut FILE) -> c_int {
    if file.is_null() || unsafe { libc::feof(file) } != 0 {
        return 1;
    }

    let c = unsafe { libc::fgetc(file) };
    if c == libc::EOF {
        return 1;
    }
    unsafe {
        libc::ungetc(c, file);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::zround;

    #[test]
    fn zround_matches_web2c_boundaries() {
        assert_eq!(zround(1.49), 1);
        assert_eq!(zround(1.5), 2);
        assert_eq!(zround(-1.49), -1);
        assert_eq!(zround(-1.5), -2);
        assert_eq!(zround(3_000_000_000.0), 2_147_483_647);
        assert_eq!(zround(-3_000_000_000.0), -2_147_483_647);
    }
}
