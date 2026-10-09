use std::ffi::{c_char, c_double, c_int, c_uint, c_void};
use std::io::Cursor;
use std::ptr;
use std::sync::Arc;

const PNG_LIBPNG_VER_STRING: &[u8] = b"1.6.58\0";

const PNG_INFO_GAMA: c_uint = 0x1;
const PNG_INFO_SBIT: c_uint = 0x2;
const PNG_INFO_CHRM: c_uint = 0x4;
const PNG_INFO_TRNS: c_uint = 0x10;
const PNG_INFO_BKGD: c_uint = 0x20;
const PNG_INFO_HIST: c_uint = 0x40;
const PNG_INFO_PHYS: c_uint = 0x80;
const PNG_INFO_SRGB: c_uint = 0x800;
const PNG_INFO_ICCP: c_uint = 0x1000;
const PNG_INFO_SPLT: c_uint = 0x2000;

const PNG_COLOR_TYPE_GRAY: u8 = 0;
const PNG_COLOR_TYPE_RGB: u8 = 2;
const PNG_COLOR_TYPE_PALETTE: u8 = 3;
const PNG_COLOR_TYPE_GRAY_ALPHA: u8 = 4;
const PNG_COLOR_TYPE_RGB_ALPHA: u8 = 6;

const PNG_INTERLACE_NONE: u8 = 0;
const PNG_INTERLACE_ADAM7: u8 = 1;

#[repr(C)]
pub struct png_struct_def {
    _private: [u8; 0],
}

#[repr(C)]
pub struct png_info_def {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct png_color_struct {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

type JmpBuf = [c_int; 48];
type PngErrorPtr = Option<unsafe extern "C" fn(*mut png_struct_def, *const c_char)>;
type PngLongjmpPtr = Option<unsafe extern "C" fn(*mut c_int, c_int)>;

struct PngInfo {
    state: *mut PngState,
}

#[derive(Clone)]
struct DecodedImage {
    data: Vec<u8>,
    rowbytes: usize,
    bit_depth: u8,
    color_type: u8,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct FileIdentity {
    dev: libc::dev_t,
    ino: libc::ino_t,
    size: libc::off_t,
    mtime_sec: libc::time_t,
    mtime_nsec: libc::c_long,
    ctime_sec: libc::time_t,
    ctime_nsec: libc::c_long,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct PngDecodeCacheKey {
    file: FileIdentity,
    strip_16: bool,
    trns_to_alpha: bool,
    strip_alpha: bool,
    gamma: Option<(u64, u64)>,
}

thread_local! {
    static PNG_DECODE_CACHE: std::cell::RefCell<crate::cache::BudgetCache<PngDecodeCacheKey, Arc<DecodedImage>>> =
        std::cell::RefCell::new(crate::cache::BudgetCache::new(PNG_CACHE_BYTES, 256));
}

const PNG_CACHE_BYTES: usize = 16 * 1024 * 1024;

pub(crate) fn reset_decode_cache() {
    PNG_DECODE_CACHE
        .with(|cache| *cache.borrow_mut() = crate::cache::BudgetCache::new(PNG_CACHE_BYTES, 256));
}

struct PngState {
    info_ptr: *mut PngInfo,
    file: *mut libc::FILE,
    data: Vec<u8>,
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    interlace_type: u8,
    valid: c_uint,
    x_pixels_per_meter: u32,
    y_pixels_per_meter: u32,
    gamma_scaled: i32,
    palette: Vec<png_color_struct>,
    trns: Option<Vec<u8>>,
    strip_16: bool,
    trns_to_alpha: bool,
    strip_alpha: bool,
    gamma: Option<(f64, f64)>,
    decoded: Option<Arc<DecodedImage>>,
    row_cursor: usize,
    jmp: Box<JmpBuf>,
}

impl PngState {
    fn new() -> Self {
        Self {
            info_ptr: ptr::null_mut(),
            file: ptr::null_mut(),
            data: Vec::new(),
            width: 0,
            height: 0,
            bit_depth: 8,
            color_type: PNG_COLOR_TYPE_RGB,
            interlace_type: PNG_INTERLACE_NONE,
            valid: 0,
            x_pixels_per_meter: 0,
            y_pixels_per_meter: 0,
            gamma_scaled: 0,
            palette: Vec::new(),
            trns: None,
            strip_16: false,
            trns_to_alpha: false,
            strip_alpha: false,
            gamma: None,
            decoded: None,
            row_cursor: 0,
            jmp: Box::new([0; 48]),
        }
    }

    fn invalidate_decode(&mut self) {
        self.decoded = None;
        self.row_cursor = 0;
    }

    fn output_bit_depth(&self) -> u8 {
        self.decoded
            .as_ref()
            .map(|decoded| decoded.bit_depth)
            .unwrap_or_else(|| {
                if self.strip_16 && self.bit_depth == 16 {
                    8
                } else {
                    self.bit_depth
                }
            })
    }

    fn output_color_type(&self) -> u8 {
        self.decoded
            .as_ref()
            .map(|decoded| decoded.color_type)
            .unwrap_or_else(|| {
                if self.strip_alpha {
                    match self.color_type {
                        PNG_COLOR_TYPE_GRAY_ALPHA => PNG_COLOR_TYPE_GRAY,
                        PNG_COLOR_TYPE_RGB_ALPHA => PNG_COLOR_TYPE_RGB,
                        other => other,
                    }
                } else {
                    self.color_type
                }
            })
    }

    fn output_rowbytes(&self) -> usize {
        self.decoded
            .as_ref()
            .map(|decoded| decoded.rowbytes)
            .unwrap_or_else(|| {
                rowbytes(
                    self.width,
                    self.output_color_type(),
                    self.output_bit_depth(),
                )
            })
    }

    fn ensure_decoded(&mut self) -> Result<(), String> {
        if self.decoded.is_some() {
            return Ok(());
        }
        let cache_key = self.decode_cache_key();
        if let Some(key) = cache_key {
            if let Some(decoded) =
                PNG_DECODE_CACHE.with(|cache| cache.borrow_mut().get(&key).cloned())
            {
                self.decoded = Some(decoded);
                self.row_cursor = 0;
                return Ok(());
            }
        }
        if self.data.is_empty() {
            self.data = unsafe { read_file(self.file)? };
        }

        let mut decoder = png::Decoder::new(Cursor::new(self.data.as_slice()));
        let mut transforms = png::Transformations::IDENTITY;
        if self.strip_16 {
            transforms |= png::Transformations::STRIP_16;
        }
        if self.trns_to_alpha {
            transforms |= png::Transformations::EXPAND;
        }
        decoder.set_transformations(transforms);

        let mut reader = decoder.read_info().map_err(|err| err.to_string())?;
        let mut data = vec![0; reader.output_buffer_size()];
        let output = reader
            .next_frame(&mut data)
            .map_err(|err| err.to_string())?;
        data.truncate(output.buffer_size());
        drop(reader);
        // Keep neither the compressed file nor the decoder's temporary buffers
        // alongside the pixels. Transform changes can reread the open file.
        self.data = Vec::new();

        let mut color_type = color_type_to_u8(output.color_type);
        let bit_depth = bit_depth_to_u8(output.bit_depth);
        let mut rowbytes = output.line_size;

        if self.strip_alpha {
            let stripped = strip_alpha(
                &data,
                self.width,
                self.height,
                color_type,
                bit_depth,
                rowbytes,
            );
            if let Some((new_data, new_color_type, new_rowbytes)) = stripped {
                data = new_data;
                color_type = new_color_type;
                rowbytes = new_rowbytes;
            }
        }

        if let Some((screen_gamma, file_gamma)) = self.gamma {
            apply_gamma(&mut data, color_type, bit_depth, screen_gamma, file_gamma);
        }

        let decoded = Arc::new(DecodedImage {
            data,
            rowbytes,
            bit_depth,
            color_type,
        });
        if let Some(key) = cache_key {
            PNG_DECODE_CACHE.with(|cache| {
                cache
                    .borrow_mut()
                    .insert(key, Arc::clone(&decoded), decoded.data.capacity() + 256);
            });
        }
        self.decoded = Some(decoded);
        self.row_cursor = 0;
        Ok(())
    }

    fn decode_cache_key(&self) -> Option<PngDecodeCacheKey> {
        Some(PngDecodeCacheKey {
            file: file_identity(self.file)?,
            strip_16: self.strip_16,
            trns_to_alpha: self.trns_to_alpha,
            strip_alpha: self.strip_alpha,
            gamma: self
                .gamma
                .map(|(screen_gamma, file_gamma)| (screen_gamma.to_bits(), file_gamma.to_bits())),
        })
    }
}

unsafe fn state<'a>(png_ptr: *const png_struct_def) -> Option<&'a mut PngState> {
    (png_ptr as *mut PngState).as_mut()
}

unsafe fn info<'a>(info_ptr: *const png_info_def) -> Option<&'a mut PngInfo> {
    (info_ptr as *mut PngInfo).as_mut()
}

unsafe fn state_from_info<'a>(info_ptr: *const png_info_def) -> Option<&'a mut PngState> {
    let info = info(info_ptr)?;
    info.state.as_mut()
}

unsafe fn state_from_any<'a>(
    png_ptr: *const png_struct_def,
    info_ptr: *const png_info_def,
) -> Option<&'a mut PngState> {
    state(png_ptr).or_else(|| state_from_info(info_ptr))
}

#[no_mangle]
pub extern "C" fn png_get_libpng_ver(_: *mut c_void) -> *const c_char {
    PNG_LIBPNG_VER_STRING.as_ptr().cast()
}

#[no_mangle]
pub unsafe extern "C" fn png_create_read_struct(
    _user_png_ver: *const c_char,
    _error_ptr: *mut c_void,
    _error_fn: PngErrorPtr,
    _warn_fn: PngErrorPtr,
) -> *mut png_struct_def {
    Box::into_raw(Box::new(PngState::new())).cast()
}

#[no_mangle]
pub unsafe extern "C" fn png_create_info_struct(
    png_ptr: *const png_struct_def,
) -> *mut png_info_def {
    let Some(state) = state(png_ptr) else {
        return ptr::null_mut();
    };
    let info = Box::into_raw(Box::new(PngInfo {
        state: state as *mut PngState,
    }));
    state.info_ptr = info;
    info.cast()
}

#[no_mangle]
pub unsafe extern "C" fn png_set_longjmp_fn(
    png_ptr: *mut png_struct_def,
    _longjmp_fn: PngLongjmpPtr,
    _jmp_buf_size: usize,
) -> *mut JmpBuf {
    state(png_ptr)
        .map(|state| state.jmp.as_mut() as *mut JmpBuf)
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn png_init_io(png_ptr: *mut png_struct_def, fp: *mut libc::FILE) {
    if let Some(state) = state(png_ptr) {
        state.file = fp;
    }
}

#[no_mangle]
pub unsafe extern "C" fn png_read_info(png_ptr: *mut png_struct_def, _info_ptr: *mut png_info_def) {
    let Some(state) = state(png_ptr) else {
        return;
    };
    let metadata = match parse_metadata_from_file(state.file) {
        Ok(metadata) => metadata,
        Err(error) => {
            let message = std::ffi::CString::new(error)
                .unwrap_or_else(|_| c"invalid PNG metadata".to_owned());
            crate::utils::pdftex_fail_args(
                c"invalid PNG metadata: %s".as_ptr(),
                &[crate::utils::PrintfArg::from(message.as_ptr())],
            );
        }
    };
    state.width = metadata.width;
    state.height = metadata.height;
    state.bit_depth = metadata.bit_depth;
    state.color_type = metadata.color_type;
    state.interlace_type = metadata.interlace_type;
    state.valid = metadata.valid;
    state.x_pixels_per_meter = metadata.x_pixels_per_meter;
    state.y_pixels_per_meter = metadata.y_pixels_per_meter;
    state.gamma_scaled = metadata.gamma_scaled;
    state.palette = metadata.palette;
    state.trns = metadata.trns;
}

#[no_mangle]
pub unsafe extern "C" fn png_destroy_read_struct(
    png_ptr_ptr: *mut *mut png_struct_def,
    info_ptr_ptr: *mut *mut png_info_def,
    _end_info_ptr_ptr: *mut *mut png_info_def,
) {
    if !info_ptr_ptr.is_null() {
        let info_ptr = *info_ptr_ptr;
        if !info_ptr.is_null() {
            drop(Box::from_raw(info_ptr.cast::<PngInfo>()));
            *info_ptr_ptr = ptr::null_mut();
        }
    }
    if !png_ptr_ptr.is_null() {
        let png_ptr = *png_ptr_ptr;
        if !png_ptr.is_null() {
            drop(Box::from_raw(png_ptr.cast::<PngState>()));
            *png_ptr_ptr = ptr::null_mut();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn png_set_tRNS_to_alpha(png_ptr: *mut png_struct_def) {
    if let Some(state) = state(png_ptr) {
        state.trns_to_alpha = true;
        state.invalidate_decode();
    }
}

#[no_mangle]
pub unsafe extern "C" fn png_set_strip_alpha(png_ptr: *mut png_struct_def) {
    if let Some(state) = state(png_ptr) {
        state.strip_alpha = true;
        state.invalidate_decode();
    }
}

#[no_mangle]
pub unsafe extern "C" fn png_set_interlace_handling(_png_ptr: *mut png_struct_def) -> c_int {
    1
}

#[no_mangle]
pub unsafe extern "C" fn png_set_strip_16(png_ptr: *mut png_struct_def) {
    if let Some(state) = state(png_ptr) {
        state.strip_16 = true;
        state.invalidate_decode();
    }
}

#[no_mangle]
pub unsafe extern "C" fn png_set_gamma(
    png_ptr: *mut png_struct_def,
    screen_gamma: c_double,
    override_file_gamma: c_double,
) {
    if let Some(state) = state(png_ptr) {
        state.gamma = Some((screen_gamma, override_file_gamma));
        state.invalidate_decode();
    }
}

#[no_mangle]
pub unsafe extern "C" fn png_read_update_info(
    png_ptr: *mut png_struct_def,
    _info_ptr: *mut png_info_def,
) {
    if let Some(state) = state(png_ptr) {
        if state.strip_16 || state.trns_to_alpha || state.strip_alpha || state.gamma.is_some() {
            let _ = state.ensure_decoded();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn png_read_row(
    png_ptr: *mut png_struct_def,
    row: *mut u8,
    _display_row: *mut u8,
) {
    let Some(state) = state(png_ptr) else {
        return;
    };
    if state.ensure_decoded().is_err() {
        return;
    }
    let Some(decoded) = state.decoded.as_ref() else {
        return;
    };
    let start = state.row_cursor.saturating_mul(decoded.rowbytes);
    let end = start.saturating_add(decoded.rowbytes);
    if !row.is_null() && end <= decoded.data.len() {
        ptr::copy_nonoverlapping(decoded.data[start..end].as_ptr(), row, decoded.rowbytes);
    }
    state.row_cursor = state.row_cursor.saturating_add(1);
}

#[no_mangle]
pub unsafe extern "C" fn png_read_image(png_ptr: *mut png_struct_def, image: *mut *mut u8) {
    let Some(state) = state(png_ptr) else {
        return;
    };
    if state.ensure_decoded().is_err() {
        return;
    }
    let Some(decoded) = state.decoded.as_ref() else {
        return;
    };
    if image.is_null() {
        return;
    }
    for y in 0..state.height as usize {
        let row_ptr = *image.add(y);
        if row_ptr.is_null() {
            continue;
        }
        let start = y * decoded.rowbytes;
        let end = start + decoded.rowbytes;
        if end <= decoded.data.len() {
            ptr::copy_nonoverlapping(decoded.data[start..end].as_ptr(), row_ptr, decoded.rowbytes);
        }
    }
    state.row_cursor = state.height as usize;
}

#[no_mangle]
pub unsafe extern "C" fn png_decoded_data(
    png_ptr: *mut c_void,
    rowbytes: *mut usize,
    len: *mut usize,
) -> *const u8 {
    let Some(state) = state(png_ptr.cast::<png_struct_def>()) else {
        return ptr::null();
    };
    if state.ensure_decoded().is_err() {
        return ptr::null();
    }
    let Some(decoded) = state.decoded.as_ref() else {
        return ptr::null();
    };
    if !rowbytes.is_null() {
        *rowbytes = decoded.rowbytes;
    }
    if !len.is_null() {
        *len = decoded.data.len();
    }
    decoded.data.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn png_get_io_ptr(png_ptr: *const png_struct_def) -> *mut c_void {
    state(png_ptr)
        .map(|state| state.file.cast::<c_void>())
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn png_get_valid(
    png_ptr: *const png_struct_def,
    info_ptr: *const png_info_def,
    flag: c_uint,
) -> c_uint {
    state_from_any(png_ptr, info_ptr)
        .map(|state| state.valid & flag)
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn png_get_rowbytes(
    png_ptr: *const png_struct_def,
    info_ptr: *const png_info_def,
) -> usize {
    state_from_any(png_ptr, info_ptr)
        .map(|state| state.output_rowbytes())
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn png_get_image_width(
    png_ptr: *const png_struct_def,
    info_ptr: *const png_info_def,
) -> c_uint {
    state_from_any(png_ptr, info_ptr)
        .map(|state| state.width)
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn png_get_image_height(
    png_ptr: *const png_struct_def,
    info_ptr: *const png_info_def,
) -> c_uint {
    state_from_any(png_ptr, info_ptr)
        .map(|state| state.height)
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn png_get_bit_depth(
    png_ptr: *const png_struct_def,
    info_ptr: *const png_info_def,
) -> u8 {
    state_from_any(png_ptr, info_ptr)
        .map(|state| state.output_bit_depth())
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn png_get_color_type(
    png_ptr: *const png_struct_def,
    info_ptr: *const png_info_def,
) -> u8 {
    state_from_any(png_ptr, info_ptr)
        .map(|state| state.output_color_type())
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn png_get_interlace_type(
    png_ptr: *const png_struct_def,
    info_ptr: *const png_info_def,
) -> u8 {
    state_from_any(png_ptr, info_ptr)
        .map(|state| state.interlace_type)
        .unwrap_or(PNG_INTERLACE_NONE)
}

#[no_mangle]
pub unsafe extern "C" fn png_get_x_pixels_per_meter(
    png_ptr: *const png_struct_def,
    info_ptr: *const png_info_def,
) -> c_uint {
    state_from_any(png_ptr, info_ptr)
        .map(|state| state.x_pixels_per_meter)
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn png_get_y_pixels_per_meter(
    png_ptr: *const png_struct_def,
    info_ptr: *const png_info_def,
) -> c_uint {
    state_from_any(png_ptr, info_ptr)
        .map(|state| state.y_pixels_per_meter)
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn png_get_gAMA(
    png_ptr: *const png_struct_def,
    info_ptr: *const png_info_def,
    file_gamma: *mut c_double,
) -> c_uint {
    let Some(state) = state_from_any(png_ptr, info_ptr) else {
        return 0;
    };
    if state.gamma_scaled == 0 || file_gamma.is_null() {
        return 0;
    }
    *file_gamma = state.gamma_scaled as c_double / 100000.0;
    PNG_INFO_GAMA
}

#[no_mangle]
pub unsafe extern "C" fn png_get_gAMA_fixed(
    png_ptr: *const png_struct_def,
    info_ptr: *const png_info_def,
    int_file_gamma: *mut c_int,
) -> c_uint {
    let Some(state) = state_from_any(png_ptr, info_ptr) else {
        return 0;
    };
    if state.gamma_scaled == 0 || int_file_gamma.is_null() {
        return 0;
    }
    *int_file_gamma = state.gamma_scaled;
    PNG_INFO_GAMA
}

#[no_mangle]
pub unsafe extern "C" fn png_get_PLTE(
    png_ptr: *const png_struct_def,
    info_ptr: *mut png_info_def,
    palette: *mut *mut png_color_struct,
    num_palette: *mut c_int,
) -> c_uint {
    let Some(state) = state_from_any(png_ptr, info_ptr) else {
        return 0;
    };
    if !palette.is_null() {
        *palette = if state.palette.is_empty() {
            ptr::null_mut()
        } else {
            state.palette.as_mut_ptr()
        };
    }
    if !num_palette.is_null() {
        *num_palette = state.palette.len() as c_int;
    }
    if state.palette.is_empty() {
        0
    } else {
        1
    }
}

#[no_mangle]
pub unsafe extern "C" fn png_set_option(
    _png_ptr: *mut png_struct_def,
    _option: c_int,
    _onoff: c_int,
) -> c_int {
    0
}

unsafe fn read_file(fp: *mut libc::FILE) -> Result<Vec<u8>, String> {
    if fp.is_null() {
        return Err("null FILE".to_string());
    }
    if libc::fseeko(fp, 0, libc::SEEK_END) != 0 {
        return Err("seek end failed".to_string());
    }
    let len = libc::ftello(fp);
    if len < 0 {
        return Err("tell failed".to_string());
    }
    if libc::fseeko(fp, 0, libc::SEEK_SET) != 0 {
        return Err("seek start failed".to_string());
    }
    let mut data = vec![0u8; len as usize];
    if !data.is_empty() {
        let read = libc::fread(data.as_mut_ptr().cast(), 1, data.len(), fp);
        if read != data.len() {
            data.truncate(read);
        }
    }
    let _ = libc::fseeko(fp, 0, libc::SEEK_SET);
    Ok(data)
}

fn file_identity(fp: *mut libc::FILE) -> Option<FileIdentity> {
    if fp.is_null() {
        return None;
    }
    let fd = unsafe { libc::fileno(fp) };
    if fd < 0 {
        return None;
    }
    let mut stat = std::mem::MaybeUninit::<libc::stat>::uninit();
    if unsafe { libc::fstat(fd, stat.as_mut_ptr()) } != 0 {
        return None;
    }
    let stat = unsafe { stat.assume_init() };
    Some(FileIdentity {
        dev: stat.st_dev,
        ino: stat.st_ino,
        size: stat.st_size,
        mtime_sec: stat_mtime_sec(&stat),
        mtime_nsec: stat_mtime_nsec(&stat),
        ctime_sec: stat.st_ctime,
        ctime_nsec: stat.st_ctime_nsec,
    })
}

fn stat_mtime_sec(stat: &libc::stat) -> libc::time_t {
    stat.st_mtime
}

fn stat_mtime_nsec(stat: &libc::stat) -> libc::c_long {
    stat.st_mtime_nsec
}

unsafe fn read_exact_file(fp: *mut libc::FILE, buf: &mut [u8]) -> Result<(), String> {
    if buf.is_empty() {
        return Ok(());
    }
    let read = unsafe { libc::fread(buf.as_mut_ptr().cast(), 1, buf.len(), fp) };
    if read == buf.len() {
        Ok(())
    } else {
        Err("short PNG read".to_string())
    }
}

unsafe fn skip_file_bytes(fp: *mut libc::FILE, len: usize) -> Result<(), String> {
    if len == 0 {
        return Ok(());
    }
    if unsafe { libc::fseeko(fp, len as libc::off_t, libc::SEEK_CUR) } == 0 {
        Ok(())
    } else {
        Err("PNG seek failed".to_string())
    }
}

unsafe fn read_be_u32_file(fp: *mut libc::FILE) -> Result<u32, String> {
    let mut buf = [0u8; 4];
    unsafe { read_exact_file(fp, &mut buf)? };
    Ok(u32::from_be_bytes(buf))
}

struct Metadata {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    interlace_type: u8,
    valid: c_uint,
    x_pixels_per_meter: u32,
    y_pixels_per_meter: u32,
    gamma_scaled: i32,
    palette: Vec<png_color_struct>,
    trns: Option<Vec<u8>>,
}

unsafe fn parse_metadata_from_file(fp: *mut libc::FILE) -> Result<Metadata, String> {
    if fp.is_null() {
        return Err("null FILE".to_string());
    }
    if unsafe { libc::fseeko(fp, 0, libc::SEEK_SET) } != 0 {
        return Err("seek start failed".to_string());
    }
    let mut signature = [0u8; 8];
    unsafe { read_exact_file(fp, &mut signature)? };
    if &signature != b"\x89PNG\r\n\x1a\n" {
        return Err("invalid PNG signature".to_string());
    }
    let mut metadata = Metadata {
        width: 0,
        height: 0,
        bit_depth: 8,
        color_type: PNG_COLOR_TYPE_RGB,
        interlace_type: PNG_INTERLACE_NONE,
        valid: 0,
        x_pixels_per_meter: 0,
        y_pixels_per_meter: 0,
        gamma_scaled: 0,
        palette: Vec::new(),
        trns: None,
    };

    let mut seen_header = false;
    loop {
        let len = unsafe { read_be_u32_file(fp)? } as usize;
        let mut typ = [0u8; 4];
        unsafe { read_exact_file(fp, &mut typ)? };
        if !seen_header && &typ != b"IHDR" {
            return Err("PNG IHDR must be the first chunk".to_string());
        }
        match &typ {
            b"IHDR" => {
                if seen_header || len != 13 {
                    return Err("invalid or duplicate PNG IHDR".to_string());
                }
                let mut chunk = [0u8; 13];
                unsafe { read_exact_file(fp, &mut chunk)? };
                metadata.width = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                metadata.height = u32::from_be_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);
                metadata.bit_depth = chunk[8];
                metadata.color_type = chunk[9];
                metadata.interlace_type = chunk[12];
                let valid_depth = match metadata.color_type {
                    PNG_COLOR_TYPE_GRAY => matches!(metadata.bit_depth, 1 | 2 | 4 | 8 | 16),
                    PNG_COLOR_TYPE_PALETTE => matches!(metadata.bit_depth, 1 | 2 | 4 | 8),
                    PNG_COLOR_TYPE_RGB | PNG_COLOR_TYPE_GRAY_ALPHA | PNG_COLOR_TYPE_RGB_ALPHA => {
                        matches!(metadata.bit_depth, 8 | 16)
                    }
                    _ => false,
                };
                if !valid_depth
                    || metadata.width == 0
                    || metadata.height == 0
                    || metadata.width > i32::MAX as u32
                    || metadata.height > i32::MAX as u32
                    || chunk[10] != 0
                    || chunk[11] != 0
                    || !matches!(chunk[12], PNG_INTERLACE_NONE | PNG_INTERLACE_ADAM7)
                {
                    return Err("invalid PNG IHDR fields".to_string());
                }
                seen_header = true;
            }
            b"gAMA" if len == 4 => {
                metadata.valid |= PNG_INFO_GAMA;
                metadata.gamma_scaled = unsafe { read_be_u32_file(fp)? } as i32;
            }
            b"pHYs" if len == 9 => {
                let mut chunk = [0u8; 9];
                unsafe { read_exact_file(fp, &mut chunk)? };
                metadata.valid |= PNG_INFO_PHYS;
                metadata.x_pixels_per_meter =
                    u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                metadata.y_pixels_per_meter =
                    u32::from_be_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);
            }
            b"PLTE" => {
                // PNG palettes have 1..=256 RGB entries. Validate before allocation.
                if len == 0 || len > 768 || len % 3 != 0 {
                    return Err("invalid PNG PLTE length".to_string());
                }
                if !metadata.palette.is_empty()
                    || metadata.trns.is_some()
                    || !matches!(
                        metadata.color_type,
                        PNG_COLOR_TYPE_PALETTE | PNG_COLOR_TYPE_RGB | PNG_COLOR_TYPE_RGB_ALPHA
                    )
                    || (metadata.color_type == PNG_COLOR_TYPE_PALETTE
                        && len / 3 > (1usize << metadata.bit_depth))
                {
                    return Err("invalid or duplicate PNG PLTE".to_string());
                }
                let mut chunk = vec![0u8; len];
                unsafe { read_exact_file(fp, &mut chunk)? };
                metadata.palette = chunk
                    .chunks_exact(3)
                    .map(|rgb| png_color_struct {
                        red: rgb[0],
                        green: rgb[1],
                        blue: rgb[2],
                    })
                    .collect();
            }
            b"tRNS" => {
                let valid_length = match metadata.color_type {
                    PNG_COLOR_TYPE_GRAY => len == 2,
                    PNG_COLOR_TYPE_RGB => len == 6,
                    PNG_COLOR_TYPE_PALETTE => {
                        !metadata.palette.is_empty() && len <= metadata.palette.len()
                    }
                    _ => false,
                };
                if !valid_length || metadata.trns.is_some() {
                    return Err("invalid or duplicate PNG tRNS length".to_string());
                }
                metadata.valid |= PNG_INFO_TRNS;
                let mut chunk = vec![0u8; len];
                unsafe { read_exact_file(fp, &mut chunk)? };
                metadata.trns = Some(chunk);
            }
            b"sBIT" => {
                metadata.valid |= PNG_INFO_SBIT;
                unsafe { skip_file_bytes(fp, len)? };
            }
            b"cHRM" => {
                metadata.valid |= PNG_INFO_CHRM;
                unsafe { skip_file_bytes(fp, len)? };
            }
            b"iCCP" => {
                metadata.valid |= PNG_INFO_ICCP;
                unsafe { skip_file_bytes(fp, len)? };
            }
            b"sRGB" => {
                metadata.valid |= PNG_INFO_SRGB;
                unsafe { skip_file_bytes(fp, len)? };
            }
            b"bKGD" => {
                metadata.valid |= PNG_INFO_BKGD;
                unsafe { skip_file_bytes(fp, len)? };
            }
            b"hIST" => {
                metadata.valid |= PNG_INFO_HIST;
                unsafe { skip_file_bytes(fp, len)? };
            }
            b"sPLT" => {
                metadata.valid |= PNG_INFO_SPLT;
                unsafe { skip_file_bytes(fp, len)? };
            }
            b"IDAT" => {
                if metadata.color_type == PNG_COLOR_TYPE_PALETTE && metadata.palette.is_empty() {
                    return Err("indexed PNG requires a PLTE chunk".to_string());
                }
                break;
            }
            b"IEND" => {
                unsafe { skip_file_bytes(fp, len)? };
                unsafe { skip_file_bytes(fp, 4)? };
                break;
            }
            _ => unsafe { skip_file_bytes(fp, len)? },
        }
        unsafe { skip_file_bytes(fp, 4)? };
    }
    let _ = unsafe { libc::fseeko(fp, 0, libc::SEEK_SET) };
    Ok(metadata)
}

fn color_type_to_u8(color_type: png::ColorType) -> u8 {
    match color_type {
        png::ColorType::Grayscale => PNG_COLOR_TYPE_GRAY,
        png::ColorType::Rgb => PNG_COLOR_TYPE_RGB,
        png::ColorType::Indexed => PNG_COLOR_TYPE_PALETTE,
        png::ColorType::GrayscaleAlpha => PNG_COLOR_TYPE_GRAY_ALPHA,
        png::ColorType::Rgba => PNG_COLOR_TYPE_RGB_ALPHA,
    }
}

fn bit_depth_to_u8(bit_depth: png::BitDepth) -> u8 {
    match bit_depth {
        png::BitDepth::One => 1,
        png::BitDepth::Two => 2,
        png::BitDepth::Four => 4,
        png::BitDepth::Eight => 8,
        png::BitDepth::Sixteen => 16,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::fs;
    use std::io::Write;
    use std::os::unix::ffi::OsStrExt;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn append_metadata_chunk(png: &mut Vec<u8>, typ: &[u8; 4], data: &[u8]) {
        png.extend_from_slice(&(data.len() as u32).to_be_bytes());
        png.extend_from_slice(typ);
        png.extend_from_slice(data);
        png.extend_from_slice(&[0; 4]);
    }

    fn metadata_fixture(color_type: u8, bit_depth: u8) -> Vec<u8> {
        let mut png = b"\x89PNG\r\n\x1a\n".to_vec();
        let mut ihdr = [0; 13];
        ihdr[..4].copy_from_slice(&1u32.to_be_bytes());
        ihdr[4..8].copy_from_slice(&1u32.to_be_bytes());
        ihdr[8] = bit_depth;
        ihdr[9] = color_type;
        append_metadata_chunk(&mut png, b"IHDR", &ihdr);
        png
    }

    // These fixtures exercise metadata reads, not IDAT decoding or CRC validation.
    fn read_metadata_fixture(png: &[u8]) -> Result<Metadata, String> {
        let fp = unsafe { libc::tmpfile() };
        assert!(!fp.is_null());
        let written = unsafe { libc::fwrite(png.as_ptr().cast(), 1, png.len(), fp) };
        assert_eq!(written, png.len());
        let result = unsafe { parse_metadata_from_file(fp) };
        unsafe { libc::fclose(fp) };
        result
    }

    #[test]
    fn palette_lengths_are_checked_before_reading_or_allocating_payloads() {
        for length in [0u32, 1, 2, 4, 769, 24 * 1024 * 1024] {
            let mut png = metadata_fixture(PNG_COLOR_TYPE_RGB, 8);
            // Only the header is present. A rejected length must not reach a payload read.
            png.extend_from_slice(&length.to_be_bytes());
            png.extend_from_slice(b"PLTE");
            let error = read_metadata_fixture(&png)
                .err()
                .expect("invalid palette accepted");
            assert_eq!(error, "invalid PNG PLTE length", "length {length}");
        }
    }

    #[test]
    fn palettes_preserve_valid_entries_and_enforce_color_and_depth_limits() {
        for (color_type, bit_depth, entries) in [
            (PNG_COLOR_TYPE_RGB, 8, 1),
            (PNG_COLOR_TYPE_RGB_ALPHA, 16, 256),
            (PNG_COLOR_TYPE_PALETTE, 1, 2),
            (PNG_COLOR_TYPE_PALETTE, 8, 256),
        ] {
            let mut png = metadata_fixture(color_type, bit_depth);
            append_metadata_chunk(&mut png, b"PLTE", &[1, 2, 3].repeat(entries));
            append_metadata_chunk(&mut png, b"IDAT", &[]);
            let metadata = read_metadata_fixture(&png).unwrap();
            assert_eq!(metadata.palette.len(), entries);
            assert_eq!(metadata.palette[entries - 1].red, 1);
            assert_eq!(metadata.palette[entries - 1].green, 2);
            assert_eq!(metadata.palette[entries - 1].blue, 3);
        }
        for (color_type, bit_depth, entries) in [
            (PNG_COLOR_TYPE_GRAY, 8, 1),
            (PNG_COLOR_TYPE_GRAY_ALPHA, 8, 1),
            (PNG_COLOR_TYPE_PALETTE, 1, 3),
        ] {
            let mut png = metadata_fixture(color_type, bit_depth);
            append_metadata_chunk(&mut png, b"PLTE", &vec![0; entries * 3]);
            append_metadata_chunk(&mut png, b"IDAT", &[]);
            assert_eq!(
                read_metadata_fixture(&png).err().unwrap(),
                "invalid or duplicate PNG PLTE"
            );
        }
        let mut png = metadata_fixture(PNG_COLOR_TYPE_PALETTE, 8);
        append_metadata_chunk(&mut png, b"IDAT", &[]);
        assert_eq!(
            read_metadata_fixture(&png).err().unwrap(),
            "indexed PNG requires a PLTE chunk"
        );
    }

    #[test]
    fn transparency_lengths_are_checked_before_reading_or_allocating_payloads() {
        for (color_type, length) in [
            (PNG_COLOR_TYPE_GRAY, 3u32),
            (PNG_COLOR_TYPE_RGB, 7),
            (PNG_COLOR_TYPE_PALETTE, 257),
            (PNG_COLOR_TYPE_RGB_ALPHA, 6),
            (PNG_COLOR_TYPE_GRAY_ALPHA, 2),
            (PNG_COLOR_TYPE_RGB, 24 * 1024 * 1024),
        ] {
            let mut png = metadata_fixture(color_type, 8);
            if color_type == PNG_COLOR_TYPE_PALETTE {
                append_metadata_chunk(&mut png, b"PLTE", &[0; 768]);
            }
            png.extend_from_slice(&length.to_be_bytes());
            png.extend_from_slice(b"tRNS");
            let error = read_metadata_fixture(&png)
                .err()
                .expect("invalid transparency accepted");
            assert_eq!(error, "invalid or duplicate PNG tRNS length");
        }
    }

    #[test]
    fn transparency_preserves_valid_grayscale_rgb_and_palette_values() {
        for (color_type, length) in [
            (PNG_COLOR_TYPE_GRAY, 2),
            (PNG_COLOR_TYPE_RGB, 6),
            (PNG_COLOR_TYPE_PALETTE, 1),
            (PNG_COLOR_TYPE_PALETTE, 256),
        ] {
            let mut png = metadata_fixture(color_type, 8);
            if color_type == PNG_COLOR_TYPE_PALETTE {
                append_metadata_chunk(&mut png, b"PLTE", &[0; 768]);
            }
            let transparency = vec![0; length];
            append_metadata_chunk(&mut png, b"tRNS", &transparency);
            append_metadata_chunk(&mut png, b"IDAT", &[]);
            let metadata = read_metadata_fixture(&png).unwrap();
            assert_eq!(metadata.valid & PNG_INFO_TRNS, PNG_INFO_TRNS);
            assert_eq!(metadata.trns.unwrap(), transparency);
        }
    }

    #[test]
    fn metadata_rejects_invalid_headers_and_repeated_bounded_chunks() {
        for (index, value) in [(16, 128), (24, 1), (25, 255), (26, 1), (27, 1), (28, 2)] {
            let mut png = metadata_fixture(PNG_COLOR_TYPE_RGB, 8);
            png[index] = value;
            append_metadata_chunk(&mut png, b"IDAT", &[]);
            assert_eq!(
                read_metadata_fixture(&png).err().unwrap(),
                "invalid PNG IHDR fields"
            );
        }
        for offset in [16, 20] {
            let mut png = metadata_fixture(PNG_COLOR_TYPE_RGB, 8);
            png[offset..offset + 4].fill(0);
            append_metadata_chunk(&mut png, b"IDAT", &[]);
            assert_eq!(
                read_metadata_fixture(&png).err().unwrap(),
                "invalid PNG IHDR fields"
            );
        }
        for typ in [b"IHDR", b"PLTE", b"tRNS"] {
            let mut png = metadata_fixture(PNG_COLOR_TYPE_RGB, 8);
            let header = png[16..29].to_vec();
            let payload = match typ {
                b"PLTE" => &[0; 3][..],
                b"tRNS" => &[0; 6][..],
                _ => &header,
            };
            if typ != b"IHDR" {
                append_metadata_chunk(&mut png, typ, payload);
            }
            append_metadata_chunk(&mut png, typ, payload);
            append_metadata_chunk(&mut png, b"IDAT", &[]);
            let expected = match typ {
                b"IHDR" => "invalid or duplicate PNG IHDR",
                b"PLTE" => "invalid or duplicate PNG PLTE",
                _ => "invalid or duplicate PNG tRNS length",
            };
            assert_eq!(read_metadata_fixture(&png).err().unwrap(), expected);
        }
        let mut png = b"\x89PNG\r\n\x1a\n".to_vec();
        append_metadata_chunk(&mut png, b"PLTE", &[0; 3]);
        append_metadata_chunk(&mut png, b"IDAT", &[]);
        assert_eq!(
            read_metadata_fixture(&png).err().unwrap(),
            "PNG IHDR must be the first chunk"
        );
    }

    #[test]
    fn decoded_cache_is_bounded_and_eviction_keeps_active_pixels_valid() {
        reset_decode_cache();
        let active = Arc::new(DecodedImage {
            data: vec![123; 1024 * 1024],
            rowbytes: 1024,
            bit_depth: 8,
            color_type: PNG_COLOR_TYPE_GRAY,
        });
        let key = |ino| PngDecodeCacheKey {
            file: FileIdentity {
                dev: 1,
                ino,
                size: 1,
                mtime_sec: 1,
                mtime_nsec: 0,
                ctime_sec: 1,
                ctime_nsec: 0,
            },
            strip_16: false,
            trns_to_alpha: false,
            strip_alpha: false,
            gamma: None,
        };
        PNG_DECODE_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            cache.insert(key(0), Arc::clone(&active), active.data.capacity() + 256);
            assert!(Arc::ptr_eq(cache.get(&key(0)).unwrap(), &active));
            for n in 1..64 {
                let decoded = Arc::new(DecodedImage {
                    data: vec![n as u8; 1024 * 1024],
                    rowbytes: 1024,
                    bit_depth: 8,
                    color_type: PNG_COLOR_TYPE_GRAY,
                });
                cache.insert(key(n), decoded, 1024 * 1024 + 256);
                assert!(cache.retained_bytes() <= PNG_CACHE_BYTES);
            }
            assert!(cache.get(&key(0)).is_none());
        });
        assert_eq!(Arc::strong_count(&active), 1);
        assert!(active.data.iter().all(|pixel| *pixel == 123));
        reset_decode_cache();
    }

    #[test]
    fn decoding_releases_compressed_data_and_preserves_alpha_transform() {
        let mut bytes = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut bytes, 1, 1);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            encoder
                .write_header()
                .unwrap()
                .write_image_data(&[10, 20, 30, 128])
                .unwrap();
        }
        let mut state = PngState::new();
        state.data = bytes;
        state.width = 1;
        state.height = 1;
        state.strip_alpha = true;
        state.ensure_decoded().unwrap();
        assert!(state.data.is_empty());
        assert_eq!(state.data.capacity(), 0);
        let decoded = state.decoded.unwrap();
        assert_eq!(decoded.color_type, PNG_COLOR_TYPE_RGB);
        assert_eq!(decoded.data, [10, 20, 30]);
    }

    #[test]
    fn real_file_cache_reuses_only_matching_transforms_and_file_identity() {
        fn encoded(pixel: &[u8]) -> Vec<u8> {
            let mut bytes = Vec::new();
            {
                let mut encoder = png::Encoder::new(&mut bytes, 1, 1);
                encoder.set_color(png::ColorType::Rgba);
                encoder.set_depth(png::BitDepth::Eight);
                encoder
                    .write_header()
                    .unwrap()
                    .write_image_data(pixel)
                    .unwrap();
            }
            bytes
        }
        reset_decode_cache();
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "tekai-png-cache-{}-{nonce}.png",
            std::process::id()
        ));
        fs::write(&path, encoded(&[10, 20, 30, 128])).unwrap();
        let original_mtime = fs::metadata(&path).unwrap().modified().unwrap();
        let name = CString::new(path.as_os_str().as_bytes()).unwrap();
        let file = unsafe { libc::fopen(name.as_ptr(), c"rb".as_ptr()) };
        assert!(!file.is_null());
        let decode = |strip_alpha| {
            let mut state = PngState::new();
            state.file = file;
            state.width = 1;
            state.height = 1;
            state.strip_alpha = strip_alpha;
            state.ensure_decoded().unwrap();
            state.decoded.unwrap()
        };
        let rgba = decode(false);
        let rgb = decode(true);
        assert_eq!(rgba.data, [10, 20, 30, 128]);
        assert_eq!(rgb.data, [10, 20, 30]);
        assert!(!Arc::ptr_eq(&rgba, &rgb));
        assert!(Arc::ptr_eq(&rgb, &decode(true)));
        let identity = file_identity(file).unwrap();
        fs::write(&path, encoded(&[50, 60, 70, 128])).unwrap();
        fs::File::options()
            .write(true)
            .open(&path)
            .unwrap()
            .set_times(fs::FileTimes::new().set_modified(original_mtime))
            .unwrap();
        assert_ne!(file_identity(file).unwrap(), identity);
        assert_eq!(decode(true).data, [50, 60, 70]);
        unsafe {
            libc::fclose(file);
        }
        fs::remove_file(path).unwrap();
        reset_decode_cache();
    }

    #[test]
    fn read_update_info_keeps_untransformed_png_lazy() {
        let mut state = PngState::new();
        let png_ptr = (&mut state as *mut PngState).cast::<png_struct_def>();

        unsafe {
            png_read_update_info(png_ptr, ptr::null_mut());
        }

        assert!(state.decoded.is_none());
    }

    #[test]
    fn read_info_stops_at_first_idat() {
        fn push_chunk(png: &mut Vec<u8>, typ: &[u8; 4], data: &[u8]) {
            png.extend_from_slice(&(data.len() as u32).to_be_bytes());
            png.extend_from_slice(typ);
            png.extend_from_slice(data);
            png.extend_from_slice(&[0, 0, 0, 0]);
        }

        let mut png = Vec::from(&b"\x89PNG\r\n\x1a\n"[..]);
        let mut ihdr = [0u8; 13];
        ihdr[0..4].copy_from_slice(&1u32.to_be_bytes());
        ihdr[4..8].copy_from_slice(&1u32.to_be_bytes());
        ihdr[8] = 8;
        ihdr[9] = PNG_COLOR_TYPE_RGB;
        push_chunk(&mut png, b"IHDR", &ihdr);
        push_chunk(&mut png, b"gAMA", &100000u32.to_be_bytes());
        push_chunk(&mut png, b"IDAT", &[0u8; 1024]);
        push_chunk(&mut png, b"pHYs", &[0, 0, 0, 1, 0, 0, 0, 1, 1]);
        push_chunk(&mut png, b"IEND", &[]);

        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "pngshim-read-info-{}-{nanos}.png",
            std::process::id()
        ));
        {
            let mut file = fs::File::create(&path).unwrap();
            file.write_all(&png).unwrap();
        }

        let c_path = CString::new(path.as_os_str().as_bytes()).unwrap();
        let mode = CString::new("rb").unwrap();
        let fp = unsafe { libc::fopen(c_path.as_ptr(), mode.as_ptr()) };
        assert!(!fp.is_null());
        let metadata = unsafe { parse_metadata_from_file(fp) }.unwrap();
        unsafe { libc::fclose(fp) };
        fs::remove_file(path).unwrap();

        assert_eq!(metadata.width, 1);
        assert_eq!(metadata.height, 1);
        assert_eq!(metadata.valid & PNG_INFO_GAMA, PNG_INFO_GAMA);
        assert_eq!(metadata.valid & PNG_INFO_PHYS, 0);
    }
}

fn samples_per_pixel(color_type: u8) -> usize {
    match color_type {
        PNG_COLOR_TYPE_GRAY | PNG_COLOR_TYPE_PALETTE => 1,
        PNG_COLOR_TYPE_RGB => 3,
        PNG_COLOR_TYPE_GRAY_ALPHA => 2,
        PNG_COLOR_TYPE_RGB_ALPHA => 4,
        _ => 1,
    }
}

fn rowbytes(width: u32, color_type: u8, bit_depth: u8) -> usize {
    let samples = width as usize * samples_per_pixel(color_type);
    if bit_depth < 8 {
        (samples * bit_depth as usize + 7) / 8
    } else {
        samples * (bit_depth as usize / 8)
    }
}

fn strip_alpha(
    data: &[u8],
    width: u32,
    height: u32,
    color_type: u8,
    bit_depth: u8,
    rowbytes: usize,
) -> Option<(Vec<u8>, u8, usize)> {
    let (channels, kept_channels, new_color_type) = match color_type {
        PNG_COLOR_TYPE_GRAY_ALPHA => (2usize, 1usize, PNG_COLOR_TYPE_GRAY),
        PNG_COLOR_TYPE_RGB_ALPHA => (4usize, 3usize, PNG_COLOR_TYPE_RGB),
        _ => return None,
    };
    let bytes_per_sample = match bit_depth {
        8 => 1usize,
        16 => 2usize,
        _ => return None,
    };
    let pixel_stride = channels * bytes_per_sample;
    let kept_stride = kept_channels * bytes_per_sample;
    let new_rowbytes = width as usize * kept_stride;
    let mut out = vec![0; new_rowbytes * height as usize];
    for y in 0..height as usize {
        let src_row = &data[y * rowbytes..y * rowbytes + rowbytes];
        let dst_row = &mut out[y * new_rowbytes..y * new_rowbytes + new_rowbytes];
        for x in 0..width as usize {
            let src = x * pixel_stride;
            let dst = x * kept_stride;
            dst_row[dst..dst + kept_stride].copy_from_slice(&src_row[src..src + kept_stride]);
        }
    }
    Some((out, new_color_type, new_rowbytes))
}

fn apply_gamma(data: &mut [u8], color_type: u8, bit_depth: u8, screen_gamma: f64, file_gamma: f64) {
    let exponent = screen_gamma * file_gamma;
    if !exponent.is_finite() || exponent <= 0.0 || (exponent - 1.0).abs() < f64::EPSILON {
        return;
    }
    let samples = samples_per_pixel(color_type);
    let alpha_sample = match color_type {
        PNG_COLOR_TYPE_GRAY_ALPHA => Some(1usize),
        PNG_COLOR_TYPE_RGB_ALPHA => Some(3usize),
        _ => None,
    };
    match bit_depth {
        8 => {
            let lut = gamma_lut_8(exponent);
            for (i, byte) in data.iter_mut().enumerate() {
                if alpha_sample.is_some_and(|alpha| i % samples == alpha) {
                    continue;
                }
                *byte = lut[*byte as usize];
            }
        }
        16 => {
            let bytes_per_pixel = samples * 2;
            for pixel in data.chunks_exact_mut(bytes_per_pixel) {
                for sample in 0..samples {
                    if alpha_sample == Some(sample) {
                        continue;
                    }
                    let offset = sample * 2;
                    let value = u16::from_be_bytes([pixel[offset], pixel[offset + 1]]);
                    let corrected = ((value as f64 / 65535.0).powf(exponent) * 65535.0)
                        .round()
                        .clamp(0.0, 65535.0) as u16;
                    let bytes = corrected.to_be_bytes();
                    pixel[offset] = bytes[0];
                    pixel[offset + 1] = bytes[1];
                }
            }
        }
        _ => {}
    }
}

fn gamma_lut_8(exponent: f64) -> [u8; 256] {
    let mut lut = [0u8; 256];
    for (i, value) in lut.iter_mut().enumerate() {
        *value = ((i as f64 / 255.0).powf(exponent) * 255.0)
            .round()
            .clamp(0.0, 255.0) as u8;
    }
    lut
}
