//! Shared core TeX types used by initialization, execution and state storage.
//!
//! Record field order and union views are part of the existing format-file
//! contract. Their C representation preserves that layout independently of
//! the Rust calling convention used between engine modules.

#![warn(clippy::all)]

use crate::stdio::Stream;

pub type Integer = i32;
pub type LongInteger = i64;
pub type Boolean = i32;
pub type SChar = i8;
pub type Real = f64;
pub type GlueRatio = f64;
pub type Halfword = Integer;
pub type Quarterword = u8;
pub type Scaled = Integer;
pub type NonnegativeInteger = Integer;
pub type SmallNumber = u8;
pub type AsciiCode = u8;
pub type PackedAsciiCode = u8;
pub type EightBits = u8;
pub type NineBits = i16;
pub type PoolPointer = Integer;
pub type StrNumber = Integer;
pub type InternalFontNumber = Integer;
pub type FontIndex = Integer;
pub type TriePointer = Integer;
pub type TrieOpcode = u16;
pub type HyphPointer = u16;
pub type VfStackIndex = Integer;
pub type GlueOrder = u8;
pub type GroupCode = u8;
pub type SavePointer = Integer;
pub type FmEntryPtr = *mut Integer;
pub type CharUsedArray = [EightBits; 32];
pub type Text = *mut Stream;
pub type WordFile = *mut Stream;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HalfwordPair {
    pub LH: Halfword,
    pub RH: Halfword,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HalfwordBytes {
    pub B1: i16,
    pub B0: i16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union TwoHalves {
    pub v: HalfwordPair,
    pub u: HalfwordBytes,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct QuarterBytes {
    pub B3: Quarterword,
    pub B2: Quarterword,
    pub B1: Quarterword,
    pub B0: Quarterword,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FourQuarters {
    pub u: QuarterBytes,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryIntegerView {
    pub junk: Halfword,
    pub CINT: Integer,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryQuartersView {
    pub junk: Halfword,
    pub QQQQ: FourQuarters,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union MemoryWord {
    pub gr: GlueRatio,
    pub hh: TwoHalves,
    pub u: MemoryIntegerView,
    pub v: MemoryQuartersView,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FontIntegerView {
    pub CINT: Integer,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FontQuartersView {
    pub QQQQ: FourQuarters,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union FontMemoryWord {
    pub u: FontIntegerView,
    pub v: FontQuartersView,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ListStateRecord {
    pub modefield: i16,
    pub headfield: Halfword,
    pub tailfield: Halfword,
    pub eTeXauxfield: Halfword,
    pub pgfield: Integer,
    pub mlfield: Integer,
    pub auxfield: MemoryWord,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct InputStateRecord {
    pub statefield: Quarterword,
    pub indexfield: Quarterword,
    pub startfield: Halfword,
    pub locfield: Halfword,
    pub limitfield: Halfword,
    pub namefield: Halfword,
    pub synctextagfield: Integer,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjectEntry {
    pub int0: Integer,
    pub int1: Integer,
    pub int2: LongInteger,
    pub int3: Integer,
    pub int4: Integer,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DestinationNameEntry {
    pub objname: StrNumber,
    pub objnum: Integer,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VfStackRecord {
    pub stackh: Scaled,
    pub stackv: Scaled,
    pub stackw: Scaled,
    pub stackx: Scaled,
    pub stacky: Scaled,
    pub stackz: Scaled,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PdfLinkStackRecord {
    pub nestinglevel: Integer,
    pub linknode: Halfword,
    pub reflinknode: Halfword,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::{pdftex0, pdftexextra, pdftexini};
    use proptest::prelude::*;
    use std::mem::{align_of, offset_of, size_of};

    fn assert_layout<T>(size: usize, alignment: usize) {
        assert_eq!(size_of::<T>(), size);
        assert_eq!(align_of::<T>(), alignment);
    }

    #[test]
    fn core_scalar_widths_are_format_compatible() {
        assert_layout::<Integer>(4, 4);
        assert_layout::<LongInteger>(8, 8);
        assert_layout::<Boolean>(4, 4);
        assert_layout::<SChar>(1, 1);
        assert_layout::<Real>(8, 8);
        assert_layout::<GlueRatio>(8, 8);
        assert_layout::<Halfword>(4, 4);
        assert_layout::<Quarterword>(1, 1);
        assert_layout::<NineBits>(2, 2);
        assert_layout::<TrieOpcode>(2, 2);
        assert_layout::<HyphPointer>(2, 2);
        assert_layout::<CharUsedArray>(32, 1);
        assert_eq!(size_of::<Integer>(), size_of::<libc::c_int>());
        assert_eq!(size_of::<LongInteger>(), size_of::<libc::off_t>());
        assert_eq!(size_of::<Boolean>(), size_of::<libc::c_int>());
    }

    #[test]
    fn core_records_keep_every_field_offset() {
        assert_layout::<HalfwordPair>(8, 4);
        assert_eq!(offset_of!(HalfwordPair, LH), 0);
        assert_eq!(offset_of!(HalfwordPair, RH), 4);
        assert_layout::<HalfwordBytes>(4, 2);
        assert_eq!(offset_of!(HalfwordBytes, B1), 0);
        assert_eq!(offset_of!(HalfwordBytes, B0), 2);
        assert_layout::<TwoHalves>(8, 4);
        assert_eq!(offset_of!(TwoHalves, v), 0);
        assert_eq!(offset_of!(TwoHalves, u), 0);
        assert_layout::<QuarterBytes>(4, 1);
        assert_eq!(offset_of!(QuarterBytes, B3), 0);
        assert_eq!(offset_of!(QuarterBytes, B2), 1);
        assert_eq!(offset_of!(QuarterBytes, B1), 2);
        assert_eq!(offset_of!(QuarterBytes, B0), 3);
        assert_layout::<FourQuarters>(4, 1);
        assert_eq!(offset_of!(FourQuarters, u), 0);
        assert_layout::<MemoryIntegerView>(8, 4);
        assert_eq!(offset_of!(MemoryIntegerView, junk), 0);
        assert_eq!(offset_of!(MemoryIntegerView, CINT), 4);
        assert_layout::<MemoryQuartersView>(8, 4);
        assert_eq!(offset_of!(MemoryQuartersView, junk), 0);
        assert_eq!(offset_of!(MemoryQuartersView, QQQQ), 4);
        assert_layout::<MemoryWord>(8, 8);
        assert_eq!(offset_of!(MemoryWord, gr), 0);
        assert_eq!(offset_of!(MemoryWord, hh), 0);
        assert_eq!(offset_of!(MemoryWord, u), 0);
        assert_eq!(offset_of!(MemoryWord, v), 0);
        assert_layout::<FontIntegerView>(4, 4);
        assert_eq!(offset_of!(FontIntegerView, CINT), 0);
        assert_layout::<FontQuartersView>(4, 1);
        assert_eq!(offset_of!(FontQuartersView, QQQQ), 0);
        assert_layout::<FontMemoryWord>(4, 4);
        assert_eq!(offset_of!(FontMemoryWord, u), 0);
        assert_eq!(offset_of!(FontMemoryWord, v), 0);
        assert_layout::<ListStateRecord>(32, 8);
        assert_eq!(offset_of!(ListStateRecord, modefield), 0);
        assert_eq!(offset_of!(ListStateRecord, headfield), 4);
        assert_eq!(offset_of!(ListStateRecord, tailfield), 8);
        assert_eq!(offset_of!(ListStateRecord, eTeXauxfield), 12);
        assert_eq!(offset_of!(ListStateRecord, pgfield), 16);
        assert_eq!(offset_of!(ListStateRecord, mlfield), 20);
        assert_eq!(offset_of!(ListStateRecord, auxfield), 24);
        assert_layout::<InputStateRecord>(24, 4);
        assert_eq!(offset_of!(InputStateRecord, statefield), 0);
        assert_eq!(offset_of!(InputStateRecord, indexfield), 1);
        assert_eq!(offset_of!(InputStateRecord, startfield), 4);
        assert_eq!(offset_of!(InputStateRecord, locfield), 8);
        assert_eq!(offset_of!(InputStateRecord, limitfield), 12);
        assert_eq!(offset_of!(InputStateRecord, namefield), 16);
        assert_eq!(offset_of!(InputStateRecord, synctextagfield), 20);
        assert_layout::<ObjectEntry>(24, 8);
        assert_eq!(offset_of!(ObjectEntry, int0), 0);
        assert_eq!(offset_of!(ObjectEntry, int1), 4);
        assert_eq!(offset_of!(ObjectEntry, int2), 8);
        assert_eq!(offset_of!(ObjectEntry, int3), 16);
        assert_eq!(offset_of!(ObjectEntry, int4), 20);
        assert_layout::<DestinationNameEntry>(8, 4);
        assert_eq!(offset_of!(DestinationNameEntry, objname), 0);
        assert_eq!(offset_of!(DestinationNameEntry, objnum), 4);
        assert_layout::<VfStackRecord>(24, 4);
        assert_eq!(offset_of!(VfStackRecord, stackh), 0);
        assert_eq!(offset_of!(VfStackRecord, stackv), 4);
        assert_eq!(offset_of!(VfStackRecord, stackw), 8);
        assert_eq!(offset_of!(VfStackRecord, stackx), 12);
        assert_eq!(offset_of!(VfStackRecord, stacky), 16);
        assert_eq!(offset_of!(VfStackRecord, stackz), 20);
        assert_layout::<PdfLinkStackRecord>(12, 4);
        assert_eq!(offset_of!(PdfLinkStackRecord, nestinglevel), 0);
        assert_eq!(offset_of!(PdfLinkStackRecord, linknode), 4);
        assert_eq!(offset_of!(PdfLinkStackRecord, reflinknode), 8);
    }

    #[test]
    fn generated_core_modules_share_nominal_types() {
        fn same_type<T>(_: Option<T>, _: Option<T>, _: Option<T>) {}
        same_type(
            None::<pdftex0::twohalves>,
            None::<pdftexini::twohalves>,
            None::<pdftexextra::twohalves>,
        );
        same_type(
            None::<pdftex0::fourquarters>,
            None::<pdftexini::fourquarters>,
            None::<pdftexextra::fourquarters>,
        );
        same_type(
            None::<pdftex0::memoryword>,
            None::<pdftexini::memoryword>,
            None::<pdftexextra::memoryword>,
        );
        same_type(
            None::<pdftex0::fmemoryword>,
            None::<pdftexini::fmemoryword>,
            None::<pdftexextra::fmemoryword>,
        );
        same_type(
            None::<pdftex0::liststaterecord>,
            None::<pdftexini::liststaterecord>,
            None::<pdftexextra::liststaterecord>,
        );
        same_type(
            None::<pdftex0::instaterecord>,
            None::<pdftexini::instaterecord>,
            None::<pdftexextra::instaterecord>,
        );
        same_type(
            None::<pdftex0::objentry>,
            None::<pdftexini::objentry>,
            None::<pdftexextra::objentry>,
        );
        same_type(
            None::<pdftex0::destnameentry>,
            None::<pdftexini::destnameentry>,
            None::<pdftexextra::destnameentry>,
        );
        same_type(
            None::<pdftex0::vfstackrecord>,
            None::<pdftexextra::vfstackrecord>,
            None::<VfStackRecord>,
        );
        same_type(
            None::<pdftex0::pdflinkstackrecord>,
            None::<pdftexextra::pdflinkstackrecord>,
            None::<PdfLinkStackRecord>,
        );
    }

    proptest! {
        #[test]
        fn core_memory_union_views_preserve_native_bytes(bits in any::<u64>()) {
            // The floating member initializes all eight bytes, so every tested
            // integer and quarter-byte view reads fully initialized storage.
            let word = MemoryWord { gr: f64::from_bits(bits) };
            let bytes = bits.to_ne_bytes();
            unsafe {
                prop_assert_eq!(word.gr.to_bits(), bits);
                prop_assert_eq!(word.hh.v.LH, i32::from_ne_bytes(bytes[..4].try_into().unwrap()));
                prop_assert_eq!(word.hh.v.RH, i32::from_ne_bytes(bytes[4..].try_into().unwrap()));
                prop_assert_eq!(word.hh.u.B1, i16::from_ne_bytes(bytes[..2].try_into().unwrap()));
                prop_assert_eq!(word.hh.u.B0, i16::from_ne_bytes(bytes[2..4].try_into().unwrap()));
                prop_assert_eq!(word.u.junk, word.hh.v.LH);
                prop_assert_eq!(word.u.CINT, word.hh.v.RH);
                prop_assert_eq!(word.v.QQQQ.u.B3, bytes[4]);
                prop_assert_eq!(word.v.QQQQ.u.B2, bytes[5]);
                prop_assert_eq!(word.v.QQQQ.u.B1, bytes[6]);
                prop_assert_eq!(word.v.QQQQ.u.B0, bytes[7]);
            }
        }

        #[test]
        fn core_font_union_views_preserve_native_bytes(integer in any::<i32>()) {
            let word = FontMemoryWord { u: FontIntegerView { CINT: integer } };
            let bytes = integer.to_ne_bytes();
            unsafe {
                prop_assert_eq!(word.u.CINT, integer);
                prop_assert_eq!(word.v.QQQQ.u.B3, bytes[0]);
                prop_assert_eq!(word.v.QQQQ.u.B2, bytes[1]);
                prop_assert_eq!(word.v.QQQQ.u.B1, bytes[2]);
                prop_assert_eq!(word.v.QQQQ.u.B0, bytes[3]);
            }
        }
    }
}
