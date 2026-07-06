use std::ffi::{c_char, c_double, c_int, c_uint, c_void};
use std::io::Cursor;
use std::ptr;

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

struct DecodedImage {
    data: Vec<u8>,
    rowbytes: usize,
    bit_depth: u8,
    color_type: u8,
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
    decoded: Option<DecodedImage>,
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

        let mut decoder = png::Decoder::new(Cursor::new(self.data.clone()));
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

        self.decoded = Some(DecodedImage {
            data,
            rowbytes,
            bit_depth,
            color_type,
        });
        self.row_cursor = 0;
        Ok(())
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
    let Ok(data) = read_file(state.file) else {
        return;
    };
    state.data = data;
    if let Ok(metadata) = parse_metadata(&state.data) {
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
        let _ = state.ensure_decoded();
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

fn parse_metadata(data: &[u8]) -> Result<Metadata, String> {
    let decoder = png::Decoder::new(Cursor::new(data.to_vec()));
    let reader = decoder.read_info().map_err(|err| err.to_string())?;
    let info = reader.info();
    let mut metadata = Metadata {
        width: info.width,
        height: info.height,
        bit_depth: bit_depth_to_u8(info.bit_depth),
        color_type: color_type_to_u8(info.color_type),
        interlace_type: if info.interlaced {
            PNG_INTERLACE_ADAM7
        } else {
            PNG_INTERLACE_NONE
        },
        valid: 0,
        x_pixels_per_meter: 0,
        y_pixels_per_meter: 0,
        gamma_scaled: info
            .gama_chunk
            .map(|gamma| gamma.into_scaled() as i32)
            .unwrap_or(0),
        palette: info
            .palette
            .as_ref()
            .map(|palette| {
                palette
                    .chunks_exact(3)
                    .map(|rgb| png_color_struct {
                        red: rgb[0],
                        green: rgb[1],
                        blue: rgb[2],
                    })
                    .collect()
            })
            .unwrap_or_default(),
        trns: info.trns.as_ref().map(|trns| trns.to_vec()),
    };
    if metadata.gamma_scaled != 0 {
        metadata.valid |= PNG_INFO_GAMA;
    }
    if info.sbit.is_some() {
        metadata.valid |= PNG_INFO_SBIT;
    }
    if metadata.trns.is_some() {
        metadata.valid |= PNG_INFO_TRNS;
    }
    if let Some(dims) = info.pixel_dims {
        metadata.valid |= PNG_INFO_PHYS;
        metadata.x_pixels_per_meter = dims.xppu;
        metadata.y_pixels_per_meter = dims.yppu;
    }
    if info.chrm_chunk.is_some() {
        metadata.valid |= PNG_INFO_CHRM;
    }
    if info.srgb.is_some() {
        metadata.valid |= PNG_INFO_SRGB;
    }
    if info.icc_profile.is_some() {
        metadata.valid |= PNG_INFO_ICCP;
    }
    if info.bkgd.is_some() {
        metadata.valid |= PNG_INFO_BKGD;
    }
    scan_extra_chunk_flags(data, &mut metadata);
    Ok(metadata)
}

fn scan_extra_chunk_flags(data: &[u8], metadata: &mut Metadata) {
    if data.len() < 8 || &data[..8] != b"\x89PNG\r\n\x1a\n" {
        return;
    }
    let mut offset = 8usize;
    while offset
        .checked_add(12)
        .map_or(false, |end| end <= data.len())
    {
        let len = u32::from_be_bytes([
            data[offset],
            data[offset + 1],
            data[offset + 2],
            data[offset + 3],
        ]) as usize;
        let typ = &data[offset + 4..offset + 8];
        let Some(chunk_data_start) = offset.checked_add(8) else {
            break;
        };
        let Some(chunk_data_end) = chunk_data_start.checked_add(len) else {
            break;
        };
        let Some(next) = chunk_data_end.checked_add(4) else {
            break;
        };
        if next > data.len() {
            break;
        }
        let chunk_data = &data[chunk_data_start..chunk_data_end];
        match typ {
            b"gAMA" if chunk_data.len() == 4 => {
                metadata.valid |= PNG_INFO_GAMA;
                metadata.gamma_scaled = u32::from_be_bytes([
                    chunk_data[0],
                    chunk_data[1],
                    chunk_data[2],
                    chunk_data[3],
                ]) as i32;
            }
            b"pHYs" if chunk_data.len() == 9 => {
                metadata.valid |= PNG_INFO_PHYS;
                metadata.x_pixels_per_meter = u32::from_be_bytes([
                    chunk_data[0],
                    chunk_data[1],
                    chunk_data[2],
                    chunk_data[3],
                ]);
                metadata.y_pixels_per_meter = u32::from_be_bytes([
                    chunk_data[4],
                    chunk_data[5],
                    chunk_data[6],
                    chunk_data[7],
                ]);
            }
            b"PLTE" => {
                if metadata.palette.is_empty() {
                    metadata.palette = chunk_data
                        .chunks_exact(3)
                        .map(|rgb| png_color_struct {
                            red: rgb[0],
                            green: rgb[1],
                            blue: rgb[2],
                        })
                        .collect();
                }
            }
            b"tRNS" => {
                metadata.valid |= PNG_INFO_TRNS;
                if metadata.trns.is_none() {
                    metadata.trns = Some(chunk_data.to_vec());
                }
            }
            b"sBIT" => metadata.valid |= PNG_INFO_SBIT,
            b"cHRM" => metadata.valid |= PNG_INFO_CHRM,
            b"iCCP" => metadata.valid |= PNG_INFO_ICCP,
            b"sRGB" => metadata.valid |= PNG_INFO_SRGB,
            b"bKGD" => metadata.valid |= PNG_INFO_BKGD,
            b"hIST" => metadata.valid |= PNG_INFO_HIST,
            b"sPLT" => metadata.valid |= PNG_INFO_SPLT,
            b"IEND" => break,
            _ => {}
        }
        offset = next;
    }
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
