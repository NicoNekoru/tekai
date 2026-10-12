//! Rust byte operations for the translated engine's pointer-based callers.
//!
//! These adapters have Rust calling conventions and no parser or locale state.
//! Pointer searches retain the original allocation's provenance. Comparisons
//! use unsigned bytes, and case-insensitive comparison uses the ASCII folding
//! of the C locale. Safe slice/CStr helpers are available to migrating callers.

#![warn(clippy::all)]

use std::ffi::{c_char, c_int, c_void, CStr};
use std::ptr;

/// The locale-independent ASCII digit predicate used by the former Darwin
/// `__isctype` wrappers. Values outside ASCII, including EOF, are false.
pub(crate) fn isdigit(character: c_int) -> c_int {
    c_int::from((c_int::from(b'0')..=c_int::from(b'9')).contains(&character))
}

/// Compare full initialized byte slices, with the usual negative/zero/positive
/// result. A shorter matching prefix sorts first. No locale is consulted.
pub(crate) fn compare_bytes(left: &[u8], right: &[u8]) -> c_int {
    for (&left, &right) in left.iter().zip(right) {
        if left != right {
            return c_int::from(left) - c_int::from(right);
        }
    }
    match left.len().cmp(&right.len()) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

pub(crate) fn compare_c_str(left: &CStr, right: &CStr) -> c_int {
    compare_bytes(left.to_bytes(), right.to_bytes())
}

pub(crate) fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    // memmem guarantees linear worst-case time and constant space, including
    // periodic near-matches. Its empty-needle result is the C string rule too.
    memchr::memmem::find(haystack, needle)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BufferError {
    MissingTerminator,
    InsufficientCapacity { required: usize, available: usize },
    LengthOverflow,
}

/// Copy a complete C string into a checked buffer. Returns the byte length
/// without its NUL. On insufficient capacity, the destination is unchanged.
pub(crate) fn copy_c_str(destination: &mut [u8], source: &CStr) -> Result<usize, BufferError> {
    let bytes = source.to_bytes_with_nul();
    if bytes.len() > destination.len() {
        return Err(BufferError::InsufficientCapacity {
            required: bytes.len(),
            available: destination.len(),
        });
    }
    destination[..bytes.len()].copy_from_slice(bytes);
    Ok(bytes.len() - 1)
}

/// Append to a checked, already terminated buffer. Returns the combined byte
/// length without its NUL. Every error leaves the destination unchanged.
pub(crate) fn append_c_str(destination: &mut [u8], source: &CStr) -> Result<usize, BufferError> {
    let Some(length) = destination.iter().position(|&byte| byte == 0) else {
        return Err(BufferError::MissingTerminator);
    };
    let bytes = source.to_bytes_with_nul();
    let required = length
        .checked_add(bytes.len())
        .ok_or(BufferError::LengthOverflow)?;
    if required > destination.len() {
        return Err(BufferError::InsufficientCapacity {
            required,
            available: destination.len(),
        });
    }
    destination[length..required].copy_from_slice(bytes);
    Ok(required - 1)
}

/// # Safety
/// `string` must point to initialized bytes through a NUL in one live
/// allocation. Those bytes must remain readable and unmodified during the call.
pub(crate) unsafe fn strlen(string: *const c_char) -> usize {
    let mut length = 0;
    while unsafe { *string.add(length) } != 0 {
        length += 1;
    }
    length
}

/// # Safety
/// Both pointers must point to live, initialized NUL-terminated byte strings.
/// The strings must remain readable and unmodified during the call.
pub(crate) unsafe fn strcmp(left: *const c_char, right: *const c_char) -> c_int {
    let mut index = 0;
    loop {
        let left = unsafe { *left.add(index) } as u8;
        let right = unsafe { *right.add(index) } as u8;
        if left != right {
            return c_int::from(left) - c_int::from(right);
        }
        if left == 0 {
            return 0;
        }
        index += 1;
    }
}

/// # Safety
/// For nonzero `count`, each pointer must provide readable, initialized bytes
/// through either its first NUL or `count` bytes, whichever comes first. The
/// readable spans must remain unmodified. At zero, neither pointer is accessed.
pub(crate) unsafe fn strncmp(left: *const c_char, right: *const c_char, count: usize) -> c_int {
    for index in 0..count {
        let left = unsafe { *left.add(index) } as u8;
        let right = unsafe { *right.add(index) } as u8;
        if left != right {
            return c_int::from(left) - c_int::from(right);
        }
        if left == 0 {
            return 0;
        }
    }
    0
}

/// # Safety
/// `string` must provide a live, initialized NUL-terminated string which stays
/// readable and unmodified. The returned pointer is borrowed from that same
/// allocation; mutating through it requires exclusive access to those bytes.
pub(crate) unsafe fn strchr(string: *const c_char, character: c_int) -> *mut c_char {
    let character = character as u8;
    let mut index = 0;
    loop {
        let byte = unsafe { *string.add(index) } as u8;
        if byte == character {
            return unsafe { string.add(index) }.cast_mut();
        }
        if byte == 0 {
            return ptr::null_mut();
        }
        index += 1;
    }
}

/// # Safety
/// The same read, termination, lifetime and returned-pointer requirements as
/// `strchr` apply. Searching for NUL returns the terminator's address.
pub(crate) unsafe fn strrchr(string: *const c_char, character: c_int) -> *mut c_char {
    let character = character as u8;
    let mut found = ptr::null_mut();
    let mut index = 0;
    loop {
        let byte = unsafe { *string.add(index) } as u8;
        if byte == character {
            found = unsafe { string.add(index) }.cast_mut();
        }
        if byte == 0 {
            return found;
        }
        index += 1;
    }
}

/// # Safety
/// Both pointers must provide live, initialized NUL-terminated byte strings,
/// unmodified during the call. The result borrows the haystack allocation, and
/// writing through it requires exclusive access. An empty needle returns the
/// haystack pointer, without reading the haystack.
pub(crate) unsafe fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char {
    let needle_length = unsafe { strlen(needle) };
    if needle_length == 0 {
        return haystack.cast_mut();
    }
    let haystack_length = unsafe { strlen(haystack) };
    // SAFETY: strlen established the initialized string spans. The contracts
    // exclude concurrent writes, and each length fits its source allocation.
    let haystack_bytes = unsafe { std::slice::from_raw_parts(haystack.cast(), haystack_length) };
    let needle_bytes = unsafe { std::slice::from_raw_parts(needle.cast(), needle_length) };
    match find_bytes(haystack_bytes, needle_bytes) {
        Some(index) => unsafe { haystack.add(index) }.cast_mut(),
        None => ptr::null_mut(),
    }
}

/// # Safety
/// `source` must provide a readable, initialized NUL-terminated string.
/// `destination` must provide exclusive write access for its length plus one
/// bytes. The two spans must not overlap and must be in live allocations.
pub(crate) unsafe fn strcpy(destination: *mut c_char, source: *const c_char) -> *mut c_char {
    let count = unsafe { strlen(source) } + 1;
    unsafe { ptr::copy_nonoverlapping(source, destination, count) };
    destination
}

/// # Safety
/// For nonzero `count`, destination must provide exclusive write access for
/// `count` bytes. Source must provide initialized readable bytes through its
/// first NUL or `count` bytes, whichever comes first. Read/write spans must not
/// overlap and must be live. At zero, neither pointer is accessed. A short
/// source pads with NULs; a source of at least `count` bytes adds no terminator.
pub(crate) unsafe fn strncpy(
    destination: *mut c_char,
    source: *const c_char,
    count: usize,
) -> *mut c_char {
    if count == 0 {
        return destination;
    }
    let mut length = 0;
    while length < count && unsafe { *source.add(length) } != 0 {
        length += 1;
    }
    unsafe {
        ptr::copy_nonoverlapping(source, destination, length);
        ptr::write_bytes(destination.add(length), 0, count - length);
    }
    destination
}

/// # Safety
/// Source and destination must provide initialized, readable NUL-terminated
/// strings. Destination must also provide exclusive write access and capacity
/// for both byte lengths plus one. The source and destination spans must not
/// overlap and must be in live allocations.
pub(crate) unsafe fn strcat(destination: *mut c_char, source: *const c_char) -> *mut c_char {
    let length = unsafe { strlen(destination) };
    unsafe { strcpy(destination.add(length), source) };
    destination
}

/// # Safety
/// Destination must provide an initialized NUL-terminated string and exclusive
/// writable capacity for its length, at most `count` source bytes and one NUL.
/// Source must be readable through its first NUL or `count` bytes. The live
/// source and destination spans must not overlap. At zero, source is not read.
pub(crate) unsafe fn strncat(
    destination: *mut c_char,
    source: *const c_char,
    count: usize,
) -> *mut c_char {
    let destination_length = unsafe { strlen(destination) };
    let mut source_length = 0;
    while source_length < count && unsafe { *source.add(source_length) } != 0 {
        source_length += 1;
    }
    unsafe {
        let tail = destination.add(destination_length);
        if source_length != 0 {
            ptr::copy_nonoverlapping(source, tail, source_length);
        }
        *tail.add(source_length) = 0;
    }
    destination
}

/// # Safety
/// Both pointers must provide live, initialized NUL-terminated byte strings,
/// readable and unmodified during the call. Uses the C locale's ASCII folding,
/// independently of process locale. Bytes above ASCII remain unchanged.
pub(crate) unsafe fn strcasecmp(left: *const c_char, right: *const c_char) -> c_int {
    let mut index = 0;
    loop {
        let left = (unsafe { *left.add(index) } as u8).to_ascii_lowercase();
        let right = (unsafe { *right.add(index) } as u8).to_ascii_lowercase();
        if left != right {
            return c_int::from(left) - c_int::from(right);
        }
        if left == 0 {
            return 0;
        }
        index += 1;
    }
}

/// # Safety
/// For nonzero count, source must provide `count` readable bytes and destination
/// must provide `count` exclusively writable bytes, in live allocations. The
/// spans must not overlap. Uninitialized bytes may be copied without inspection.
/// At zero, neither pointer is accessed or required to be valid/aligned.
pub(crate) unsafe fn memcpy(
    destination: *mut c_void,
    source: *const c_void,
    count: usize,
) -> *mut c_void {
    if count != 0 {
        unsafe { ptr::copy_nonoverlapping(source.cast::<u8>(), destination.cast::<u8>(), count) };
    }
    destination
}

/// # Safety
/// For nonzero count, source must provide `count` readable bytes and destination
/// must provide `count` writable bytes, in live allocations. Overlap is allowed;
/// there must be no other accesses during the copy. Uninitialized bytes can be
/// copied without inspection. At zero, neither pointer is accessed or validated.
pub(crate) unsafe fn memmove(
    destination: *mut c_void,
    source: *const c_void,
    count: usize,
) -> *mut c_void {
    if count != 0 {
        unsafe { ptr::copy(source.cast::<u8>(), destination.cast::<u8>(), count) };
    }
    destination
}

/// # Safety
/// For nonzero count, destination must provide `count` exclusively writable
/// bytes in a live allocation. The value converts to its low unsigned byte.
/// At zero, the pointer is returned without access or validation.
pub(crate) unsafe fn memset(destination: *mut c_void, value: c_int, count: usize) -> *mut c_void {
    if count != 0 {
        unsafe { ptr::write_bytes(destination.cast::<u8>(), value as u8, count) };
    }
    destination
}

/// # Safety
/// For nonzero count, both pointers must provide `count` initialized, readable
/// bytes in live allocations, unmodified during the call. At zero, neither
/// pointer is accessed. Unlike copying, inspecting uninitialized padding is
/// invalid. The result's sign follows the first differing unsigned byte.
pub(crate) unsafe fn memcmp(left: *const c_void, right: *const c_void, count: usize) -> c_int {
    for index in 0..count {
        let left = unsafe { *left.cast::<u8>().add(index) };
        let right = unsafe { *right.cast::<u8>().add(index) };
        if left != right {
            return c_int::from(left) - c_int::from(right);
        }
    }
    0
}

/// Per-parse, non-mutating replacement for fixed-delimiter strtok callers.
/// Delimiter runs yield no empty tokens; an input NUL ends the initialized text.
/// Multiple tokenizers can be interleaved without sharing state.
pub(crate) struct Tokenizer<'a> {
    remaining: &'a [u8],
    delimiters: &'a [u8],
}

impl<'a> Tokenizer<'a> {
    pub(crate) fn new(bytes: &'a [u8], delimiters: &'a [u8]) -> Self {
        let end = bytes
            .iter()
            .position(|&byte| byte == 0)
            .unwrap_or(bytes.len());
        Self {
            remaining: &bytes[..end],
            delimiters,
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let Some(start) = self
            .remaining
            .iter()
            .position(|byte| !self.delimiters.contains(byte))
        else {
            self.remaining = &[];
            return None;
        };
        let tail = &self.remaining[start..];
        let length = tail
            .iter()
            .position(|byte| self.delimiters.contains(byte))
            .unwrap_or(tail.len());
        let token = &tail[..length];
        self.remaining = &tail[length..];
        Some(token)
    }
}

impl std::iter::FusedIterator for Tokenizer<'_> {}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::ffi::CString;

    fn offset(base: *const c_char, found: *const c_char) -> Option<usize> {
        if found.is_null() {
            None
        } else {
            // SAFETY: every nonnull search result borrows this same allocation.
            Some(unsafe { found.offset_from(base) } as usize)
        }
    }

    fn nonzero_bytes() -> impl Strategy<Value = Vec<u8>> {
        prop::collection::vec(1u8..=255, 0..96)
    }

    #[test]
    fn zero_length_operations_do_not_touch_null_pointers_or_change_bytes() {
        let mut byte = 42u8;
        let byte_pointer = (&mut byte as *mut u8).cast::<c_void>();
        unsafe {
            assert_eq!(strncmp(ptr::null(), ptr::null(), 0), 0);
            assert_eq!(strncpy(ptr::null_mut(), ptr::null(), 0), ptr::null_mut());
            assert_eq!(memcpy(ptr::null_mut(), ptr::null(), 0), ptr::null_mut());
            assert_eq!(memmove(ptr::null_mut(), ptr::null(), 0), ptr::null_mut());
            assert_eq!(memset(ptr::null_mut(), 255, 0), ptr::null_mut());
            assert_eq!(memcmp(ptr::null(), ptr::null(), 0), 0);
            assert_eq!(memcpy(byte_pointer, ptr::null(), 0), byte_pointer);
        }
        assert_eq!(byte, 42);
    }

    #[test]
    fn string_boundaries_preserve_nul_and_unsigned_character_conversion() {
        let string = c"abc";
        unsafe {
            assert_eq!(offset(string.as_ptr(), strchr(string.as_ptr(), 0)), Some(3));
            assert_eq!(
                offset(string.as_ptr(), strrchr(string.as_ptr(), 256)),
                Some(3)
            );
            assert_eq!(
                offset(
                    string.as_ptr(),
                    strchr(string.as_ptr(), 256 + b'b' as c_int)
                ),
                Some(1)
            );
            assert_eq!(
                strstr(string.as_ptr(), c"".as_ptr()),
                string.as_ptr().cast_mut()
            );
            assert!(strstr(string.as_ptr(), c"abcd".as_ptr()).is_null());
            assert!(strcmp(c"\x7f".as_ptr(), CString::new(vec![0x80]).unwrap().as_ptr()) < 0);
            assert_eq!(
                strncmp(b"abc".as_ptr().cast(), b"abd".as_ptr().cast(), 2),
                0
            );
            assert!(strncmp(b"abc".as_ptr().cast(), b"abd".as_ptr().cast(), 3) < 0);
            let mut padded = [0x7f as c_char; 6];
            strncpy(padded.as_mut_ptr(), c"a".as_ptr(), 4);
            assert_eq!(padded, [b'a' as c_char, 0, 0, 0, 0x7f, 0x7f]);
            strncpy(padded.as_mut_ptr(), b"abcdef".as_ptr().cast(), 3);
            assert_eq!(
                &padded[..3],
                &[b'a' as c_char, b'b' as c_char, b'c' as c_char]
            );
        }
    }

    #[test]
    fn case_folding_is_ascii_only_and_ignores_process_locale() {
        let left = CString::new(vec![b'A', 0xc0, b'Z']).unwrap();
        let right = CString::new(vec![b'a', 0xc0, b'z']).unwrap();
        let different = CString::new(vec![b'a', 0xe0, b'z']).unwrap();
        unsafe {
            assert_eq!(strcasecmp(left.as_ptr(), right.as_ptr()), 0);
            assert!(strcasecmp(left.as_ptr(), different.as_ptr()) < 0);
        }
    }

    #[test]
    fn digit_classification_matches_native_for_all_byte_values_and_eof() {
        for character in -1..=255 {
            assert_eq!(
                isdigit(character) != 0,
                unsafe { libc::isdigit(character) } != 0
            );
        }
        for character in [c_int::MIN, -2, 256, 1024, c_int::MAX] {
            assert_eq!(isdigit(character), 0);
        }
    }

    #[test]
    fn periodic_near_matches_and_long_needles_match_native_exactly() {
        let periodic = (0..8192).map(|index| b"ab"[index % 2]).collect::<Vec<_>>();
        for length in [1, 2, 63, 64, 65, 255, 511, 1024, 2048, 4096, 8192, 8193] {
            let mut needle = (0..length)
                .map(|index| b"ab"[index % 2])
                .collect::<Vec<_>>();
            needle[length - 1] = b'c';
            let needle_c = CString::new(needle.as_slice()).unwrap();
            let haystack_c = CString::new(periodic.as_slice()).unwrap();
            assert_eq!(find_bytes(&periodic, &needle), None);
            assert_eq!(
                unsafe { strstr(haystack_c.as_ptr(), needle_c.as_ptr()) },
                unsafe { libc::strstr(haystack_c.as_ptr(), needle_c.as_ptr()) }
            );

            if length <= periodic.len() {
                let mut haystack = periodic.clone();
                let mut needle = haystack[haystack.len() - length..].to_vec();
                *needle.last_mut().unwrap() = b'c';
                *haystack.last_mut().unwrap() = b'c';
                let needle_c = CString::new(needle.as_slice()).unwrap();
                let haystack_c = CString::new(haystack.as_slice()).unwrap();
                assert_eq!(
                    find_bytes(&haystack, &needle),
                    Some(haystack.len() - length)
                );
                assert_eq!(
                    unsafe { strstr(haystack_c.as_ptr(), needle_c.as_ptr()) },
                    unsafe { libc::strstr(haystack_c.as_ptr(), needle_c.as_ptr()) }
                );
            }
        }
        assert_eq!(find_bytes(&[], &[]), Some(0));
        assert_eq!(find_bytes(&periodic, &[]), Some(0));
    }

    #[test]
    fn checked_copy_and_append_leave_failed_destinations_unchanged() {
        let mut buffer = [0xaa; 5];
        assert_eq!(copy_c_str(&mut buffer, c"ab"), Ok(2));
        assert_eq!(buffer, [b'a', b'b', 0, 0xaa, 0xaa]);
        assert_eq!(append_c_str(&mut buffer, c"c"), Ok(3));
        assert_eq!(buffer, [b'a', b'b', b'c', 0, 0xaa]);
        let before = buffer;
        assert!(matches!(
            append_c_str(&mut buffer, c"de"),
            Err(BufferError::InsufficientCapacity { .. })
        ));
        assert_eq!(buffer, before);
        assert!(matches!(
            copy_c_str(&mut buffer, c"abcde"),
            Err(BufferError::InsufficientCapacity { .. })
        ));
        assert_eq!(buffer, before);
        let mut unterminated = [b'x'; 3];
        assert_eq!(
            append_c_str(&mut unterminated, c"a"),
            Err(BufferError::MissingTerminator)
        );
        assert_eq!(unterminated, [b'x'; 3]);
    }

    #[test]
    fn tokenizers_have_independent_borrowed_state() {
        let mut first = Tokenizer::new(b", par,,math display\0ignored", b", ");
        let mut second = Tokenizer::new(b"a:b", b":");
        assert_eq!(first.next(), Some(b"par".as_slice()));
        assert_eq!(second.next(), Some(b"a".as_slice()));
        assert_eq!(first.next(), Some(b"math".as_slice()));
        assert_eq!(second.next(), Some(b"b".as_slice()));
        assert_eq!(first.next(), Some(b"display".as_slice()));
        assert_eq!(first.next(), None);
        assert_eq!(first.next(), None);
        assert_eq!(
            Tokenizer::new(b"a b", b"").collect::<Vec<_>>(),
            vec![b"a b".as_slice()]
        );
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(128))]

        #[test]
        fn string_reads_match_native(
            left in nonzero_bytes(), right in nonzero_bytes(), count in 0usize..128,
            character in any::<c_int>(),
        ) {
            let left = CString::new(left).unwrap();
            let right = CString::new(right).unwrap();
            unsafe {
                // Invoke the native oracle deliberately, not CString's helper.
                #[allow(clippy::strlen_on_c_strings)]
                let native_length = libc::strlen(left.as_ptr());
                prop_assert_eq!(strlen(left.as_ptr()), native_length);
                prop_assert_eq!(strcmp(left.as_ptr(), right.as_ptr()).signum(), libc::strcmp(left.as_ptr(), right.as_ptr()).signum());
                prop_assert_eq!(strncmp(left.as_ptr(), right.as_ptr(), count).signum(), libc::strncmp(left.as_ptr(), right.as_ptr(), count).signum());
                prop_assert_eq!(compare_c_str(&left, &right).signum(), libc::strcmp(left.as_ptr(), right.as_ptr()).signum());
                prop_assert_eq!(offset(left.as_ptr(), strchr(left.as_ptr(), character)), offset(left.as_ptr(), libc::strchr(left.as_ptr(), character)));
                prop_assert_eq!(offset(left.as_ptr(), strrchr(left.as_ptr(), character)), offset(left.as_ptr(), libc::strrchr(left.as_ptr(), character)));
                prop_assert_eq!(offset(left.as_ptr(), strstr(left.as_ptr(), right.as_ptr())), offset(left.as_ptr(), libc::strstr(left.as_ptr(), right.as_ptr())));
            }
        }

        #[test]
        fn c_locale_case_comparison_matches_native(left in nonzero_bytes(), right in nonzero_bytes()) {
            let left = CString::new(left).unwrap();
            let right = CString::new(right).unwrap();
            // The engine never changes libc's initial C locale. Do not mutate
            // process locale in these tests, which may run in parallel.
            unsafe {
                prop_assert_eq!(strcasecmp(left.as_ptr(), right.as_ptr()).signum(), libc::strcasecmp(left.as_ptr(), right.as_ptr()).signum());
            }
        }

        #[test]
        fn present_substrings_match_native(bytes in nonzero_bytes(), start in 0usize..97, wanted in 0usize..97) {
            let start = start.min(bytes.len());
            let length = wanted.min(bytes.len() - start);
            let needle = CString::new(&bytes[start..start + length]).unwrap();
            let haystack = CString::new(bytes).unwrap();
            unsafe {
                let found = offset(haystack.as_ptr(), strstr(haystack.as_ptr(), needle.as_ptr()));
                prop_assert!(found.is_some());
                prop_assert_eq!(found, offset(haystack.as_ptr(), libc::strstr(haystack.as_ptr(), needle.as_ptr())));
            }
        }

        #[test]
        fn string_writes_match_native(prefix in nonzero_bytes(), suffix in nonzero_bytes(), count in 0usize..128) {
            let prefix = CString::new(prefix).unwrap();
            let suffix = CString::new(suffix).unwrap();
            let capacity = prefix.as_bytes().len() + suffix.as_bytes().len() + count + 4;
            let mut rust = vec![0x55 as c_char; capacity];
            let mut native = rust.clone();
            unsafe {
                let result = strcpy(rust.as_mut_ptr(), prefix.as_ptr());
                prop_assert_eq!(result, rust.as_mut_ptr());
                libc::strcpy(native.as_mut_ptr(), prefix.as_ptr());
                prop_assert_eq!(&rust, &native);
                let result = strcat(rust.as_mut_ptr(), suffix.as_ptr());
                prop_assert_eq!(result, rust.as_mut_ptr());
                libc::strcat(native.as_mut_ptr(), suffix.as_ptr());
                prop_assert_eq!(&rust, &native);
                // Reset so the bounded append has the same sufficient space.
                strcpy(rust.as_mut_ptr(), prefix.as_ptr());
                libc::strcpy(native.as_mut_ptr(), prefix.as_ptr());
                let result = strncat(rust.as_mut_ptr(), suffix.as_ptr(), count);
                prop_assert_eq!(result, rust.as_mut_ptr());
                libc::strncat(native.as_mut_ptr(), suffix.as_ptr(), count);
                prop_assert_eq!(&rust, &native);
                let result = strncpy(rust.as_mut_ptr(), suffix.as_ptr(), count);
                prop_assert_eq!(result, rust.as_mut_ptr());
                libc::strncpy(native.as_mut_ptr(), suffix.as_ptr(), count);
                prop_assert_eq!(rust, native);
            }
        }

        #[test]
        fn memory_operations_match_native(
            source in prop::collection::vec(any::<u8>(), 128..129),
            initial in prop::collection::vec(any::<u8>(), 128..129),
            count in 0usize..129, value in any::<c_int>(),
        ) {
            let mut rust = initial.clone();
            let mut native = initial.clone();
            unsafe {
                prop_assert_eq!(memcmp(source.as_ptr().cast(), initial.as_ptr().cast(), count).signum(), libc::memcmp(source.as_ptr().cast(), initial.as_ptr().cast(), count).signum());
                let result = memcpy(rust.as_mut_ptr().cast(), source.as_ptr().cast(), count);
                prop_assert_eq!(result, rust.as_mut_ptr().cast());
                libc::memcpy(native.as_mut_ptr().cast(), source.as_ptr().cast(), count);
                prop_assert_eq!(&rust, &native);
                let result = memset(rust.as_mut_ptr().cast(), value, count);
                prop_assert_eq!(result, rust.as_mut_ptr().cast());
                libc::memset(native.as_mut_ptr().cast(), value, count);
                prop_assert_eq!(rust, native);
            }
        }

        #[test]
        fn overlapping_moves_match_native(
            initial in prop::collection::vec(any::<u8>(), 128..129),
            source in 0usize..129, destination in 0usize..129, wanted in 0usize..129,
        ) {
            let count = wanted.min(128 - source).min(128 - destination);
            let mut rust = initial.clone();
            let mut native = initial;
            unsafe {
                let source_ptr = rust.as_ptr().add(source).cast();
                let destination_ptr = rust.as_mut_ptr().add(destination).cast();
                prop_assert_eq!(memmove(destination_ptr, source_ptr, count), destination_ptr);
                libc::memmove(native.as_mut_ptr().add(destination).cast(), native.as_ptr().add(source).cast(), count);
            }
            prop_assert_eq!(rust, native);
        }

        #[test]
        fn substring_and_tokenizer_models_are_bounded(
            bytes in prop::collection::vec(any::<u8>(), 0..128),
            needle in prop::collection::vec(any::<u8>(), 0..32),
            delimiters in prop::collection::vec(any::<u8>(), 0..16),
        ) {
            let expected = if needle.is_empty() { Some(0) } else {
                (0..bytes.len()).find(|&index| bytes[index..].starts_with(&needle))
            };
            prop_assert_eq!(find_bytes(&bytes, &needle), expected);
            let end = bytes.iter().position(|&byte| byte == 0).unwrap_or(bytes.len());
            let expected = bytes[..end].split(|byte| delimiters.contains(byte)).filter(|token| !token.is_empty()).collect::<Vec<_>>();
            let actual = Tokenizer::new(&bytes, &delimiters).collect::<Vec<_>>();
            prop_assert!(actual.len() <= bytes.len());
            prop_assert_eq!(actual, expected);
        }

        #[test]
        fn tokenizer_matches_native_reentrant_tokens(
            bytes in prop::collection::vec(any::<u8>(), 0..128),
            delimiters in prop::collection::vec(1u8..=255, 0..16),
        ) {
            let end = bytes.iter().position(|&byte| byte == 0).unwrap_or(bytes.len());
            let mut native_input = bytes[..end].to_vec();
            native_input.push(0);
            let native_delimiters = CString::new(delimiters.as_slice()).unwrap();
            let mut save = ptr::null_mut();
            let mut native_tokens = Vec::new();
            unsafe {
                let mut token = libc::strtok_r(native_input.as_mut_ptr().cast(), native_delimiters.as_ptr(), &mut save);
                while !token.is_null() {
                    native_tokens.push(CStr::from_ptr(token).to_bytes().to_vec());
                    token = libc::strtok_r(ptr::null_mut(), native_delimiters.as_ptr(), &mut save);
                }
            }
            let rust_tokens = Tokenizer::new(&bytes, &delimiters).map(<[u8]>::to_vec).collect::<Vec<_>>();
            prop_assert_eq!(rust_tokens, native_tokens);
        }
    }
}
