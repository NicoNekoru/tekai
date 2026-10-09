use libc::{c_char, c_double, c_float, c_int, size_t};
use lopdf::{
    Dictionary as LoDictionary, Document as LoDocument, Object as LoObject, ObjectId as LoObjectId,
    Stream as LoStream,
};
use std::collections::HashSet;
use std::ffi::{CStr, CString};
use std::ptr;

const PDF_STRING_BOOLEAN: &[u8] = b"boolean\0";
const PDF_STRING_INTEGER: &[u8] = b"integer\0";
const PDF_STRING_REAL: &[u8] = b"real\0";
const PDF_STRING_STRING: &[u8] = b"string\0";
const PDF_STRING_NAME: &[u8] = b"name\0";
const PDF_STRING_NULL: &[u8] = b"null\0";
const PDF_STRING_ARRAY: &[u8] = b"array\0";
const PDF_STRING_DICT: &[u8] = b"dict\0";
const PDF_STRING_STREAM: &[u8] = b"stream\0";
const PDF_STRING_REF: &[u8] = b"ref\0";

#[cfg(test)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct CacheCloneCounts {
    object_dict: usize,
    object_stream: usize,
    object_stream_bytes: usize,
    stream_dict: usize,
    page_group: usize,
    page_resources: usize,
}

#[cfg(test)]
thread_local! {
    static CACHE_CLONES: std::cell::Cell<CacheCloneCounts> = const {
        std::cell::Cell::new(CacheCloneCounts {
            object_dict: 0,
            object_stream: 0,
            object_stream_bytes: 0,
            stream_dict: 0,
            page_group: 0,
            page_resources: 0,
        })
    };
}

#[repr(C)]
pub struct Object {
    value: LoObject,
    doc: *mut PDFDoc,
    dict_cache: Option<Box<Dict>>,
    stream_cache: Option<Box<Stream>>,
    string_cache: Option<Box<GString>>,
    name_cache: CString,
}

#[repr(C)]
pub struct Dict {
    dict: LoDictionary,
    doc: *mut PDFDoc,
    keys: Vec<CString>,
}

#[repr(C)]
pub struct Stream {
    stream: LoStream,
    doc: *mut PDFDoc,
    pos: usize,
    dict_cache: Option<Box<Dict>>,
}

#[repr(C)]
pub struct GString {
    bytes: Vec<u8>,
    c_bytes: Vec<c_char>,
}

#[repr(C)]
pub struct PDFDoc {
    doc: Option<LoDocument>,
    ok: bool,
    page_error: Option<PageBuildError>,
    catalog: *mut Catalog,
    xref: *mut XRef,
    pages: Vec<Box<Page>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PageBuildError {
    CyclicInheritance,
}

#[repr(C)]
pub struct Catalog {
    doc: *mut PDFDoc,
}

#[repr(C)]
pub struct Page {
    doc: *mut PDFDoc,
    id: LoObjectId,
    dict: LoDictionary,
    media_box: Rect,
    crop_box: Rect,
    bleed_box: Rect,
    trim_box: Rect,
    art_box: Rect,
    rotate: c_int,
    resources: Option<LoDictionary>,
    group: Option<LoDictionary>,
    resources_cache: Option<Box<Dict>>,
    group_cache: Option<Box<Dict>>,
}

#[repr(C)]
pub struct XRef {
    doc: *mut PDFDoc,
}

#[repr(C)]
pub struct LinkDest {
    page_ref: Ref,
    ok: bool,
}

#[repr(C)]
pub struct GfxFont {
    _private: [u8; 0],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Ref {
    pub num: c_int,
    pub gen: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rect {
    pub x1: c_double,
    pub y1: c_double,
    pub x2: c_double,
    pub y2: c_double,
}

impl Object {
    fn new(value: LoObject, doc: *mut PDFDoc) -> Self {
        Self {
            value,
            doc,
            dict_cache: None,
            stream_cache: None,
            string_cache: None,
            name_cache: cstring_from_bytes(b""),
        }
    }

    fn reset(&mut self, value: LoObject, doc: *mut PDFDoc) {
        self.value = value;
        self.doc = doc;
        self.dict_cache = None;
        self.stream_cache = None;
        self.string_cache = None;
        self.name_cache = cstring_from_bytes(b"");
    }

    fn dict(&mut self) -> Option<*mut Dict> {
        if self.dict_cache.is_none() {
            let dict = match &self.value {
                LoObject::Dictionary(dict) => dict.clone(),
                LoObject::Stream(stream) => stream.dict.clone(),
                _ => return None,
            };
            #[cfg(test)]
            CACHE_CLONES.with(|counts| {
                let mut value = counts.get();
                value.object_dict += 1;
                counts.set(value);
            });
            self.dict_cache = Some(Box::new(Dict::new(dict, self.doc)));
        }
        self.dict_cache.as_deref_mut().map(|dict| dict as *mut Dict)
    }

    fn stream(&mut self) -> Option<*mut Stream> {
        if self.stream_cache.is_none() {
            let stream = match &self.value {
                LoObject::Stream(stream) => stream.clone(),
                _ => return None,
            };
            #[cfg(test)]
            CACHE_CLONES.with(|counts| {
                let mut value = counts.get();
                value.object_stream += 1;
                value.object_stream_bytes += stream.content.len();
                counts.set(value);
            });
            self.stream_cache = Some(Box::new(Stream::new(stream, self.doc)));
        }
        self.stream_cache
            .as_deref_mut()
            .map(|stream| stream as *mut Stream)
    }
}

impl Dict {
    fn new(dict: LoDictionary, doc: *mut PDFDoc) -> Self {
        let keys = dict
            .iter()
            .map(|(key, _)| cstring_from_bytes(key))
            .collect();
        Self { dict, doc, keys }
    }
}

impl Stream {
    fn new(stream: LoStream, doc: *mut PDFDoc) -> Self {
        Self {
            stream,
            doc,
            pos: 0,
            dict_cache: None,
        }
    }

    fn dict(&mut self) -> *mut Dict {
        if self.dict_cache.is_none() {
            let dict = self.stream.dict.clone();
            #[cfg(test)]
            CACHE_CLONES.with(|counts| {
                let mut value = counts.get();
                value.stream_dict += 1;
                counts.set(value);
            });
            self.dict_cache = Some(Box::new(Dict::new(dict, self.doc)));
        }
        self.dict_cache
            .as_deref_mut()
            .map(|dict| dict as *mut Dict)
            .unwrap_or(ptr::null_mut())
    }
}

impl GString {
    fn new(bytes: Vec<u8>) -> Self {
        let mut c_bytes = bytes.iter().map(|&byte| byte as c_char).collect::<Vec<_>>();
        c_bytes.push(0);
        Self { bytes, c_bytes }
    }
}

fn cstring_from_bytes(bytes: &[u8]) -> CString {
    let sanitized = bytes
        .iter()
        .map(|&byte| if byte == 0 { b'?' } else { byte })
        .collect::<Vec<_>>();
    CString::new(sanitized).unwrap_or_else(|_| c"".to_owned())
}

fn ref_from_id(id: LoObjectId) -> Ref {
    Ref {
        num: id.0 as c_int,
        gen: id.1 as c_int,
    }
}

fn id_from_ref(ref_: Ref) -> LoObjectId {
    (ref_.num.max(0) as u32, ref_.gen.max(0) as u16)
}

unsafe fn object_mut<'a>(obj: *mut Object) -> Option<&'a mut Object> {
    unsafe { obj.as_mut() }
}

unsafe fn dict_mut<'a>(dict: *mut Dict) -> Option<&'a mut Dict> {
    unsafe { dict.as_mut() }
}

unsafe fn stream_mut<'a>(stream: *mut Stream) -> Option<&'a mut Stream> {
    unsafe { stream.as_mut() }
}

unsafe fn doc_ref<'a>(doc: *mut PDFDoc) -> Option<&'a PDFDoc> {
    unsafe { doc.as_ref() }
}

unsafe fn doc_mut<'a>(doc: *mut PDFDoc) -> Option<&'a mut PDFDoc> {
    unsafe { doc.as_mut() }
}

unsafe fn lopdf_doc<'a>(doc: *mut PDFDoc) -> Option<&'a LoDocument> {
    unsafe { doc_ref(doc)?.doc.as_ref() }
}

fn deref_object(doc: *mut PDFDoc, object: &LoObject) -> LoObject {
    let mut current = object.clone();
    for _ in 0..128 {
        let LoObject::Reference(id) = current else {
            return current;
        };
        let Some(pdf) = (unsafe { lopdf_doc(doc) }) else {
            return LoObject::Reference(id);
        };
        let Some(next) = pdf.objects.get(&id).cloned() else {
            return LoObject::Reference(id);
        };
        current = next;
    }
    current
}

fn lookup_dict(dict: &LoDictionary, key: *const c_char) -> LoObject {
    if key.is_null() {
        return LoObject::Null;
    }
    let key = unsafe { CStr::from_ptr(key).to_bytes() };
    dict.get(key).cloned().unwrap_or(LoObject::Null)
}

fn dict_get(dict: &Dict, index: c_int, deref: bool) -> LoObject {
    if index < 0 {
        return LoObject::Null;
    }
    let Some(key) = dict.keys.get(index as usize) else {
        return LoObject::Null;
    };
    let value = dict
        .dict
        .get(key.as_bytes())
        .cloned()
        .unwrap_or(LoObject::Null);
    if deref {
        deref_object(dict.doc, &value)
    } else {
        value
    }
}

fn array_get(object: &Object, index: c_int, deref: bool) -> LoObject {
    if index < 0 {
        return LoObject::Null;
    }
    let LoObject::Array(array) = &object.value else {
        return LoObject::Null;
    };
    let Some(value) = array.get(index as usize) else {
        return LoObject::Null;
    };
    if deref {
        deref_object(object.doc, value)
    } else {
        value.clone()
    }
}

fn object_as_dict(object: &LoObject) -> Option<LoDictionary> {
    match object {
        LoObject::Dictionary(dict) => Some(dict.clone()),
        LoObject::Stream(stream) => Some(stream.dict.clone()),
        _ => None,
    }
}

fn lookup_inherited(
    doc: &LoDocument,
    dict: &LoDictionary,
    key: &[u8],
) -> Result<Option<LoObject>, PageBuildError> {
    let mut current = dict;
    let mut visited = HashSet::new();
    loop {
        // A local value, including null or a wrong type, shadows all ancestors.
        if let Ok(value) = current.get(key) {
            return Ok(Some(value.clone()));
        }
        let Some(parent_id) = current
            .get(b"Parent")
            .ok()
            .and_then(|parent| parent.as_reference().ok())
        else {
            return Ok(None);
        };
        if !visited.insert(parent_id) {
            return Err(PageBuildError::CyclicInheritance);
        }
        let Some(parent) = doc
            .get_object(parent_id)
            .ok()
            .and_then(|parent| parent.as_dict().ok())
        else {
            return Ok(None);
        };
        current = parent;
    }
}

fn lookup_inherited_dict(
    doc: &LoDocument,
    dict: &LoDictionary,
    key: &[u8],
) -> Result<Option<LoDictionary>, PageBuildError> {
    Ok(lookup_inherited(doc, dict, key)?
        .and_then(|value| object_as_dict(&deref_object_from_doc(doc, &value))))
}

fn deref_object_from_doc(doc: &LoDocument, object: &LoObject) -> LoObject {
    let mut current = object.clone();
    for _ in 0..128 {
        let LoObject::Reference(id) = current else {
            return current;
        };
        let Some(next) = doc.objects.get(&id).cloned() else {
            return LoObject::Reference(id);
        };
        current = next;
    }
    current
}

fn lookup_direct_dict(doc: &LoDocument, dict: &LoDictionary, key: &[u8]) -> Option<LoDictionary> {
    dict.get(key)
        .ok()
        .map(|value| deref_object_from_doc(doc, value))
        .and_then(|value| object_as_dict(&value))
}

fn rect_from_object(object: Option<LoObject>, default: Rect) -> Rect {
    let Some(LoObject::Array(values)) = object else {
        return default;
    };
    if values.len() < 4 {
        return default;
    }
    let mut coords = [0.0; 4];
    for (idx, value) in values.iter().take(4).enumerate() {
        coords[idx] = match value {
            LoObject::Integer(value) => *value as f64,
            LoObject::Real(value) => *value as f64,
            _ => return default,
        };
    }
    Rect {
        x1: coords[0],
        y1: coords[1],
        x2: coords[2],
        y2: coords[3],
    }
}

fn int_from_object(object: Option<LoObject>, default: c_int) -> c_int {
    match object {
        Some(LoObject::Integer(value)) => value as c_int,
        Some(LoObject::Real(value)) => value as c_int,
        _ => default,
    }
}

fn build_pages(doc_ptr: *mut PDFDoc, doc: &LoDocument) -> Result<Vec<Box<Page>>, PageBuildError> {
    let mut pages = Vec::new();
    for (_, id) in doc.get_pages() {
        let page_obj = doc.objects.get(&id).cloned().unwrap_or(LoObject::Null);
        let page_dict = deref_object_from_doc(doc, &page_obj)
            .as_dict()
            .cloned()
            .unwrap_or_default();
        let media_box = rect_from_object(
            lookup_inherited(doc, &page_dict, b"MediaBox")?
                .map(|value| deref_object_from_doc(doc, &value)),
            Rect {
                x1: 0.0,
                y1: 0.0,
                x2: 612.0,
                y2: 792.0,
            },
        );
        let crop_box = rect_from_object(
            lookup_inherited(doc, &page_dict, b"CropBox")?
                .map(|value| deref_object_from_doc(doc, &value)),
            media_box,
        );
        let bleed_box = rect_from_object(
            lookup_inherited(doc, &page_dict, b"BleedBox")?
                .map(|value| deref_object_from_doc(doc, &value)),
            crop_box,
        );
        let trim_box = rect_from_object(
            lookup_inherited(doc, &page_dict, b"TrimBox")?
                .map(|value| deref_object_from_doc(doc, &value)),
            crop_box,
        );
        let art_box = rect_from_object(
            lookup_inherited(doc, &page_dict, b"ArtBox")?
                .map(|value| deref_object_from_doc(doc, &value)),
            crop_box,
        );
        let rotate = int_from_object(
            lookup_inherited(doc, &page_dict, b"Rotate")?
                .map(|value| deref_object_from_doc(doc, &value)),
            0,
        );
        pages.push(Box::new(Page {
            doc: doc_ptr,
            id,
            dict: page_dict.clone(),
            media_box,
            crop_box,
            bleed_box,
            trim_box,
            art_box,
            rotate,
            resources: lookup_inherited_dict(doc, &page_dict, b"Resources")?,
            group: lookup_direct_dict(doc, &page_dict, b"Group"),
            resources_cache: None,
            group_cache: None,
        }));
    }
    Ok(pages)
}

fn pdf_version(doc: &LoDocument) -> c_float {
    doc.version.parse::<f32>().unwrap_or(1.4) as c_float
}

#[no_mangle]
pub extern "C" fn xpdf_global_params_init() {}

#[no_mangle]
pub extern "C" fn xpdf_global_params_free() {}

#[no_mangle]
pub extern "C" fn xpdf_object_new() -> *mut Object {
    Box::into_raw(Box::new(Object::new(LoObject::Null, ptr::null_mut())))
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_delete(obj: *mut Object) {
    if !obj.is_null() {
        unsafe {
            drop(Box::from_raw(obj));
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_free_contents(obj: *mut Object) {
    if let Some(obj) = unsafe { object_mut(obj) } {
        obj.reset(LoObject::Null, obj.doc);
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_type_name(obj: *mut Object) -> *const c_char {
    let Some(obj) = (unsafe { object_mut(obj) }) else {
        return PDF_STRING_NULL.as_ptr().cast();
    };
    match obj.value {
        LoObject::Boolean(_) => PDF_STRING_BOOLEAN.as_ptr().cast(),
        LoObject::Integer(_) => PDF_STRING_INTEGER.as_ptr().cast(),
        LoObject::Real(_) => PDF_STRING_REAL.as_ptr().cast(),
        LoObject::String(_, _) => PDF_STRING_STRING.as_ptr().cast(),
        LoObject::Name(_) => PDF_STRING_NAME.as_ptr().cast(),
        LoObject::Null => PDF_STRING_NULL.as_ptr().cast(),
        LoObject::Array(_) => PDF_STRING_ARRAY.as_ptr().cast(),
        LoObject::Dictionary(_) => PDF_STRING_DICT.as_ptr().cast(),
        LoObject::Stream(_) => PDF_STRING_STREAM.as_ptr().cast(),
        LoObject::Reference(_) => PDF_STRING_REF.as_ptr().cast(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_is_bool(obj: *mut Object) -> c_int {
    unsafe { object_mut(obj) }.is_some_and(|obj| matches!(obj.value, LoObject::Boolean(_))) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_is_int(obj: *mut Object) -> c_int {
    unsafe { object_mut(obj) }.is_some_and(|obj| matches!(obj.value, LoObject::Integer(_))) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_is_real(obj: *mut Object) -> c_int {
    unsafe { object_mut(obj) }.is_some_and(|obj| matches!(obj.value, LoObject::Real(_))) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_is_num(obj: *mut Object) -> c_int {
    unsafe { object_mut(obj) }
        .is_some_and(|obj| matches!(obj.value, LoObject::Integer(_) | LoObject::Real(_)))
        as c_int
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_is_string(obj: *mut Object) -> c_int {
    unsafe { object_mut(obj) }.is_some_and(|obj| matches!(obj.value, LoObject::String(_, _)))
        as c_int
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_is_name(obj: *mut Object) -> c_int {
    unsafe { object_mut(obj) }.is_some_and(|obj| matches!(obj.value, LoObject::Name(_))) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_is_null(obj: *mut Object) -> c_int {
    unsafe { object_mut(obj) }.is_none_or(|obj| matches!(obj.value, LoObject::Null)) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_is_array(obj: *mut Object) -> c_int {
    unsafe { object_mut(obj) }.is_some_and(|obj| matches!(obj.value, LoObject::Array(_))) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_is_dict(obj: *mut Object) -> c_int {
    unsafe { object_mut(obj) }.is_some_and(|obj| matches!(obj.value, LoObject::Dictionary(_)))
        as c_int
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_is_stream(obj: *mut Object) -> c_int {
    unsafe { object_mut(obj) }.is_some_and(|obj| matches!(obj.value, LoObject::Stream(_))) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_is_ref(obj: *mut Object) -> c_int {
    unsafe { object_mut(obj) }.is_some_and(|obj| matches!(obj.value, LoObject::Reference(_)))
        as c_int
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_get_bool(obj: *mut Object) -> c_int {
    match unsafe { object_mut(obj) }.map(|obj| &obj.value) {
        Some(LoObject::Boolean(value)) => *value as c_int,
        _ => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_get_int(obj: *mut Object) -> c_int {
    match unsafe { object_mut(obj) }.map(|obj| &obj.value) {
        Some(LoObject::Integer(value)) => *value as c_int,
        Some(LoObject::Real(value)) => *value as c_int,
        _ => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_get_real(obj: *mut Object) -> c_double {
    match unsafe { object_mut(obj) }.map(|obj| &obj.value) {
        Some(LoObject::Real(value)) => *value as c_double,
        Some(LoObject::Integer(value)) => *value as c_double,
        _ => 0.0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_get_num(obj: *mut Object) -> c_double {
    unsafe { xpdf_object_get_real(obj) }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_get_string(obj: *mut Object) -> *mut GString {
    let Some(obj) = (unsafe { object_mut(obj) }) else {
        return ptr::null_mut();
    };
    let LoObject::String(bytes, _) = &obj.value else {
        return ptr::null_mut();
    };
    obj.string_cache = Some(Box::new(GString::new(bytes.clone())));
    obj.string_cache
        .as_deref_mut()
        .map(|string| string as *mut GString)
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_get_name(obj: *mut Object) -> *mut c_char {
    let Some(obj) = (unsafe { object_mut(obj) }) else {
        return ptr::null_mut();
    };
    let LoObject::Name(bytes) = &obj.value else {
        return ptr::null_mut();
    };
    obj.name_cache = cstring_from_bytes(bytes);
    obj.name_cache.as_ptr() as *mut c_char
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_get_dict(obj: *mut Object) -> *mut Dict {
    unsafe { object_mut(obj) }
        .and_then(Object::dict)
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_get_stream(obj: *mut Object) -> *mut Stream {
    unsafe { object_mut(obj) }
        .and_then(Object::stream)
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_get_ref(obj: *mut Object) -> Ref {
    match unsafe { object_mut(obj) }.map(|obj| &obj.value) {
        Some(LoObject::Reference(id)) => ref_from_id(*id),
        _ => Ref { num: 0, gen: 0 },
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_fetch(
    obj: *mut Object,
    xref: *mut XRef,
    out: *mut Object,
) -> *mut Object {
    let doc = unsafe { xref.as_ref() }
        .map(|xref| xref.doc)
        .or_else(|| unsafe { object_mut(obj) }.map(|obj| obj.doc))
        .unwrap_or(ptr::null_mut());
    let value = unsafe { object_mut(obj) }
        .map(|obj| deref_object(doc, &obj.value))
        .unwrap_or(LoObject::Null);
    if let Some(out) = unsafe { object_mut(out) } {
        out.reset(value, doc);
        out as *mut Object
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_array_get_length(obj: *mut Object) -> c_int {
    match unsafe { object_mut(obj) }.map(|obj| &obj.value) {
        Some(LoObject::Array(array)) => array.len() as c_int,
        _ => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_array_get(
    obj: *mut Object,
    index: c_int,
    out: *mut Object,
) -> *mut Object {
    let (value, doc) = unsafe { object_mut(obj) }
        .map(|obj| (array_get(obj, index, true), obj.doc))
        .unwrap_or((LoObject::Null, ptr::null_mut()));
    if let Some(out) = unsafe { object_mut(out) } {
        out.reset(value, doc);
        out as *mut Object
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_array_get_nf(
    obj: *mut Object,
    index: c_int,
    out: *mut Object,
) -> *mut Object {
    let (value, doc) = unsafe { object_mut(obj) }
        .map(|obj| (array_get(obj, index, false), obj.doc))
        .unwrap_or((LoObject::Null, ptr::null_mut()));
    if let Some(out) = unsafe { object_mut(out) } {
        out.reset(value, doc);
        out as *mut Object
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_dict_get_length(obj: *mut Object) -> c_int {
    let Some(dict_ptr) = (unsafe { xpdf_object_get_dict(obj).as_mut() }) else {
        return 0;
    };
    dict_ptr.dict.len() as c_int
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_dict_get_key(obj: *mut Object, index: c_int) -> *mut c_char {
    let dict_ptr = unsafe { xpdf_object_get_dict(obj) };
    unsafe { xpdf_dict_get_key(dict_ptr, index) }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_dict_get_val(
    obj: *mut Object,
    index: c_int,
    out: *mut Object,
) -> *mut Object {
    let dict_ptr = unsafe { xpdf_object_get_dict(obj) };
    unsafe { xpdf_dict_get_val(dict_ptr, index, out) }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_dict_get_val_nf(
    obj: *mut Object,
    index: c_int,
    out: *mut Object,
) -> *mut Object {
    let dict_ptr = unsafe { xpdf_object_get_dict(obj) };
    unsafe { xpdf_dict_get_val_nf(dict_ptr, index, out) }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_dict_lookup(
    obj: *mut Object,
    key: *const c_char,
    out: *mut Object,
) -> *mut Object {
    let dict_ptr = unsafe { xpdf_object_get_dict(obj) };
    unsafe { xpdf_dict_lookup(dict_ptr, key, out) }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_dict_lookup_nf(
    obj: *mut Object,
    key: *const c_char,
    out: *mut Object,
) -> *mut Object {
    let dict_ptr = unsafe { xpdf_object_get_dict(obj) };
    unsafe { xpdf_dict_lookup_nf(dict_ptr, key, out) }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_stream_get_dict(obj: *mut Object) -> *mut Dict {
    let stream = unsafe { xpdf_object_get_stream(obj) };
    unsafe { xpdf_stream_get_dict(stream) }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_stream_get_undecoded_stream(obj: *mut Object) -> *mut Stream {
    unsafe { xpdf_object_get_stream(obj) }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_object_init_dict_from_dict(
    obj: *mut Object,
    _xref: *mut XRef,
    dict: *mut Dict,
) {
    let Some(dict) = (unsafe { dict_mut(dict) }) else {
        return;
    };
    if let Some(obj) = unsafe { object_mut(obj) } {
        obj.reset(LoObject::Dictionary(dict.dict.clone()), dict.doc);
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_dict_get_length(dict: *mut Dict) -> c_int {
    unsafe { dict_mut(dict) }
        .map(|dict| dict.dict.len() as c_int)
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_dict_get_key(dict: *mut Dict, index: c_int) -> *mut c_char {
    if index < 0 {
        return ptr::null_mut();
    }
    let Some(dict) = (unsafe { dict_mut(dict) }) else {
        return ptr::null_mut();
    };
    dict.keys
        .get(index as usize)
        .map(|key| key.as_ptr() as *mut c_char)
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_dict_get_val(
    dict: *mut Dict,
    index: c_int,
    out: *mut Object,
) -> *mut Object {
    let (value, doc) = unsafe { dict_mut(dict) }
        .map(|dict| (dict_get(dict, index, true), dict.doc))
        .unwrap_or((LoObject::Null, ptr::null_mut()));
    if let Some(out) = unsafe { object_mut(out) } {
        out.reset(value, doc);
        out as *mut Object
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_dict_get_val_nf(
    dict: *mut Dict,
    index: c_int,
    out: *mut Object,
) -> *mut Object {
    let (value, doc) = unsafe { dict_mut(dict) }
        .map(|dict| (dict_get(dict, index, false), dict.doc))
        .unwrap_or((LoObject::Null, ptr::null_mut()));
    if let Some(out) = unsafe { object_mut(out) } {
        out.reset(value, doc);
        out as *mut Object
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_dict_lookup(
    dict: *mut Dict,
    key: *const c_char,
    out: *mut Object,
) -> *mut Object {
    let (value, doc) = unsafe { dict_mut(dict) }
        .map(|dict| {
            (
                deref_object(dict.doc, &lookup_dict(&dict.dict, key)),
                dict.doc,
            )
        })
        .unwrap_or((LoObject::Null, ptr::null_mut()));
    if let Some(out) = unsafe { object_mut(out) } {
        out.reset(value, doc);
        out as *mut Object
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_dict_lookup_nf(
    dict: *mut Dict,
    key: *const c_char,
    out: *mut Object,
) -> *mut Object {
    let (value, doc) = unsafe { dict_mut(dict) }
        .map(|dict| (lookup_dict(&dict.dict, key), dict.doc))
        .unwrap_or((LoObject::Null, ptr::null_mut()));
    if let Some(out) = unsafe { object_mut(out) } {
        out.reset(value, doc);
        out as *mut Object
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_gstring_cstr(s: *mut GString) -> *const c_char {
    unsafe { s.as_ref() }
        .map(|s| s.c_bytes.as_ptr())
        .unwrap_or(ptr::null())
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_gstring_len(s: *mut GString) -> c_int {
    unsafe { s.as_ref() }
        .map(|s| s.bytes.len() as c_int)
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_gstring_char(s: *mut GString, index: c_int) -> c_int {
    if index < 0 {
        return 0;
    }
    unsafe { s.as_ref() }
        .and_then(|s| s.bytes.get(index as usize).copied())
        .map(c_int::from)
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_stream_reset(stream: *mut Stream) {
    if let Some(stream) = unsafe { stream_mut(stream) } {
        stream.pos = 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_stream_get_char(stream: *mut Stream) -> c_int {
    let Some(stream) = (unsafe { stream_mut(stream) }) else {
        return libc::EOF;
    };
    let Some(byte) = stream.stream.content.get(stream.pos).copied() else {
        return libc::EOF;
    };
    stream.pos += 1;
    byte as c_int
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_stream_get_remaining_data(
    stream: *mut Stream,
    len: *mut size_t,
) -> *const u8 {
    let Some(stream) = (unsafe { stream_mut(stream) }) else {
        if !len.is_null() {
            unsafe {
                *len = 0;
            }
        }
        return ptr::null();
    };
    let start = stream.pos.min(stream.stream.content.len());
    let bytes = &stream.stream.content[start..];
    stream.pos = stream.stream.content.len();
    if !len.is_null() {
        unsafe {
            *len = bytes.len();
        }
    }
    if bytes.is_empty() {
        ptr::null()
    } else {
        bytes.as_ptr()
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_stream_get_dict(stream: *mut Stream) -> *mut Dict {
    unsafe { stream_mut(stream) }
        .map(Stream::dict)
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_stream_get_undecoded_stream(stream: *mut Stream) -> *mut Stream {
    stream
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_doc_new(file_name: *const c_char) -> *mut PDFDoc {
    if file_name.is_null() {
        return Box::into_raw(Box::new(PDFDoc {
            doc: None,
            ok: false,
            page_error: None,
            catalog: ptr::null_mut(),
            xref: ptr::null_mut(),
            pages: Vec::new(),
        }));
    }
    let path = unsafe { CStr::from_ptr(file_name) }
        .to_string_lossy()
        .into_owned();
    let (doc, ok) = match LoDocument::load(&path) {
        Ok(doc) => (Some(doc), true),
        Err(_) => (None, false),
    };
    let pdf_doc = Box::into_raw(Box::new(PDFDoc {
        doc,
        ok,
        page_error: None,
        catalog: ptr::null_mut(),
        xref: ptr::null_mut(),
        pages: Vec::new(),
    }));
    if let Some(doc) = unsafe { doc_ref(pdf_doc) }.and_then(|pdf| pdf.doc.as_ref()) {
        let pages = build_pages(pdf_doc, doc);
        if let Some(pdf) = unsafe { doc_mut(pdf_doc) } {
            match pages {
                Ok(pages) => pdf.pages = pages,
                Err(error) => {
                    pdf.ok = false;
                    pdf.page_error = Some(error);
                }
            }
        }
    }
    let catalog = Box::into_raw(Box::new(Catalog { doc: pdf_doc }));
    let xref = Box::into_raw(Box::new(XRef { doc: pdf_doc }));
    if let Some(pdf) = unsafe { doc_mut(pdf_doc) } {
        pdf.catalog = catalog;
        pdf.xref = xref;
    }
    pdf_doc
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_doc_delete(doc: *mut PDFDoc) {
    if doc.is_null() {
        return;
    }
    let doc = unsafe { Box::from_raw(doc) };
    if !doc.catalog.is_null() {
        unsafe {
            drop(Box::from_raw(doc.catalog));
        }
    }
    if !doc.xref.is_null() {
        unsafe {
            drop(Box::from_raw(doc.xref));
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_doc_is_ok(doc: *mut PDFDoc) -> c_int {
    unsafe { doc_ref(doc) }.is_some_and(|doc| doc.ok) as c_int
}

#[no_mangle]
/// The returned diagnostic has static storage and survives document deletion.
pub unsafe extern "C" fn xpdf_doc_error_message(doc: *mut PDFDoc) -> *const c_char {
    match unsafe { doc_ref(doc) }.and_then(|doc| doc.page_error) {
        Some(PageBuildError::CyclicInheritance) => c"xpdf: cyclic PDF page Parent chain".as_ptr(),
        None => c"xpdf: reading PDF image failed".as_ptr(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_doc_ok_to_print(doc: *mut PDFDoc) -> c_int {
    unsafe { xpdf_doc_is_ok(doc) }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_doc_pdf_version(doc: *mut PDFDoc) -> c_float {
    unsafe { doc_ref(doc) }
        .and_then(|doc| doc.doc.as_ref())
        .map(pdf_version)
        .unwrap_or(1.4)
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_doc_catalog(doc: *mut PDFDoc) -> *mut Catalog {
    unsafe { doc_ref(doc) }
        .map(|doc| doc.catalog)
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_doc_xref(doc: *mut PDFDoc) -> *mut XRef {
    unsafe { doc_ref(doc) }
        .map(|doc| doc.xref)
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_doc_find_dest(
    _doc: *mut PDFDoc,
    _name: *const c_char,
) -> *mut LinkDest {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_doc_get_info_nf(doc: *mut PDFDoc, out: *mut Object) {
    let value = unsafe { doc_ref(doc) }
        .and_then(|doc| doc.doc.as_ref())
        .and_then(|doc| doc.trailer.get(b"Info").ok().cloned())
        .unwrap_or(LoObject::Null);
    if let Some(out) = unsafe { object_mut(out) } {
        out.reset(value, doc);
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_link_dest_delete(dest: *mut LinkDest) {
    if !dest.is_null() {
        unsafe {
            drop(Box::from_raw(dest));
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_link_dest_is_ok(dest: *mut LinkDest) -> c_int {
    unsafe { dest.as_ref() }.is_some_and(|dest| dest.ok) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_link_dest_page_ref(dest: *mut LinkDest) -> Ref {
    unsafe { dest.as_ref() }
        .map(|dest| dest.page_ref)
        .unwrap_or(Ref { num: 0, gen: 0 })
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_catalog_num_pages(catalog: *mut Catalog) -> c_int {
    let Some(catalog) = (unsafe { catalog.as_ref() }) else {
        return 0;
    };
    unsafe { doc_ref(catalog.doc) }
        .map(|doc| doc.pages.len() as c_int)
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_catalog_get_page(
    catalog: *mut Catalog,
    page_num: c_int,
) -> *mut Page {
    if page_num <= 0 {
        return ptr::null_mut();
    }
    let Some(catalog) = (unsafe { catalog.as_ref() }) else {
        return ptr::null_mut();
    };
    unsafe { doc_mut(catalog.doc) }
        .and_then(|doc| doc.pages.get_mut((page_num - 1) as usize))
        .map(|page| page.as_mut() as *mut Page)
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_catalog_get_page_ref(catalog: *mut Catalog, page_num: c_int) -> Ref {
    let page = unsafe { xpdf_catalog_get_page(catalog, page_num) };
    unsafe { page.as_ref() }
        .map(|page| ref_from_id(page.id))
        .unwrap_or(Ref { num: 0, gen: 0 })
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_catalog_find_page(
    catalog: *mut Catalog,
    num: c_int,
    gen: c_int,
) -> c_int {
    let Some(catalog) = (unsafe { catalog.as_ref() }) else {
        return 0;
    };
    let id = (num.max(0) as u32, gen.max(0) as u16);
    unsafe { doc_ref(catalog.doc) }
        .and_then(|doc| {
            doc.pages
                .iter()
                .position(|page| page.id == id)
                .map(|idx| idx as c_int + 1)
        })
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_xref_fetch(
    xref: *mut XRef,
    num: c_int,
    gen: c_int,
    out: *mut Object,
) {
    let doc = unsafe { xref.as_ref() }
        .map(|xref| xref.doc)
        .unwrap_or(ptr::null_mut());
    let value = unsafe { lopdf_doc(doc) }
        .and_then(|doc| doc.objects.get(&id_from_ref(Ref { num, gen })).cloned())
        .unwrap_or(LoObject::Null);
    if let Some(out) = unsafe { object_mut(out) } {
        out.reset(value, doc);
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_page_media_box(page: *mut Page) -> Rect {
    unsafe { page.as_ref() }
        .map(|page| page.media_box)
        .unwrap_or(Rect {
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 0.0,
        })
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_page_crop_box(page: *mut Page) -> Rect {
    unsafe { page.as_ref() }
        .map(|page| page.crop_box)
        .unwrap_or_else(|| unsafe { xpdf_page_media_box(page) })
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_page_bleed_box(page: *mut Page) -> Rect {
    unsafe { page.as_ref() }
        .map(|page| page.bleed_box)
        .unwrap_or_else(|| unsafe { xpdf_page_crop_box(page) })
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_page_trim_box(page: *mut Page) -> Rect {
    unsafe { page.as_ref() }
        .map(|page| page.trim_box)
        .unwrap_or_else(|| unsafe { xpdf_page_crop_box(page) })
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_page_art_box(page: *mut Page) -> Rect {
    unsafe { page.as_ref() }
        .map(|page| page.art_box)
        .unwrap_or_else(|| unsafe { xpdf_page_crop_box(page) })
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_page_rotate(page: *mut Page) -> c_int {
    unsafe { page.as_ref() }
        .map(|page| page.rotate)
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_page_group(page: *mut Page) -> *mut Dict {
    let Some(page) = (unsafe { page.as_mut() }) else {
        return ptr::null_mut();
    };
    if page.group_cache.is_none() {
        let Some(group) = page.group.clone() else {
            return ptr::null_mut();
        };
        #[cfg(test)]
        CACHE_CLONES.with(|counts| {
            let mut value = counts.get();
            value.page_group += 1;
            counts.set(value);
        });
        page.group_cache = Some(Box::new(Dict::new(group, page.doc)));
    }
    page.group_cache
        .as_deref_mut()
        .map(|dict| dict as *mut Dict)
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_page_resource_dict(page: *mut Page) -> *mut Dict {
    let Some(page) = (unsafe { page.as_mut() }) else {
        return ptr::null_mut();
    };
    if page.resources_cache.is_none() {
        let Some(resources) = page.resources.clone() else {
            return ptr::null_mut();
        };
        #[cfg(test)]
        CACHE_CLONES.with(|counts| {
            let mut value = counts.get();
            value.page_resources += 1;
            counts.set(value);
        });
        page.resources_cache = Some(Box::new(Dict::new(resources, page.doc)));
    }
    page.resources_cache
        .as_deref_mut()
        .map(|dict| dict as *mut Dict)
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_page_get_contents(page: *mut Page, out: *mut Object) {
    let Some(page) = (unsafe { page.as_ref() }) else {
        return;
    };
    let value = page
        .dict
        .get(b"Contents")
        .map(|value| deref_object(page.doc, value))
        .unwrap_or(LoObject::Null);
    if let Some(out) = unsafe { object_mut(out) } {
        out.reset(value, page.doc);
    }
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_gfx_font_make(
    _xref: *mut XRef,
    _tag: *const c_char,
    _id: Ref,
    _font_dict: *mut Dict,
) -> *mut GfxFont {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_gfx_font_delete(_font: *mut GfxFont) {}

#[no_mangle]
pub unsafe extern "C" fn xpdf_gfx_font_is_cid(_font: *mut GfxFont) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn xpdf_gfx_8bit_font_char_name(
    _font: *mut GfxFont,
    _code: c_int,
) -> *mut c_char {
    ptr::null_mut()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::slice;

    fn reset_clone_counts() {
        CACHE_CLONES.with(|counts| counts.set(CacheCloneCounts::default()));
    }

    fn clone_counts() -> CacheCloneCounts {
        CACHE_CLONES.with(std::cell::Cell::get)
    }

    fn test_document(doc: LoDocument) -> Box<PDFDoc> {
        Box::new(PDFDoc {
            doc: Some(doc),
            ok: true,
            page_error: None,
            catalog: ptr::null_mut(),
            xref: ptr::null_mut(),
            pages: Vec::new(),
        })
    }

    fn test_page(resources: Option<LoDictionary>, group: Option<LoDictionary>) -> Page {
        let rect = Rect {
            x1: 0.0,
            y1: 0.0,
            x2: 1.0,
            y2: 1.0,
        };
        Page {
            doc: ptr::null_mut(),
            id: (1, 0),
            dict: LoDictionary::new(),
            media_box: rect,
            crop_box: rect,
            bleed_box: rect,
            trim_box: rect,
            art_box: rect,
            rotate: 0,
            resources,
            group,
            resources_cache: None,
            group_cache: None,
        }
    }

    fn parent_dictionary(parent: LoObjectId) -> LoDictionary {
        let mut dict = LoDictionary::new();
        dict.set("Parent", LoObject::Reference(parent));
        dict
    }

    #[test]
    fn inherited_values_reject_self_two_parent_and_return_to_page_cycles() {
        for edges in [
            vec![((1, 0), (1, 0))],
            vec![((1, 0), (2, 0)), ((2, 0), (1, 0))],
            vec![((1, 0), (2, 0)), ((2, 0), (3, 0)), ((3, 0), (1, 0))],
        ] {
            let mut doc = LoDocument::new();
            for (id, parent) in &edges {
                doc.objects
                    .insert(*id, LoObject::Dictionary(parent_dictionary(*parent)));
            }
            let page = doc.objects.get(&(1, 0)).unwrap().as_dict().unwrap();
            assert_eq!(
                lookup_inherited(&doc, page, b"Missing").unwrap_err(),
                PageBuildError::CyclicInheritance
            );
        }
    }

    #[test]
    fn inherited_values_use_complete_object_ids_and_nearest_values() {
        let mut doc = LoDocument::new();
        let page = parent_dictionary((7, 0));
        doc.objects
            .insert((7, 0), LoObject::Dictionary(parent_dictionary((7, 1))));
        let mut ancestor = LoDictionary::new();
        ancestor.set("Rotate", 90);
        doc.objects.insert((7, 1), LoObject::Dictionary(ancestor));
        assert_eq!(
            lookup_inherited(&doc, &page, b"Rotate")
                .unwrap()
                .unwrap()
                .as_i64()
                .unwrap(),
            90
        );
        doc.objects
            .get_mut(&(7, 0))
            .unwrap()
            .as_dict_mut()
            .unwrap()
            .set("Rotate", 180);
        assert_eq!(
            lookup_inherited(&doc, &page, b"Rotate")
                .unwrap()
                .unwrap()
                .as_i64()
                .unwrap(),
            180
        );
    }

    #[test]
    fn local_values_shadow_ancestors_even_on_cyclic_pages() {
        let mut doc = LoDocument::new();
        let mut page = parent_dictionary((1, 0));
        page.set("Rotate", 90);
        page.set("Resources", LoObject::Null);
        doc.objects
            .insert((1, 0), LoObject::Dictionary(page.clone()));
        assert_eq!(
            lookup_inherited(&doc, &page, b"Rotate")
                .unwrap()
                .unwrap()
                .as_i64()
                .unwrap(),
            90
        );
        assert!(lookup_inherited(&doc, &page, b"Resources")
            .unwrap()
            .unwrap()
            .is_null());
        assert!(lookup_inherited_dict(&doc, &page, b"Resources")
            .unwrap()
            .is_none());
        page.set("Resources", 12);
        assert!(lookup_inherited_dict(&doc, &page, b"Resources")
            .unwrap()
            .is_none());
    }

    #[test]
    fn missing_broken_and_nonreference_parents_end_inheritance() {
        let mut doc = LoDocument::new();
        doc.objects.insert((2, 0), LoObject::Integer(1));
        for page in [
            LoDictionary::new(),
            parent_dictionary((99, 0)),
            parent_dictionary((2, 0)),
            {
                let mut dict = LoDictionary::new();
                dict.set("Parent", LoObject::Dictionary(LoDictionary::new()));
                dict
            },
        ] {
            assert!(lookup_inherited(&doc, &page, b"Resources")
                .unwrap()
                .is_none());
        }
    }

    #[test]
    fn parent_aliases_and_referenced_resources_keep_existing_resolution_rules() {
        let mut doc = LoDocument::new();
        let page = parent_dictionary((1, 0));
        doc.objects.insert((1, 0), LoObject::Reference((2, 0)));
        let mut parent = LoDictionary::new();
        parent.set("Resources", LoObject::Reference((3, 0)));
        doc.objects.insert((2, 0), LoObject::Dictionary(parent));
        let mut resources = LoDictionary::new();
        resources.set("Font", LoObject::Reference((17, 3)));
        doc.objects.insert((3, 0), LoObject::Dictionary(resources));
        assert_eq!(
            lookup_inherited_dict(&doc, &page, b"Resources")
                .unwrap()
                .unwrap()
                .get(b"Font")
                .unwrap()
                .as_reference()
                .unwrap(),
            (17, 3)
        );
        doc.objects
            .get_mut(&(2, 0))
            .unwrap()
            .as_dict_mut()
            .unwrap()
            .set("Parent", LoObject::Reference((1, 0)));
        assert_eq!(
            lookup_inherited(&doc, &page, b"Missing").unwrap_err(),
            PageBuildError::CyclicInheritance
        );

        // lopdf's existing reference limit treats reference-only loops as broken parents.
        doc.objects.insert((2, 0), LoObject::Reference((1, 0)));
        assert!(lookup_inherited(&doc, &page, b"Missing").unwrap().is_none());
    }

    #[test]
    fn deep_acyclic_inheritance_does_not_use_the_host_stack_or_reference_limit() {
        let mut doc = LoDocument::new();
        let depth = 4096;
        for number in 1..depth {
            doc.objects.insert(
                (number, 0),
                LoObject::Dictionary(parent_dictionary((number + 1, 0))),
            );
        }
        let mut root = LoDictionary::new();
        root.set("Rotate", 270);
        doc.objects.insert((depth, 0), LoObject::Dictionary(root));
        let page = parent_dictionary((1, 0));
        assert_eq!(
            lookup_inherited(&doc, &page, b"Rotate")
                .unwrap()
                .unwrap()
                .as_i64()
                .unwrap(),
            270
        );
        assert!(lookup_inherited(&doc, &page, b"Missing").unwrap().is_none());
    }

    fn page_tree_document(cyclic: bool) -> LoDocument {
        let mut doc = LoDocument::new();
        let mut catalog = LoDictionary::new();
        catalog.set("Type", LoObject::Name(b"Catalog".to_vec()));
        catalog.set("Pages", LoObject::Reference((2, 0)));
        let mut parent = LoDictionary::new();
        parent.set("Type", LoObject::Name(b"Pages".to_vec()));
        parent.set("Kids", LoObject::Array(vec![LoObject::Reference((3, 0))]));
        parent.set("Count", 1);
        if cyclic {
            parent.set("Parent", LoObject::Reference((2, 0)));
        }
        parent.set("Group", LoObject::Dictionary(LoDictionary::new()));
        let mut page = parent_dictionary((2, 0));
        page.set("Type", LoObject::Name(b"Page".to_vec()));
        page.set(
            "MediaBox",
            LoObject::Array(vec![0.into(), 0.into(), 200.into(), 300.into()]),
        );
        doc.objects.insert((1, 0), LoObject::Dictionary(catalog));
        doc.objects.insert((2, 0), LoObject::Dictionary(parent));
        doc.objects.insert((3, 0), LoObject::Dictionary(page));
        doc.trailer.set("Root", LoObject::Reference((1, 0)));
        doc
    }

    #[test]
    fn page_construction_propagates_cycles_and_keeps_group_direct_only() {
        let cyclic = page_tree_document(true);
        assert_eq!(
            build_pages(ptr::null_mut(), &cyclic).err(),
            Some(PageBuildError::CyclicInheritance)
        );
        let valid = page_tree_document(false);
        let pages = build_pages(ptr::null_mut(), &valid).unwrap();
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].media_box.x2, 200.0);
        assert_eq!(pages[0].crop_box.y2, 300.0);
        assert!(pages[0].group.is_none());
    }

    #[test]
    fn page_error_message_is_static_and_safe_after_document_deletion() {
        let mut doc = test_document(LoDocument::new());
        doc.ok = false;
        doc.page_error = Some(PageBuildError::CyclicInheritance);
        let doc_ptr = Box::into_raw(doc);
        let message = unsafe { xpdf_doc_error_message(doc_ptr) };
        assert_eq!(unsafe { xpdf_doc_is_ok(doc_ptr) }, 0);
        unsafe { xpdf_doc_delete(doc_ptr) };
        assert_eq!(
            unsafe { CStr::from_ptr(message) }.to_bytes(),
            b"xpdf: cyclic PDF page Parent chain"
        );
        assert_eq!(
            unsafe { CStr::from_ptr(xpdf_doc_error_message(ptr::null_mut())) }.to_bytes(),
            b"xpdf: reading PDF image failed"
        );
    }

    #[test]
    fn wide_dictionary_getters_clone_wrapper_source_once() {
        reset_clone_counts();
        let mut dict = LoDictionary::new();
        for index in 0..1024 {
            dict.set(format!("Key{index:04}"), LoObject::Integer(index));
        }
        let mut object = Object::new(LoObject::Dictionary(dict), ptr::null_mut());
        let object_ptr = &mut object as *mut Object;
        let mut out = Object::new(LoObject::Null, ptr::null_mut());
        let out_ptr = &mut out as *mut Object;

        assert_eq!(unsafe { xpdf_object_dict_get_length(object_ptr) }, 1024);
        let dict_ptr = unsafe { xpdf_object_get_dict(object_ptr) };
        assert!(!dict_ptr.is_null());
        let first_key = unsafe { xpdf_object_dict_get_key(object_ptr, 0) };
        for _ in 0..3 {
            for index in 0..1024 {
                let key = unsafe { xpdf_object_dict_get_key(object_ptr, index) };
                assert_eq!(
                    unsafe { CStr::from_ptr(key) }.to_bytes(),
                    format!("Key{index:04}").as_bytes()
                );
                assert_eq!(
                    unsafe { xpdf_object_dict_get_val_nf(object_ptr, index, out_ptr) },
                    out_ptr
                );
                assert_eq!(unsafe { xpdf_object_get_int(out_ptr) }, index);
            }
            assert_eq!(unsafe { xpdf_object_get_dict(object_ptr) }, dict_ptr);
            assert_eq!(
                unsafe { xpdf_object_dict_get_key(object_ptr, 0) },
                first_key
            );
            assert_eq!(unsafe { xpdf_object_dict_get_length(object_ptr) }, 1024);
        }
        assert_eq!(
            clone_counts(),
            CacheCloneCounts {
                object_dict: 1,
                ..CacheCloneCounts::default()
            }
        );
    }

    #[test]
    fn stream_cache_clones_payload_once_and_preserves_cursor() {
        reset_clone_counts();
        let payload_len = 1024 * 1024;
        let payload = (0..payload_len)
            .map(|index| b'a' + (index % 26) as u8)
            .collect::<Vec<_>>();
        let mut dict = LoDictionary::new();
        dict.set("Referenced", LoObject::Reference((17, 3)));
        let mut object = Object::new(
            LoObject::Stream(LoStream::new(dict, payload)),
            ptr::null_mut(),
        );
        let object_ptr = &mut object as *mut Object;
        let stream_ptr = unsafe { xpdf_object_get_stream(object_ptr) };
        assert!(!stream_ptr.is_null());
        assert_eq!(unsafe { xpdf_stream_get_char(stream_ptr) }, b'a' as c_int);
        let stream_dict_ptr = unsafe { xpdf_object_stream_get_dict(object_ptr) };
        let object_dict_ptr = unsafe { xpdf_object_get_dict(object_ptr) };
        assert!(!stream_dict_ptr.is_null());
        assert!(!object_dict_ptr.is_null());
        assert_ne!(stream_dict_ptr, object_dict_ptr);
        for _ in 0..32 {
            assert_eq!(unsafe { xpdf_object_get_stream(object_ptr) }, stream_ptr);
            assert_eq!(
                unsafe { xpdf_object_stream_get_undecoded_stream(object_ptr) },
                stream_ptr
            );
            assert_eq!(
                unsafe { xpdf_stream_get_undecoded_stream(stream_ptr) },
                stream_ptr
            );
            assert_eq!(
                unsafe { xpdf_object_stream_get_dict(object_ptr) },
                stream_dict_ptr
            );
            assert_eq!(unsafe { xpdf_stream_get_dict(stream_ptr) }, stream_dict_ptr);
            assert_eq!(unsafe { xpdf_object_get_dict(object_ptr) }, object_dict_ptr);
        }
        assert_eq!(unsafe { xpdf_stream_get_char(stream_ptr) }, b'b' as c_int);

        let mut len = 0;
        let tail = unsafe { xpdf_stream_get_remaining_data(stream_ptr, &mut len) };
        assert_eq!(len, payload_len - 2);
        assert_eq!(unsafe { *tail }, b'c');
        let cached_data = unsafe { stream_ptr.as_ref() }
            .unwrap()
            .stream
            .content
            .as_ptr();
        assert_eq!(tail, unsafe { cached_data.add(2) });
        assert_eq!(unsafe { xpdf_object_get_stream(object_ptr) }, stream_ptr);
        assert_eq!(unsafe { xpdf_stream_get_char(stream_ptr) }, libc::EOF);
        unsafe { xpdf_stream_reset(stream_ptr) };
        assert_eq!(unsafe { xpdf_stream_get_char(stream_ptr) }, b'a' as c_int);
        assert_eq!(unsafe { *tail }, b'c');
        assert_eq!(
            clone_counts(),
            CacheCloneCounts {
                object_dict: 1,
                object_stream: 1,
                object_stream_bytes: payload_len,
                stream_dict: 1,
                ..CacheCloneCounts::default()
            }
        );
    }

    #[test]
    fn page_dictionary_caches_keep_handles_and_keys_stable() {
        reset_clone_counts();
        let mut nested = LoDictionary::new();
        nested.set("Font", LoObject::Reference((17, 3)));
        let mut resources = LoDictionary::new();
        resources.set("Nested", LoObject::Dictionary(nested));
        let mut group = LoDictionary::new();
        group.set("S", LoObject::Name(b"Transparency".to_vec()));
        let mut page = test_page(Some(resources), Some(group));
        let page_ptr = &mut page as *mut Page;
        let resources_ptr = unsafe { xpdf_page_resource_dict(page_ptr) };
        let group_ptr = unsafe { xpdf_page_group(page_ptr) };
        assert!(!resources_ptr.is_null());
        assert!(!group_ptr.is_null());
        let resources_key = unsafe { xpdf_dict_get_key(resources_ptr, 0) };
        let group_key = unsafe { xpdf_dict_get_key(group_ptr, 0) };
        for _ in 0..32 {
            assert_eq!(unsafe { xpdf_page_resource_dict(page_ptr) }, resources_ptr);
            assert_eq!(unsafe { xpdf_page_group(page_ptr) }, group_ptr);
            assert_eq!(
                unsafe { xpdf_dict_get_key(resources_ptr, 0) },
                resources_key
            );
            assert_eq!(unsafe { xpdf_dict_get_key(group_ptr, 0) }, group_key);
        }
        assert_eq!(
            unsafe { CStr::from_ptr(resources_key) }.to_bytes(),
            b"Nested"
        );
        assert_eq!(unsafe { CStr::from_ptr(group_key) }.to_bytes(), b"S");
        let mut out = Object::new(LoObject::Null, ptr::null_mut());
        unsafe { xpdf_dict_get_val_nf(resources_ptr, 0, &mut out) };
        assert_eq!(
            out.value
                .as_dict()
                .unwrap()
                .get(b"Font")
                .unwrap()
                .as_reference()
                .unwrap(),
            (17, 3)
        );
        unsafe { xpdf_dict_get_val_nf(group_ptr, 0, &mut out) };
        assert_eq!(out.value.as_name().unwrap(), b"Transparency");
        assert_eq!(
            clone_counts(),
            CacheCloneCounts {
                page_group: 1,
                page_resources: 1,
                ..CacheCloneCounts::default()
            }
        );
    }

    #[test]
    fn cached_dictionary_getters_preserve_references_and_owned_outputs() {
        reset_clone_counts();
        let mut doc = LoDocument::new();
        doc.objects.insert((17, 3), LoObject::Integer(41));
        let mut doc = test_document(doc);
        let doc_ptr = doc.as_mut() as *mut PDFDoc;
        let mut nested = LoDictionary::new();
        nested.set("Reference", LoObject::Reference((17, 3)));
        let mut dict = LoDictionary::new();
        dict.set("Reference", LoObject::Reference((17, 3)));
        dict.set("Nested", LoObject::Dictionary(nested));
        let mut object = Object::new(LoObject::Dictionary(dict), doc_ptr);
        let object_ptr = &mut object as *mut Object;
        let dict_ptr = unsafe { xpdf_object_get_dict(object_ptr) };
        let mut out = Object::new(LoObject::Null, ptr::null_mut());
        let out_ptr = &mut out as *mut Object;
        for _ in 0..4 {
            unsafe { xpdf_object_dict_get_val_nf(object_ptr, 0, out_ptr) };
            let reference = unsafe { xpdf_object_get_ref(out_ptr) };
            assert_eq!((reference.num, reference.gen), (17, 3));
            assert_eq!(out.doc, doc_ptr);
            unsafe { xpdf_object_dict_lookup_nf(object_ptr, c"Reference".as_ptr(), out_ptr) };
            assert_eq!(out.value.as_reference().unwrap(), (17, 3));
            unsafe { xpdf_object_dict_get_val(object_ptr, 0, out_ptr) };
            assert_eq!(unsafe { xpdf_object_get_int(out_ptr) }, 41);
            unsafe { xpdf_object_dict_lookup(object_ptr, c"Reference".as_ptr(), out_ptr) };
            assert_eq!(unsafe { xpdf_object_get_int(out_ptr) }, 41);
            assert_eq!(unsafe { xpdf_object_get_dict(object_ptr) }, dict_ptr);
        }

        unsafe { xpdf_object_init_dict_from_dict(out_ptr, ptr::null_mut(), dict_ptr) };
        assert_eq!(out.doc, doc_ptr);
        out.value.as_dict_mut().unwrap().set("OwnedOutput", true);
        assert_eq!(unsafe { xpdf_object_dict_get_length(out_ptr) }, 3);
        assert_eq!(unsafe { xpdf_dict_get_length(dict_ptr) }, 2);
        unsafe { xpdf_object_free_contents(out_ptr) };
        assert_eq!(unsafe { xpdf_object_is_null(out_ptr) }, 1);
        assert_eq!(unsafe { xpdf_object_get_dict(object_ptr) }, dict_ptr);
        assert_eq!(unsafe { xpdf_dict_get_length(dict_ptr) }, 2);
        assert_eq!(
            clone_counts(),
            CacheCloneCounts {
                object_dict: 2,
                ..CacheCloneCounts::default()
            }
        );
    }

    #[test]
    fn object_reset_and_free_contents_invalidate_caches() {
        reset_clone_counts();
        let mut first_doc = test_document(LoDocument::new());
        let first_doc_ptr = first_doc.as_mut() as *mut PDFDoc;
        let mut second_doc = test_document(LoDocument::new());
        let second_doc_ptr = second_doc.as_mut() as *mut PDFDoc;
        let mut object = Object::new(
            LoObject::Stream(LoStream::new(LoDictionary::new(), b"ab".to_vec())),
            first_doc_ptr,
        );
        let object_ptr = &mut object as *mut Object;
        for (content, doc_ptr) in [(b"ab", first_doc_ptr), (b"cd", second_doc_ptr)] {
            if doc_ptr == second_doc_ptr {
                object.reset(
                    LoObject::Stream(LoStream::new(LoDictionary::new(), content.to_vec())),
                    doc_ptr,
                );
                assert!(object.dict_cache.is_none());
                assert!(object.stream_cache.is_none());
            }
            let dict_ptr = unsafe { xpdf_object_get_dict(object_ptr) };
            let stream_ptr = unsafe { xpdf_object_get_stream(object_ptr) };
            let stream_dict_ptr = unsafe { xpdf_object_stream_get_dict(object_ptr) };
            assert_eq!(unsafe { dict_ptr.as_ref() }.unwrap().doc, doc_ptr);
            assert_eq!(unsafe { stream_ptr.as_ref() }.unwrap().doc, doc_ptr);
            assert_eq!(unsafe { stream_dict_ptr.as_ref() }.unwrap().doc, doc_ptr);
            assert_eq!(
                unsafe { xpdf_stream_get_char(stream_ptr) },
                content[0] as c_int
            );
            assert_eq!(unsafe { xpdf_object_get_stream(object_ptr) }, stream_ptr);
            assert_eq!(
                unsafe { xpdf_stream_get_char(stream_ptr) },
                content[1] as c_int
            );
        }
        // Cached handles must not be inspected after their owning object is reset.
        unsafe { xpdf_object_free_contents(object_ptr) };
        assert!(object.dict_cache.is_none());
        assert!(object.stream_cache.is_none());
        assert!(object.string_cache.is_none());
        assert_eq!(object.name_cache.as_bytes(), b"");
        assert_eq!(object.doc, second_doc_ptr);
        assert!(unsafe { xpdf_object_get_dict(object_ptr) }.is_null());
        assert!(unsafe { xpdf_object_get_stream(object_ptr) }.is_null());

        let mut dict = LoDictionary::new();
        dict.set("New", 99);
        object.reset(LoObject::Dictionary(dict), ptr::null_mut());
        let dict_ptr = unsafe { xpdf_object_get_dict(object_ptr) };
        let mut out = Object::new(LoObject::Null, ptr::null_mut());
        unsafe { xpdf_dict_lookup(dict_ptr, c"New".as_ptr(), &mut out) };
        assert_eq!(unsafe { xpdf_object_get_int(&mut out) }, 99);
        assert_eq!(
            clone_counts(),
            CacheCloneCounts {
                object_dict: 3,
                object_stream: 2,
                object_stream_bytes: 4,
                stream_dict: 2,
                ..CacheCloneCounts::default()
            }
        );
    }

    #[test]
    fn absent_and_wrong_type_getters_do_not_clone() {
        reset_clone_counts();
        for value in [
            LoObject::Null,
            LoObject::Integer(1),
            LoObject::Boolean(true),
            LoObject::Name(b"Name".to_vec()),
            LoObject::Array(Vec::new()),
            LoObject::Reference((17, 3)),
        ] {
            let mut object = Object::new(value, ptr::null_mut());
            for _ in 0..3 {
                assert!(unsafe { xpdf_object_get_dict(&mut object) }.is_null());
                assert!(unsafe { xpdf_object_get_stream(&mut object) }.is_null());
            }
        }
        let mut page = test_page(None, None);
        for _ in 0..3 {
            assert!(unsafe { xpdf_page_group(&mut page) }.is_null());
            assert!(unsafe { xpdf_page_resource_dict(&mut page) }.is_null());
            assert!(unsafe { xpdf_page_group(ptr::null_mut()) }.is_null());
            assert!(unsafe { xpdf_page_resource_dict(ptr::null_mut()) }.is_null());
            assert!(unsafe { xpdf_object_get_dict(ptr::null_mut()) }.is_null());
            assert!(unsafe { xpdf_object_get_stream(ptr::null_mut()) }.is_null());
        }
        assert_eq!(clone_counts(), CacheCloneCounts::default());
    }

    #[test]
    fn cache_clone_counters_are_thread_local() {
        reset_clone_counts();
        let counts = std::thread::spawn(|| {
            reset_clone_counts();
            let mut object =
                Object::new(LoObject::Dictionary(LoDictionary::new()), ptr::null_mut());
            assert!(!unsafe { xpdf_object_get_dict(&mut object) }.is_null());
            clone_counts()
        })
        .join()
        .unwrap();
        assert_eq!(counts.object_dict, 1);
        assert_eq!(clone_counts(), CacheCloneCounts::default());
    }

    #[test]
    fn stream_remaining_data_returns_tail_and_advances_to_eof() {
        let lo_stream = LoStream::new(LoDictionary::new(), b"abcd".to_vec());
        let mut stream = Stream::new(lo_stream, ptr::null_mut());
        let stream_ptr = &mut stream as *mut Stream;

        assert_eq!(unsafe { xpdf_stream_get_char(stream_ptr) }, b'a' as c_int);

        let mut len = 0;
        let data = unsafe { xpdf_stream_get_remaining_data(stream_ptr, &mut len) };
        assert_eq!(len, 3);
        assert_eq!(unsafe { slice::from_raw_parts(data, len) }, b"bcd");
        assert_eq!(unsafe { xpdf_stream_get_char(stream_ptr) }, libc::EOF);

        unsafe { xpdf_stream_reset(stream_ptr) };
        let data = unsafe { xpdf_stream_get_remaining_data(stream_ptr, &mut len) };
        assert_eq!(len, 4);
        assert_eq!(unsafe { slice::from_raw_parts(data, len) }, b"abcd");
        assert_eq!(unsafe { xpdf_stream_get_char(stream_ptr) }, libc::EOF);
    }
}
