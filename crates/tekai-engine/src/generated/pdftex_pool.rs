pub use crate::generated::pdftex0::makestring;

use crate::byte_string::strlen;

pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type integer = ::core::ffi::c_int;
pub type strnumber = integer;
pub type packedASCIIcode = ::core::ffi::c_uchar;
pub type poolpointer = integer;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub unsafe fn loadpoolstrings(mut spare_size: integer) -> ::core::ffi::c_int {
    let __tekai_state_generated_pdftex_pool: *mut crate::generated::pdftex_pool::State =
        unsafe { crate::generated::pdftex_pool::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut g: strnumber = 0 as strnumber;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let fresh0 = j;
        j = j + 1;
        s = (*__tekai_state_generated_pdftex_pool).poolfilearr[fresh0 as usize];
        if s.is_null() {
            break;
        }
        let mut l: ::core::ffi::c_int = strlen(s) as ::core::ffi::c_int;
        i += l;
        if i >= spare_size {
            return 0 as ::core::ffi::c_int;
        }
        loop {
            let fresh1 = l;
            l = l - 1;
            if !(fresh1 > 0 as ::core::ffi::c_int) {
                break;
            }
            let fresh2 = s;
            s = s.offset(1);
            let fresh3 = (*__tekai_state_generated_pdftexextra).poolptr;
            (*__tekai_state_generated_pdftexextra).poolptr =
                (*__tekai_state_generated_pdftexextra).poolptr + 1;
            *(*__tekai_state_generated_pdftexextra)
                .strpool
                .offset(fresh3 as isize) = *fresh2 as packedASCIIcode;
        }
        g = makestring();
    }
    return g as ::core::ffi::c_int;
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;

pub(crate) struct State {
    poolfilearr: [*const ::core::ffi::c_char; 1850],
}

#[inline(always)]
fn __tekai_require_copy<T: Copy>() {}

pub(crate) struct StateInitialization {
    destination: *mut State,
    initialized: [bool; 1],
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
            if self.initialized[0] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).poolfilearr));
            }
        }
    }
}
impl StateInitialization {
    pub(crate) unsafe fn begin(destination: *mut State) -> Self {
        Self {
            destination,
            initialized: [false; 1],
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
            assert!((1850) == 1850);
        }
        __tekai_require_copy::<*const ::core::ffi::c_char>();
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[0])
            .write(b".6\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1])
            .write(b"29\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[2])
            .write(b"buffer size\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[3])
            .write(b"pool size\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[4])
            .write(b"number of strings\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[5])
            .write(b"???\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[6])
            .write(b"m2d5c2l5x2v5i\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[7])
            .write(b"End of file on the terminal!\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[8])
            .write(b"! \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[9]).write(
            b"(That makes 100 errors; please try again.)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[10])
            .write(b"? \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[11]).write(
            b"Type <return> to proceed, S to scroll future error messages,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[12]).write(
            b"R to run without stopping, Q to run quietly,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[13])
            .write(b"I to insert something, \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[14])
            .write(b"E to edit your file,\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[15]).write(
            b"1 or ... or 9 to ignore the next 1 to 9 tokens of input,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[16])
            .write(b"H for help, X to quit.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[17])
            .write(b"OK, entering \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[18])
            .write(b"batchmode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[19])
            .write(b"nonstopmode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[20])
            .write(b"scrollmode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[21])
            .write(b"...\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[22])
            .write(b"insert>\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[23]).write(
            b"I have just deleted some text, as you asked.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[24]).write(
            b"You can now delete more, or insert, or whatever.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[25]).write(
            b"Sorry, I don't know how to help in this situation.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[26]).write(
            b"Maybe you should try asking a human?\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[27]).write(
            b"Sorry, I already gave what help I could...\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[28]).write(
            b"An error might have occurred before I noticed any problems.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[29]).write(
            b"``If all else fails, read the instructions.''\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[30])
            .write(b" (\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[31])
            .write(b"Emergency stop\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[32])
            .write(b"TeX capacity exceeded, sorry [\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[33]).write(
            b"If you really absolutely need more capacity,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[34]).write(
            b"you can ask a wizard to enlarge me.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[35])
            .write(b"This can't happen (\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[36]).write(
            b"I'm broken. Please show this to someone who can fix can fix\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[37]).write(
            b"I can't go on meeting you like this\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[38]).write(
            b"One of your faux pas seems to have wounded me deeply...\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[39]).write(
            b"in fact, I'm barely conscious. Please fix it and try again.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[40])
            .write(b"Interruption\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[41])
            .write(b"You rang?\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[42]).write(
            b"Try to insert an instruction for me (e.g., `I\\showlists'),\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[43]).write(
            b"unless you just want to quit by typing `X'.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[44])
            .write(b"Logarithm of \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[45])
            .write(b" has been replaced by 0\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[46]).write(
            b"Since I don't take logs of non-positive numbers,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[47]).write(
            b"I'm zeroing this one. Proceed, with fingers crossed.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[48])
            .write(b"main memory size\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[49])
            .write(b"AVAIL list clobbered at \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[50])
            .write(b"Double-AVAIL list clobbered at \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[51])
            .write(b"Doubly free location at \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[52])
            .write(b"Bad flag at \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[53])
            .write(b"New busy locs:\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[54])
            .write(b"LINK(\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[55])
            .write(b"INFO(\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[56])
            .write(b"pt\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[57])
            .write(b"[]\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[58])
            .write(b"CLOBBERED.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[59])
            .write(b"foul\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[60])
            .write(b"fil\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[61])
            .write(b" plus \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[62])
            .write(b" minus \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[63])
            .write(b" []\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[64])
            .write(b"Bad link, display aborted.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[65])
            .write(b"etc.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[66])
            .write(b"kern\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[67])
            .write(b" (left margin)\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[68])
            .write(b" (right margin)\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[69])
            .write(b"Unknown node type!\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[70])
            .write(b"unset\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[71])
            .write(b"box(\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[72])
            .write(b")x\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[73])
            .write(b", shifted \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[74])
            .write(b" columns)\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[75])
            .write(b", stretch \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[76])
            .write(b", shrink \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[77])
            .write(b", glue set \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[78])
            .write(b"- \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[79])
            .write(b"< -\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[80])
            .write(b"rule(\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[81])
            .write(b"insert\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[82])
            .write(b", natural size \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[83])
            .write(b"; split(\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[84])
            .write(b"); float cost \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[85])
            .write(b"glue\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[86])
            .write(b"nonscript\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[87])
            .write(b"mskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[88])
            .write(b"mu\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[89])
            .write(b"\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[90])
            .write(b"leaders \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[91])
            .write(b" (for accent)\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[92]).write(
            b" (for \\pdfprependkern/\\pdfappendkern)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[93])
            .write(b"mkern\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[94])
            .write(b"end\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[95])
            .write(b"begin\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[96])
            .write(b"math\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[97])
            .write(b"on\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[98])
            .write(b"off\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[99])
            .write(b", surrounded \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[100])
            .write(b" (ligature \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[101])
            .write(b"penalty \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[102])
            .write(b"discretionary\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[103])
            .write(b" replacing \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[104])
            .write(b"mark\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[105])
            .write(b"vadjust\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[106])
            .write(b" pre \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[107])
            .write(b"flushing\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[108])
            .write(b"copying\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[109])
            .write(b"vertical mode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[110])
            .write(b"horizontal mode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[111])
            .write(b"display math mode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[112])
            .write(b"no mode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[113])
            .write(b"internal vertical mode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[114])
            .write(b"restricted horizontal mode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[115])
            .write(b"math mode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[116])
            .write(b"' in vertical mode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[117])
            .write(b"' in horizontal mode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[118])
            .write(b"' in display math mode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[119])
            .write(b"' in no mode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[120])
            .write(b"' in internal vertical mode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[121])
            .write(b"' in restricted horizontal mode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[122])
            .write(b"' in math mode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[123])
            .write(b"semantic nest size\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[124])
            .write(b"### \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[125])
            .write(b" entered at line \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[126])
            .write(b" (language\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[127])
            .write(b":hyphenmin\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[128])
            .write(b" (\\output routine)\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[129])
            .write(b"### recent contributions:\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[130])
            .write(b"prevdepth \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[131])
            .write(b"ignored\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[132])
            .write(b", prevgraf \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[133])
            .write(b" lines\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[134])
            .write(b" line\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[135])
            .write(b"spacefactor \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[136])
            .write(b", current language \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[137])
            .write(b"this will begin denominator of:\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[138])
            .write(b"lineskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[139])
            .write(b"baselineskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[140])
            .write(b"parskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[141])
            .write(b"abovedisplayskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[142])
            .write(b"belowdisplayskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[143])
            .write(b"abovedisplayshortskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[144])
            .write(b"belowdisplayshortskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[145])
            .write(b"leftskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[146])
            .write(b"rightskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[147])
            .write(b"topskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[148])
            .write(b"splittopskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[149])
            .write(b"tabskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[150])
            .write(b"spaceskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[151])
            .write(b"xspaceskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[152])
            .write(b"parfillskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[153])
            .write(b"thinmuskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[154])
            .write(b"medmuskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[155])
            .write(b"thickmuskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[156])
            .write(b"[unknown glue parameter!]\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[157])
            .write(b"skip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[158])
            .write(b"muskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[159])
            .write(b"output\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[160])
            .write(b"everypar\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[161])
            .write(b"everymath\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[162])
            .write(b"everydisplay\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[163])
            .write(b"everyhbox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[164])
            .write(b"everyvbox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[165])
            .write(b"everyjob\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[166])
            .write(b"everycr\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[167])
            .write(b"errhelp\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[168])
            .write(b"pdfpagesattr\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[169])
            .write(b"pdfpageattr\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[170])
            .write(b"pdfpageresources\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[171])
            .write(b"pdfpkmode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[172])
            .write(b"toks\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[173])
            .write(b"ETC.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[174])
            .write(b"box\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[175])
            .write(b"void\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[176])
            .write(b"current font\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[177])
            .write(b"textfont\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[178])
            .write(b"scriptfont\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[179])
            .write(b"scriptscriptfont\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[180])
            .write(b"catcode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[181])
            .write(b"lccode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[182])
            .write(b"uccode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[183])
            .write(b"sfcode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[184])
            .write(b"mathcode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[185])
            .write(b"pretolerance\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[186])
            .write(b"tolerance\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[187])
            .write(b"linepenalty\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[188])
            .write(b"hyphenpenalty\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[189])
            .write(b"exhyphenpenalty\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[190])
            .write(b"clubpenalty\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[191])
            .write(b"widowpenalty\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[192])
            .write(b"displaywidowpenalty\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[193])
            .write(b"brokenpenalty\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[194])
            .write(b"binoppenalty\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[195])
            .write(b"relpenalty\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[196])
            .write(b"predisplaypenalty\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[197])
            .write(b"postdisplaypenalty\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[198])
            .write(b"interlinepenalty\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[199])
            .write(b"doublehyphendemerits\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[200])
            .write(b"finalhyphendemerits\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[201])
            .write(b"adjdemerits\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[202])
            .write(b"mag\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[203])
            .write(b"delimiterfactor\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[204])
            .write(b"looseness\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[205])
            .write(b"time\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[206])
            .write(b"day\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[207])
            .write(b"month\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[208])
            .write(b"year\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[209])
            .write(b"showboxbreadth\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[210])
            .write(b"showboxdepth\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[211])
            .write(b"hbadness\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[212])
            .write(b"vbadness\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[213])
            .write(b"pausing\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[214])
            .write(b"tracingonline\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[215])
            .write(b"tracingmacros\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[216])
            .write(b"tracingstats\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[217])
            .write(b"tracingparagraphs\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[218])
            .write(b"tracingpages\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[219])
            .write(b"tracingoutput\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[220])
            .write(b"tracinglostchars\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[221])
            .write(b"tracingcommands\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[222])
            .write(b"tracingrestores\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[223])
            .write(b"uchyph\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[224])
            .write(b"outputpenalty\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[225])
            .write(b"maxdeadcycles\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[226])
            .write(b"hangafter\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[227])
            .write(b"floatingpenalty\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[228])
            .write(b"globaldefs\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[229])
            .write(b"fam\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[230])
            .write(b"escapechar\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[231])
            .write(b"defaulthyphenchar\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[232])
            .write(b"defaultskewchar\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[233])
            .write(b"endlinechar\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[234])
            .write(b"newlinechar\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[235])
            .write(b"language\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[236])
            .write(b"lefthyphenmin\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[237])
            .write(b"righthyphenmin\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[238])
            .write(b"holdinginserts\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[239])
            .write(b"errorcontextlines\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[240])
            .write(b"charsubdefmin\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[241])
            .write(b"charsubdefmax\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[242])
            .write(b"tracingcharsubdef\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[243])
            .write(b"mubytein\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[244])
            .write(b"mubyteout\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[245])
            .write(b"mubytelog\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[246])
            .write(b"specialout\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[247])
            .write(b"tracingstacklevels\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[248])
            .write(b"partokencontext\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[249])
            .write(b"showstream\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[250])
            .write(b"pdfoutput\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[251])
            .write(b"pdfcompresslevel\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[252])
            .write(b"pdfobjcompresslevel\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[253])
            .write(b"pdfdecimaldigits\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[254])
            .write(b"pdfmovechars\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[255])
            .write(b"pdfimageresolution\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[256])
            .write(b"pdfpkresolution\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[257])
            .write(b"pdfuniqueresname\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[258])
            .write(b"pdfoptionalwaysusepdfpagebox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[259])
            .write(b"pdfoptionpdfinclusionerrorlevel\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[260])
            .write(b"pdfmajorversion\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[261])
            .write(b"pdfminorversion\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[262])
            .write(b"pdfforcepagebox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[263])
            .write(b"pdfpagebox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[264])
            .write(b"pdfinclusionerrorlevel\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[265])
            .write(b"pdfgamma\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[266])
            .write(b"pdfimagegamma\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[267])
            .write(b"pdfimagehicolor\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[268])
            .write(b"pdfimageapplygamma\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[269])
            .write(b"pdfadjustspacing\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[270])
            .write(b"pdfprotrudechars\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[271])
            .write(b"pdftracingfonts\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[272])
            .write(b"pdfadjustinterwordglue\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[273])
            .write(b"pdfprependkern\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[274])
            .write(b"pdfappendkern\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[275])
            .write(b"pdfgentounicode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[276])
            .write(b"pdfdraftmode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[277])
            .write(b"pdfinclusioncopyfonts\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[278])
            .write(b"pdfsuppresswarningdupdest\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[279])
            .write(b"pdfsuppresswarningdupmap\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[280])
            .write(b"pdfsuppresswarningpagegroup\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[281])
            .write(b"pdfinfoomitdate\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[282])
            .write(b"pdfsuppressptexinfo\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[283])
            .write(b"pdfomitcharset\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[284])
            .write(b"pdfomitinfodict\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[285])
            .write(b"pdfomitprocset\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[286])
            .write(b"pdfptexuseunderscore\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[287])
            .write(b"[unknown integer parameter!]\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[288])
            .write(b"partokenname\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[289])
            .write(b"pdfoptionpdfminorversion\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[290])
            .write(b"count\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[291])
            .write(b"delcode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[292])
            .write(b"parindent\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[293])
            .write(b"mathsurround\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[294])
            .write(b"lineskiplimit\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[295])
            .write(b"hsize\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[296])
            .write(b"vsize\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[297])
            .write(b"maxdepth\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[298])
            .write(b"splitmaxdepth\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[299])
            .write(b"boxmaxdepth\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[300])
            .write(b"hfuzz\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[301])
            .write(b"vfuzz\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[302])
            .write(b"delimitershortfall\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[303])
            .write(b"nulldelimiterspace\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[304])
            .write(b"scriptspace\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[305])
            .write(b"predisplaysize\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[306])
            .write(b"displaywidth\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[307])
            .write(b"displayindent\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[308])
            .write(b"overfullrule\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[309])
            .write(b"hangindent\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[310])
            .write(b"hoffset\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[311])
            .write(b"voffset\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[312])
            .write(b"emergencystretch\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[313])
            .write(b"pdfhorigin\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[314])
            .write(b"pdfvorigin\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[315])
            .write(b"pdfpagewidth\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[316])
            .write(b"pdfpageheight\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[317])
            .write(b"pdflinkmargin\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[318])
            .write(b"pdfdestmargin\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[319])
            .write(b"pdfthreadmargin\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[320])
            .write(b"pdffirstlineheight\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[321])
            .write(b"pdflastlinedepth\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[322])
            .write(b"pdfeachlineheight\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[323])
            .write(b"pdfeachlinedepth\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[324])
            .write(b"pdfignoreddimen\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[325])
            .write(b"pdfpxdimen\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[326])
            .write(b"[unknown dimen parameter!]\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[327])
            .write(b"dimen\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[328])
            .write(b"EQUIV(\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[329])
            .write(b"notexpanded:\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[330])
            .write(b"pdfprimitive\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[331])
            .write(b"hash size\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[332])
            .write(b"primitive size\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[333])
            .write(b"csname\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[334])
            .write(b"endcsname\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[335])
            .write(b"IMPOSSIBLE.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[336])
            .write(b"NONEXISTENT.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[337])
            .write(b"accent\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[338])
            .write(b"advance\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[339])
            .write(b"afterassignment\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[340])
            .write(b"aftergroup\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[341])
            .write(b"begingroup\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[342])
            .write(b"char\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[343])
            .write(b"delimiter\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[344])
            .write(b"divide\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[345])
            .write(b"endmubyte\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[346])
            .write(b"endgroup\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[347])
            .write(b"expandafter\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[348])
            .write(b"font\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[349])
            .write(b"letterspacefont\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[350])
            .write(b"pdfcopyfont\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[351])
            .write(b"fontdimen\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[352])
            .write(b"halign\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[353])
            .write(b"hrule\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[354])
            .write(b"ignorespaces\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[355])
            .write(b"mathaccent\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[356])
            .write(b"mathchar\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[357])
            .write(b"mathchoice\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[358])
            .write(b"multiply\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[359])
            .write(b"noalign\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[360])
            .write(b"noboundary\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[361])
            .write(b"noexpand\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[362])
            .write(b"omit\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[363])
            .write(b"parshape\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[364])
            .write(b"penalty\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[365])
            .write(b"prevgraf\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[366])
            .write(b"radical\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[367])
            .write(b"read\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[368])
            .write(b"relax\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[369])
            .write(b"setbox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[370])
            .write(b"the\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[371])
            .write(b"valign\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[372])
            .write(b"vcenter\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[373])
            .write(b"vrule\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[374])
            .write(b"save size\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[375])
            .write(b"grouping levels\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[376])
            .write(b"reassigning\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[377])
            .write(b"changing\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[378])
            .write(b"into\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[379])
            .write(b"globally changing\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[380])
            .write(b"curlevel\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[381])
            .write(b"retaining\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[382])
            .write(b"restoring\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[383])
            .write(b"SAVE(\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[384])
            .write(b"Incompatible magnification (\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[385])
            .write(b");\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[386]).write(
            b" the previous value will be retained\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[387]).write(
            b"I can handle only one magnification ratio per job. So I've\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[388]).write(
            b"reverted to the magnification you used earlier on this run.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[389]).write(
            b"Illegal magnification has been changed to 1000\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[390]).write(
            b"The magnification ratio must be between 1 and 32768.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[391])
            .write(b"BAD.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[392])
            .write(b"->\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[393])
            .write(b"begin-group character \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[394])
            .write(b"end-group character \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[395])
            .write(b"math shift character \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[396])
            .write(b"macro parameter character \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[397])
            .write(b"superscript character \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[398])
            .write(b"subscript character \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[399])
            .write(b"end of alignment template\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[400])
            .write(b"blank space \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[401])
            .write(b"the letter \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[402])
            .write(b"the character \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[403])
            .write(b"[unknown command code!]\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[404])
            .write(b": \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[405])
            .write(b"(level \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[406])
            .write(b"Runaway definition\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[407])
            .write(b"Runaway argument\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[408])
            .write(b"Runaway preamble\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[409])
            .write(b"Runaway text\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[410])
            .write(b"<*>\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[411])
            .write(b"<insert> \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[412])
            .write(b"<read \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[413])
            .write(b"l.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[414])
            .write(b"<argument> \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[415])
            .write(b"<template> \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[416])
            .write(b"<recently read> \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[417])
            .write(b"<to be read again> \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[418])
            .write(b"<inserted text> \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[419])
            .write(b"<output> \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[420])
            .write(b"<everypar> \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[421])
            .write(b"<everymath> \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[422])
            .write(b"<everydisplay> \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[423])
            .write(b"<everyhbox> \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[424])
            .write(b"<everyvbox> \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[425])
            .write(b"<everyjob> \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[426])
            .write(b"<everycr> \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[427])
            .write(b"<mark> \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[428])
            .write(b"<everyeof> \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[429])
            .write(b"<write> \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[430])
            .write(b"input stack size\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[431])
            .write(b"write\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[432])
            .write(b"Unbalanced output routine\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[433]).write(
            b"(interwoven alignment preambles are not allowed)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[434])
            .write(b"text input levels\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[435])
            .write(b"par\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[436])
            .write(b"Incomplete \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[437]).write(
            b"; all text was ignored after line \0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[438]).write(
            b"A forbidden control sequence occurred in skipped text.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[439]).write(
            b"This kind of error happens when you say `\\if...' and forget\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[440]).write(
            b"the matching `\\fi'. I've inserted a `\\fi'; this might work.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[441]).write(
            b"The file ended while I was skipping conditional text.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[442])
            .write(b"File ended\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[443]).write(
            b"Forbidden control sequence found\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[444])
            .write(b" of \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[445]).write(
            b"I suspect you have forgotten a `}', causing me\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[446]).write(
            b"to read past where you wanted me to stop.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[447]).write(
            b"I'll try to recover; but if the error is serious,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[448]).write(
            b"you'd better type `E' or `X' now and fix your file.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[449])
            .write(b" while scanning definition\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[450])
            .write(b" while scanning use\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[451])
            .write(b" while scanning preamble\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[452])
            .write(b" while scanning text\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[453]).write(
            b"Text line contains an invalid character\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[454]).write(
            b"A funny symbol that I can't read has just been input.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[455]).write(
            b"Continue, and I'll forget that it ever happened.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[456])
            .write(b"Unbalanced write command\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[457]).write(
            b"(Please type a command or say `\\end')\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[458]).write(
            b"*** (job aborted, no legal \\end found)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[459])
            .write(b"=>\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[460])
            .write(b"expansion depth\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[461])
            .write(b"Undefined control sequence\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[462]).write(
            b"The control sequence at the end of the top line\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[463]).write(
            b"of your error message was never \\def'ed. If you have\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[464]).write(
            b"misspelled it (e.g., `\\hobx'), type `I' and the correct\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[465]).write(
            b"spelling (e.g., `I\\hbox'). Otherwise just continue,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[466]).write(
            b"and I'll forget about whatever was undefined.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[467])
            .write(b"Missing \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[468])
            .write(b" inserted\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[469]).write(
            b"The control sequence marked <to be read again> should\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[470]).write(
            b"not appear between \\csname and \\endcsname.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[471])
            .write(b"input\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[472])
            .write(b"endinput\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[473])
            .write(b"topmark\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[474])
            .write(b"firstmark\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[475])
            .write(b"botmark\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[476])
            .write(b"splitfirstmark\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[477])
            .write(b"splitbotmark\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[478])
            .write(b"parameter stack size\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[479])
            .write(b"Argument of \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[480])
            .write(b" has an extra }\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[481]).write(
            b"I've run across a `}' that doesn't seem to match anything.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[482]).write(
            b"For example, `\\def\\a#1{...}' and `\\a}' would produce\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[483]).write(
            b"this error. If you simply proceed now, the `\\par' that\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[484]).write(
            b"I've just inserted will cause me to report a runaway\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[485]).write(
            b"argument that might be the root of the problem. But if\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[486]).write(
            b"your `}' was spurious, just type `2' and it will go away.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[487])
            .write(b"Paragraph ended before \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[488])
            .write(b" was complete\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[489]).write(
            b"I suspect you've forgotten a `}', causing me to apply this\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[490]).write(
            b"control sequence to too much text. How can we recover?\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[491]).write(
            b"My plan is to forget the whole thing and hope for the best.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[492])
            .write(b"Use of \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[493])
            .write(b" doesn't match its definition\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[494]).write(
            b"If you say, e.g., `\\def\\a1{...}', then you must always\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[495]).write(
            b"put `1' after `\\a', since control sequence names are\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[496]).write(
            b"made up of letters only. The macro here has not been\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[497]).write(
            b"followed by the required stuff, so I'm ignoring it.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[498])
            .write(b"<-\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[499])
            .write(b"Missing { inserted\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[500]).write(
            b"A left brace was mandatory here, so I've put one in.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[501]).write(
            b"You might want to delete and/or insert some corrections\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[502]).write(
            b"so that I will find a matching right brace soon.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[503]).write(
            b"(If you're confused by all this, try typing `I}' now.)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[504])
            .write(b"Incompatible glue units\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[505]).write(
            b"I'm going to assume that 1mu=1pt when they're mixed.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[506])
            .write(b"Missing number, treated as zero\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[507]).write(
            b"A number should have been here; I inserted `0'.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[508]).write(
            b"(If you can't figure out why I needed to see a number,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[509]).write(
            b"look up `weird error' in the index to The TeXbook.)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[510])
            .write(b"spacefactor\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[511])
            .write(b"prevdepth\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[512])
            .write(b"deadcycles\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[513])
            .write(b"insertpenalties\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[514])
            .write(b"wd\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[515])
            .write(b"ht\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[516])
            .write(b"dp\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[517])
            .write(b"lastpenalty\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[518])
            .write(b"lastkern\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[519])
            .write(b"lastskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[520])
            .write(b"inputlineno\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[521])
            .write(b"badness\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[522])
            .write(b"pdftexversion\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[523])
            .write(b"pdflastobj\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[524])
            .write(b"pdflastxform\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[525])
            .write(b"pdflastximage\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[526])
            .write(b"pdflastximagepages\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[527])
            .write(b"pdflastannot\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[528])
            .write(b"pdflastxpos\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[529])
            .write(b"pdflastypos\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[530])
            .write(b"pdfretval\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[531])
            .write(b"pdflastximagecolordepth\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[532])
            .write(b"pdfelapsedtime\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[533])
            .write(b"pdfshellescape\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[534])
            .write(b"pdfrandomseed\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[535])
            .write(b"pdflastlink\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[536])
            .write(b"Improper \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[537]).write(
            b"You can refer to \\spacefactor only in horizontal mode;\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[538]).write(
            b"you can refer to \\prevdepth only in vertical mode; and\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[539]).write(
            b"neither of these is meaningful inside \\write. So\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[540]).write(
            b"I'm forgetting what you said and using zero instead.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[541])
            .write(b"You can't use `\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[542])
            .write(b"' after \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[543])
            .write(b"Bad register code\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[544]).write(
            b"A register number must be between 0 and 255.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[545])
            .write(b"I changed this one to zero.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[546])
            .write(b"Bad character code\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[547]).write(
            b"A character number must be between 0 and 255.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[548])
            .write(b"Bad number\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[549]).write(
            b"Since I expected to read a number between 0 and 15,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[550])
            .write(b"Bad mathchar\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[551]).write(
            b"A mathchar number must be between 0 and 32767.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[552])
            .write(b"Bad delimiter code\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[553]).write(
            b"A numeric delimiter code must be between 0 and 2^{27}-1.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[554])
            .write(b"Improper alphabetic constant\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[555]).write(
            b"A one-character control sequence belongs after a ` mark.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[556]).write(
            b"So I'm essentially inserting \\0 here.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[557])
            .write(b"Number too big\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[558]).write(
            b"I can only go up to 2147483647='17777777777=\"7FFFFFFF,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[559]).write(
            b"so I'm using that number instead of yours.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[560])
            .write(b"true\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[561])
            .write(b"Illegal unit of measure (\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[562])
            .write(b"replaced by filll)\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[563]).write(
            b"I dddon't go any higher than filll.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[564])
            .write(b"em\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[565])
            .write(b"ex\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[566])
            .write(b"px\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[567])
            .write(b"mu inserted)\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[568]).write(
            b"The unit of measurement in math glue must be mu.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[569]).write(
            b"To recover gracefully from this error, it's best to\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[570]).write(
            b"delete the erroneous units; e.g., type `2' to delete\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[571]).write(
            b"two letters. (See Chapter 27 of The TeXbook.)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[572])
            .write(b"in\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[573])
            .write(b"pc\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[574])
            .write(b"cm\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[575])
            .write(b"mm\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[576])
            .write(b"bp\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[577])
            .write(b"dd\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[578])
            .write(b"cc\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[579])
            .write(b"nd\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[580])
            .write(b"nc\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[581])
            .write(b"sp\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[582])
            .write(b"pt inserted)\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[583]).write(
            b"Dimensions can be in units of em, ex, in, pt, pc,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[584]).write(
            b"cm, mm, dd, cc, nd, nc, bp, or sp; but yours is a new one!\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[585]).write(
            b"I'll assume that you meant to say pt, for printer's points.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[586])
            .write(b"Dimension too large\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[587]).write(
            b"I can't work with sizes bigger than about 19 feet.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[588]).write(
            b"Continue and I'll use the largest value I can.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[589])
            .write(b"plus\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[590])
            .write(b"minus\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[591])
            .write(b"width\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[592])
            .write(b"height\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[593])
            .write(b"depth\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[594])
            .write(b"number\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[595])
            .write(b"romannumeral\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[596])
            .write(b"string\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[597])
            .write(b"meaning\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[598])
            .write(b"fontname\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[599])
            .write(b"expanded\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[600])
            .write(b"pdftexrevision\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[601])
            .write(b"pdftexbanner\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[602])
            .write(b"pdffontname\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[603])
            .write(b"pdffontobjnum\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[604])
            .write(b"pdffontsize\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[605])
            .write(b"pdfpageref\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[606])
            .write(b"leftmarginkern\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[607])
            .write(b"rightmarginkern\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[608])
            .write(b"pdfxformname\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[609])
            .write(b"pdfescapestring\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[610])
            .write(b"pdfescapename\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[611])
            .write(b"pdfescapehex\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[612])
            .write(b"pdfunescapehex\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[613])
            .write(b"pdfcreationdate\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[614])
            .write(b"pdffilemoddate\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[615])
            .write(b"pdffilesize\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[616])
            .write(b"pdfmdfivesum\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[617])
            .write(b"pdffiledump\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[618])
            .write(b"pdfmatch\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[619])
            .write(b"pdflastmatch\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[620])
            .write(b"pdfstrcmp\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[621])
            .write(b"pdfcolorstackinit\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[622])
            .write(b"pdfuniformdeviate\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[623])
            .write(b"pdfnormaldeviate\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[624])
            .write(b"jobname\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[625])
            .write(b"pdfinsertht\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[626])
            .write(b"pdfximagebbox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[627])
            .write(b"eTeXrevision\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[628])
            .write(b"invalid font identifier\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[629])
            .write(b"pageref\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[630])
            .write(b"invalid page number\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[631])
            .write(b"marginkern\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[632])
            .write(b"a non-empty hbox expected\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[633])
            .write(b"file\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[634])
            .write(b"offset\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[635])
            .write(b"Bad file offset\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[636]).write(
            b"A file offset must be between 0 and 2^{31}-1,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[637])
            .write(b"length\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[638])
            .write(b"Bad dump length\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[639]).write(
            b"A dump length must be between 0 and 2^{31}-1,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[640])
            .write(b"icase\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[641])
            .write(b"subcount\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[642])
            .write(b"Bad match number\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[643]).write(
            b"Since I expected zero or a positive number,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[644])
            .write(b"page\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[645])
            .write(b"direct\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[646])
            .write(b"Too many color stacks\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[647]).write(
            b"The number of color stacks is limited to 32768.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[648]).write(
            b"I'll use the default color stack 0 here.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[649])
            .write(b"invalid parameter\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[650])
            .write(b" at \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[651]).write(
            b"Where was the left brace? You said something like `\\def\\a}',\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[652]).write(
            b"which I'm going to interpret as `\\def\\a{}'.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[653]).write(
            b"You already have nine parameters\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[654]).write(
            b"I'm going to ignore the # sign you just used,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[655]).write(
            b"as well as the token that followed it.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[656]).write(
            b"Parameters must be numbered consecutively\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[657]).write(
            b"I've inserted the digit you should have used after the #.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[658]).write(
            b"Type `1' to delete what you did use.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[659]).write(
            b"Illegal parameter number in definition of \0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[660]).write(
            b"You meant to type ## instead of #, right?\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[661]).write(
            b"Or maybe a } was forgotten somewhere earlier, and things\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[662]).write(
            b"are all screwed up? I'm going to assume that you meant ##.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[663]).write(
            b"*** (cannot \\read from terminal in nonstop modes)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[664])
            .write(b"File ended within \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[665]).write(
            b"This \\read has unbalanced braces.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[666])
            .write(b"if\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[667])
            .write(b"ifcat\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[668])
            .write(b"ifnum\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[669])
            .write(b"ifdim\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[670])
            .write(b"ifodd\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[671])
            .write(b"ifvmode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[672])
            .write(b"ifhmode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[673])
            .write(b"ifmmode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[674])
            .write(b"ifinner\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[675])
            .write(b"ifvoid\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[676])
            .write(b"ifhbox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[677])
            .write(b"ifvbox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[678])
            .write(b"ifx\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[679])
            .write(b"ifeof\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[680])
            .write(b"iftrue\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[681])
            .write(b"iffalse\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[682])
            .write(b"ifcase\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[683])
            .write(b"ifpdfprimitive\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[684])
            .write(b"unless\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[685])
            .write(b"fi\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[686])
            .write(b"or\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[687])
            .write(b"else\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[688])
            .write(b"Extra \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[689]).write(
            b"I'm ignoring this; it doesn't match any \\if.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[690])
            .write(b"{true}\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[691])
            .write(b"{false}\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[692])
            .write(b"Missing = inserted for \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[693]).write(
            b"I was expecting to see `<', `=', or `>'. Didn't.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[694])
            .write(b"{case \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[695])
            .write(b".fmt\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[696])
            .write(b"input file name\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[697])
            .write(b"I can't find file `\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[698])
            .write(b"I can't write on file `\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[699])
            .write(b"'.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[700])
            .write(b".tex\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[701])
            .write(b"; default file extension is `\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[702])
            .write(b"Please type another \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[703]).write(
            b"*** (job aborted, file error in nonstop mode)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[704])
            .write(b".dvi\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[705])
            .write(b"file name for output\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[706])
            .write(b"texput\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[707])
            .write(b".fls\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[708])
            .write(b".log\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[709])
            .write(b"**\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[710])
            .write(b"transcript file name\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[711])
            .write(b"  \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[712])
            .write(b"INPUT \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[713])
            .write(b"nullfont\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[714])
            .write(b"Font \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[715])
            .write(b" scaled \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[716]).write(
            b" not loadable: Bad metric (TFM) file\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[717]).write(
            b" not loadable: Metric (TFM) file name too long\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[718]).write(
            b" not loadable: Metric (TFM) file not found\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[719]).write(
            b"I wasn't able to read the size data for this font,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[720]).write(
            b"so I will ignore the font specification.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[721]).write(
            b"[Wizards can fix TFM files using TFtoPL/PLtoTF.]\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[722]).write(
            b"You might try inserting a different font spec;\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[723]).write(
            b"e.g., type `I\\font<same font id>=<substitute font name>'.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[724]).write(
            b" not loaded: Not enough room left\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[725]).write(
            b"I'm afraid I won't be able to make use of this font,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[726]).write(
            b"because my memory for character-size data is too small.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[727]).write(
            b"If you're really stuck, ask a wizard to enlarge me.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[728]).write(
            b"Or maybe try `I\\font<same font id>=<name of loaded font>'.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[729])
            .write(b" scaled to 2048pt or higher\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[730]).write(
            b"I will ignore the scaling factor.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[731])
            .write(b"size is too large\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[732])
            .write(b"store_scaled_f\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[733])
            .write(b"vf scaling\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[734])
            .write(b"Missing font identifier\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[735]).write(
            b"I was looking for a control sequence whose\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[736]).write(
            b"current meaning has been defined by \\font.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[737])
            .write(b" has only \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[738])
            .write(b" fontdimen parameters\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[739]).write(
            b"To increase the number of font parameters, you must\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[740]).write(
            b"use \\fontdimen immediately after the \\font is loaded.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[741])
            .write(b"font memory\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[742])
            .write(b"Missing character: There is no \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[743])
            .write(b" in font \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[744])
            .write(b"dvi length exceeds \"7FFFFFFF\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[745])
            .write(b" TeX output \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[746])
            .write(b"vlistout\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[747])
            .write(b"Completed box being shipped out\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[748])
            .write(b"Memory usage before: \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[749])
            .write(b" after: \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[750])
            .write(b"; still untouched: \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[751])
            .write(b"Huge page cannot be shipped out\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[752]).write(
            b"The page just created is more than 18 feet tall or\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[753]).write(
            b"more than 18 feet wide, so I suspect something went wrong.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[754]).write(
            b"The following box has been deleted:\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[755])
            .write(b"No pages of output.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[756])
            .write(b"Output written on \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[757])
            .write(b" pages\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[758])
            .write(b" page\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[759])
            .write(b", \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[760])
            .write(b" bytes).\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[761])
            .write(b"PDF memory size (pdf_mem_size)\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[762])
            .write(b"PDF output buffer\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[763]).write(
            b"pdfTeX error (invalid pdfmajorversion)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[764]).write(
            b"The pdfmajorversion must be 1 or greater.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[765])
            .write(b"I changed this to 1.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[766]).write(
            b"pdfTeX error (invalid pdfminorversion)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[767]).write(
            b"The pdfminorversion must be between 0 and 9.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[768])
            .write(b"I changed this to 4.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[769])
            .write(b"Object streams\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[770]).write(
            b"\\pdfobjcompresslevel > 0 requires PDF-1.5 or greater. Object streams disabled now.\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[771])
            .write(b"%PDF-\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[772])
            .write(b"setup\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[773]).write(
            b"PDF version cannot be changed after data is written to the PDF file\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[774])
            .write(b".pdf\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[775])
            .write(b"file size\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[776]).write(
            b"File size exceeds architectural limits (pdf_gone wraps around)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[777])
            .write(b"/Length           \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[778])
            .write(b"/Filter /FlateDecode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[779])
            .write(b">>\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[780])
            .write(b"stream\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[781])
            .write(b"endstream\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[782])
            .write(b"pdfTeX error\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[783])
            .write(b"pdfTeX warning\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[784])
            .write(b"PDF object stream buffer\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[785])
            .write(b"tokens\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[786]).write(
            b"tokens_to_string() called while selector = new_string\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[787])
            .write(b"arithmetic\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[788])
            .write(b"divided by zero\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[789])
            .write(b"number too big\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[790])
            .write(b"1 0 0 1 \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[791])
            .write(b" cm\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[792])
            .write(b")]TJ\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[793])
            .write(b" 0 0 1 \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[794])
            .write(b" Tm\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[795])
            .write(b" Td\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[796]).write(
            b"Primitive \\pdfmovechars is obsolete.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[797])
            .write(b"font expansion\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[798]).write(
            b"auto expansion is only possible with scalable fonts\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[799])
            .write(b"/F\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[800])
            .write(b" Tf\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[801])
            .write(b"BT\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[802])
            .write(b"( )Tj\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[803])
            .write(b" [\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[804])
            .write(b"ET\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[805])
            .write(b"[]0 d 0 J \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[806])
            .write(b" w 0 0 m \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[807])
            .write(b" 0 l S\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[808])
            .write(b" w 0 0 m 0 \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[809])
            .write(b" l S\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[810])
            .write(b"0 0 \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[811])
            .write(b" re f\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[812])
            .write(b"/Rect [\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[813])
            .write(b"PDF:\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[814])
            .write(b"pdf:\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[815])
            .write(b"SRC:\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[816])
            .write(b"src:\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[817])
            .write(b"Non-PDF special ignored!\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[818])
            .write(b"<special> \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[819])
            .write(b"direct:\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[820])
            .write(b"page:\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[821])
            .write(b"literal1\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[822]).write(
            b"number of destination names (dest_names_size)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[823])
            .write(b"indirect objects table size\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[824])
            .write(b" 0 obj\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[825])
            .write(b"% \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[826])
            .write(b"endobj\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[827])
            .write(b"<<\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[828])
            .write(b"/Type /ObjStm\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[829])
            .write(b"/N \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[830])
            .write(b"/First \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[831])
            .write(b" 0 R\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[832])
            .write(b"()\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[833])
            .write(b"///...\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[834])
            .write(b"adjust_interword_glue\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[835])
            .write(b"g is not a glue\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[836]).write(
            b"maximum internal font number (font_max)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[837]).write(
            b"number of words of font memory (font_mem_size)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[838])
            .write(b"uninitialized pdf_font_elink\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[839]).write(
            b"\\pdffontexpand cannot be used this way (the base font has been expanded)\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[840])
            .write(b"invalid step\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[841])
            .write(b"invalid limit(s)\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[842])
            .write(b"autoexpand\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[843]).write(
            b"this font has been expanded by another font so it cannot be used now\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[844]).write(
            b"font has been expanded with different expansion step\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[845]).write(
            b"font has been expanded with different stretch limit\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[846]).write(
            b"font has been expanded with different shrink limit\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[847]).write(
            b"font has been expanded with different auto expansion value\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[848]).write(
            b"font should be expanded before its first use\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[849])
            .write(b"nolig\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[850])
            .write(b"\\letterspacefont\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[851]).write(
            b"font has zero em size (\\fontdimen6)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[852])
            .write(b"ls\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[853])
            .write(b"FONT\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[854])
            .write(b"\\pdfcopyfont\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[855])
            .write(b"cannot copy an expanded font\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[856])
            .write(b"cannot copy a letterspaced font\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[857])
            .write(b".vf\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[858])
            .write(b"vf\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[859])
            .write(b"unexpected EOF or error\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[860])
            .write(b" in local font \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[861])
            .write(b" in virtual font \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[862])
            .write(b".vf ignored.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[863])
            .write(b"checksum mismatch\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[864])
            .write(b"design size mismatch\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[865])
            .write(b"POST command expected\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[866])
            .write(b"PRE command expected\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[867])
            .write(b"wrong id byte\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[868])
            .write(b"checksum mismatch in font \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[869])
            .write(b".vf ignored\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[870])
            .write(b"design size mismatch in font \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[871])
            .write(b"invalid character code\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[872])
            .write(b"negative packet length\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[873])
            .write(b"packet length too long\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[874]).write(
            b"character width mismatch in font \0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[875]).write(
            b"more PUSHs than POPs in character packet\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[876]).write(
            b"invalid packet length or DVI command in packet\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[877])
            .write(b"too many local fonts\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[878])
            .write(b"undefined local font\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[879])
            .write(b"string of negative length\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[880])
            .write(b"virtual font stack size\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[881]).write(
            b"more POPs than PUSHs in character\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[882])
            .write(b"improver DVI command\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[883]).write(
            b"command cannot be used with virtual font\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[884]).write(
            b"max level recursion of virtual fonts\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[885])
            .write(b"local font not found\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[886])
            .write(b"invalid DVI command\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[887])
            .write(b"Color stack \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[888])
            .write(b" is not initialized for use!\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[889])
            .write(b"pdfcolorstack\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[890])
            .write(b"\\pdfsetmatrix\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[891])
            .write(b"Unrecognized format.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[892])
            .write(b"pdfvlistout\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[893]).write(
            b"\\pdfoutput can only be changed before anything is written to the output\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[894]).write(
            b"\\pdfdraftmode can only be changed before anything is written to the output\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[895])
            .write(b"/Type /XObject\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[896])
            .write(b"/Subtype /Form\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[897])
            .write(b"/BBox [\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[898])
            .write(b"/FormType 1\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[899])
            .write(b"/Matrix [1 0 0 1 0 0]\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[900])
            .write(b"Resources\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[901])
            .write(b" 0 0 \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[902])
            .write(b" 0 0 cm\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[903])
            .write(b"/Font << \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[904])
            .write(b" 0 R \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[905])
            .write(b"/XObject << \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[906])
            .write(b"/Fm\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[907])
            .write(b"/Im\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[908])
            .write(b"/ProcSet [ /PDF\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[909])
            .write(b" /Text\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[910])
            .write(b" /ImageB\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[911])
            .write(b" /ImageC\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[912])
            .write(b" /ImageI\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[913])
            .write(b" ]\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[914])
            .write(b"/Type /Page\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[915])
            .write(b"Contents\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[916])
            .write(b"/MediaBox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[917])
            .write(b"/MediaBox [0 0 \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[918])
            .write(b"/Group \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[919])
            .write(b"Parent\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[920])
            .write(b"/Annots [ \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[921])
            .write(b"/B [ \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[922])
            .write(b" not found.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[923])
            .write(b"ext5\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[924])
            .write(b"cannot open file for embedding\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[925])
            .write(b"flush_whatsit_node\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[926])
            .write(b"link(p) is not null\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[927])
            .write(b"/Type /Annot\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[928])
            .write(b"/Subtype /Link\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[929])
            .write(b"/A \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[930]).write(
            b"destination has been already written (this shouldn't happen)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[931])
            .write(b"/D \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[932])
            .write(b"/XYZ \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[933])
            .write(b"null\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[934])
            .write(b"/Fit\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[935])
            .write(b"/FitH \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[936])
            .write(b"/FitV \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[937])
            .write(b"/FitB\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[938])
            .write(b"/FitBH \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[939])
            .write(b"/FitBV \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[940])
            .write(b"/FitR \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[941])
            .write(b"unknown dest type\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[942])
            .write(b"/Type /Outlines\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[943])
            .write(b"First\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[944])
            .write(b"Last\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[945])
            .write(b"Count\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[946])
            .write(b"Title\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[947])
            .write(b"Prev\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[948])
            .write(b"Next\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[949]).write(
            b"\\pdfdraftmode enabled, not changing output pdf\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[950])
            .write(b"dest\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[951])
            .write(b"name{\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[952])
            .write(b"num\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[953]).write(
            b" has been referenced but does not exist, replaced by a fixed one\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[954])
            .write(b" 0 R /Fit]\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[955])
            .write(b"structure dest\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[956]).write(
            b" has been referenced but does not exist\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[957])
            .write(b"Page \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[958]).write(
            b" has been referenced but does not exist!\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[959])
            .write(b"\\pdffontattr\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[960])
            .write(b"fonts \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[961])
            .write(b" and \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[962]).write(
            b" have conflicting attributes; I will ignore the attributes assigned to \0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[963])
            .write(b"/Type /Pages\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[964])
            .write(b"/Kids [\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[965])
            .write(b"Dests\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[966])
            .write(b"/Names [\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[967])
            .write(b"/Limits [\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[968])
            .write(b"/Type /Catalog\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[969])
            .write(b"Pages\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[970])
            .write(b"Threads\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[971])
            .write(b"Outlines\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[972])
            .write(b"Names\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[973])
            .write(b"OpenAction\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[974])
            .write(b"/Creator\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[975])
            .write(b"/Producer\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[976])
            .write(b"/CreationDate\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[977])
            .write(b"/ModDate\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[978])
            .write(b"/Trapped\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[979])
            .write(b"Creator\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[980])
            .write(b"TeX\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[981])
            .write(b"/Trapped /False\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[982])
            .write(b"PTEX_Fullbanner\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[983])
            .write(b"PTEX.Fullbanner\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[984])
            .write(b"/Producer (pdfTeX-\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[985])
            .write(b"xref\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[986])
            .write(b"0 \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[987])
            .write(b" 65535 f \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[988])
            .write(b" 00000 f \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[989])
            .write(b" 00000 n \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[990])
            .write(b"/Type /XRef\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[991])
            .write(b"/Index [0 \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[992])
            .write(b"Size\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[993])
            .write(b"/W [1 \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[994])
            .write(b" 1]\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[995])
            .write(b"Root\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[996])
            .write(b"Info\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[997])
            .write(b"trailer\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[998])
            .write(b"<< \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[999])
            .write(b" >>\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1000])
            .write(b"startxref\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1001])
            .write(b"%%EOF\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1002])
            .write(b"to\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1003])
            .write(b"spread\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1004]).write(
            b"using fonts with different step of expansion in one paragraph is not allowed\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1005]).write(
            b"using fonts with different limit of expansion in one paragraph is not allowed\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1006])
            .write(b"invalid node type\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1007])
            .write(b"margin kerning\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1008]).write(
            b"invalid pointer to marginal char node\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1009])
            .write(b"pre vadjust\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1010])
            .write(b"Underfull\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1011])
            .write(b"Loose\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1012])
            .write(b" \\hbox (badness \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1013]).write(
            b") has occurred while \\output is active\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1014])
            .write(b") in paragraph at lines \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1015])
            .write(b") in alignment at lines \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1016])
            .write(b"--\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1017])
            .write(b") detected at line \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1018])
            .write(b"Overfull \\hbox (\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1019])
            .write(b"pt too wide\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1020])
            .write(b"Tight \\hbox (badness \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1021])
            .write(b"vpack\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1022])
            .write(b" \\vbox (badness \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1023])
            .write(b"Overfull \\vbox (\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1024])
            .write(b"pt too high\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1025])
            .write(b"Tight \\vbox (badness \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1026])
            .write(b"{}\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1027])
            .write(b"displaystyle\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1028])
            .write(b"textstyle\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1029])
            .write(b"scriptstyle\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1030])
            .write(b"scriptscriptstyle\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1031])
            .write(b"Unknown style!\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1032])
            .write(b"mathord\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1033])
            .write(b"mathop\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1034])
            .write(b"mathbin\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1035])
            .write(b"mathrel\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1036])
            .write(b"mathopen\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1037])
            .write(b"mathclose\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1038])
            .write(b"mathpunct\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1039])
            .write(b"mathinner\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1040])
            .write(b"overline\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1041])
            .write(b"underline\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1042])
            .write(b"left\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1043])
            .write(b"right\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1044])
            .write(b"middle\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1045])
            .write(b"limits\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1046])
            .write(b"nolimits\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1047])
            .write(b"fraction, thickness \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1048])
            .write(b"= default\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1049])
            .write(b", left-delimiter \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1050])
            .write(b", right-delimiter \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1051])
            .write(b" is undefined (character \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1052]).write(
            b"Somewhere in the math formula just ended, you used the\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1053]).write(
            b"stated character from an undefined font family. For example,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1054]).write(
            b"plain TeX doesn't allow \\it or \\sl in subscripts. Proceed,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1055]).write(
            b"and I'll try to forget that I needed that character.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1056])
            .write(b"mlist1\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1057])
            .write(b"mlist2\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1058])
            .write(b"mlist3\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1059]).write(
            b"0234000122*4000133**3**344*0400400*000000234000111*1111112341011\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1060])
            .write(b"mlist4\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1061])
            .write(b" inside $$'s\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1062]).write(
            b"Displays can use special alignments (like \\eqalignno)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1063]).write(
            b"only if nothing but the alignment itself is between $$'s.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1064]).write(
            b"So I've deleted the formulas that preceded this alignment.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1065])
            .write(b"span\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1066])
            .write(b"cr\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1067])
            .write(b"crcr\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1068])
            .write(b"endtemplate\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1069])
            .write(b"alignment tab character \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1070]).write(
            b"Missing # inserted in alignment preamble\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1071]).write(
            b"There should be exactly one # between &'s, when an\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1072]).write(
            b"\\halign or \\valign is being set up. In this case you had\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1073]).write(
            b"none, so I've put one in; maybe that will work.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1074])
            .write(b"Only one # is allowed per tab\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1075]).write(
            b"more than one, so I'm ignoring all but the first.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1076])
            .write(b"endv\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1077]).write(
            b"Extra alignment tab has been changed to \0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1078]).write(
            b"You have given more \\span or & marks than there were\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1079]).write(
            b"in the preamble to the \\halign or \\valign now in progress.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1080]).write(
            b"So I'll assume that you meant to type \\cr instead.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1081])
            .write(b"256 spans\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1082])
            .write(b"align1\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1083])
            .write(b"align0\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1084]).write(
            b"Infinite glue shrinkage found in a paragraph\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1085]).write(
            b"The paragraph just ended includes some glue that has\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1086]).write(
            b"infinite shrinkability, e.g., `\\hskip 0pt minus 1fil'.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1087]).write(
            b"Such glue doesn't belong there---it allows a paragraph\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1088]).write(
            b"of any length to fit on one line. But it's safe to proceed,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1089]).write(
            b"since the offensive shrinkability has been made finite.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1090])
            .write(b"push_node\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1091])
            .write(b"stack overflow\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1092])
            .write(b"pop_node\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1093]).write(
            b"stack underflow (internal error)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1094])
            .write(b"disc1\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1095])
            .write(b"disc2\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1096])
            .write(b"@@\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1097])
            .write(b": line \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1098])
            .write(b" t=\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1099])
            .write(b" -> @@\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1100])
            .write(b" via @@\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1101])
            .write(b" b=\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1102])
            .write(b" p=\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1103])
            .write(b" d=\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1104])
            .write(b"@firstpass\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1105])
            .write(b"@secondpass\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1106])
            .write(b"@emergencypass\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1107])
            .write(b"paragraph\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1108])
            .write(b"disc3\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1109])
            .write(b"disc4\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1110])
            .write(b"line breaking\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1111])
            .write(b"HYPH(\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1112])
            .write(b"hyphenation\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1113])
            .write(b" will be flushed\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1114]).write(
            b"Hyphenation exceptions must contain only letters\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1115]).write(
            b"and hyphens. But continue; I'll forgive and forget.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1116])
            .write(b"Not a letter\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1117]).write(
            b"Letters in \\hyphenation words must have \\lccode>0.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1118]).write(
            b"Proceed; I'll ignore the character I just read.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1119])
            .write(b"exception dictionary\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1120])
            .write(b"pattern memory ops\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1121])
            .write(b"pattern memory ops per language\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1122])
            .write(b"pattern memory\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1123])
            .write(b"Too late for \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1124])
            .write(b"patterns\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1125]).write(
            b"All patterns must be given before typesetting begins.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1126])
            .write(b"Bad \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1127])
            .write(b"(See Appendix H.)\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1128])
            .write(b"Nonletter\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1129])
            .write(b"Duplicate pattern\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1130])
            .write(b"snap node being discarded\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1131])
            .write(b"discard_or_move: \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1132])
            .write(b"pruning\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1133])
            .write(b"vertbreak\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1134]).write(
            b"Infinite glue shrinkage found in box being split\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1135]).write(
            b"The box you are \\vsplitting contains some infinitely\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1136]).write(
            b"shrinkable glue, e.g., `\\vss' or `\\vskip 0pt minus 1fil'.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1137]).write(
            b"Such glue doesn't belong there; but you can safely proceed,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1138])
            .write(b"vsplit\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1139])
            .write(b" needs a \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1140])
            .write(b"vbox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1141]).write(
            b"The box you are trying to split is an \\hbox.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1142]).write(
            b"I can't split such a box, so I'll leave it alone.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1143])
            .write(b"pagegoal\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1144])
            .write(b"pagetotal\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1145])
            .write(b"pagestretch\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1146])
            .write(b"pagefilstretch\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1147])
            .write(b"pagefillstretch\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1148])
            .write(b"pagefilllstretch\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1149])
            .write(b"pageshrink\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1150])
            .write(b"pagedepth\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1151])
            .write(b"fill\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1152])
            .write(b"filll\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1153])
            .write(b"### current page:\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1154])
            .write(b" (held over for next output)\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1155])
            .write(b"total height \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1156])
            .write(b" goal height \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1157])
            .write(b" adds \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1158])
            .write(b", #\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1159])
            .write(b" might split\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1160])
            .write(b"%% goal height=\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1161])
            .write(b", max depth=\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1162]).write(
            b"Insertions can only be added to a vbox\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1163]).write(
            b"Tut tut: You're trying to \\insert into a\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1164]).write(
            b"\\box register that now contains an \\hbox.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1165]).write(
            b"Proceed, and I'll discard its present contents.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1166]).write(
            b"Infinite glue shrinkage found on current page\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1167]).write(
            b"The page about to be output contains some infinitely\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1168])
            .write(b" g=\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1169])
            .write(b" c=\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1170]).write(
            b"Infinite glue shrinkage inserted from \0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1171]).write(
            b"The correction glue for page breaking with insertions\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1172]).write(
            b"must have finite shrinkability. But you may proceed,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1173])
            .write(b"% split\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1174])
            .write(b" to \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1175])
            .write(b"255 is not void\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1176]).write(
            b"You shouldn't use \\box255 except in \\output routines.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1177])
            .write(b"Output loop---\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1178])
            .write(b" consecutive dead cycles\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1179]).write(
            b"I've concluded that your \\output is awry; it never does a\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1180]).write(
            b"\\shipout, so I'm shipping \\box255 out myself. Next time\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1181]).write(
            b"increase \\maxdeadcycles if you want me to be more patient!\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1182]).write(
            b"Your sneaky output routine has problematic {'s and/or }'s.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1183]).write(
            b"I can't handle that very well; good luck.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1184]).write(
            b"Output routine didn't use all of \0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1185]).write(
            b"Your \\output commands should empty \\box255,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1186]).write(
            b"e.g., by saying `\\shipout\\box255'.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1187]).write(
            b"Proceed; I'll discard its present contents.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1188])
            .write(b"Missing $ inserted\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1189]).write(
            b"I've inserted a begin-math/end-math symbol since I think\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1190]).write(
            b"you left one out. Proceed, with fingers crossed.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1191]).write(
            b"Sorry, but I'm not programmed to handle this case;\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1192]).write(
            b"I'll just pretend that you didn't ask for it.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1193]).write(
            b"If you're in the wrong mode, you might be able to\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1194]).write(
            b"return to the right one by typing `I}' or `I$' or `I\\par'.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1195])
            .write(b"dump\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1196])
            .write(b"hskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1197])
            .write(b"hfil\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1198])
            .write(b"hfill\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1199])
            .write(b"hss\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1200])
            .write(b"hfilneg\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1201])
            .write(b"vskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1202])
            .write(b"vfil\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1203])
            .write(b"vfill\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1204])
            .write(b"vss\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1205])
            .write(b"vfilneg\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1206]).write(
            b"I've inserted something that you may have forgotten.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1207]).write(
            b"(See the <inserted text> above.)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1208]).write(
            b"With luck, this will get me unwedged. But if you\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1209]).write(
            b"really didn't forget anything, try typing `2' now; then\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1210]).write(
            b"my insertion and my current dilemma will both disappear.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1211])
            .write(b"right.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1212]).write(
            b"Things are pretty mixed up, but I think the worst is over.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1213])
            .write(b"Too many }'s\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1214]).write(
            b"You've closed more groups than you opened.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1215]).write(
            b"Such booboos are generally harmless, so keep going.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1216])
            .write(b"rightbrace\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1217])
            .write(b"Extra }, or forgotten \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1218]).write(
            b"I've deleted a group-closing symbol because it seems to be\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1219]).write(
            b"spurious, as in `$x}$'. But perhaps the } is legitimate and\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1220]).write(
            b"you forgot something else, as in `\\hbox{$x}'. In such cases\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1221]).write(
            b"the way to recover is to insert both the forgotten and the\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1222]).write(
            b"deleted material, e.g., by typing `I$}'.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1223])
            .write(b"moveleft\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1224])
            .write(b"moveright\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1225])
            .write(b"raise\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1226])
            .write(b"lower\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1227])
            .write(b"copy\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1228])
            .write(b"lastbox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1229])
            .write(b"vtop\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1230])
            .write(b"hbox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1231])
            .write(b"shipout\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1232])
            .write(b"leaders\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1233])
            .write(b"cleaders\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1234])
            .write(b"xleaders\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1235]).write(
            b"Leaders not followed by proper glue\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1236]).write(
            b"You should say `\\leaders <box or rule><hskip or vskip>'.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1237]).write(
            b"I found the <box or rule>, but there's no suitable\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1238]).write(
            b"<hskip or vskip>, so I'm ignoring these leaders.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1239])
            .write(b"tail1\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1240]).write(
            b"Sorry; this \\lastbox will be void.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1241]).write(
            b"Sorry...I usually can't take things from the current page.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1242]).write(
            b"This \\lastbox will therefore be void.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1243])
            .write(b"Missing `to' inserted\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1244]).write(
            b"I'm working on `\\vsplit<box number> to <dimen>';\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1245])
            .write(b"will look for the <dimen> next.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1246])
            .write(b"A <box> was supposed to be here\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1247]).write(
            b"I was expecting to see \\hbox or \\vbox or \\copy or \\box or\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1248]).write(
            b"something like that. So you might find something missing in\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1249]).write(
            b"your output. But keep trying; you can fix this later.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1250])
            .write(b"indent\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1251])
            .write(b"noindent\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1252])
            .write(b"quitvmode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1253])
            .write(b"' here except with leaders\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1254]).write(
            b"To put a horizontal rule in an hbox or an alignment,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1255]).write(
            b"you should use \\leaders or \\hrulefill (see The TeXbook).\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1256])
            .write(b"You can't \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1257]).write(
            b"I'm changing to \\insert0; box 255 is special.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1258])
            .write(b"pre\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1259]).write(
            b"Try `I\\vskip-\\lastskip' instead.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1260]).write(
            b"Try `I\\kern-\\lastkern' instead.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1261]).write(
            b"Perhaps you can make the output routine do it.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1262])
            .write(b"unpenalty\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1263])
            .write(b"unkern\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1264])
            .write(b"unskip\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1265])
            .write(b"unhbox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1266])
            .write(b"unhcopy\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1267])
            .write(b"unvbox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1268])
            .write(b"unvcopy\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1269]).write(
            b"Incompatible list can't be unboxed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1270]).write(
            b"Sorry, Pandora. (You sneaky devil.)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1271]).write(
            b"I refuse to unbox an \\hbox in vertical mode or vice versa.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1272]).write(
            b"And I can't open any boxes in math mode.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1273])
            .write(b"Illegal math \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1274]).write(
            b"Sorry: The third part of a discretionary break must be\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1275]).write(
            b"empty, in math formulas. I had to delete your third part.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1276])
            .write(b"Discretionary list is too long\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1277]).write(
            b"Wow---I never thought anybody would tweak me here.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1278]).write(
            b"You can't seriously need such a huge discretionary list?\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1279])
            .write(b"Improper discretionary list\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1280]).write(
            b"Discretionary lists must contain only boxes and kerns.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1281]).write(
            b"The following discretionary sublist has been deleted:\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1282])
            .write(b"Missing } inserted\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1283]).write(
            b"I've put in what seems to be necessary to fix\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1284]).write(
            b"the current column of the current alignment.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1285]).write(
            b"Try to go on, since this might almost work.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1286])
            .write(b"Misplaced \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1287]).write(
            b"I can't figure out why you would want to use a tab mark\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1288]).write(
            b"here. If you just want an ampersand, the remedy is\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1289]).write(
            b"simple: Just type `I\\&' now. But if some right brace\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1290]).write(
            b"up above has ended a previous alignment prematurely,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1291]).write(
            b"you're probably due for more error messages, and you\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1292]).write(
            b"might try typing `S' now just to see what is salvageable.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1293]).write(
            b"or \\cr or \\span just now. If something like a right brace\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1294]).write(
            b"I expect to see \\noalign only after the \\cr of\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1295]).write(
            b"an alignment. Proceed, and I'll ignore this case.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1296]).write(
            b"I expect to see \\omit only after tab marks or the \\cr of\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1297]).write(
            b"I'm guessing that you meant to end an alignment here.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1298]).write(
            b"I'm ignoring this, since I wasn't doing a \\mubyte.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1299]).write(
            b"I'm ignoring this, since I wasn't doing a \\csname.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1300])
            .write(b"eqno\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1301])
            .write(b"leqno\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1302])
            .write(b"displaylimits\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1303]).write(
            b"Limit controls must follow a math operator\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1304]).write(
            b"I'm ignoring this misplaced \\limits or \\nolimits command.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1305])
            .write(b"Missing delimiter (. inserted)\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1306]).write(
            b"I was expecting to see something like `(' or `\\{' or\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1307]).write(
            b"`\\}' here. If you typed, e.g., `{' instead of `\\{', you\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1308]).write(
            b"should probably delete the `{' by typing `1' now, so that\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1309]).write(
            b"braces don't get unbalanced. Otherwise just proceed.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1310]).write(
            b"Acceptable delimiters are characters whose \\delcode is\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1311]).write(
            b"nonnegative, or you can use `\\delimiter <delimiter code>'.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1312])
            .write(b"Please use \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1313])
            .write(b" for accents in math mode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1314]).write(
            b"I'm changing \\accent to \\mathaccent here; wish me luck.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1315]).write(
            b"(Accents are not the same in formulas as they are in text.)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1316])
            .write(b"Double superscript\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1317]).write(
            b"I treat `x^1^2' essentially like `x^1{}^2'.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1318])
            .write(b"Double subscript\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1319]).write(
            b"I treat `x_1_2' essentially like `x_1{}_2'.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1320])
            .write(b"above\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1321])
            .write(b"over\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1322])
            .write(b"atop\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1323])
            .write(b"abovewithdelims\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1324])
            .write(b"overwithdelims\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1325])
            .write(b"atopwithdelims\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1326]).write(
            b"Ambiguous; you need another { and }\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1327]).write(
            b"I'm ignoring this fraction specification, since I don't\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1328]).write(
            b"know whether a construction like `x \\over y \\over z'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1329]).write(
            b"means `{x \\over y} \\over z' or `x \\over {y \\over z}'.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1330]).write(
            b"I'm ignoring a \\middle that had no matching \\left.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1331]).write(
            b"I'm ignoring a \\right that had no matching \\left.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1332]).write(
            b"Math formula deleted: Insufficient symbol fonts\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1333]).write(
            b"Sorry, but I can't typeset math unless \\textfont 2\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1334]).write(
            b"and \\scriptfont 2 and \\scriptscriptfont 2 have all\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1335]).write(
            b"the \\fontdimen values needed in math symbol fonts.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1336]).write(
            b"Math formula deleted: Insufficient extension fonts\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1337]).write(
            b"Sorry, but I can't typeset math unless \\textfont 3\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1338]).write(
            b"and \\scriptfont 3 and \\scriptscriptfont 3 have all\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1339]).write(
            b"the \\fontdimen values needed in math extension fonts.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1340])
            .write(b"Display math should end with $$\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1341]).write(
            b"The `$' that I just saw supposedly matches a previous `$$'.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1342]).write(
            b"So I shall assume that you typed `$$' both times.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1343])
            .write(b"display\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1344])
            .write(b"Missing $$ inserted\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1345])
            .write(b"long\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1346])
            .write(b"outer\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1347])
            .write(b"global\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1348])
            .write(b"def\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1349])
            .write(b"gdef\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1350])
            .write(b"edef\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1351])
            .write(b"xdef\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1352])
            .write(b"prefix\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1353])
            .write(b"You can't use a prefix with `\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1354]).write(
            b"I'll pretend you didn't say \\long or \\outer or \\global.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1355]).write(
            b"I'll pretend you didn't say \\long or \\outer or \\global or \\protected.\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1356])
            .write(b"' or `\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1357]).write(
            b"I'll pretend you didn't say \\long or \\outer here.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1358]).write(
            b"I'll pretend you didn't say \\long or \\outer or \\protected here.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1359])
            .write(b"protected\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1360])
            .write(b"' with `\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1361]).write(
            b"Missing control sequence inserted\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1362]).write(
            b"Please don't say `\\def cs{...}', say `\\def\\cs{...}'.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1363]).write(
            b"I've inserted an inaccessible control sequence so that your\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1364]).write(
            b"definition will be completed without mixing me up too badly.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1365]).write(
            b"You can recover graciously from this error, if you're\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1366]).write(
            b"careful; see exercise 27.2 in The TeXbook.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1367])
            .write(b"inaccessible\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1368])
            .write(b"let\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1369])
            .write(b"futurelet\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1370])
            .write(b"mubyte\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1371])
            .write(b"noconvert\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1372]).write(
            b"not appear in <byte sequence> between \\mubyte and \\endmubyte.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1373])
            .write(b"The empty <byte sequence>, \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1374])
            .write(b" ignored\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1375])
            .write(b"The <byte sequence> in\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1376]).write(
            b"\\mubyte <token> <byte sequence>\\endmubyte should not be empty.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1377])
            .write(b"chardef\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1378])
            .write(b"mathchardef\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1379])
            .write(b"countdef\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1380])
            .write(b"dimendef\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1381])
            .write(b"skipdef\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1382])
            .write(b"muskipdef\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1383])
            .write(b"toksdef\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1384])
            .write(b"charsubdef\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1385])
            .write(b"New character substitution: \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1386])
            .write(b" = \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1387]).write(
            b"You should have said `\\read<number> to \\cs'.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1388]).write(
            b"I'm going to look for the \\cs now.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1389])
            .write(b"xordcode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1390])
            .write(b"xchrcode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1391])
            .write(b"xprncode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1392])
            .write(b"Invalid code (\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1393])
            .write(b"), should be in the range 0..\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1394])
            .write(b"), should be at most \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1395]).write(
            b"I'm going to use 0 instead of that illegal code value.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1396])
            .write(b"by\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1397])
            .write(b"Arithmetic overflow\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1398]).write(
            b"I can't carry out that multiplication or division,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1399]).write(
            b"since the result is out of range.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1400]).write(
            b"I'm forgetting what you said and not changing anything.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1401]).write(
            b"Sorry, \\setbox is not allowed after \\halign in a display,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1402]).write(
            b"or between \\accent and an accented character.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1403])
            .write(b"Bad space factor\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1404]).write(
            b"I allow only values in the range 1..32767 here.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1405]).write(
            b"I allow only nonnegative values here.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1406]).write(
            b"Patterns can be loaded only by INITEX\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1407])
            .write(b"hyphenchar\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1408])
            .write(b"skewchar\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1409])
            .write(b"lpcode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1410])
            .write(b"rpcode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1411])
            .write(b"efcode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1412])
            .write(b"tagcode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1413])
            .write(b"knbscode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1414])
            .write(b"stbscode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1415])
            .write(b"shbscode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1416])
            .write(b"knbccode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1417])
            .write(b"knaccode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1418])
            .write(b"pdfnoligatures\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1419])
            .write(b"at\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1420])
            .write(b"scaled\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1421])
            .write(b"Improper `at' size (\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1422])
            .write(b"pt), replaced by 10pt\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1423]).write(
            b"I can only handle fonts at positive sizes that are\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1424]).write(
            b"less than 2048pt, so I've changed what you said to 10pt.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1425])
            .write(b"select font \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1426])
            .write(b"errorstopmode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1427])
            .write(b"openin\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1428])
            .write(b"closein\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1429])
            .write(b"message\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1430])
            .write(b"errmessage\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1431]).write(
            b"(That was another \\errmessage.)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1432]).write(
            b"This error message was generated by an \\errmessage\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1433]).write(
            b"command, so I can't give any explicit help.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1434]).write(
            b"Pretend that you're Hercule Poirot: Examine all clues,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1435]).write(
            b"and deduce the truth by order and method.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1436])
            .write(b"lowercase\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1437])
            .write(b"uppercase\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1438])
            .write(b"show\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1439])
            .write(b"showbox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1440])
            .write(b"showthe\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1441])
            .write(b"showlists\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1442]).write(
            b"This isn't an error message; I'm just \\showing something.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1443]).write(
            b"Type `I\\show...' to show more (e.g., \\show\\cs,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1444]).write(
            b"\\showthe\\count10, \\showbox255, \\showlists).\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1445]).write(
            b"And type `I\\tracingonline=1\\show...' to show boxes and\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1446]).write(
            b"lists on your terminal as well as in the transcript file.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1447])
            .write(b"> \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1448])
            .write(b"undefined\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1449])
            .write(b"macro\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1450])
            .write(b"outer endtemplate\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1451])
            .write(b"> \\box\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1452])
            .write(b"OK\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1453])
            .write(b" (see the transcript file)\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1454])
            .write(b" (INITEX)\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1455])
            .write(b"You can't dump inside a group\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1456])
            .write(b"`{...\\dump}' is a no-no.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1457])
            .write(b" strings of total length \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1458]).write(
            b" memory locations dumped; current usage is \0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1459])
            .write(b" multiletter control sequences\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1460])
            .write(b" words of font info for \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1461])
            .write(b" preloaded fonts\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1462])
            .write(b" preloaded font\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1463])
            .write(b"\\font\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1464])
            .write(b" hyphenation exceptions\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1465])
            .write(b" hyphenation exception\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1466])
            .write(b"Hyphenation trie of length \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1467])
            .write(b" has \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1468])
            .write(b" ops\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1469])
            .write(b" op\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1470])
            .write(b" out of \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1471])
            .write(b" for language \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1472])
            .write(b" words of pdfTeX memory\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1473])
            .write(b" indirect objects\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1474])
            .write(b" (preloaded format=\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1475])
            .write(b"format file name\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1476])
            .write(b"Beginning to dump on file \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1477]).write(
            b" ==> Fatal error occurred, no output PDF file produced!\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1478])
            .write(b"Transcript written on \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1479])
            .write(b" )\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1480])
            .write(b"end occurred \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1481])
            .write(b"inside a group at level \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1482])
            .write(b"when \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1483])
            .write(b" on line \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1484])
            .write(b" was incomplete)\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1485]).write(
            b"(see the transcript file for additional information)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1486]).write(
            b"(\\dump is performed only by INITEX)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1487])
            .write(b"debug # (-1 to exit):\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1488])
            .write(b"openout\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1489])
            .write(b"closeout\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1490])
            .write(b"special\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1491])
            .write(b"immediate\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1492])
            .write(b"setlanguage\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1493])
            .write(b"pdfliteral\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1494])
            .write(b"pdfsetmatrix\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1495])
            .write(b"pdfsave\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1496])
            .write(b"pdfrestore\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1497])
            .write(b"pdfobj\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1498])
            .write(b"pdfrefobj\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1499])
            .write(b"pdfxform\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1500])
            .write(b"pdfrefxform\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1501])
            .write(b"pdfximage\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1502])
            .write(b"pdfrefximage\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1503])
            .write(b"pdfannot\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1504])
            .write(b"pdfstartlink\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1505])
            .write(b"pdfendlink\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1506])
            .write(b"pdfoutline\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1507])
            .write(b"pdfdest\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1508])
            .write(b"pdfthread\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1509])
            .write(b"pdfstartthread\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1510])
            .write(b"pdfendthread\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1511])
            .write(b"pdfsavepos\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1512])
            .write(b"pdfsnaprefpoint\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1513])
            .write(b"pdfsnapy\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1514])
            .write(b"pdfsnapycomp\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1515])
            .write(b"pdfinfo\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1516])
            .write(b"pdfcatalog\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1517])
            .write(b"pdfnames\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1518])
            .write(b"pdfincludechars\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1519])
            .write(b"pdffontattr\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1520])
            .write(b"pdfmapfile\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1521])
            .write(b"pdfmapline\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1522])
            .write(b"pdftrailer\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1523])
            .write(b"pdftrailerid\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1524])
            .write(b"pdfresettimer\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1525])
            .write(b"pdfsetrandomseed\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1526])
            .write(b"pdffontexpand\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1527])
            .write(b"pdfglyphtounicode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1528])
            .write(b"pdfnobuiltintounicode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1529])
            .write(b"pdfinterwordspaceon\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1530])
            .write(b"pdfinterwordspaceoff\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1531])
            .write(b"pdffakespace\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1532])
            .write(b"pdfrunninglinkoff\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1533])
            .write(b"pdfrunninglinkon\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1534])
            .write(b"pdfspacefont\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1535])
            .write(b"[unknown extension!]\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1536])
            .write(b"ext1\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1537]).write(
            b"not allowed in DVI mode (\\pdfoutput <= 0)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1538]).write(
            b"not allowed in DVI mode (\\pdfoutput <= 0); ignoring it\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1539])
            .write(b"\\pdfliteral\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1540])
            .write(b"\\pdfcolorstack\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1541])
            .write(b"Unknown color stack number \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1542]).write(
            b"Allocate and initialize a color stack with \\\\pdfcolorstackinit.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1543]).write(
            b"I'll use default color stack 0 here.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1544])
            .write(b"Proceed, with fingers crossed.\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1545]).write(
            b"Invalid negative color stack number\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1546])
            .write(b"set\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1547])
            .write(b"push\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1548])
            .write(b"pop\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1549])
            .write(b"current\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1550])
            .write(b"Color stack action is missing\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1551]).write(
            b"The expected actions for \\pdfcolorstack:\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1552])
            .write(b"    set, push, pop, current\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1553]).write(
            b"I'll ignore the color stack command.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1554])
            .write(b"\\pdfsave\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1555])
            .write(b"\\pdfrestore\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1556])
            .write(b"\\pdfobj\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1557])
            .write(b"reserveobjnum\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1558])
            .write(b"useobjnum\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1559]).write(
            b"invalid object number being ignored\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1560])
            .write(b"attr\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1561])
            .write(b"cannot find referenced object\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1562])
            .write(b"\\pdfrefobj\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1563])
            .write(b"\\pdfxform\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1564])
            .write(b"resources\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1565]).write(
            b"\\pdfxform cannot be used with a void box\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1566])
            .write(b"\\pdfrefxform\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1567]).write(
            b"too large image resolution ignored\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1568])
            .write(b"invalid image dimensions\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1569]).write(
            b"too small image resolution ignored\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1570])
            .write(b"mediabox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1571])
            .write(b"cropbox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1572])
            .write(b"bleedbox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1573])
            .write(b"trimbox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1574])
            .write(b"artbox\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1575])
            .write(b"named\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1576])
            .write(b"colorspace\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1577])
            .write(b"PDF inclusion\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1578]).write(
            b"Primitive \\pdfoptionalwaysusepdfpagebox is obsolete; use \\pdfpagebox instead.\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1579]).write(b"Primitive \\pdfoptionpdfinclusionerrorlevel is obsolete; use \\pdfinclusionerrorlevel instead.\0" as * const u8 as * const :: core :: ffi :: c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1580]).write(
            b"Primitive \\pdfforcepagebox is obsolete; use \\pdfpagebox instead.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1581])
            .write(b"\\pdfximage\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1582])
            .write(b"\\pdfrefximage\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1583])
            .write(b"user\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1584])
            .write(b"goto\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1585])
            .write(b"thread\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1586])
            .write(b"action type missing\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1587])
            .write(b"struct\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1588]).write(
            b"only GoTo action can be used with `struct'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1589])
            .write(b"name\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1590])
            .write(b"num identifier must be positive\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1591])
            .write(b"identifier type missing\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1592]).write(
            b"only GoTo action can be used with `page'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1593])
            .write(b"page number must be positive\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1594]).write(
            b"`goto' option cannot be used with both `file' and `num'\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1595])
            .write(b"newwindow\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1596])
            .write(b"nonewwindow\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1597]).write(
            b"`newwindow'/`nonewwindow' must be used with `goto' and `file' option\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1598])
            .write(b"\\pdfannot\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1599])
            .write(b"invalid object number\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1600])
            .write(b"\\pdfstartlink\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1601]).write(
            b"\\pdfstartlink cannot be used in vertical mode\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1602])
            .write(b"\\pdfendlink\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1603]).write(
            b"\\pdfendlink cannot be used in vertical mode\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1604])
            .write(b"\\pdfoutline\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1605]).write(
            b"destination with the same identifier (\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1606])
            .write(b") \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1607])
            .write(b"\\pdfdest\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1608]).write(
            b"struct identifier must be positive\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1609])
            .write(b"xyz\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1610])
            .write(b"zoom\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1611])
            .write(b"fitbh\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1612])
            .write(b"fitbv\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1613])
            .write(b"fitb\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1614])
            .write(b"fith\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1615])
            .write(b"fitv\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1616])
            .write(b"fitr\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1617])
            .write(b"fit\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1618])
            .write(b"destination type missing\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1619])
            .write(b"ext4\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1620]).write(
            b"has been already used, duplicate ignored\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1621])
            .write(b"\\pdfthread\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1622])
            .write(b"\\pdfstartthread\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1623])
            .write(b"\\pdfendthread\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1624])
            .write(b"\\pdfsnaprefpoint\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1625])
            .write(b"negative snap glue\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1626])
            .write(b"\\pdfsnapy\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1627])
            .write(b"\\pdfsnapycomp\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1628])
            .write(b"\\pdfinfo\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1629])
            .write(b"\\pdfcatalog\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1630])
            .write(b"openaction\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1631])
            .write(b"duplicate of openaction\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1632])
            .write(b"\\pdfnames\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1633])
            .write(b"\\pdftrailer\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1634])
            .write(b"\\pdftrailerid\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1635])
            .write(b"\\pdfincludechars\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1636])
            .write(b"\\pdfmapfile\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1637])
            .write(b"\\pdfmapline\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1638])
            .write(b"\\pdfnobuiltintounicode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1639])
            .write(b"\\pdfinterwordspaceon\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1640])
            .write(b"\\pdfinterwordspaceoff\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1641])
            .write(b"\\pdffakespace\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1642])
            .write(b"\\pdfrunninglinkoff\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1643])
            .write(b"\\pdfrunninglinkon\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1644])
            .write(b"\\pdfspacefont\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1645])
            .write(b"/Title (\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1646])
            .write(b"destination \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1647])
            .write(b"/R [0 0 \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1648])
            .write(b"/I << \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1649])
            .write(b" shipout\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1650])
            .write(b" (hyphenmin \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1651])
            .write(b" direct\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1652])
            .write(b"literal2\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1653])
            .write(b"pdfcolorstack \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1654])
            .write(b" set \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1655])
            .write(b" push \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1656])
            .write(b" pop\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1657])
            .write(b" current\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1658])
            .write(b" attr\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1659])
            .write(b" stream\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1660])
            .write(b" file\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1661])
            .write(b" action\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1662])
            .write(b" user\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1663])
            .write(b" goto name\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1664])
            .write(b" goto num\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1665])
            .write(b" thread name\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1666])
            .write(b" thread num\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1667])
            .write(b"displaying\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1668])
            .write(b"unknown action type\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1669])
            .write(b" struct\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1670])
            .write(b" name\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1671])
            .write(b" num\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1672])
            .write(b" zoom\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1673])
            .write(b"unknown!\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1674])
            .write(b"whatsit?\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1675])
            .write(b"ext2\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1676])
            .write(b"ext3\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1677])
            .write(b"endwrite\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1678])
            .write(b"runsystem(\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1679])
            .write(b")...\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1680])
            .write(b"clobbered\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1681]).write(
            b"quotation error in system command\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1682])
            .write(b"disabled (restricted)\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1683])
            .write(b"executed\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1684])
            .write(b"executed safely (allowed)\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1685])
            .write(b"disabled\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1686]).write(
            b"On this page there's a \\write with fewer real {'s than }'s.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1687])
            .write(b"pdf node ended up in DVI mode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1688])
            .write(b"output file name\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1689])
            .write(b"\\openout\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1690])
            .write(b" = `\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1691]).write(
            b"`\\pdfobj reserveobjnum' cannot be used with \\immediate\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1692])
            .write(b"pdftexspace\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1693])
            .write(b"/F \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1694])
            .write(b"/NewWindow \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1695])
            .write(b"true \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1696])
            .write(b"false \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1697])
            .write(b"/S /GoTo /D [\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1698])
            .write(b"/S /GoToR /D [\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1699])
            .write(b"/S /GoTo \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1700])
            .write(b"/S /GoToR \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1701])
            .write(b"/S /Thread \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1702])
            .write(b"SD\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1703])
            .write(b"/SD \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1704]).write(
            b"annotations cannot be inside an XForm\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1705])
            .write(b"pdf link stack size\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1706]).write(
            b"link annotations cannot be inside an XForm\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1707]).write(
            b"pdf_link_stack empty, \\pdfendlink used without \\pdfstartlink?\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1708]).write(
            b"\\pdfendlink ended up in different nesting level than \\pdfstartlink\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1709]).write(
            b"threads cannot be inside an XForm\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1710]).write(
            b"\\pdfendthread ended up in different nesting level than \\pdfstartthread\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1711]).write(
            b"destinations cannot be inside an XForm\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1712])
            .write(b" Do\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1713])
            .write(b"snapping\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1714]).write(
            b"invalid parameter value for gap_amount\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1715])
            .write(b"snap ref pos = \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1716])
            .write(b"snap glue = \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1717])
            .write(b"gap amount = \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1718])
            .write(b"stretch amount = \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1719])
            .write(b"shrink amount = \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1720])
            .write(b"last point = \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1721])
            .write(b"cur point = \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1722])
            .write(b"next point = \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1723])
            .write(b"get_vpos\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1724]).write(
            b"invalid parameter value for do_snapy_comp\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1725])
            .write(b"do_snapy_comp: tmp_v = \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1726])
            .write(b"do_snapy_comp: cur_v = \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1727])
            .write(b"do_snapy_comp: g = \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1728])
            .write(b"do_snapy_comp: g2 = \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1729])
            .write(b"do_snapy: count = \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1730])
            .write(b"do_snapy: cur_v = \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1731])
            .write(b"do_snapy: final skip = \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1732])
            .write(b"do_snapy: cur_v after snap = \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1733]).write(
            b"\\pdfstartlink ended up in vlist\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1734])
            .write(b"\\pdfendlink ended up in vlist\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1735]).write(
            b"\\pdfstartthread ended up in hlist\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1736]).write(
            b"\\pdfendthread ended up in hlist\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1737])
            .write(b"lastnodetype\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1738])
            .write(b"eTeXversion\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1739]).write(
            b"Sorry, this optional e-TeX feature has been disabled.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1740])
            .write(b"everyeof\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1741])
            .write(b"tracingassigns\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1742])
            .write(b"tracinggroups\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1743])
            .write(b"tracingifs\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1744])
            .write(b"tracingscantokens\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1745])
            .write(b"tracingnesting\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1746])
            .write(b"predisplaydirection\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1747])
            .write(b"lastlinefit\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1748])
            .write(b"savingvdiscards\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1749])
            .write(b"savinghyphcodes\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1750])
            .write(b"ignoreprimitiveerror\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1751])
            .write(b"bottom level\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1752])
            .write(b"semi \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1753])
            .write(b"simple\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1754])
            .write(b"adjusted \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1755])
            .write(b"no \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1756])
            .write(b"align\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1757])
            .write(b"disc\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1758])
            .write(b" choice\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1759])
            .write(b" shift\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1760])
            .write(b" left\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1761])
            .write(b" group (level \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1762])
            .write(b" at line \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1763])
            .write(b"leaving \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1764])
            .write(b"entering \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1765])
            .write(b"currentgrouplevel\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1766])
            .write(b"currentgrouptype\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1767])
            .write(b"currentiflevel\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1768])
            .write(b"currentiftype\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1769])
            .write(b"currentifbranch\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1770])
            .write(b"fontcharwd\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1771])
            .write(b"fontcharht\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1772])
            .write(b"fontchardp\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1773])
            .write(b"fontcharic\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1774])
            .write(b"parshapelength\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1775])
            .write(b"parshapeindent\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1776])
            .write(b"parshapedimen\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1777])
            .write(b"showgroups\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1778])
            .write(b"align entry\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1779])
            .write(b"showtokens\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1780])
            .write(b"unexpanded\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1781])
            .write(b"detokenize\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1782])
            .write(b"showifs\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1783])
            .write(b" entered on line \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1784])
            .write(b"no active conditionals\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1785])
            .write(b"### level \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1786])
            .write(b"interactionmode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1787])
            .write(b"Bad interaction mode\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1788]).write(
            b"Modes are 0=batch, 1=nonstop, 2=scroll, and\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1789]).write(
            b"3=errorstop. Proceed, and I'll ignore this case.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1790])
            .write(b"TeXXeTstate\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1791])
            .write(b"beginL\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1792])
            .write(b"endL\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1793])
            .write(b"beginR\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1794])
            .write(b"endR\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1795])
            .write(b", display\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1796])
            .write(b"LR1\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1797])
            .write(b"\\endL or \\endR problem (\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1798])
            .write(b" missing, \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1799])
            .write(b" extra\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1800])
            .write(b"LR2\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1801])
            .write(b"LR3\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1802])
            .write(b"LR4\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1803])
            .write(b"scantokens\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1804])
            .write(b"( \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1805])
            .write(b"readline\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1806])
            .write(b"ifdefined\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1807])
            .write(b"ifcsname\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1808])
            .write(b"iffontchar\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1809])
            .write(b"ifincsname\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1810])
            .write(b"ifpdfabsnum\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1811])
            .write(b"ifpdfabsdim\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1812])
            .write(b"' before `\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1813])
            .write(b"Warning: end of \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1814])
            .write(b" of a different file\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1815])
            .write(b"Warning: end of file when \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1816])
            .write(b" is incomplete\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1817])
            .write(b"numexpr\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1818])
            .write(b"dimexpr\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1819])
            .write(b"glueexpr\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1820])
            .write(b"muexpr\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1821]).write(
            b"I can't evaluate this expression,\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1822]).write(
            b"Missing ) inserted for expression\0" as *const u8 as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1823]).write(
            b"I was expecting to see `+', `-', `*', `/', or `)'. Didn't.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1824])
            .write(b"gluestretchorder\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1825])
            .write(b"glueshrinkorder\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1826])
            .write(b"gluestretch\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1827])
            .write(b"glueshrink\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1828])
            .write(b"mutoglue\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1829])
            .write(b"gluetomu\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1830])
            .write(b"marks\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1831])
            .write(b"topmarks\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1832])
            .write(b"firstmarks\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1833])
            .write(b"botmarks\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1834])
            .write(b"splitfirstmarks\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1835])
            .write(b"splitbotmarks\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1836]).write(
            b"A register number must be between 0 and 32767.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1837])
            .write(b" s=\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1838])
            .write(b" a=\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1839])
            .write(b"pagediscards\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1840])
            .write(b"splitdiscards\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1841])
            .write(b"interlinepenalties\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1842])
            .write(b"clubpenalties\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1843])
            .write(b"widowpenalties\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1844])
            .write(b"displaywidowpenalties\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1845])
            .write(b"substitution for \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1846]).write(
            b"Missing character: Incomplete substitution \0" as *const u8
                as *const ::core::ffi::c_char,
        );
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1847])
            .write(b"Using character substitution: \0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1848])
            .write(b"synctex\0" as *const u8 as *const ::core::ffi::c_char);
        const {
            assert!(core::mem::size_of::<*const ::core::ffi::c_char>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).poolfilearr[1849])
            .write(::core::ptr::null::<::core::ffi::c_char>());
        self.initialized[0] = true;
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
        (*crate::invocation_state::current()).generated_pdftex_pool
    ))
}
#[cfg(test)]
pub(crate) unsafe fn check_state_construction(phase: u8) {
    let invocation = crate::invocation_state::current();
    let destination = state();
    assert!(!destination.is_null());
    let _ = (invocation, phase);
}
