#include <aconf.h>

#include "Object.h"
#include "Stream.h"
#include "Array.h"
#include "Dict.h"
#include "XRef.h"
#include "Catalog.h"
#include "Link.h"
#include "Page.h"
#include "GfxFont.h"
#include "PDFDoc.h"
#include "GlobalParams.h"
#include "gmem.h"

extern "C" {

struct XPdfRef {
    int num;
    int gen;
};

struct XPdfRect {
    double x1;
    double y1;
    double x2;
    double y2;
};

void xpdf_global_params_init() {
    if (globalParams == nullptr) {
        globalParams = new GlobalParams();
    }
    globalParams->setErrQuiet(gFalse);
}

void xpdf_global_params_free() {
    delete globalParams;
    globalParams = nullptr;
}

Object *xpdf_object_new() {
    return new Object();
}

void xpdf_object_delete(Object *obj) {
    if (obj != nullptr) {
        obj->free();
        delete obj;
    }
}

void xpdf_object_free_contents(Object *obj) {
    obj->free();
}

int xpdf_object_type(Object *obj) {
    return static_cast<int>(obj->getType());
}

const char *xpdf_object_type_name(Object *obj) {
    return obj->getTypeName();
}

int xpdf_object_is_bool(Object *obj) { return obj->isBool(); }
int xpdf_object_is_int(Object *obj) { return obj->isInt(); }
int xpdf_object_is_real(Object *obj) { return obj->isReal(); }
int xpdf_object_is_num(Object *obj) { return obj->isNum(); }
int xpdf_object_is_string(Object *obj) { return obj->isString(); }
int xpdf_object_is_name(Object *obj) { return obj->isName(); }
int xpdf_object_is_null(Object *obj) { return obj->isNull(); }
int xpdf_object_is_array(Object *obj) { return obj->isArray(); }
int xpdf_object_is_dict(Object *obj) { return obj->isDict(); }
int xpdf_object_is_stream(Object *obj) { return obj->isStream(); }
int xpdf_object_is_ref(Object *obj) { return obj->isRef(); }

int xpdf_object_get_bool(Object *obj) { return obj->getBool(); }
int xpdf_object_get_int(Object *obj) { return obj->getInt(); }
double xpdf_object_get_real(Object *obj) { return obj->getReal(); }
double xpdf_object_get_num(Object *obj) { return obj->getNum(); }
GString *xpdf_object_get_string(Object *obj) { return obj->getString(); }
char *xpdf_object_get_name(Object *obj) { return obj->getName(); }
Dict *xpdf_object_get_dict(Object *obj) { return obj->getDict(); }
Stream *xpdf_object_get_stream(Object *obj) { return obj->getStream(); }
XPdfRef xpdf_object_get_ref(Object *obj) {
    Ref ref = obj->getRef();
    return XPdfRef{ref.num, ref.gen};
}

Object *xpdf_object_fetch(Object *obj, XRef *xref, Object *out) {
    return obj->fetch(xref, out);
}

int xpdf_object_array_get_length(Object *obj) {
    return obj->arrayGetLength();
}

Object *xpdf_object_array_get(Object *obj, int i, Object *out) {
    return obj->arrayGet(i, out);
}

Object *xpdf_object_array_get_nf(Object *obj, int i, Object *out) {
    return obj->arrayGetNF(i, out);
}

int xpdf_object_dict_get_length(Object *obj) {
    return obj->dictGetLength();
}

char *xpdf_object_dict_get_key(Object *obj, int i) {
    return obj->dictGetKey(i);
}

Object *xpdf_object_dict_get_val(Object *obj, int i, Object *out) {
    return obj->dictGetVal(i, out);
}

Object *xpdf_object_dict_get_val_nf(Object *obj, int i, Object *out) {
    return obj->dictGetValNF(i, out);
}

Object *xpdf_object_dict_lookup(Object *obj, const char *key, Object *out) {
    return obj->dictLookup(key, out);
}

Object *xpdf_object_dict_lookup_nf(Object *obj, const char *key, Object *out) {
    return obj->dictLookupNF(key, out);
}

Dict *xpdf_object_stream_get_dict(Object *obj) {
    return obj->streamGetDict();
}

Stream *xpdf_object_stream_get_undecoded_stream(Object *obj) {
    return obj->getStream()->getUndecodedStream();
}

int xpdf_dict_get_length(Dict *dict) {
    return dict->getLength();
}

char *xpdf_dict_get_key(Dict *dict, int i) {
    return dict->getKey(i);
}

Object *xpdf_dict_get_val(Dict *dict, int i, Object *out) {
    return dict->getVal(i, out);
}

Object *xpdf_dict_get_val_nf(Dict *dict, int i, Object *out) {
    return dict->getValNF(i, out);
}

Object *xpdf_dict_lookup(Dict *dict, const char *key, Object *out) {
    return dict->lookup(key, out);
}

Object *xpdf_dict_lookup_nf(Dict *dict, const char *key, Object *out) {
    return dict->lookupNF(key, out);
}

void xpdf_object_init_dict_from_dict(Object *obj, XRef *xref, Dict *dict) {
    obj->initDict(xref);
    for (int i = 0, l = dict->getLength(); i < l; i++) {
        Object obj1;
        obj->dictAdd(copyString(dict->getKey(i)), dict->getValNF(i, &obj1));
    }
}

const char *xpdf_gstring_cstr(GString *s) {
    return s->getCString();
}

int xpdf_gstring_len(GString *s) {
    return s->getLength();
}

int xpdf_gstring_char(GString *s, int i) {
    return s->getChar(i);
}

void xpdf_stream_reset(Stream *stream) {
    stream->reset();
}

int xpdf_stream_get_char(Stream *stream) {
    return stream->getChar();
}

Dict *xpdf_stream_get_dict(Stream *stream) {
    return stream->getDict();
}

Stream *xpdf_stream_get_undecoded_stream(Stream *stream) {
    return stream->getUndecodedStream();
}

PDFDoc *xpdf_doc_new(const char *file_name) {
    return new PDFDoc(new GString(file_name));
}

void xpdf_doc_delete(PDFDoc *doc) {
    delete doc;
}

int xpdf_doc_is_ok(PDFDoc *doc) {
    return doc->isOk();
}

int xpdf_doc_ok_to_print(PDFDoc *doc) {
    return doc->okToPrint();
}

float xpdf_doc_pdf_version(PDFDoc *doc) {
    return doc->getPDFVersion();
}

Catalog *xpdf_doc_catalog(PDFDoc *doc) {
    return doc->getCatalog();
}

XRef *xpdf_doc_xref(PDFDoc *doc) {
    return doc->getXRef();
}

LinkDest *xpdf_doc_find_dest(PDFDoc *doc, const char *name) {
    GString gname(name);
    return doc->findDest(&gname);
}

void xpdf_doc_get_info_nf(PDFDoc *doc, Object *out) {
    doc->getDocInfoNF(out);
}

void xpdf_link_dest_delete(LinkDest *dest) {
    delete dest;
}

int xpdf_link_dest_is_ok(LinkDest *dest) {
    return dest->isOk();
}

XPdfRef xpdf_link_dest_page_ref(LinkDest *dest) {
    Ref ref = dest->getPageRef();
    return XPdfRef{ref.num, ref.gen};
}

int xpdf_catalog_num_pages(Catalog *catalog) {
    return catalog->getNumPages();
}

Page *xpdf_catalog_get_page(Catalog *catalog, int page_num) {
    return catalog->getPage(page_num);
}

XPdfRef xpdf_catalog_get_page_ref(Catalog *catalog, int page_num) {
    Ref *ref = catalog->getPageRef(page_num);
    return XPdfRef{ref->num, ref->gen};
}

int xpdf_catalog_find_page(Catalog *catalog, int num, int gen) {
    return catalog->findPage(num, gen);
}

void xpdf_xref_fetch(XRef *xref, int num, int gen, Object *out) {
    xref->fetch(num, gen, out);
}

static XPdfRect rect_from_pdf(PDFRectangle *rect) {
    return XPdfRect{rect->x1, rect->y1, rect->x2, rect->y2};
}

XPdfRect xpdf_page_media_box(Page *page) { return rect_from_pdf(page->getMediaBox()); }
XPdfRect xpdf_page_crop_box(Page *page) { return rect_from_pdf(page->getCropBox()); }
XPdfRect xpdf_page_bleed_box(Page *page) { return rect_from_pdf(page->getBleedBox()); }
XPdfRect xpdf_page_trim_box(Page *page) { return rect_from_pdf(page->getTrimBox()); }
XPdfRect xpdf_page_art_box(Page *page) { return rect_from_pdf(page->getArtBox()); }

int xpdf_page_rotate(Page *page) {
    return page->getRotate();
}

Dict *xpdf_page_group(Page *page) {
    return page->getGroup();
}

Dict *xpdf_page_resource_dict(Page *page) {
    return page->getResourceDict();
}

void xpdf_page_get_contents(Page *page, Object *out) {
    page->getContents(out);
}

GfxFont *xpdf_gfx_font_make(XRef *xref, const char *tag, XPdfRef id, Dict *font_dict) {
    Ref ref{id.num, id.gen};
    return GfxFont::makeFont(xref, tag, ref, font_dict);
}

void xpdf_gfx_font_delete(GfxFont *font) {
    delete font;
}

int xpdf_gfx_font_is_cid(GfxFont *font) {
    return font->isCIDFont();
}

char *xpdf_gfx_8bit_font_char_name(GfxFont *font, int code) {
    return ((Gfx8BitFont *)font)->getCharName(code);
}

}
