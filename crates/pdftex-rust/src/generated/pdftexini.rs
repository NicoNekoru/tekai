#[repr(C)]
pub struct __sFILEX {
    _unused: [u8; 0],
}

extern "C" {
    static mut __stdoutp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn fflush(_: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fputs(_: *const ::core::ffi::c_char, _: *mut FILE) -> ::core::ffi::c_int;
    fn putc(_: ::core::ffi::c_int, _: *mut FILE) -> ::core::ffi::c_int;
    fn fileno(_: *mut FILE) -> ::core::ffi::c_int;
    fn free(_: *mut ::core::ffi::c_void);
    fn exit(_: ::core::ffi::c_int) -> !;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strcpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    static mut kpse_def: kpathsea;
    fn xmalloc(size: size_t) -> address;
    fn xrealloc(old_address: address, new_size: size_t) -> address;
    fn uexit(status: ::core::ffi::c_int) -> !;
    fn open_output(_: *mut *mut FILE, fopen_mode: const_string) -> boolean;
    fn setupboundvariable(_: *mut integer, _: const_string, _: integer);
    static mut versionstring: *const ::core::ffi::c_char;
    fn loadpoolstrings(_: integer) -> ::core::ffi::c_int;
    fn initstarttime();
    fn readtcxfile();
    static mut translate_filename: string;
    fn get_seconds_and_micros(_: *mut integer, _: *mut integer);
    fn gzdopen(fd: ::core::ffi::c_int, mode: *const ::core::ffi::c_char) -> gzFile;
    fn gzsetparams(
        file: gzFile,
        level: ::core::ffi::c_int,
        strategy: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn gzclose(file: gzFile) -> ::core::ffi::c_int;
    fn do_dump(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: gzFile,
    );
    fn do_undump(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: gzFile,
    );
    static mut bad: integer;
    static mut xord: [ASCIIcode; 256];
    static mut xchr: [ASCIIcode; 256];
    static mut xprn: [ASCIIcode; 256];
    static mut mubyteread: [halfword; 256];
    static mut mubytewrite: [strnumber; 256];
    static mut mubytecswrite: [halfword; 128];
    static mut mubytekeep: integer;
    static mut mubyteprefix: integer;
    static mut mubytetablein: boolean;
    static mut mubytetableout: boolean;
    static mut mubyterelax: boolean;
    static mut mubytestart: boolean;
    static mut mubytestoken: halfword;
    static mut noconvert: boolean;
    static mut activenoconvert: boolean;
    static mut writenoexpanding: boolean;
    static mut csconverting: boolean;
    static mut specialprinting: boolean;
    static mut messageprinting: boolean;
    static mut nameoffile: *mut ASCIIcode;
    static mut buffer: *mut ASCIIcode;
    static mut first: integer;
    static mut last: integer;
    static mut maxbufstack: integer;
    static mut iniversion: boolean;
    static mut dumpline: boolean;
    static mut dump_name: const_string;
    static mut bounddefault: integer;
    static mut boundname: const_string;
    static mut membot: integer;
    static mut mainmemory: integer;
    static mut extramembot: integer;
    static mut memmin: integer;
    static mut memtop: integer;
    static mut extramemtop: integer;
    static mut memmax: integer;
    static mut errorline: integer;
    static mut halferrorline: integer;
    static mut maxprintline: integer;
    static mut maxstrings: integer;
    static mut stringsfree: integer;
    static mut stringvacancies: integer;
    static mut poolsize: integer;
    static mut poolfree: integer;
    static mut fontmemsize: integer;
    static mut fontmax: integer;
    static mut fontk: integer;
    static mut hyphsize: integer;
    static mut triesize: integer;
    static mut bufsize: integer;
    static mut stacksize: integer;
    static mut maxinopen: integer;
    static mut paramsize: integer;
    static mut nestsize: integer;
    static mut savesize: integer;
    static mut dvibufsize: integer;
    static mut expanddepth: integer;
    static mut parsefirstlinep: ::core::ffi::c_int;
    static mut filelineerrorstylep: ::core::ffi::c_int;
    static mut eightbitp: ::core::ffi::c_int;
    static mut haltingonerrorp: boolean;
    static mut srcspecialsp: boolean;
    static mut strpool: *mut packedASCIIcode;
    static mut strstart: *mut poolpointer;
    static mut poolptr: poolpointer;
    static mut strptr: strnumber;
    static mut initpoolptr: poolpointer;
    static mut initstrptr: strnumber;
    static mut selector: ::core::ffi::c_uchar;
    static mut tally: integer;
    static mut termoffset: integer;
    static mut fileoffset: integer;
    static mut interaction: ::core::ffi::c_uchar;
    static mut interactionoption: ::core::ffi::c_uchar;
    static mut deletionsallowed: boolean;
    static mut setboxallowed: boolean;
    static mut history: ::core::ffi::c_uchar;
    static mut errorcount: schar;
    static mut helpline: [strnumber; 6];
    static mut helpptr: ::core::ffi::c_uchar;
    static mut useerrhelp: boolean;
    static mut interrupt: integer;
    static mut OKtointerrupt: boolean;
    static mut randomseed: scaled;
    static mut twotothe: [integer; 31];
    static mut speclog: [integer; 29];
    static mut tempptr: halfword;
    static mut yzmem: *mut memoryword;
    static mut zmem: *mut memoryword;
    static mut lomemmax: halfword;
    static mut himemmin: halfword;
    static mut dynused: integer;
    static mut varused: integer;
    static mut avail: halfword;
    static mut memend: halfword;
    static mut rover: halfword;
    static mut fontinshortdisplay: integer;
    static mut nest: *mut liststaterecord;
    static mut nestptr: integer;
    static mut maxneststack: integer;
    static mut curlist: liststaterecord;
    static mut shownmode: ::core::ffi::c_short;
    static mut savetail: halfword;
    static mut prevtail: halfword;
    static mut zeqtb: *mut memoryword;
    static mut zzzaa: [quarterword; 916];
    static mut hash: *mut twohalves;
    static mut yhash: *mut twohalves;
    static mut hashused: halfword;
    static mut hashextra: halfword;
    static mut hashtop: halfword;
    static mut eqtbtop: halfword;
    static mut hashhigh: halfword;
    static mut nonewcontrolsequence: boolean;
    static mut cscount: integer;
    static mut prim: [twohalves; 2101];
    static mut primused: halfword;
    static mut savestack: *mut memoryword;
    static mut saveptr: integer;
    static mut maxsavestack: integer;
    static mut curlevel: quarterword;
    static mut curgroup: groupcode;
    static mut curboundary: integer;
    static mut magset: integer;
    static mut curcmd: eightbits;
    static mut curchr: halfword;
    static mut curcs: halfword;
    static mut curtok: halfword;
    static mut inputstack: *mut instaterecord;
    static mut inputptr: integer;
    static mut maxinstack: integer;
    static mut curinput: instaterecord;
    static mut inopen: integer;
    static mut openparens: integer;
    static mut inputfile: *mut alphafile;
    static mut line: integer;
    static mut linestack: *mut integer;
    static mut sourcefilenamestack: *mut strnumber;
    static mut fullsourcefilenamestack: *mut strnumber;
    static mut scannerstatus: ::core::ffi::c_uchar;
    static mut warningindex: halfword;
    static mut defref: halfword;
    static mut paramstack: *mut halfword;
    static mut paramptr: integer;
    static mut maxparamstack: integer;
    static mut alignstate: integer;
    static mut parloc: halfword;
    static mut partoken: halfword;
    static mut forceeof: boolean;
    static mut isincsname: boolean;
    static mut curmark: [halfword; 5];
    static mut curval: integer;
    static mut curvallevel: ::core::ffi::c_uchar;
    static mut radix: smallnumber;
    static mut curorder: glueord;
    static mut readopen: [::core::ffi::c_uchar; 17];
    static mut condptr: halfword;
    static mut iflimit: ::core::ffi::c_uchar;
    static mut curif: smallnumber;
    static mut ifline: integer;
    static mut formatdefaultlength: integer;
    static mut nameinprogress: boolean;
    static mut jobname: strnumber;
    static mut logopened: boolean;
    static mut outputfilename: strnumber;
    static mut fontinfo: *mut fmemoryword;
    static mut fmemptr: fontindex;
    static mut fontptr: internalfontnumber;
    static mut fontcheck: *mut fourquarters;
    static mut fontsize: *mut scaled;
    static mut fontdsize: *mut scaled;
    static mut fontparams: *mut fontindex;
    static mut fontname: *mut strnumber;
    static mut fontarea: *mut strnumber;
    static mut fontbc: *mut eightbits;
    static mut fontec: *mut eightbits;
    static mut fontglue: *mut halfword;
    static mut fontused: *mut boolean;
    static mut hyphenchar: *mut integer;
    static mut skewchar: *mut integer;
    static mut bcharlabel: *mut fontindex;
    static mut fontbchar: *mut ninebits;
    static mut fontfalsebchar: *mut ninebits;
    static mut charbase: *mut integer;
    static mut widthbase: *mut integer;
    static mut heightbase: *mut integer;
    static mut depthbase: *mut integer;
    static mut italicbase: *mut integer;
    static mut ligkernbase: *mut integer;
    static mut kernbase: *mut integer;
    static mut extenbase: *mut integer;
    static mut parambase: *mut integer;
    static mut nullcharacter: fourquarters;
    static mut totalpages: integer;
    static mut maxv: scaled;
    static mut maxh: scaled;
    static mut maxpush: integer;
    static mut lastbop: integer;
    static mut deadcycles: integer;
    static mut doingleaders: boolean;
    static mut dvibuf: *mut eightbits;
    static mut halfbuf: integer;
    static mut dvilimit: integer;
    static mut dviptr: integer;
    static mut dvioffset: integer;
    static mut dvigone: integer;
    static mut rightptr: halfword;
    static mut downptr: halfword;
    static mut curs: integer;
    static mut pdfmemsize: integer;
    static mut pdfmem: *mut integer;
    static mut pdfmemptr: integer;
    static mut pdfbuf: *mut eightbits;
    static mut pdfbufsize: integer;
    static mut pdfptr: integer;
    static mut pdfopbuf: *mut eightbits;
    static mut pdfosbuf: *mut eightbits;
    static mut pdfosbufsize: integer;
    static mut pdfosobjnum: *mut integer;
    static mut pdfosobjoff: *mut integer;
    static mut pdfoscntr: integer;
    static mut pdfopptr: integer;
    static mut pdfosptr: integer;
    static mut pdfosmode: boolean;
    static mut pdfoscurobjnum: integer;
    static mut pdfgone: longinteger;
    static mut zipwritestate: integer;
    static mut pdfversionwritten: boolean;
    static mut fixedpdfoutputset: boolean;
    static mut epochseconds: integer;
    static mut microseconds: integer;
    static mut fixedpdfdraftmodeset: boolean;
    static mut onebp: scaled;
    static mut onehundredbp: scaled;
    static mut onehundredinch: scaled;
    static mut oneinch: integer;
    static mut tenpow: [integer; 10];
    static mut initpdfoutput: boolean;
    static mut pdfoutputoption: integer;
    static mut pdfoutputvalue: integer;
    static mut pdfdraftmodeoption: integer;
    static mut pdfdraftmodevalue: integer;
    static mut pdfdummyfont: internalfontnumber;
    static mut objtabsize: integer;
    static mut objtab: *mut objentry;
    static mut headtab: [integer; 11];
    static mut objptr: integer;
    static mut sysobjptr: integer;
    static mut pdfseekwritelength: boolean;
    static mut pdfboxspecmedia: integer;
    static mut pdfboxspeccrop: integer;
    static mut pdfboxspecbleed: integer;
    static mut pdfboxspectrim: integer;
    static mut pdfboxspecart: integer;
    static mut pdffonttype: *mut eightbits;
    static mut pdffontattr: *mut strnumber;
    static mut pdffontnobuiltintounicode: *mut boolean;
    static mut pdfcharused: *mut charusedarray;
    static mut pdffontsize: *mut scaled;
    static mut pdffontnum: *mut integer;
    static mut pdffontmap: *mut fmentryptr;
    static mut pdfresnameprefix: strnumber;
    static mut lasttokensstring: strnumber;
    static mut vfpacketbase: *mut integer;
    static mut vfdefaultfont: *mut internalfontnumber;
    static mut vflocalfontnum: *mut internalfontnumber;
    static mut vfnf: internalfontnumber;
    static mut vfefnts: *mut integer;
    static mut vfifnts: *mut internalfontnumber;
    static mut vfcurs: integer;
    static mut vfstackptr: vfstackindex;
    static mut lastbadness: integer;
    static mut adjusttail: halfword;
    static mut pdffontblink: *mut internalfontnumber;
    static mut pdffontelink: *mut internalfontnumber;
    static mut pdffonthasspacechar: *mut boolean;
    static mut pdffontstretch: *mut integer;
    static mut pdffontshrink: *mut integer;
    static mut pdffontstep: *mut integer;
    static mut pdffontexpandratio: *mut integer;
    static mut pdffontautoexpand: *mut boolean;
    static mut pdffontlpbase: *mut integer;
    static mut pdffontrpbase: *mut integer;
    static mut pdffontefbase: *mut integer;
    static mut pdffontknbsbase: *mut integer;
    static mut pdffontstbsbase: *mut integer;
    static mut pdffontshbsbase: *mut integer;
    static mut pdffontknbcbase: *mut integer;
    static mut pdffontknacbase: *mut integer;
    static mut preadjusttail: halfword;
    static mut packbeginline: integer;
    static mut emptyfield: twohalves;
    static mut nulldelimiter: fourquarters;
    static mut magicoffset: integer;
    static mut curalign: halfword;
    static mut curspan: halfword;
    static mut curloop: halfword;
    static mut alignptr: halfword;
    static mut curhead: halfword;
    static mut curtail: halfword;
    static mut curprehead: halfword;
    static mut curpretail: halfword;
    static mut passive: halfword;
    static mut printednode: halfword;
    static mut passnumber: halfword;
    static mut activewidth: [scaled; 9];
    static mut background: [scaled; 9];
    static mut autobreaking: boolean;
    static mut prevp: halfword;
    static mut firstp: halfword;
    static mut prevcharp: halfword;
    static mut tryprevbreak: boolean;
    static mut prevlegal: halfword;
    static mut rejectedcurp: halfword;
    static mut beforerejectedcurp: boolean;
    static mut maxstretchratio: integer;
    static mut maxshrinkratio: integer;
    static mut curfontstep: integer;
    static mut noshrinkerroryet: boolean;
    static mut curp: halfword;
    static mut secondpass: boolean;
    static mut finalpass: boolean;
    static mut threshold: integer;
    static mut minimaldemerits: [integer; 4];
    static mut minimumdemerits: integer;
    static mut discwidth: [scaled; 9];
    static mut easyline: halfword;
    static mut lastspecialline: halfword;
    static mut firstwidth: scaled;
    static mut secondwidth: scaled;
    static mut firstindent: scaled;
    static mut secondindent: scaled;
    static mut bestbet: halfword;
    static mut fewestdemerits: integer;
    static mut bestline: halfword;
    static mut actuallooseness: integer;
    static mut linediff: integer;
    static mut hc: [::core::ffi::c_short; 66];
    static mut hn: ::core::ffi::c_uchar;
    static mut hb: halfword;
    static mut ha: halfword;
    static mut hf: internalfontnumber;
    static mut hu: [::core::ffi::c_short; 64];
    static mut hyfchar: integer;
    static mut curlang: ASCIIcode;
    static mut initcurlang: ASCIIcode;
    static mut lhyf: integer;
    static mut rhyf: integer;
    static mut initlhyf: integer;
    static mut initrhyf: integer;
    static mut hyfbchar: halfword;
    static mut hyf: [::core::ffi::c_uchar; 65];
    static mut ligaturepresent: boolean;
    static mut rthit: boolean;
    static mut lfthit: boolean;
    static mut trietrl: *mut triepointer;
    static mut trietro: *mut triepointer;
    static mut trietrc: *mut quarterword;
    static mut hyfdistance: [smallnumber; 35112];
    static mut hyfnum: [smallnumber; 35112];
    static mut hyfnext: [trieopcode; 35112];
    static mut opstart: [integer; 256];
    static mut hyphword: *mut strnumber;
    static mut hyphlist: *mut halfword;
    static mut hyphlink: *mut hyphpointer;
    static mut hyphcount: integer;
    static mut hyphnext: integer;
    static mut zzzab: [integer; 70223];
    static mut trieused: [trieopcode; 256];
    static mut trieoplang: [ASCIIcode; 35112];
    static mut trieopval: [trieopcode; 35112];
    static mut trieopptr: integer;
    static mut maxopused: trieopcode;
    static mut triec: *mut packedASCIIcode;
    static mut trieo: *mut trieopcode;
    static mut triel: *mut triepointer;
    static mut trier: *mut triepointer;
    static mut trieptr: triepointer;
    static mut triehash: *mut triepointer;
    static mut trietaken: *mut boolean;
    static mut triemin: [triepointer; 256];
    static mut triemax: triepointer;
    static mut trienotready: boolean;
    static mut pagetail: halfword;
    static mut pagecontents: ::core::ffi::c_uchar;
    static mut pagemaxdepth: scaled;
    static mut pagesofar: [scaled; 8];
    static mut lastglue: halfword;
    static mut lastpenalty: integer;
    static mut lastkern: scaled;
    static mut lastnodetype: integer;
    static mut insertpenalties: integer;
    static mut outputactive: boolean;
    static mut outputcanend: boolean;
    static mut cancelboundary: boolean;
    static mut insdisc: boolean;
    static mut aftertoken: halfword;
    static mut longhelpseen: boolean;
    static mut formatident: strnumber;
    static mut fmtfile: wordfile;
    static mut readyalready: integer;
    static mut writeopen: [boolean; 18];
    static mut writeloc: halfword;
    static mut pdflastobj: integer;
    static mut pdflastxform: integer;
    static mut pdflastximage: integer;
    static mut altrule: halfword;
    static mut warnpdfpagebox: boolean;
    static mut countdosnapy: integer;
    static mut pdfobjcount: integer;
    static mut pdfxformcount: integer;
    static mut pdfximagecount: integer;
    static mut pdfparentoutline: integer;
    static mut pdffirstoutline: integer;
    static mut pdflastoutline: integer;
    static mut pdfinfotoks: halfword;
    static mut pdfcatalogtoks: halfword;
    static mut pdfcatalogopenaction: integer;
    static mut pdfnamestoks: halfword;
    static mut pdfdestnamesptr: integer;
    static mut destnamessize: integer;
    static mut destnames: *mut destnameentry;
    static mut pkdpi: integer;
    static mut pdftrailertoks: halfword;
    static mut pdftraileridtoks: halfword;
    static mut genfakedinterwordspace: boolean;
    static mut genrunninglink: boolean;
    static mut pdfspacefontname: strnumber;
    static mut pdflinkstackptr: smallnumber;
    static mut eTeXmode: ::core::ffi::c_uchar;
    static mut etexp: boolean;
    static mut eofseen: *mut boolean;
    static mut LRptr: halfword;
    static mut LRproblems: integer;
    static mut curdir: smallnumber;
    static mut pseudofiles: halfword;
    static mut grpstack: *mut savepointer;
    static mut ifstack: *mut halfword;
    static mut maxregnum: halfword;
    static mut maxreghelpline: strnumber;
    static mut saroot: [halfword; 7];
    static mut curptr: halfword;
    static mut sanull: memoryword;
    static mut sachain: halfword;
    static mut salevel: quarterword;
    static mut lastlinefill: halfword;
    static mut dolastlinefit: boolean;
    static mut activenodesize: smallnumber;
    static mut fillwidth: [scaled; 3];
    static mut hyphstart: triepointer;
    static mut hyphindex: triepointer;
    static mut discptr: [halfword; 4];
    static mut editnamestart: poolpointer;
    static mut stopatspace: boolean;
    static mut shellenabledp: ::core::ffi::c_int;
    static mut restrictedshell: ::core::ffi::c_int;
    static mut k: ::core::ffi::c_uchar;
    static mut debugformatfile: boolean;
    static mut expanddepthcount: integer;
    static mut mltexp: boolean;
    static mut mltexenabledp: boolean;
    static mut enctexp: boolean;
    static mut enctexenabledp: boolean;
    static mut synctexoffset: integer;
    fn synctexinitcommand();
    fn println();
    fn zprintchar(s: ASCIIcode);
    fn zprint(s: integer);
    fn zslowprint(s: integer);
    fn zprintnl(s: strnumber);
    fn zprintesc(s: strnumber);
    fn zprintint(n: longinteger);
    fn zprintfilename(n: integer, a: integer, e: integer);
    fn zprintcsnames(hstart: integer, hfinish: integer);
    fn printfileline();
    fn jumpout() -> !;
    fn error();
    fn zoverflow(s: strnumber, n: integer) -> !;
    fn zconfusion(s: strnumber) -> !;
    fn initterminal() -> boolean;
    fn makestring() -> strnumber;
    fn slowmakestring() -> strnumber;
    fn normalizeselector();
    fn zprintscaled(s: scaled);
    fn zinitrandoms(seed: integer);
    fn getavail() -> halfword;
    fn zflushlist(p: halfword);
    fn zgetnode(s: integer) -> halfword;
    fn zfreenode(p: halfword, s: halfword);
    fn znewspec(p: halfword) -> halfword;
    fn znewparamglue(n: smallnumber) -> halfword;
    fn znewpenalty(m: integer) -> halfword;
    fn zdeletetokenref(p: halfword);
    fn zdeleteglueref(p: halfword);
    fn zflushnodelist(p: halfword);
    fn popnest();
    fn fixdateandtime();
    fn begindiagnostic();
    fn zenddiagnostic(blankline: boolean);
    fn zprintcmdchr(cmd: quarterword, chrcode: halfword);
    fn zidlookup(j: integer, l: integer) -> halfword;
    fn zprimlookup(s: strnumber) -> halfword;
    fn pseudoclose();
    fn zsadef(p: halfword, e: halfword);
    fn zgsadef(p: halfword, e: halfword);
    fn zeqdefine(p: halfword, t: quarterword, e: halfword);
    fn zeqworddefine(p: halfword, w: integer);
    fn zgeqdefine(p: halfword, t: quarterword, e: halfword);
    fn zgeqworddefine(p: halfword, w: integer);
    fn showcurcmdchr();
    fn endtokenlist();
    fn backinput();
    fn backerror();
    fn endfilereading();
    fn mubyteupdate();
    fn zdisposemunode(p: halfword);
    fn zdisposemutableout(cs: halfword);
    fn gettoken();
    fn zfindsaelement(t: smallnumber, n: halfword, w: boolean);
    fn getxtoken();
    fn scanleftbrace();
    fn scanoptionalequals();
    fn zscankeyword(s: strnumber) -> boolean;
    fn scancharnum();
    fn scanfourbitint();
    fn scanfifteenbitint();
    fn scanregisternum();
    fn zeffectivechar(errp: boolean, f: internalfontnumber, c: quarterword) -> integer;
    fn scanfontident();
    fn zfindfontdimen(writing: boolean);
    fn scanint();
    fn zscandimen(mu: boolean, inf: boolean, shortcut: boolean);
    fn zscanglue(level: smallnumber);
    fn zscantoks(macrodef: boolean, xpand: boolean) -> halfword;
    fn zreadtoks(n: integer, r: halfword, j: halfword);
    fn zzwmakenamestring(f: *mut wordfile) -> strnumber;
    fn zpackjobname(s: strnumber);
    fn zpromptfilename(s: strnumber, e: strnumber);
    fn openlogfile();
    fn startinput();
    fn zsettagcode(f: internalfontnumber, c: eightbits, i: integer);
    fn zsetnoligatures(f: internalfontnumber);
    fn zsetlpcode(f: internalfontnumber, c: eightbits, i: integer);
    fn zsetrpcode(f: internalfontnumber, c: eightbits, i: integer);
    fn zsetefcode(f: internalfontnumber, c: eightbits, i: integer);
    fn zsetknbscode(f: internalfontnumber, c: eightbits, i: integer);
    fn zsetstbscode(f: internalfontnumber, c: eightbits, i: integer);
    fn zsetshbscode(f: internalfontnumber, c: eightbits, i: integer);
    fn zsetknbccode(f: internalfontnumber, c: eightbits, i: integer);
    fn zsetknaccode(f: internalfontnumber, c: eightbits, i: integer);
    fn znewletterspacedfont(a: smallnumber);
    fn zmakefontcopy(a: smallnumber);
    fn zcheckexpandpars(f: internalfontnumber) -> boolean;
    fn zcharstretch(f: internalfontnumber, c: eightbits) -> scaled;
    fn zcharshrink(f: internalfontnumber, c: eightbits) -> scaled;
    fn zkernstretch(p: halfword) -> scaled;
    fn zkernshrink(p: halfword) -> scaled;
    fn zfiniteshrink(p: halfword) -> halfword;
    fn ztrybreak(pi: integer, breaktype: smallnumber);
    fn zpostlinebreak(d: boolean);
    fn hyphenate();
    fn showsavegroups();
    fn zscanbox(boxcontext: integer);
    fn getrtoken();
    fn trapzeroglue();
    fn zdoregistercommand(a: smallnumber);
    fn alteraux();
    fn alterprevgraf();
    fn alterpagesofar();
    fn alterinteger();
    fn alterboxdimen();
    fn znewfont(a: smallnumber);
    fn newinteraction();
    fn maincontrol();
    fn openfmtfile() -> boolean;
    fn closefilesandterminate();
    fn pdfinitmapfile(map_name: const_string);
    fn dumptounicode();
    fn undumptounicode();
    fn makepdftexbanner();
    fn dumpimagemeta();
    fn undumpimagemeta(_: integer, _: integer, _: integer);
}
pub type __int64_t = i64;
pub type __darwin_size_t = usize;
pub type __darwin_off_t = __int64_t;
pub type uintptr_t = usize;
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut ::core::ffi::c_uchar,
    pub _size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut ::core::ffi::c_uchar,
    pub _r: ::core::ffi::c_int,
    pub _w: ::core::ffi::c_int,
    pub _flags: ::core::ffi::c_short,
    pub _file: ::core::ffi::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::core::ffi::c_int,
    pub _cookie: *mut ::core::ffi::c_void,
    pub _close: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub _read: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _seek: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, fpos_t, ::core::ffi::c_int) -> fpos_t,
    >,
    pub _write: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::core::ffi::c_int,
    pub _ubuf: [::core::ffi::c_uchar; 3],
    pub _nbuf: [::core::ffi::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: ::core::ffi::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
pub type off_t = __darwin_off_t;
pub type boolean = ::core::ffi::c_int;
pub type string = *mut ::core::ffi::c_char;
pub type const_string = *const ::core::ffi::c_char;
pub type address = *mut ::core::ffi::c_void;
pub type p_record_input = Option<unsafe extern "C" fn(const_string) -> ()>;
pub type p_record_output = Option<unsafe extern "C" fn(const_string) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct str_llist_elt {
    pub str_0: string,
    pub moved: boolean,
    pub next: *mut str_llist_elt,
}
pub type str_llist_type = *mut str_llist_elt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cache_entry {
    pub key: const_string,
    pub value: *mut str_llist_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct expansion_type {
    pub var: const_string,
    pub expanding: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_element_struct {
    pub key: const_string,
    pub value: const_string,
    pub next: *mut hash_element_struct,
}
pub type hash_element_type = hash_element_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table_type {
    pub buckets: *mut *mut hash_element_type,
    pub size: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct str_list_type {
    pub length: ::core::ffi::c_uint,
    pub list: *mut string,
}
pub type kpse_src_type = ::core::ffi::c_uint;
pub const kpse_src_cmdline: kpse_src_type = 6;
pub const kpse_src_x: kpse_src_type = 5;
pub const kpse_src_env: kpse_src_type = 4;
pub const kpse_src_client_cnf: kpse_src_type = 3;
pub const kpse_src_texmf_cnf: kpse_src_type = 2;
pub const kpse_src_compile: kpse_src_type = 1;
pub const kpse_src_implicit: kpse_src_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kpse_format_info_type {
    pub type_0: const_string,
    pub path: string,
    pub raw_path: const_string,
    pub path_source: const_string,
    pub override_path: const_string,
    pub client_path: const_string,
    pub cnf_path: const_string,
    pub default_path: const_string,
    pub suffix: *mut const_string,
    pub alt_suffix: *mut const_string,
    pub suffix_search_only: boolean,
    pub program: const_string,
    pub argc: ::core::ffi::c_int,
    pub argv: *mut const_string,
    pub program_enabled_p: boolean,
    pub program_enable_level: kpse_src_type,
    pub binmode: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kpathsea_instance {
    pub record_input: p_record_input,
    pub record_output: p_record_output,
    pub cnf_hash: hash_table_type,
    pub doing_cnf_init: boolean,
    pub db: hash_table_type,
    pub alias_db: hash_table_type,
    pub db_dir_list: str_list_type,
    pub debug: ::core::ffi::c_uint,
    pub link_table: hash_table_type,
    pub the_cache: *mut cache_entry,
    pub cache_length: ::core::ffi::c_uint,
    pub map: hash_table_type,
    pub map_path: const_string,
    pub debug_hash_lookup_int: boolean,
    pub elt: string,
    pub elt_alloc: ::core::ffi::c_uint,
    pub path: const_string,
    pub followup_search: boolean,
    pub log_file: *mut FILE,
    pub log_opened: boolean,
    pub invocation_name: string,
    pub invocation_short_name: string,
    pub program_name: string,
    pub ll_verbose: ::core::ffi::c_int,
    pub fallback_font: const_string,
    pub fallback_resolutions_string: const_string,
    pub fallback_resolutions: *mut ::core::ffi::c_uint,
    pub format_info: [kpse_format_info_type; 59],
    pub make_tex_discard_errors: boolean,
    pub missfont: *mut FILE,
    pub expansions: *mut expansion_type,
    pub expansion_len: ::core::ffi::c_uint,
    pub saved_env: *mut *mut ::core::ffi::c_char,
    pub saved_count: ::core::ffi::c_int,
}
pub type kpathsea = *mut kpathsea_instance;
pub type schar = ::core::ffi::c_schar;
pub type integer = ::core::ffi::c_int;
pub type longinteger = off_t;
pub type text = *mut FILE;
pub type strnumber = integer;
pub type wordfile = gzFile;
pub type gzFile = *mut gzFile_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: ::core::ffi::c_uint,
    pub next: *mut ::core::ffi::c_uchar,
    pub pos: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub junk: halfword,
    pub CINT: integer,
}
pub type halfword = integer;
#[derive(Copy, Clone)]
#[repr(C)]
pub union memoryword {
    pub gr: glueratio,
    pub hh: twohalves,
    pub u: C2RustUnnamed,
    pub v: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub junk: halfword,
    pub QQQQ: fourquarters,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fourquarters {
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub B3: quarterword,
    pub B2: quarterword,
    pub B1: quarterword,
    pub B0: quarterword,
}
pub type quarterword = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub union twohalves {
    pub v: C2RustUnnamed_3,
    pub u: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub B1: ::core::ffi::c_short,
    pub B0: ::core::ffi::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub LH: halfword,
    pub RH: halfword,
}
pub type glueratio = ::core::ffi::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct objentry {
    pub int0: integer,
    pub int1: integer,
    pub int2: longinteger,
    pub int3: integer,
    pub int4: integer,
}
pub type trieopcode = ::core::ffi::c_ushort;
pub type triepointer = integer;
pub type smallnumber = ::core::ffi::c_uchar;
pub type ASCIIcode = ::core::ffi::c_uchar;
pub type packedASCIIcode = ::core::ffi::c_uchar;
pub type hyphpointer = ::core::ffi::c_ushort;
pub type internalfontnumber = integer;
pub type fontindex = integer;
pub type scaled = integer;
pub type ninebits = ::core::ffi::c_short;
pub type eightbits = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub union fmemoryword {
    pub u: C2RustUnnamed_5,
    pub v: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub QQQQ: fourquarters,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub CINT: integer,
}
pub type poolpointer = integer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct instaterecord {
    pub statefield: quarterword,
    pub indexfield: quarterword,
    pub startfield: halfword,
    pub locfield: halfword,
    pub limitfield: halfword,
    pub namefield: halfword,
    pub synctextagfield: integer,
}
pub type fmentryptr = *mut integer;
pub type charusedarray = [eightbits; 32];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct liststaterecord {
    pub modefield: ::core::ffi::c_short,
    pub headfield: halfword,
    pub tailfield: halfword,
    pub eTeXauxfield: halfword,
    pub pgfield: integer,
    pub mlfield: integer,
    pub auxfield: memoryword,
}
pub type vfstackindex = integer;
pub type glueord = ::core::ffi::c_uchar;
pub type groupcode = ::core::ffi::c_uchar;
pub type savepointer = integer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct destnameentry {
    pub objname: strnumber,
    pub objnum: integer,
}
pub type alphafile = text;
pub const FOPEN_WBIN_MODE: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"wb\0") };
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INTEGER_MAX: ::core::ffi::c_int = INT_MAX;
pub const maxint: ::core::ffi::c_int = INTEGER_MAX;
pub const TEXMFENGINENAME: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"pdftex\0") };
pub const Z_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Z_DEFAULT_STRATEGY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const hashoffset: ::core::ffi::c_int = 514 as ::core::ffi::c_int;
pub const trieopsize: ::core::ffi::c_long = 35111 as ::core::ffi::c_long;
pub const negtrieopsize: ::core::ffi::c_long = -(35111 as ::core::ffi::c_long);
pub const mintrieop: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const maxtrieop: ::core::ffi::c_long = 65535 as ::core::ffi::c_long;
pub const enginename: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"pdftex\0") };
pub const infmembot: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const supmembot: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const infmainmemory: ::core::ffi::c_int = 3000 as ::core::ffi::c_int;
pub const supmainmemory: ::core::ffi::c_long = 256000000 as ::core::ffi::c_long;
pub const inftriesize: ::core::ffi::c_int = 8000 as ::core::ffi::c_int;
pub const suptriesize: ::core::ffi::c_long = 4194303 as ::core::ffi::c_long;
pub const infmaxstrings: ::core::ffi::c_int = 3000 as ::core::ffi::c_int;
pub const supmaxstrings: ::core::ffi::c_long = 2097151 as ::core::ffi::c_long;
pub const infstringsfree: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const supstringsfree: ::core::ffi::c_long = 2097151 as ::core::ffi::c_long;
pub const infbufsize: ::core::ffi::c_int = 500 as ::core::ffi::c_int;
pub const supbufsize: ::core::ffi::c_long = 30000000 as ::core::ffi::c_long;
pub const infnestsize: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const supnestsize: ::core::ffi::c_int = 4000 as ::core::ffi::c_int;
pub const infmaxinopen: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const supmaxinopen: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const infparamsize: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const supparamsize: ::core::ffi::c_int = 32767 as ::core::ffi::c_int;
pub const infsavesize: ::core::ffi::c_int = 600 as ::core::ffi::c_int;
pub const supsavesize: ::core::ffi::c_long = 30000000 as ::core::ffi::c_long;
pub const infstacksize: ::core::ffi::c_int = 200 as ::core::ffi::c_int;
pub const supstacksize: ::core::ffi::c_int = 30000 as ::core::ffi::c_int;
pub const infdvibufsize: ::core::ffi::c_int = 800 as ::core::ffi::c_int;
pub const supdvibufsize: ::core::ffi::c_long = 65536 as ::core::ffi::c_long;
pub const inffontmemsize: ::core::ffi::c_int = 20000 as ::core::ffi::c_int;
pub const supfontmemsize: ::core::ffi::c_long = 147483647 as ::core::ffi::c_long;
pub const supfontmax: ::core::ffi::c_int = 9000 as ::core::ffi::c_int;
pub const inffontmax: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const infpoolsize: ::core::ffi::c_int = 32000 as ::core::ffi::c_int;
pub const suppoolsize: ::core::ffi::c_long = 40000000 as ::core::ffi::c_long;
pub const infpoolfree: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const suppoolfree: ::core::ffi::c_long = 40000000 as ::core::ffi::c_long;
pub const infstringvacancies: ::core::ffi::c_int = 8000 as ::core::ffi::c_int;
pub const supstringvacancies: ::core::ffi::c_long = suppoolsize - 23000 as ::core::ffi::c_long;
pub const suphashextra: ::core::ffi::c_long = 2097151 as ::core::ffi::c_long;
pub const infhashextra: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const suphyphsize: ::core::ffi::c_long = 65535 as ::core::ffi::c_long;
pub const infhyphsize: ::core::ffi::c_int = 610 as ::core::ffi::c_int;
pub const infpdfmemsize: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
pub const suppdfmemsize: ::core::ffi::c_long = 10000000 as ::core::ffi::c_long;
pub const pdfopbufsize: ::core::ffi::c_int = 16384 as ::core::ffi::c_int;
pub const infpdfosbufsize: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const infobjtabsize: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const supobjtabsize: ::core::ffi::c_long = 8388607 as ::core::ffi::c_long;
pub const infdestnamessize: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const supdestnamessize: ::core::ffi::c_long = 500000 as ::core::ffi::c_long;
pub const infpkdpi: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const suppkdpi: ::core::ffi::c_int = 8000 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn initialize() {
    let mut mem: *mut memoryword = zmem;
    let mut eqtb: *mut memoryword = zeqtb;
    let mut i: integer = 0;
    let mut k_0: integer = 0;
    let mut z: hyphpointer = 0;
    xchr[32 as ::core::ffi::c_int as usize] = ' ' as i32 as ASCIIcode;
    xchr[33 as ::core::ffi::c_int as usize] = '!' as i32 as ASCIIcode;
    xchr[34 as ::core::ffi::c_int as usize] = '"' as i32 as ASCIIcode;
    xchr[35 as ::core::ffi::c_int as usize] = '#' as i32 as ASCIIcode;
    xchr[36 as ::core::ffi::c_int as usize] = '$' as i32 as ASCIIcode;
    xchr[37 as ::core::ffi::c_int as usize] = '%' as i32 as ASCIIcode;
    xchr[38 as ::core::ffi::c_int as usize] = '&' as i32 as ASCIIcode;
    xchr[39 as ::core::ffi::c_int as usize] = '\'' as i32 as ASCIIcode;
    xchr[40 as ::core::ffi::c_int as usize] = '(' as i32 as ASCIIcode;
    xchr[41 as ::core::ffi::c_int as usize] = ')' as i32 as ASCIIcode;
    xchr[42 as ::core::ffi::c_int as usize] = '*' as i32 as ASCIIcode;
    xchr[43 as ::core::ffi::c_int as usize] = '+' as i32 as ASCIIcode;
    xchr[44 as ::core::ffi::c_int as usize] = ',' as i32 as ASCIIcode;
    xchr[45 as ::core::ffi::c_int as usize] = '-' as i32 as ASCIIcode;
    xchr[46 as ::core::ffi::c_int as usize] = '.' as i32 as ASCIIcode;
    xchr[47 as ::core::ffi::c_int as usize] = '/' as i32 as ASCIIcode;
    xchr[48 as ::core::ffi::c_int as usize] = '0' as i32 as ASCIIcode;
    xchr[49 as ::core::ffi::c_int as usize] = '1' as i32 as ASCIIcode;
    xchr[50 as ::core::ffi::c_int as usize] = '2' as i32 as ASCIIcode;
    xchr[51 as ::core::ffi::c_int as usize] = '3' as i32 as ASCIIcode;
    xchr[52 as ::core::ffi::c_int as usize] = '4' as i32 as ASCIIcode;
    xchr[53 as ::core::ffi::c_int as usize] = '5' as i32 as ASCIIcode;
    xchr[54 as ::core::ffi::c_int as usize] = '6' as i32 as ASCIIcode;
    xchr[55 as ::core::ffi::c_int as usize] = '7' as i32 as ASCIIcode;
    xchr[56 as ::core::ffi::c_int as usize] = '8' as i32 as ASCIIcode;
    xchr[57 as ::core::ffi::c_int as usize] = '9' as i32 as ASCIIcode;
    xchr[58 as ::core::ffi::c_int as usize] = ':' as i32 as ASCIIcode;
    xchr[59 as ::core::ffi::c_int as usize] = ';' as i32 as ASCIIcode;
    xchr[60 as ::core::ffi::c_int as usize] = '<' as i32 as ASCIIcode;
    xchr[61 as ::core::ffi::c_int as usize] = '=' as i32 as ASCIIcode;
    xchr[62 as ::core::ffi::c_int as usize] = '>' as i32 as ASCIIcode;
    xchr[63 as ::core::ffi::c_int as usize] = '?' as i32 as ASCIIcode;
    xchr[64 as ::core::ffi::c_int as usize] = '@' as i32 as ASCIIcode;
    xchr[65 as ::core::ffi::c_int as usize] = 'A' as i32 as ASCIIcode;
    xchr[66 as ::core::ffi::c_int as usize] = 'B' as i32 as ASCIIcode;
    xchr[67 as ::core::ffi::c_int as usize] = 'C' as i32 as ASCIIcode;
    xchr[68 as ::core::ffi::c_int as usize] = 'D' as i32 as ASCIIcode;
    xchr[69 as ::core::ffi::c_int as usize] = 'E' as i32 as ASCIIcode;
    xchr[70 as ::core::ffi::c_int as usize] = 'F' as i32 as ASCIIcode;
    xchr[71 as ::core::ffi::c_int as usize] = 'G' as i32 as ASCIIcode;
    xchr[72 as ::core::ffi::c_int as usize] = 'H' as i32 as ASCIIcode;
    xchr[73 as ::core::ffi::c_int as usize] = 'I' as i32 as ASCIIcode;
    xchr[74 as ::core::ffi::c_int as usize] = 'J' as i32 as ASCIIcode;
    xchr[75 as ::core::ffi::c_int as usize] = 'K' as i32 as ASCIIcode;
    xchr[76 as ::core::ffi::c_int as usize] = 'L' as i32 as ASCIIcode;
    xchr[77 as ::core::ffi::c_int as usize] = 'M' as i32 as ASCIIcode;
    xchr[78 as ::core::ffi::c_int as usize] = 'N' as i32 as ASCIIcode;
    xchr[79 as ::core::ffi::c_int as usize] = 'O' as i32 as ASCIIcode;
    xchr[80 as ::core::ffi::c_int as usize] = 'P' as i32 as ASCIIcode;
    xchr[81 as ::core::ffi::c_int as usize] = 'Q' as i32 as ASCIIcode;
    xchr[82 as ::core::ffi::c_int as usize] = 'R' as i32 as ASCIIcode;
    xchr[83 as ::core::ffi::c_int as usize] = 'S' as i32 as ASCIIcode;
    xchr[84 as ::core::ffi::c_int as usize] = 'T' as i32 as ASCIIcode;
    xchr[85 as ::core::ffi::c_int as usize] = 'U' as i32 as ASCIIcode;
    xchr[86 as ::core::ffi::c_int as usize] = 'V' as i32 as ASCIIcode;
    xchr[87 as ::core::ffi::c_int as usize] = 'W' as i32 as ASCIIcode;
    xchr[88 as ::core::ffi::c_int as usize] = 'X' as i32 as ASCIIcode;
    xchr[89 as ::core::ffi::c_int as usize] = 'Y' as i32 as ASCIIcode;
    xchr[90 as ::core::ffi::c_int as usize] = 'Z' as i32 as ASCIIcode;
    xchr[91 as ::core::ffi::c_int as usize] = '[' as i32 as ASCIIcode;
    xchr[92 as ::core::ffi::c_int as usize] = '\\' as i32 as ASCIIcode;
    xchr[93 as ::core::ffi::c_int as usize] = ']' as i32 as ASCIIcode;
    xchr[94 as ::core::ffi::c_int as usize] = '^' as i32 as ASCIIcode;
    xchr[95 as ::core::ffi::c_int as usize] = '_' as i32 as ASCIIcode;
    xchr[96 as ::core::ffi::c_int as usize] = '`' as i32 as ASCIIcode;
    xchr[97 as ::core::ffi::c_int as usize] = 'a' as i32 as ASCIIcode;
    xchr[98 as ::core::ffi::c_int as usize] = 'b' as i32 as ASCIIcode;
    xchr[99 as ::core::ffi::c_int as usize] = 'c' as i32 as ASCIIcode;
    xchr[100 as ::core::ffi::c_int as usize] = 'd' as i32 as ASCIIcode;
    xchr[101 as ::core::ffi::c_int as usize] = 'e' as i32 as ASCIIcode;
    xchr[102 as ::core::ffi::c_int as usize] = 'f' as i32 as ASCIIcode;
    xchr[103 as ::core::ffi::c_int as usize] = 'g' as i32 as ASCIIcode;
    xchr[104 as ::core::ffi::c_int as usize] = 'h' as i32 as ASCIIcode;
    xchr[105 as ::core::ffi::c_int as usize] = 'i' as i32 as ASCIIcode;
    xchr[106 as ::core::ffi::c_int as usize] = 'j' as i32 as ASCIIcode;
    xchr[107 as ::core::ffi::c_int as usize] = 'k' as i32 as ASCIIcode;
    xchr[108 as ::core::ffi::c_int as usize] = 'l' as i32 as ASCIIcode;
    xchr[109 as ::core::ffi::c_int as usize] = 'm' as i32 as ASCIIcode;
    xchr[110 as ::core::ffi::c_int as usize] = 'n' as i32 as ASCIIcode;
    xchr[111 as ::core::ffi::c_int as usize] = 'o' as i32 as ASCIIcode;
    xchr[112 as ::core::ffi::c_int as usize] = 'p' as i32 as ASCIIcode;
    xchr[113 as ::core::ffi::c_int as usize] = 'q' as i32 as ASCIIcode;
    xchr[114 as ::core::ffi::c_int as usize] = 'r' as i32 as ASCIIcode;
    xchr[115 as ::core::ffi::c_int as usize] = 's' as i32 as ASCIIcode;
    xchr[116 as ::core::ffi::c_int as usize] = 't' as i32 as ASCIIcode;
    xchr[117 as ::core::ffi::c_int as usize] = 'u' as i32 as ASCIIcode;
    xchr[118 as ::core::ffi::c_int as usize] = 'v' as i32 as ASCIIcode;
    xchr[119 as ::core::ffi::c_int as usize] = 'w' as i32 as ASCIIcode;
    xchr[120 as ::core::ffi::c_int as usize] = 'x' as i32 as ASCIIcode;
    xchr[121 as ::core::ffi::c_int as usize] = 'y' as i32 as ASCIIcode;
    xchr[122 as ::core::ffi::c_int as usize] = 'z' as i32 as ASCIIcode;
    xchr[123 as ::core::ffi::c_int as usize] = '{' as i32 as ASCIIcode;
    xchr[124 as ::core::ffi::c_int as usize] = '|' as i32 as ASCIIcode;
    xchr[125 as ::core::ffi::c_int as usize] = '}' as i32 as ASCIIcode;
    xchr[126 as ::core::ffi::c_int as usize] = '~' as i32 as ASCIIcode;
    let mut for_end: integer = 0;
    i = 0 as ::core::ffi::c_int as integer;
    for_end = 31 as ::core::ffi::c_int as integer;
    if i <= for_end {
        loop {
            xchr[i as usize] = i as ASCIIcode;
            let fresh47 = i;
            i = i + 1;
            if !(fresh47 < for_end) {
                break;
            }
        }
    }
    let mut for_end_0: integer = 0;
    i = 127 as ::core::ffi::c_int as integer;
    for_end_0 = 255 as ::core::ffi::c_int as integer;
    if i <= for_end_0 {
        loop {
            xchr[i as usize] = i as ASCIIcode;
            let fresh48 = i;
            i = i + 1;
            if !(fresh48 < for_end_0) {
                break;
            }
        }
    }
    let mut for_end_1: integer = 0;
    i = 0 as ::core::ffi::c_int as integer;
    for_end_1 = 255 as ::core::ffi::c_int as integer;
    if i <= for_end_1 {
        loop {
            mubyteread[i as usize] = -(268435455 as ::core::ffi::c_long) as halfword;
            let fresh49 = i;
            i = i + 1;
            if !(fresh49 < for_end_1) {
                break;
            }
        }
    }
    let mut for_end_2: integer = 0;
    i = 0 as ::core::ffi::c_int as integer;
    for_end_2 = 255 as ::core::ffi::c_int as integer;
    if i <= for_end_2 {
        loop {
            mubytewrite[i as usize] = 0 as ::core::ffi::c_int as strnumber;
            let fresh50 = i;
            i = i + 1;
            if !(fresh50 < for_end_2) {
                break;
            }
        }
    }
    let mut for_end_3: integer = 0;
    i = 0 as ::core::ffi::c_int as integer;
    for_end_3 = 127 as ::core::ffi::c_int as integer;
    if i <= for_end_3 {
        loop {
            mubytecswrite[i as usize] = -(268435455 as ::core::ffi::c_long) as halfword;
            let fresh51 = i;
            i = i + 1;
            if !(fresh51 < for_end_3) {
                break;
            }
        }
    }
    mubytekeep = 0 as ::core::ffi::c_int as integer;
    mubytestart = false_0 as boolean;
    writenoexpanding = false_0 as boolean;
    csconverting = false_0 as boolean;
    specialprinting = false_0 as boolean;
    messageprinting = false_0 as boolean;
    noconvert = false_0 as boolean;
    activenoconvert = false_0 as boolean;
    let mut for_end_4: integer = 0;
    i = 0 as ::core::ffi::c_int as integer;
    for_end_4 = 255 as ::core::ffi::c_int as integer;
    if i <= for_end_4 {
        loop {
            xord[i as usize] = 127 as ASCIIcode;
            let fresh52 = i;
            i = i + 1;
            if !(fresh52 < for_end_4) {
                break;
            }
        }
    }
    let mut for_end_5: integer = 0;
    i = 128 as ::core::ffi::c_int as integer;
    for_end_5 = 255 as ::core::ffi::c_int as integer;
    if i <= for_end_5 {
        loop {
            xord[xchr[i as usize] as usize] = i as ASCIIcode;
            let fresh53 = i;
            i = i + 1;
            if !(fresh53 < for_end_5) {
                break;
            }
        }
    }
    let mut for_end_6: integer = 0;
    i = 0 as ::core::ffi::c_int as integer;
    for_end_6 = 126 as ::core::ffi::c_int as integer;
    if i <= for_end_6 {
        loop {
            xord[xchr[i as usize] as usize] = i as ASCIIcode;
            let fresh54 = i;
            i = i + 1;
            if !(fresh54 < for_end_6) {
                break;
            }
        }
    }
    let mut for_end_7: integer = 0;
    i = 0 as ::core::ffi::c_int as integer;
    for_end_7 = 255 as ::core::ffi::c_int as integer;
    if i <= for_end_7 {
        loop {
            xprn[i as usize] = (eightbitp != 0
                || i >= 32 as ::core::ffi::c_int && i <= 126 as ::core::ffi::c_int)
                as ::core::ffi::c_int as ASCIIcode;
            let fresh55 = i;
            i = i + 1;
            if !(fresh55 < for_end_7) {
                break;
            }
        }
    }
    if !translate_filename.is_null() {
        readtcxfile();
    }
    if interactionoption as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
        interaction = 3 as ::core::ffi::c_uchar;
    } else {
        interaction = interactionoption;
    }
    deletionsallowed = true_0 as boolean;
    setboxallowed = true_0 as boolean;
    errorcount = 0 as schar;
    helpptr = 0 as ::core::ffi::c_uchar;
    useerrhelp = false_0 as boolean;
    interrupt = 0 as ::core::ffi::c_int as integer;
    OKtointerrupt = true_0 as boolean;
    twotothe[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int as integer;
    let mut for_end_8: integer = 0;
    k_0 = 1 as ::core::ffi::c_int as integer;
    for_end_8 = 30 as ::core::ffi::c_int as integer;
    if k_0 <= for_end_8 {
        loop {
            twotothe[k_0 as usize] = 2 as integer
                * twotothe[(k_0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize];
            let fresh56 = k_0;
            k_0 = k_0 + 1;
            if !(fresh56 < for_end_8) {
                break;
            }
        }
    }
    speclog[1 as ::core::ffi::c_int as usize] = 93032640 as ::core::ffi::c_long as integer;
    speclog[2 as ::core::ffi::c_int as usize] = 38612034 as ::core::ffi::c_long as integer;
    speclog[3 as ::core::ffi::c_int as usize] = 17922280 as ::core::ffi::c_long as integer;
    speclog[4 as ::core::ffi::c_int as usize] = 8662214 as ::core::ffi::c_long as integer;
    speclog[5 as ::core::ffi::c_int as usize] = 4261238 as ::core::ffi::c_long as integer;
    speclog[6 as ::core::ffi::c_int as usize] = 2113709 as ::core::ffi::c_long as integer;
    speclog[7 as ::core::ffi::c_int as usize] = 1052693 as ::core::ffi::c_long as integer;
    speclog[8 as ::core::ffi::c_int as usize] = 525315 as ::core::ffi::c_long as integer;
    speclog[9 as ::core::ffi::c_int as usize] = 262400 as ::core::ffi::c_long as integer;
    speclog[10 as ::core::ffi::c_int as usize] = 131136 as ::core::ffi::c_long as integer;
    speclog[11 as ::core::ffi::c_int as usize] = 65552 as ::core::ffi::c_long as integer;
    speclog[12 as ::core::ffi::c_int as usize] = 32772 as ::core::ffi::c_long as integer;
    speclog[13 as ::core::ffi::c_int as usize] = 16385 as ::core::ffi::c_int as integer;
    let mut for_end_9: integer = 0;
    k_0 = 14 as ::core::ffi::c_int as integer;
    for_end_9 = 27 as ::core::ffi::c_int as integer;
    if k_0 <= for_end_9 {
        loop {
            speclog[k_0 as usize] = twotothe[(27 as integer - k_0) as usize];
            let fresh57 = k_0;
            k_0 = k_0 + 1;
            if !(fresh57 < for_end_9) {
                break;
            }
        }
    }
    speclog[28 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int as integer;
    nestptr = 0 as ::core::ffi::c_int as integer;
    maxneststack = 0 as ::core::ffi::c_int as integer;
    curlist.modefield = 1 as ::core::ffi::c_short;
    curlist.headfield = (memtop as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as halfword;
    curlist.tailfield = (memtop as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as halfword;
    curlist.eTeXauxfield = -(268435455 as ::core::ffi::c_long) as halfword;
    savetail = -(268435455 as ::core::ffi::c_long) as halfword;
    curlist.auxfield.u.CINT = -(65536000 as ::core::ffi::c_long) as integer;
    curlist.mlfield = 0 as ::core::ffi::c_int as integer;
    curlist.pgfield = 0 as ::core::ffi::c_int as integer;
    shownmode = 0 as ::core::ffi::c_short;
    pagecontents = 0 as ::core::ffi::c_uchar;
    pagetail = (memtop as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as halfword;
    lastglue = 268435455 as ::core::ffi::c_long as halfword;
    lastpenalty = 0 as ::core::ffi::c_int as integer;
    lastkern = 0 as ::core::ffi::c_int as scaled;
    lastnodetype = -(1 as ::core::ffi::c_int) as integer;
    pagesofar[7 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int as scaled;
    pagemaxdepth = 0 as ::core::ffi::c_int as scaled;
    let mut for_end_10: integer = 0;
    k_0 = 29277 as ::core::ffi::c_int as integer;
    for_end_10 = 30192 as ::core::ffi::c_int as integer;
    if k_0 <= for_end_10 {
        loop {
            *(&raw mut zzzaa as *mut quarterword)
                .offset(-(29277 as ::core::ffi::c_int as isize))
                .offset(k_0 as isize) = 1 as quarterword;
            let fresh58 = k_0;
            k_0 = k_0 + 1;
            if !(fresh58 < for_end_10) {
                break;
            }
        }
    }
    nonewcontrolsequence = true_0 as boolean;
    prim[0 as ::core::ffi::c_int as usize].v.LH = 0 as ::core::ffi::c_int as halfword;
    prim[0 as ::core::ffi::c_int as usize].v.RH = 0 as ::core::ffi::c_int as halfword;
    let mut for_end_11: integer = 0;
    k_0 = 1 as ::core::ffi::c_int as integer;
    for_end_11 = 2100 as ::core::ffi::c_int as integer;
    if k_0 <= for_end_11 {
        loop {
            prim[k_0 as usize] = prim[0 as ::core::ffi::c_int as usize];
            let fresh59 = k_0;
            k_0 = k_0 + 1;
            if !(fresh59 < for_end_11) {
                break;
            }
        }
    }
    saveptr = 0 as ::core::ffi::c_int as integer;
    curlevel = 1 as quarterword;
    curgroup = 0 as groupcode;
    curboundary = 0 as ::core::ffi::c_int as integer;
    maxsavestack = 0 as ::core::ffi::c_int as integer;
    magset = 0 as ::core::ffi::c_int as integer;
    isincsname = false_0 as boolean;
    curmark[0 as ::core::ffi::c_int as usize] = -(268435455 as ::core::ffi::c_long) as halfword;
    curmark[1 as ::core::ffi::c_int as usize] = -(268435455 as ::core::ffi::c_long) as halfword;
    curmark[2 as ::core::ffi::c_int as usize] = -(268435455 as ::core::ffi::c_long) as halfword;
    curmark[3 as ::core::ffi::c_int as usize] = -(268435455 as ::core::ffi::c_long) as halfword;
    curmark[4 as ::core::ffi::c_int as usize] = -(268435455 as ::core::ffi::c_long) as halfword;
    curval = 0 as ::core::ffi::c_int as integer;
    curvallevel = 0 as ::core::ffi::c_uchar;
    radix = 0 as smallnumber;
    curorder = 0 as glueord;
    let mut for_end_12: integer = 0;
    k_0 = 0 as ::core::ffi::c_int as integer;
    for_end_12 = 16 as ::core::ffi::c_int as integer;
    if k_0 <= for_end_12 {
        loop {
            readopen[k_0 as usize] = 2 as ::core::ffi::c_uchar;
            let fresh60 = k_0;
            k_0 = k_0 + 1;
            if !(fresh60 < for_end_12) {
                break;
            }
        }
    }
    condptr = -(268435455 as ::core::ffi::c_long) as halfword;
    iflimit = 0 as ::core::ffi::c_uchar;
    curif = 0 as smallnumber;
    ifline = 0 as ::core::ffi::c_int as integer;
    nullcharacter.u.B0 = 0 as quarterword;
    nullcharacter.u.B1 = 0 as quarterword;
    nullcharacter.u.B2 = 0 as quarterword;
    nullcharacter.u.B3 = 0 as quarterword;
    totalpages = 0 as ::core::ffi::c_int as integer;
    maxv = 0 as ::core::ffi::c_int as scaled;
    maxh = 0 as ::core::ffi::c_int as scaled;
    maxpush = 0 as ::core::ffi::c_int as integer;
    lastbop = -(1 as ::core::ffi::c_int) as integer;
    doingleaders = false_0 as boolean;
    deadcycles = 0 as ::core::ffi::c_int as integer;
    curs = -(1 as ::core::ffi::c_int) as integer;
    halfbuf = (dvibufsize as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as integer;
    dvilimit = dvibufsize;
    dviptr = 0 as ::core::ffi::c_int as integer;
    dvioffset = 0 as ::core::ffi::c_int as integer;
    dvigone = 0 as ::core::ffi::c_int as integer;
    downptr = -(268435455 as ::core::ffi::c_long) as halfword;
    rightptr = -(268435455 as ::core::ffi::c_long) as halfword;
    pdfmemptr = 1 as ::core::ffi::c_int as integer;
    pdfmemsize = infpdfmemsize as integer;
    pdfgone = 0 as longinteger;
    pdfosmode = false_0 as boolean;
    pdfptr = 0 as ::core::ffi::c_int as integer;
    pdfopptr = 0 as ::core::ffi::c_int as integer;
    pdfosptr = 0 as ::core::ffi::c_int as integer;
    pdfoscurobjnum = 0 as ::core::ffi::c_int as integer;
    pdfoscntr = 0 as ::core::ffi::c_int as integer;
    pdfbufsize = pdfopbufsize as integer;
    pdfosbufsize = infpdfosbufsize as integer;
    pdfbuf = pdfopbuf;
    pdfseekwritelength = false_0 as boolean;
    zipwritestate = 0 as ::core::ffi::c_int as integer;
    pdfversionwritten = false_0 as boolean;
    fixedpdfoutputset = false_0 as boolean;
    fixedpdfdraftmodeset = false_0 as boolean;
    onebp = 65782 as ::core::ffi::c_long as scaled;
    onehundredbp = 6578176 as ::core::ffi::c_long as scaled;
    onehundredinch = 473628672 as ::core::ffi::c_long as scaled;
    oneinch = 226 as ::core::ffi::c_int as integer;
    tenpow[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int as integer;
    let mut for_end_13: integer = 0;
    i = 1 as ::core::ffi::c_int as integer;
    for_end_13 = 9 as ::core::ffi::c_int as integer;
    if i <= for_end_13 {
        loop {
            tenpow[i as usize] = 10 as integer
                * tenpow[(i as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize];
            let fresh61 = i;
            i = i + 1;
            if !(fresh61 < for_end_13) {
                break;
            }
        }
    }
    initpdfoutput = false_0 as boolean;
    objptr = 0 as ::core::ffi::c_int as integer;
    sysobjptr = 0 as ::core::ffi::c_int as integer;
    objtabsize = infobjtabsize as integer;
    destnamessize = infdestnamessize as integer;
    let mut for_end_14: integer = 0;
    k_0 = 1 as ::core::ffi::c_int as integer;
    for_end_14 = 10 as ::core::ffi::c_int as integer;
    if k_0 <= for_end_14 {
        loop {
            headtab[k_0 as usize] = 0 as ::core::ffi::c_int as integer;
            let fresh62 = k_0;
            k_0 = k_0 + 1;
            if !(fresh62 < for_end_14) {
                break;
            }
        }
    }
    pdfboxspecmedia = 1 as ::core::ffi::c_int as integer;
    pdfboxspeccrop = 2 as ::core::ffi::c_int as integer;
    pdfboxspecbleed = 3 as ::core::ffi::c_int as integer;
    pdfboxspectrim = 4 as ::core::ffi::c_int as integer;
    pdfboxspecart = 5 as ::core::ffi::c_int as integer;
    pdfdummyfont = 0 as ::core::ffi::c_int as internalfontnumber;
    pdfresnameprefix = 0 as ::core::ffi::c_int as strnumber;
    lasttokensstring = 0 as ::core::ffi::c_int as strnumber;
    vfnf = 0 as ::core::ffi::c_int as internalfontnumber;
    vfcurs = 0 as ::core::ffi::c_int as integer;
    vfstackptr = 0 as ::core::ffi::c_int as vfstackindex;
    adjusttail = -(268435455 as ::core::ffi::c_long) as halfword;
    lastbadness = 0 as ::core::ffi::c_int as integer;
    preadjusttail = -(268435455 as ::core::ffi::c_long) as halfword;
    packbeginline = 0 as ::core::ffi::c_int as integer;
    emptyfield.v.RH = 0 as ::core::ffi::c_int as halfword;
    emptyfield.v.LH = -(268435455 as ::core::ffi::c_long) as halfword;
    nulldelimiter.u.B0 = 0 as quarterword;
    nulldelimiter.u.B1 = 0 as quarterword;
    nulldelimiter.u.B2 = 0 as quarterword;
    nulldelimiter.u.B3 = 0 as quarterword;
    alignptr = -(268435455 as ::core::ffi::c_long) as halfword;
    curalign = -(268435455 as ::core::ffi::c_long) as halfword;
    curspan = -(268435455 as ::core::ffi::c_long) as halfword;
    curloop = -(268435455 as ::core::ffi::c_long) as halfword;
    curhead = -(268435455 as ::core::ffi::c_long) as halfword;
    curtail = -(268435455 as ::core::ffi::c_long) as halfword;
    curprehead = -(268435455 as ::core::ffi::c_long) as halfword;
    curpretail = -(268435455 as ::core::ffi::c_long) as halfword;
    let mut for_end_15: integer = 0;
    z = 0 as hyphpointer;
    for_end_15 = hyphsize;
    if z as ::core::ffi::c_int <= for_end_15 {
        loop {
            *hyphword.offset(z as isize) = 0 as ::core::ffi::c_int as strnumber;
            *hyphlist.offset(z as isize) = -(268435455 as ::core::ffi::c_long) as halfword;
            *hyphlink.offset(z as isize) = 0 as hyphpointer;
            let fresh63 = z;
            z = z.wrapping_add(1);
            if !((fresh63 as ::core::ffi::c_int) < for_end_15) {
                break;
            }
        }
    }
    hyphcount = 0 as ::core::ffi::c_int as integer;
    hyphnext = 608 as ::core::ffi::c_int as integer;
    if hyphnext > hyphsize {
        hyphnext = 607 as ::core::ffi::c_int as integer;
    }
    outputactive = false_0 as boolean;
    outputcanend = false_0 as boolean;
    insertpenalties = 0 as ::core::ffi::c_int as integer;
    ligaturepresent = false_0 as boolean;
    cancelboundary = false_0 as boolean;
    lfthit = false_0 as boolean;
    rthit = false_0 as boolean;
    insdisc = false_0 as boolean;
    aftertoken = 0 as ::core::ffi::c_int as halfword;
    longhelpseen = false_0 as boolean;
    formatident = 0 as ::core::ffi::c_int as strnumber;
    let mut for_end_16: integer = 0;
    k_0 = 0 as ::core::ffi::c_int as integer;
    for_end_16 = 17 as ::core::ffi::c_int as integer;
    if k_0 <= for_end_16 {
        loop {
            writeopen[k_0 as usize] = false_0 as boolean;
            let fresh64 = k_0;
            k_0 = k_0 + 1;
            if !(fresh64 < for_end_16) {
                break;
            }
        }
    }
    altrule = -(268435455 as ::core::ffi::c_long) as halfword;
    warnpdfpagebox = true_0 as boolean;
    countdosnapy = 0 as ::core::ffi::c_int as integer;
    get_seconds_and_micros(&raw mut epochseconds, &raw mut microseconds);
    initstarttime();
    pdffirstoutline = 0 as ::core::ffi::c_int as integer;
    pdflastoutline = 0 as ::core::ffi::c_int as integer;
    pdfparentoutline = 0 as ::core::ffi::c_int as integer;
    pdfobjcount = 0 as ::core::ffi::c_int as integer;
    pdfxformcount = 0 as ::core::ffi::c_int as integer;
    pdfximagecount = 0 as ::core::ffi::c_int as integer;
    pdfdestnamesptr = 0 as ::core::ffi::c_int as integer;
    pdfinfotoks = -(268435455 as ::core::ffi::c_long) as halfword;
    pdfcatalogtoks = -(268435455 as ::core::ffi::c_long) as halfword;
    pdfnamestoks = -(268435455 as ::core::ffi::c_long) as halfword;
    pdfcatalogopenaction = 0 as ::core::ffi::c_int as integer;
    pdftrailertoks = -(268435455 as ::core::ffi::c_long) as halfword;
    pdftraileridtoks = -(268435455 as ::core::ffi::c_long) as halfword;
    genfakedinterwordspace = false_0 as boolean;
    genrunninglink = true_0 as boolean;
    pdfspacefontname = 1948 as ::core::ffi::c_int as strnumber;
    pdflinkstackptr = 0 as smallnumber;
    LRptr = -(268435455 as ::core::ffi::c_long) as halfword;
    LRproblems = 0 as ::core::ffi::c_int as integer;
    curdir = 0 as smallnumber;
    pseudofiles = -(268435455 as ::core::ffi::c_long) as halfword;
    saroot[6 as ::core::ffi::c_int as usize] = -(268435455 as ::core::ffi::c_long) as halfword;
    sanull.hh.v.LH = -(268435455 as ::core::ffi::c_long) as halfword;
    sanull.hh.v.RH = -(268435455 as ::core::ffi::c_long) as halfword;
    sachain = -(268435455 as ::core::ffi::c_long) as halfword;
    salevel = 0 as quarterword;
    discptr[2 as ::core::ffi::c_int as usize] = -(268435455 as ::core::ffi::c_long) as halfword;
    discptr[3 as ::core::ffi::c_int as usize] = -(268435455 as ::core::ffi::c_long) as halfword;
    editnamestart = 0 as ::core::ffi::c_int as poolpointer;
    stopatspace = true_0 as boolean;
    haltingonerrorp = false_0 as boolean;
    expanddepthcount = 0 as ::core::ffi::c_int as integer;
    mltexenabledp = false_0 as boolean;
    enctexenabledp = false_0 as boolean;
    if iniversion != 0 {
        let mut for_end_17: integer = 0;
        k_0 = (membot as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as integer;
        for_end_17 = (membot as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as integer;
        if k_0 <= for_end_17 {
            loop {
                (*mem.offset(k_0 as isize)).u.CINT = 0 as ::core::ffi::c_int as integer;
                let fresh65 = k_0;
                k_0 = k_0 + 1;
                if !(fresh65 < for_end_17) {
                    break;
                }
            }
        }
        k_0 = membot;
        while k_0 <= membot as ::core::ffi::c_int + 19 as ::core::ffi::c_int {
            (*mem.offset(k_0 as isize)).hh.v.RH = -(268435454 as ::core::ffi::c_long) as halfword;
            (*mem.offset(k_0 as isize)).hh.u.B0 = 0 as ::core::ffi::c_short;
            (*mem.offset(k_0 as isize)).hh.u.B1 = 0 as ::core::ffi::c_short;
            k_0 = (k_0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as integer;
        }
        (*mem.offset((membot as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize))
            .u
            .CINT = 65536 as ::core::ffi::c_long as integer;
        (*mem.offset((membot as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize))
            .hh
            .u
            .B0 = 1 as ::core::ffi::c_short;
        (*mem.offset((membot as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize))
            .u
            .CINT = 65536 as ::core::ffi::c_long as integer;
        (*mem.offset((membot as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize))
            .hh
            .u
            .B0 = 2 as ::core::ffi::c_short;
        (*mem.offset((membot as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize))
            .u
            .CINT = 65536 as ::core::ffi::c_long as integer;
        (*mem.offset((membot as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize))
            .hh
            .u
            .B0 = 1 as ::core::ffi::c_short;
        (*mem.offset((membot as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize))
            .u
            .CINT = 65536 as ::core::ffi::c_long as integer;
        (*mem.offset((membot as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize))
            .hh
            .u
            .B1 = 1 as ::core::ffi::c_short;
        (*mem.offset((membot as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize))
            .u
            .CINT = -(65536 as ::core::ffi::c_long) as integer;
        (*mem.offset((membot as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize))
            .hh
            .u
            .B0 = 1 as ::core::ffi::c_short;
        rover = (membot as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as halfword;
        (*mem.offset(rover as isize)).hh.v.RH = 268435455 as ::core::ffi::c_long as halfword;
        (*mem.offset(rover as isize)).hh.v.LH = 1000 as ::core::ffi::c_int as halfword;
        (*mem.offset((rover as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
            .hh
            .v
            .LH = rover;
        (*mem.offset((rover as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
            .hh
            .v
            .RH = rover;
        lomemmax = (rover as ::core::ffi::c_int + 1000 as ::core::ffi::c_int) as halfword;
        (*mem.offset(lomemmax as isize)).hh.v.RH = -(268435455 as ::core::ffi::c_long) as halfword;
        (*mem.offset(lomemmax as isize)).hh.v.LH = -(268435455 as ::core::ffi::c_long) as halfword;
        let mut for_end_18: integer = 0;
        k_0 = (memtop as ::core::ffi::c_int - 14 as ::core::ffi::c_int) as integer;
        for_end_18 = memtop;
        if k_0 <= for_end_18 {
            loop {
                *mem.offset(k_0 as isize) = *mem.offset(lomemmax as isize);
                let fresh66 = k_0;
                k_0 = k_0 + 1;
                if !(fresh66 < for_end_18) {
                    break;
                }
            }
        }
        (*mem.offset((memtop as ::core::ffi::c_int - 10 as ::core::ffi::c_int) as isize))
            .hh
            .v
            .LH = 19614 as ::core::ffi::c_int as halfword;
        (*mem.offset((memtop as ::core::ffi::c_int - 9 as ::core::ffi::c_int) as isize))
            .hh
            .v
            .RH = 256 as ::core::ffi::c_int as halfword;
        (*mem.offset((memtop as ::core::ffi::c_int - 9 as ::core::ffi::c_int) as isize))
            .hh
            .v
            .LH = -(268435455 as ::core::ffi::c_long) as halfword;
        (*mem.offset((memtop as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as isize))
            .hh
            .u
            .B0 = 1 as ::core::ffi::c_short;
        (*mem.offset((memtop as ::core::ffi::c_int - 6 as ::core::ffi::c_int) as isize))
            .hh
            .v
            .LH = 268435455 as ::core::ffi::c_long as halfword;
        (*mem.offset((memtop as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as isize))
            .hh
            .u
            .B1 = 0 as ::core::ffi::c_short;
        (*mem.offset(memtop as isize)).hh.u.B1 = 255 as ::core::ffi::c_short;
        (*mem.offset(memtop as isize)).hh.u.B0 = 1 as ::core::ffi::c_short;
        (*mem.offset(memtop as isize)).hh.v.RH = memtop as halfword;
        (*mem.offset((memtop as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize))
            .hh
            .u
            .B0 = 10 as ::core::ffi::c_short;
        (*mem.offset((memtop as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize))
            .hh
            .u
            .B1 = 0 as ::core::ffi::c_short;
        avail = -(268435455 as ::core::ffi::c_long) as halfword;
        memend = memtop as halfword;
        himemmin = (memtop as ::core::ffi::c_int - 14 as ::core::ffi::c_int) as halfword;
        varused = membot + 20 as integer - membot;
        dynused = 15 as ::core::ffi::c_int as integer;
        (*eqtb.offset(26627 as ::core::ffi::c_int as isize)).hh.u.B0 = 104 as ::core::ffi::c_short;
        (*eqtb.offset(26627 as ::core::ffi::c_int as isize)).hh.v.RH =
            -(268435455 as ::core::ffi::c_long) as halfword;
        (*eqtb.offset(26627 as ::core::ffi::c_int as isize)).hh.u.B1 = 0 as ::core::ffi::c_short;
        let mut for_end_19: integer = 0;
        k_0 = 1 as ::core::ffi::c_int as integer;
        for_end_19 = eqtbtop as integer;
        if k_0 <= for_end_19 {
            loop {
                *eqtb.offset(k_0 as isize) = *eqtb.offset(26627 as ::core::ffi::c_int as isize);
                let fresh67 = k_0;
                k_0 = k_0 + 1;
                if !(fresh67 < for_end_19) {
                    break;
                }
            }
        }
        (*eqtb.offset(26628 as ::core::ffi::c_int as isize)).hh.v.RH = membot as halfword;
        (*eqtb.offset(26628 as ::core::ffi::c_int as isize)).hh.u.B1 = 1 as ::core::ffi::c_short;
        (*eqtb.offset(26628 as ::core::ffi::c_int as isize)).hh.u.B0 = 120 as ::core::ffi::c_short;
        let mut for_end_20: integer = 0;
        k_0 = 26629 as ::core::ffi::c_int as integer;
        for_end_20 = 27157 as ::core::ffi::c_int as integer;
        if k_0 <= for_end_20 {
            loop {
                *eqtb.offset(k_0 as isize) = *eqtb.offset(26628 as ::core::ffi::c_int as isize);
                let fresh68 = k_0;
                k_0 = k_0 + 1;
                if !(fresh68 < for_end_20) {
                    break;
                }
            }
        }
        (*mem.offset(membot as isize)).hh.v.RH =
            ((*mem.offset(membot as isize)).hh.v.RH as ::core::ffi::c_int
                + 530 as ::core::ffi::c_int) as halfword;
        (*eqtb.offset(27158 as ::core::ffi::c_int as isize)).hh.v.RH =
            -(268435455 as ::core::ffi::c_long) as halfword;
        (*eqtb.offset(27158 as ::core::ffi::c_int as isize)).hh.u.B0 = 121 as ::core::ffi::c_short;
        (*eqtb.offset(27158 as ::core::ffi::c_int as isize)).hh.u.B1 = 1 as ::core::ffi::c_short;
        let mut for_end_21: integer = 0;
        k_0 = 27429 as ::core::ffi::c_int as integer;
        for_end_21 = 27432 as ::core::ffi::c_int as integer;
        if k_0 <= for_end_21 {
            loop {
                *eqtb.offset(k_0 as isize) = *eqtb.offset(27158 as ::core::ffi::c_int as isize);
                let fresh69 = k_0;
                k_0 = k_0 + 1;
                if !(fresh69 < for_end_21) {
                    break;
                }
            }
        }
        let mut for_end_22: integer = 0;
        k_0 = 27159 as ::core::ffi::c_int as integer;
        for_end_22 = 27428 as ::core::ffi::c_int as integer;
        if k_0 <= for_end_22 {
            loop {
                *eqtb.offset(k_0 as isize) = *eqtb.offset(26627 as ::core::ffi::c_int as isize);
                let fresh70 = k_0;
                k_0 = k_0 + 1;
                if !(fresh70 < for_end_22) {
                    break;
                }
            }
        }
        (*eqtb.offset(27433 as ::core::ffi::c_int as isize)).hh.v.RH =
            -(268435455 as ::core::ffi::c_long) as halfword;
        (*eqtb.offset(27433 as ::core::ffi::c_int as isize)).hh.u.B0 = 122 as ::core::ffi::c_short;
        (*eqtb.offset(27433 as ::core::ffi::c_int as isize)).hh.u.B1 = 1 as ::core::ffi::c_short;
        let mut for_end_23: integer = 0;
        k_0 = 27434 as ::core::ffi::c_int as integer;
        for_end_23 = 27688 as ::core::ffi::c_int as integer;
        if k_0 <= for_end_23 {
            loop {
                *eqtb.offset(k_0 as isize) = *eqtb.offset(27433 as ::core::ffi::c_int as isize);
                let fresh71 = k_0;
                k_0 = k_0 + 1;
                if !(fresh71 < for_end_23) {
                    break;
                }
            }
        }
        (*eqtb.offset(27689 as ::core::ffi::c_int as isize)).hh.v.RH =
            0 as ::core::ffi::c_int as halfword;
        (*eqtb.offset(27689 as ::core::ffi::c_int as isize)).hh.u.B0 = 123 as ::core::ffi::c_short;
        (*eqtb.offset(27689 as ::core::ffi::c_int as isize)).hh.u.B1 = 1 as ::core::ffi::c_short;
        let mut for_end_24: integer = 0;
        k_0 = 27693 as ::core::ffi::c_int as integer;
        for_end_24 = 27740 as ::core::ffi::c_int as integer;
        if k_0 <= for_end_24 {
            loop {
                *eqtb.offset(k_0 as isize) = *eqtb.offset(27689 as ::core::ffi::c_int as isize);
                let fresh72 = k_0;
                k_0 = k_0 + 1;
                if !(fresh72 < for_end_24) {
                    break;
                }
            }
        }
        (*eqtb.offset(27741 as ::core::ffi::c_int as isize)).hh.v.RH =
            0 as ::core::ffi::c_int as halfword;
        (*eqtb.offset(27741 as ::core::ffi::c_int as isize)).hh.u.B0 = 123 as ::core::ffi::c_short;
        (*eqtb.offset(27741 as ::core::ffi::c_int as isize)).hh.u.B1 = 1 as ::core::ffi::c_short;
        let mut for_end_25: integer = 0;
        k_0 = 27742 as ::core::ffi::c_int as integer;
        for_end_25 = 29276 as ::core::ffi::c_int as integer;
        if k_0 <= for_end_25 {
            loop {
                *eqtb.offset(k_0 as isize) = *eqtb.offset(27741 as ::core::ffi::c_int as isize);
                let fresh73 = k_0;
                k_0 = k_0 + 1;
                if !(fresh73 < for_end_25) {
                    break;
                }
            }
        }
        let mut for_end_26: integer = 0;
        k_0 = 0 as ::core::ffi::c_int as integer;
        for_end_26 = 255 as ::core::ffi::c_int as integer;
        if k_0 <= for_end_26 {
            loop {
                (*eqtb.offset((27741 as integer + k_0) as isize)).hh.v.RH =
                    12 as ::core::ffi::c_int as halfword;
                (*eqtb.offset((28765 as integer + k_0) as isize)).hh.v.RH = k_0 as halfword;
                (*eqtb.offset((28509 as integer + k_0) as isize)).hh.v.RH =
                    1000 as ::core::ffi::c_int as halfword;
                let fresh74 = k_0;
                k_0 = k_0 + 1;
                if !(fresh74 < for_end_26) {
                    break;
                }
            }
        }
        (*eqtb.offset(27754 as ::core::ffi::c_int as isize)).hh.v.RH =
            5 as ::core::ffi::c_int as halfword;
        (*eqtb.offset(27773 as ::core::ffi::c_int as isize)).hh.v.RH =
            10 as ::core::ffi::c_int as halfword;
        (*eqtb.offset(27833 as ::core::ffi::c_int as isize)).hh.v.RH =
            0 as ::core::ffi::c_int as halfword;
        (*eqtb.offset(27778 as ::core::ffi::c_int as isize)).hh.v.RH =
            14 as ::core::ffi::c_int as halfword;
        (*eqtb.offset(27868 as ::core::ffi::c_int as isize)).hh.v.RH =
            15 as ::core::ffi::c_int as halfword;
        (*eqtb.offset(27741 as ::core::ffi::c_int as isize)).hh.v.RH =
            9 as ::core::ffi::c_int as halfword;
        let mut for_end_27: integer = 0;
        k_0 = 48 as ::core::ffi::c_int as integer;
        for_end_27 = 57 as ::core::ffi::c_int as integer;
        if k_0 <= for_end_27 {
            loop {
                (*eqtb.offset((28765 as integer + k_0) as isize)).hh.v.RH =
                    (k_0 as ::core::ffi::c_int + 28672 as ::core::ffi::c_int) as halfword;
                let fresh75 = k_0;
                k_0 = k_0 + 1;
                if !(fresh75 < for_end_27) {
                    break;
                }
            }
        }
        let mut for_end_28: integer = 0;
        k_0 = 65 as ::core::ffi::c_int as integer;
        for_end_28 = 90 as ::core::ffi::c_int as integer;
        if k_0 <= for_end_28 {
            loop {
                (*eqtb.offset((27741 as integer + k_0) as isize)).hh.v.RH =
                    11 as ::core::ffi::c_int as halfword;
                (*eqtb.offset(
                    (27741 as ::core::ffi::c_int
                        + k_0 as ::core::ffi::c_int
                        + 32 as ::core::ffi::c_int) as isize,
                ))
                .hh
                .v
                .RH = 11 as ::core::ffi::c_int as halfword;
                (*eqtb.offset((28765 as integer + k_0) as isize)).hh.v.RH =
                    (k_0 as ::core::ffi::c_int + 28928 as ::core::ffi::c_int) as halfword;
                (*eqtb.offset(
                    (28765 as ::core::ffi::c_int
                        + k_0 as ::core::ffi::c_int
                        + 32 as ::core::ffi::c_int) as isize,
                ))
                .hh
                .v
                .RH = (k_0 as ::core::ffi::c_int + 28960 as ::core::ffi::c_int) as halfword;
                (*eqtb.offset((27997 as integer + k_0) as isize)).hh.v.RH =
                    (k_0 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as halfword;
                (*eqtb.offset(
                    (27997 as ::core::ffi::c_int
                        + k_0 as ::core::ffi::c_int
                        + 32 as ::core::ffi::c_int) as isize,
                ))
                .hh
                .v
                .RH = (k_0 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as halfword;
                (*eqtb.offset((28253 as integer + k_0) as isize)).hh.v.RH = k_0 as halfword;
                (*eqtb.offset(
                    (28253 as ::core::ffi::c_int
                        + k_0 as ::core::ffi::c_int
                        + 32 as ::core::ffi::c_int) as isize,
                ))
                .hh
                .v
                .RH = k_0 as halfword;
                (*eqtb.offset((28509 as integer + k_0) as isize)).hh.v.RH =
                    999 as ::core::ffi::c_int as halfword;
                let fresh76 = k_0;
                k_0 = k_0 + 1;
                if !(fresh76 < for_end_28) {
                    break;
                }
            }
        }
        let mut for_end_29: integer = 0;
        k_0 = 29277 as ::core::ffi::c_int as integer;
        for_end_29 = 29646 as ::core::ffi::c_int as integer;
        if k_0 <= for_end_29 {
            loop {
                (*eqtb.offset(k_0 as isize)).u.CINT = 0 as ::core::ffi::c_int as integer;
                let fresh77 = k_0;
                k_0 = k_0 + 1;
                if !(fresh77 < for_end_29) {
                    break;
                }
            }
        }
        (*eqtb.offset(29332 as ::core::ffi::c_int as isize)).u.CINT =
            256 as ::core::ffi::c_int as integer;
        (*eqtb.offset(29333 as ::core::ffi::c_int as isize)).u.CINT =
            -(1 as ::core::ffi::c_int) as integer;
        (*eqtb.offset(29294 as ::core::ffi::c_int as isize)).u.CINT =
            1000 as ::core::ffi::c_int as integer;
        (*eqtb.offset(29278 as ::core::ffi::c_int as isize)).u.CINT =
            10000 as ::core::ffi::c_int as integer;
        (*eqtb.offset(29318 as ::core::ffi::c_int as isize)).u.CINT =
            1 as ::core::ffi::c_int as integer;
        (*eqtb.offset(29317 as ::core::ffi::c_int as isize)).u.CINT =
            25 as ::core::ffi::c_int as integer;
        (*eqtb.offset(29322 as ::core::ffi::c_int as isize)).u.CINT =
            92 as ::core::ffi::c_int as integer;
        (*eqtb.offset(29325 as ::core::ffi::c_int as isize)).u.CINT =
            13 as ::core::ffi::c_int as integer;
        let mut for_end_30: integer = 0;
        k_0 = 0 as ::core::ffi::c_int as integer;
        for_end_30 = 255 as ::core::ffi::c_int as integer;
        if k_0 <= for_end_30 {
            loop {
                (*eqtb.offset((29647 as integer + k_0) as isize)).u.CINT =
                    -(1 as ::core::ffi::c_int) as integer;
                let fresh78 = k_0;
                k_0 = k_0 + 1;
                if !(fresh78 < for_end_30) {
                    break;
                }
            }
        }
        (*eqtb.offset(29693 as ::core::ffi::c_int as isize)).u.CINT =
            0 as ::core::ffi::c_int as integer;
        (*eqtb.offset(29337 as ::core::ffi::c_int as isize)).u.CINT =
            -(1 as ::core::ffi::c_int) as integer;
        let mut for_end_31: integer = 0;
        k_0 = 29903 as ::core::ffi::c_int as integer;
        for_end_31 = 30192 as ::core::ffi::c_int as integer;
        if k_0 <= for_end_31 {
            loop {
                (*eqtb.offset(k_0 as isize)).u.CINT = 0 as ::core::ffi::c_int as integer;
                let fresh79 = k_0;
                k_0 = k_0 + 1;
                if !(fresh79 < for_end_31) {
                    break;
                }
            }
        }
        primused = 2100 as ::core::ffi::c_int as halfword;
        hashused = 15514 as ::core::ffi::c_int as halfword;
        hashhigh = 0 as ::core::ffi::c_int as halfword;
        cscount = 0 as ::core::ffi::c_int as integer;
        (*eqtb.offset(15523 as ::core::ffi::c_int as isize)).hh.u.B0 = 119 as ::core::ffi::c_short;
        (*hash.offset(15523 as ::core::ffi::c_int as isize)).v.RH =
            585 as ::core::ffi::c_int as halfword;
        (*eqtb.offset(15525 as ::core::ffi::c_int as isize)).hh.u.B0 = 39 as ::core::ffi::c_short;
        (*eqtb.offset(15525 as ::core::ffi::c_int as isize)).hh.v.RH =
            1 as ::core::ffi::c_int as halfword;
        (*eqtb.offset(15525 as ::core::ffi::c_int as isize)).hh.u.B1 = 1 as ::core::ffi::c_short;
        (*hash.offset(15525 as ::core::ffi::c_int as isize)).v.RH =
            586 as ::core::ffi::c_int as halfword;
        (*eqtb.offset(29924 as ::core::ffi::c_int as isize)).u.CINT =
            ((onehundredinch as ::core::ffi::c_int + 50 as ::core::ffi::c_int)
                / 100 as ::core::ffi::c_int) as integer;
        (*eqtb.offset(29925 as ::core::ffi::c_int as isize)).u.CINT =
            ((onehundredinch as ::core::ffi::c_int + 50 as ::core::ffi::c_int)
                / 100 as ::core::ffi::c_int) as integer;
        (*eqtb.offset(29343 as ::core::ffi::c_int as isize)).u.CINT =
            9 as ::core::ffi::c_int as integer;
        (*eqtb.offset(29363 as ::core::ffi::c_int as isize)).u.CINT =
            0 as ::core::ffi::c_int as integer;
        (*eqtb.offset(29344 as ::core::ffi::c_int as isize)).u.CINT =
            3 as ::core::ffi::c_int as integer;
        (*eqtb.offset(29346 as ::core::ffi::c_int as isize)).u.CINT =
            72 as ::core::ffi::c_int as integer;
        (*eqtb.offset(29351 as ::core::ffi::c_int as isize)).u.CINT =
            1 as ::core::ffi::c_int as integer;
        (*eqtb.offset(29352 as ::core::ffi::c_int as isize)).u.CINT =
            4 as ::core::ffi::c_int as integer;
        (*eqtb.offset(29356 as ::core::ffi::c_int as isize)).u.CINT =
            1000 as ::core::ffi::c_int as integer;
        (*eqtb.offset(29357 as ::core::ffi::c_int as isize)).u.CINT =
            2200 as ::core::ffi::c_int as integer;
        (*eqtb.offset(29358 as ::core::ffi::c_int as isize)).u.CINT =
            1 as ::core::ffi::c_int as integer;
        (*eqtb.offset(29359 as ::core::ffi::c_int as isize)).u.CINT =
            0 as ::core::ffi::c_int as integer;
        (*eqtb.offset(29936 as ::core::ffi::c_int as isize)).u.CINT = onebp as integer;
        (*eqtb.offset(29368 as ::core::ffi::c_int as isize)).u.CINT =
            0 as ::core::ffi::c_int as integer;
        (*eqtb.offset(29935 as ::core::ffi::c_int as isize)).u.CINT =
            -(65536000 as ::core::ffi::c_long) as integer;
        (*eqtb.offset(29933 as ::core::ffi::c_int as isize)).u.CINT =
            (*eqtb.offset(29935 as ::core::ffi::c_int as isize)).u.CINT;
        (*eqtb.offset(29934 as ::core::ffi::c_int as isize)).u.CINT =
            (*eqtb.offset(29935 as ::core::ffi::c_int as isize)).u.CINT;
        (*eqtb.offset(29931 as ::core::ffi::c_int as isize)).u.CINT =
            (*eqtb.offset(29935 as ::core::ffi::c_int as isize)).u.CINT;
        (*eqtb.offset(29932 as ::core::ffi::c_int as isize)).u.CINT =
            (*eqtb.offset(29935 as ::core::ffi::c_int as isize)).u.CINT;
        let mut for_end_32: integer = 0;
        k_0 = -(trieopsize as integer);
        for_end_32 = trieopsize as integer;
        if k_0 <= for_end_32 {
            loop {
                *(&raw mut zzzab as *mut integer)
                    .offset(-(-(35111 as ::core::ffi::c_long) as ::core::ffi::c_int as isize))
                    .offset(k_0 as isize) = 0 as ::core::ffi::c_int as integer;
                let fresh80 = k_0;
                k_0 = k_0 + 1;
                if !(fresh80 < for_end_32) {
                    break;
                }
            }
        }
        let mut for_end_33: integer = 0;
        k_0 = 0 as ::core::ffi::c_int as integer;
        for_end_33 = 255 as ::core::ffi::c_int as integer;
        if k_0 <= for_end_33 {
            loop {
                trieused[k_0 as usize] = mintrieop as trieopcode;
                let fresh81 = k_0;
                k_0 = k_0 + 1;
                if !(fresh81 < for_end_33) {
                    break;
                }
            }
        }
        maxopused = mintrieop as trieopcode;
        trieopptr = 0 as ::core::ffi::c_int as integer;
        trienotready = true_0 as boolean;
        (*hash.offset(15514 as ::core::ffi::c_int as isize)).v.RH =
            1623 as ::core::ffi::c_int as halfword;
        if iniversion != 0 {
            formatident = 1710 as ::core::ffi::c_int as strnumber;
        }
        (*hash.offset(15522 as ::core::ffi::c_int as isize)).v.RH =
            1933 as ::core::ffi::c_int as halfword;
        (*eqtb.offset(15522 as ::core::ffi::c_int as isize)).hh.u.B1 = 1 as ::core::ffi::c_short;
        (*eqtb.offset(15522 as ::core::ffi::c_int as isize)).hh.u.B0 = 116 as ::core::ffi::c_short;
        (*eqtb.offset(15522 as ::core::ffi::c_int as isize)).hh.v.RH =
            -(268435455 as ::core::ffi::c_long) as halfword;
        eTeXmode = 0 as ::core::ffi::c_uchar;
        maxregnum = 255 as ::core::ffi::c_int as halfword;
        maxreghelpline = 800 as ::core::ffi::c_int as strnumber;
        let mut for_end_34: integer = 0;
        i = 0 as ::core::ffi::c_int as integer;
        for_end_34 = 5 as ::core::ffi::c_int as integer;
        if i <= for_end_34 {
            loop {
                saroot[i as usize] = -(268435455 as ::core::ffi::c_long) as halfword;
                let fresh82 = i;
                i = i + 1;
                if !(fresh82 < for_end_34) {
                    break;
                }
            }
        }
    }
    synctexoffset = 29390 as ::core::ffi::c_int as integer;
}
#[no_mangle]
pub unsafe extern "C" fn getstringsstarted() -> boolean {
    let mut Result: boolean = 0;
    let mut k_0: ::core::ffi::c_uchar = 0;
    let mut l: ::core::ffi::c_uchar = 0;
    let mut g: strnumber = 0;
    poolptr = 0 as ::core::ffi::c_int as integer;
    strptr = 0 as ::core::ffi::c_int as strnumber;
    *strstart.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int as poolpointer;
    let mut for_end: integer = 0;
    k_0 = 0 as ::core::ffi::c_uchar;
    for_end = 255 as ::core::ffi::c_int as integer;
    if k_0 as ::core::ffi::c_int <= for_end {
        loop {
            if (k_0 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
                || k_0 as ::core::ffi::c_int > 126 as ::core::ffi::c_int
            {
                *strpool.offset(poolptr as isize) = 94 as packedASCIIcode;
                poolptr += 1;
                *strpool.offset(poolptr as isize) = 94 as packedASCIIcode;
                poolptr += 1;
                if (k_0 as ::core::ffi::c_int) < 64 as ::core::ffi::c_int {
                    *strpool.offset(poolptr as isize) =
                        (k_0 as ::core::ffi::c_int + 64 as ::core::ffi::c_int) as packedASCIIcode;
                    poolptr += 1;
                } else if (k_0 as ::core::ffi::c_int) < 128 as ::core::ffi::c_int {
                    *strpool.offset(poolptr as isize) =
                        (k_0 as ::core::ffi::c_int - 64 as ::core::ffi::c_int) as packedASCIIcode;
                    poolptr += 1;
                } else {
                    l = (k_0 as ::core::ffi::c_int / 16 as ::core::ffi::c_int)
                        as ::core::ffi::c_uchar;
                    if (l as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
                        *strpool.offset(poolptr as isize) =
                            (l as ::core::ffi::c_int + 48 as ::core::ffi::c_int) as packedASCIIcode;
                        poolptr += 1;
                    } else {
                        *strpool.offset(poolptr as isize) =
                            (l as ::core::ffi::c_int + 87 as ::core::ffi::c_int) as packedASCIIcode;
                        poolptr += 1;
                    }
                    l = (k_0 as ::core::ffi::c_int % 16 as ::core::ffi::c_int)
                        as ::core::ffi::c_uchar;
                    if (l as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
                        *strpool.offset(poolptr as isize) =
                            (l as ::core::ffi::c_int + 48 as ::core::ffi::c_int) as packedASCIIcode;
                        poolptr += 1;
                    } else {
                        *strpool.offset(poolptr as isize) =
                            (l as ::core::ffi::c_int + 87 as ::core::ffi::c_int) as packedASCIIcode;
                        poolptr += 1;
                    }
                }
            } else {
                *strpool.offset(poolptr as isize) = k_0 as packedASCIIcode;
                poolptr += 1;
            }
            g = makestring();
            let fresh84 = k_0;
            k_0 = k_0.wrapping_add(1);
            if !((fresh84 as ::core::ffi::c_int) < for_end) {
                break;
            }
        }
    }
    g = loadpoolstrings(poolsize - stringvacancies) as strnumber;
    if g == 0 as ::core::ffi::c_int {
        fprintf(
            __stdoutp,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"! You have to increase POOLSIZE.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        Result = false_0 as boolean;
        return Result;
    }
    Result = true_0 as boolean;
    return Result;
}
#[no_mangle]
pub unsafe extern "C" fn sortavail() {
    let mut mem: *mut memoryword = zmem;
    let mut p: halfword = 0;
    let mut q: halfword = 0;
    let mut r: halfword = 0;
    let mut oldrover: halfword = 0;
    p = zgetnode(1073741824 as ::core::ffi::c_long as integer);
    p = (*mem.offset((rover as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
        .hh
        .v
        .RH;
    (*mem.offset((rover as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
        .hh
        .v
        .RH = 268435455 as ::core::ffi::c_long as halfword;
    oldrover = rover;
    while p != oldrover {
        if p < rover {
            q = p;
            p = (*mem.offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                .hh
                .v
                .RH;
            (*mem.offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                .hh
                .v
                .RH = rover;
            rover = q;
        } else {
            q = rover;
            while (*mem.offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                .hh
                .v
                .RH
                < p
            {
                q = (*mem.offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                    .hh
                    .v
                    .RH;
            }
            r = (*mem.offset((p as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                .hh
                .v
                .RH;
            (*mem.offset((p as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                .hh
                .v
                .RH = (*mem.offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                .hh
                .v
                .RH;
            (*mem.offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                .hh
                .v
                .RH = p;
            p = r;
        }
    }
    p = rover;
    while (*mem.offset((p as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
        .hh
        .v
        .RH as ::core::ffi::c_long
        != 268435455 as ::core::ffi::c_long
    {
        (*mem.offset(
            ((*mem.offset((p as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                .hh
                .v
                .RH as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as isize,
        ))
        .hh
        .v
        .LH = p;
        p = (*mem.offset((p as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
            .hh
            .v
            .RH;
    }
    (*mem.offset((p as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
        .hh
        .v
        .RH = rover;
    (*mem.offset((rover as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
        .hh
        .v
        .LH = p;
}
#[no_mangle]
pub unsafe extern "C" fn zprimitive(mut s: strnumber, mut c: quarterword, mut o: halfword) {
    let mut eqtb: *mut memoryword = zeqtb;
    let mut k_0: poolpointer = 0;
    let mut j: integer = 0;
    let mut l: smallnumber = 0;
    let mut primval: integer = 0;
    if s < 256 as ::core::ffi::c_int {
        curval = (s as ::core::ffi::c_int + 257 as ::core::ffi::c_int) as integer;
        primval = zprimlookup(s) as integer;
    } else {
        k_0 = *strstart.offset(s as isize);
        l = (*strstart.offset((s as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) - k_0)
            as smallnumber;
        if first as ::core::ffi::c_int + l as ::core::ffi::c_int
            > bufsize as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        {
            zoverflow(258 as ::core::ffi::c_int, bufsize);
        }
        let mut for_end: integer = 0;
        j = 0 as ::core::ffi::c_int as integer;
        for_end = (l as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as integer;
        if j <= for_end {
            loop {
                *buffer.offset((first + j) as isize) =
                    *strpool.offset((k_0 as integer + j) as isize) as ASCIIcode;
                let fresh83 = j;
                j = j + 1;
                if !(fresh83 < for_end) {
                    break;
                }
            }
        }
        curval = zidlookup(first, l as integer) as integer;
        strptr -= 1;
        poolptr = *strstart.offset(strptr as isize) as integer;
        (*hash.offset(curval as isize)).v.RH = s as halfword;
        primval = zprimlookup(s) as integer;
    }
    (*eqtb.offset(curval as isize)).hh.u.B1 = 1 as ::core::ffi::c_short;
    (*eqtb.offset(curval as isize)).hh.u.B0 = c as ::core::ffi::c_short;
    (*eqtb.offset(curval as isize)).hh.v.RH = o;
    (*eqtb.offset((15526 as integer + primval) as isize))
        .hh
        .u
        .B1 = 1 as ::core::ffi::c_short;
    (*eqtb.offset((15526 as integer + primval) as isize))
        .hh
        .u
        .B0 = c as ::core::ffi::c_short;
    (*eqtb.offset((15526 as integer + primval) as isize))
        .hh
        .v
        .RH = o;
}
#[no_mangle]
pub unsafe extern "C" fn znewtrieop(
    mut d: smallnumber,
    mut n: smallnumber,
    mut v: trieopcode,
) -> trieopcode {
    let mut Result: trieopcode = 0;
    let mut h: integer = 0;
    let mut u: trieopcode = 0;
    let mut l: integer = 0;
    h = ((if n as ::core::ffi::c_int
        + 313 as ::core::ffi::c_int * d as ::core::ffi::c_int
        + 361 as ::core::ffi::c_int * v as ::core::ffi::c_int
        + 1009 as ::core::ffi::c_int * curlang as ::core::ffi::c_int
        >= 0 as ::core::ffi::c_int
    {
        n as ::core::ffi::c_int
            + 313 as ::core::ffi::c_int * d as ::core::ffi::c_int
            + 361 as ::core::ffi::c_int * v as ::core::ffi::c_int
            + 1009 as ::core::ffi::c_int * curlang as ::core::ffi::c_int
    } else {
        -(n as ::core::ffi::c_int
            + 313 as ::core::ffi::c_int * d as ::core::ffi::c_int
            + 361 as ::core::ffi::c_int * v as ::core::ffi::c_int
            + 1009 as ::core::ffi::c_int * curlang as ::core::ffi::c_int)
    }) as ::core::ffi::c_long
        % (trieopsize - negtrieopsize)
        + negtrieopsize) as integer;
    loop {
        l = *(&raw mut zzzab as *mut integer)
            .offset(-(-(35111 as ::core::ffi::c_long) as ::core::ffi::c_int as isize))
            .offset(h as isize);
        if l == 0 as ::core::ffi::c_int {
            if trieopptr as ::core::ffi::c_long == trieopsize {
                zoverflow(
                    1376 as ::core::ffi::c_int,
                    35111 as ::core::ffi::c_long as integer,
                );
            }
            u = trieused[curlang as usize];
            if u as ::core::ffi::c_long == maxtrieop {
                zoverflow(
                    1377 as ::core::ffi::c_int,
                    (65535 as ::core::ffi::c_long - 0 as ::core::ffi::c_long) as integer,
                );
            }
            trieopptr += 1;
            u = u.wrapping_add(1);
            trieused[curlang as usize] = u;
            if u as ::core::ffi::c_int > maxopused as ::core::ffi::c_int {
                maxopused = u;
            }
            hyfdistance[trieopptr as usize] = d;
            hyfnum[trieopptr as usize] = n;
            hyfnext[trieopptr as usize] = v;
            trieoplang[trieopptr as usize] = curlang;
            *(&raw mut zzzab as *mut integer)
                .offset(-(-(35111 as ::core::ffi::c_long) as ::core::ffi::c_int as isize))
                .offset(h as isize) = trieopptr;
            trieopval[trieopptr as usize] = u;
            Result = u;
            return Result;
        }
        if hyfdistance[l as usize] as ::core::ffi::c_int == d as ::core::ffi::c_int
            && hyfnum[l as usize] as ::core::ffi::c_int == n as ::core::ffi::c_int
            && hyfnext[l as usize] as ::core::ffi::c_int == v as ::core::ffi::c_int
            && trieoplang[l as usize] as ::core::ffi::c_int == curlang as ::core::ffi::c_int
        {
            Result = trieopval[l as usize];
            return Result;
        }
        if h > -(trieopsize as integer) {
            h -= 1;
        } else {
            h = trieopsize as integer;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ztrienode(mut p: triepointer) -> triepointer {
    let mut Result: triepointer = 0;
    let mut h: triepointer = 0;
    let mut q: triepointer = 0;
    h = ((if *triec.offset(p as isize) as triepointer
        + 1009 as triepointer * *trieo.offset(p as isize) as triepointer
        + 2718 as triepointer * *triel.offset(p as isize)
        + 3142 as triepointer * *trier.offset(p as isize)
        >= 0 as ::core::ffi::c_int
    {
        *triec.offset(p as isize) as triepointer
            + 1009 as triepointer * *trieo.offset(p as isize) as triepointer
            + 2718 as triepointer * *triel.offset(p as isize)
            + 3142 as triepointer * *trier.offset(p as isize)
    } else {
        -(*triec.offset(p as isize) as triepointer
            + 1009 as triepointer * *trieo.offset(p as isize) as triepointer
            + 2718 as triepointer * *triel.offset(p as isize)
            + 3142 as triepointer * *trier.offset(p as isize))
    }) % triesize) as triepointer;
    loop {
        q = *triehash.offset(h as isize);
        if q == 0 as ::core::ffi::c_int {
            *triehash.offset(h as isize) = p;
            Result = p;
            return Result;
        }
        if *triec.offset(q as isize) as ::core::ffi::c_int
            == *triec.offset(p as isize) as ::core::ffi::c_int
            && *trieo.offset(q as isize) as ::core::ffi::c_int
                == *trieo.offset(p as isize) as ::core::ffi::c_int
            && *triel.offset(q as isize) == *triel.offset(p as isize)
            && *trier.offset(q as isize) == *trier.offset(p as isize)
        {
            Result = q;
            return Result;
        }
        if h > 0 as ::core::ffi::c_int {
            h -= 1;
        } else {
            h = triesize as triepointer;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn zcompresstrie(mut p: triepointer) -> triepointer {
    let mut Result: triepointer = 0;
    if p == 0 as ::core::ffi::c_int {
        Result = 0 as ::core::ffi::c_int as triepointer;
    } else {
        *triel.offset(p as isize) = zcompresstrie(*triel.offset(p as isize));
        *trier.offset(p as isize) = zcompresstrie(*trier.offset(p as isize));
        Result = ztrienode(p);
    }
    return Result;
}
#[no_mangle]
pub unsafe extern "C" fn zfirstfit(mut p: triepointer) {
    let mut h: triepointer = 0;
    let mut z: triepointer = 0;
    let mut q: triepointer = 0;
    let mut c: ASCIIcode = 0;
    let mut l: triepointer = 0;
    let mut r: triepointer = 0;
    let mut ll: ::core::ffi::c_short = 0;
    c = *triec.offset(p as isize) as ASCIIcode;
    z = triemin[c as usize];
    's_22: loop {
        h = (z as ::core::ffi::c_int - c as ::core::ffi::c_int) as triepointer;
        if triemax < h as ::core::ffi::c_int + 256 as ::core::ffi::c_int {
            if triesize <= h as ::core::ffi::c_int + 256 as ::core::ffi::c_int {
                zoverflow(1378 as ::core::ffi::c_int, triesize);
            }
            loop {
                triemax += 1;
                *trietaken.offset(triemax as isize) = false_0 as boolean;
                *trietrl.offset(triemax as isize) =
                    (triemax as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as triepointer;
                *trietro.offset(triemax as isize) =
                    (triemax as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as triepointer;
                if triemax == h as ::core::ffi::c_int + 256 as ::core::ffi::c_int {
                    break;
                }
            }
        }
        if !(*trietaken.offset(h as isize) != 0) {
            q = *trier.offset(p as isize);
            loop {
                if !(q > 0 as ::core::ffi::c_int) {
                    break 's_22;
                }
                if *trietrl.offset(
                    (h as ::core::ffi::c_int + *triec.offset(q as isize) as ::core::ffi::c_int)
                        as isize,
                ) == 0 as ::core::ffi::c_int
                {
                    break;
                }
                q = *trier.offset(q as isize);
            }
        }
        z = *trietrl.offset(z as isize);
    }
    *trietaken.offset(h as isize) = true_0 as boolean;
    *triehash.offset(p as isize) = h;
    q = p;
    loop {
        z = (h as ::core::ffi::c_int + *triec.offset(q as isize) as ::core::ffi::c_int)
            as triepointer;
        l = *trietro.offset(z as isize);
        r = *trietrl.offset(z as isize);
        *trietro.offset(r as isize) = l;
        *trietrl.offset(l as isize) = r;
        *trietrl.offset(z as isize) = 0 as ::core::ffi::c_int as triepointer;
        if l < 256 as ::core::ffi::c_int {
            if z < 256 as ::core::ffi::c_int {
                ll = z as ::core::ffi::c_short;
            } else {
                ll = 256 as ::core::ffi::c_short;
            }
            loop {
                triemin[l as usize] = r;
                l += 1;
                if l == ll as ::core::ffi::c_int {
                    break;
                }
            }
        }
        q = *trier.offset(q as isize);
        if q == 0 as ::core::ffi::c_int {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ztriepack(mut p: triepointer) {
    let mut q: triepointer = 0;
    loop {
        q = *triel.offset(p as isize);
        if q > 0 as ::core::ffi::c_int && *triehash.offset(q as isize) == 0 as ::core::ffi::c_int {
            zfirstfit(q);
            ztriepack(q);
        }
        p = *trier.offset(p as isize);
        if p == 0 as ::core::ffi::c_int {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ztriefix(mut p: triepointer) {
    let mut q: triepointer = 0;
    let mut c: ASCIIcode = 0;
    let mut z: triepointer = 0;
    z = *triehash.offset(p as isize);
    loop {
        q = *triel.offset(p as isize);
        c = *triec.offset(p as isize) as ASCIIcode;
        *trietrl.offset((z as ::core::ffi::c_int + c as ::core::ffi::c_int) as isize) =
            *triehash.offset(q as isize);
        *trietrc.offset((z as ::core::ffi::c_int + c as ::core::ffi::c_int) as isize) =
            c as quarterword;
        *trietro.offset((z as ::core::ffi::c_int + c as ::core::ffi::c_int) as isize) =
            *trieo.offset(p as isize) as triepointer;
        if q > 0 as ::core::ffi::c_int {
            ztriefix(q);
        }
        p = *trier.offset(p as isize);
        if p == 0 as ::core::ffi::c_int {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn newpatterns() {
    let mut mem: *mut memoryword = zmem;
    let mut eqtb: *mut memoryword = zeqtb;
    let mut k_0: ::core::ffi::c_uchar = 0;
    let mut l: ::core::ffi::c_uchar = 0;
    let mut digitsensed: boolean = 0;
    let mut v: trieopcode = 0;
    let mut p: triepointer = 0;
    let mut q: triepointer = 0;
    let mut firstchild: boolean = 0;
    let mut c: ASCIIcode = 0;
    if trienotready != 0 {
        if (*eqtb.offset(29327 as ::core::ffi::c_int as isize)).u.CINT <= 0 as ::core::ffi::c_int {
            curlang = 0 as ASCIIcode;
        } else if (*eqtb.offset(29327 as ::core::ffi::c_int as isize)).u.CINT
            > 255 as ::core::ffi::c_int
        {
            curlang = 0 as ASCIIcode;
        } else {
            curlang = (*eqtb.offset(29327 as ::core::ffi::c_int as isize)).u.CINT as ASCIIcode;
        }
        scanleftbrace();
        k_0 = 0 as ::core::ffi::c_uchar;
        hyf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
        digitsensed = false_0 as boolean;
        loop {
            getxtoken();
            match curcmd as ::core::ffi::c_int {
                11 | 12 => {
                    if digitsensed != 0
                        || curchr < 48 as ::core::ffi::c_int
                        || curchr > 57 as ::core::ffi::c_int
                    {
                        if curchr == 46 as ::core::ffi::c_int {
                            curchr = 0 as ::core::ffi::c_int as halfword;
                        } else {
                            curchr = (*eqtb.offset((27997 as halfword + curchr) as isize))
                                .hh
                                .v
                                .RH;
                            if curchr == 0 as ::core::ffi::c_int {
                                interaction as ::core::ffi::c_int == 3 as ::core::ffi::c_int;
                                if filelineerrorstylep != 0 {
                                    printfileline();
                                } else {
                                    zprintnl(264 as ::core::ffi::c_int);
                                }
                                zprint(1384 as ::core::ffi::c_int);
                                helpptr = 1 as ::core::ffi::c_uchar;
                                helpline[0 as ::core::ffi::c_int as usize] =
                                    1383 as ::core::ffi::c_int as strnumber;
                                error();
                            }
                        }
                        if (k_0 as ::core::ffi::c_int) < 63 as ::core::ffi::c_int {
                            k_0 = k_0.wrapping_add(1);
                            hc[k_0 as usize] = curchr as ::core::ffi::c_short;
                            hyf[k_0 as usize] = 0 as ::core::ffi::c_uchar;
                            digitsensed = false_0 as boolean;
                        }
                    } else if (k_0 as ::core::ffi::c_int) < 63 as ::core::ffi::c_int {
                        hyf[k_0 as usize] = (curchr as ::core::ffi::c_int
                            - 48 as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        digitsensed = true_0 as boolean;
                    }
                }
                10 | 2 => {
                    if k_0 as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                        if hc[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            hyf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
                        }
                        if hc[k_0 as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            hyf[k_0 as usize] = 0 as ::core::ffi::c_uchar;
                        }
                        l = k_0;
                        v = mintrieop as trieopcode;
                        loop {
                            if hyf[l as usize] as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                                v = znewtrieop(
                                    (k_0 as ::core::ffi::c_int - l as ::core::ffi::c_int)
                                        as smallnumber,
                                    hyf[l as usize],
                                    v,
                                );
                            }
                            if !(l as ::core::ffi::c_int > 0 as ::core::ffi::c_int) {
                                break;
                            }
                            l = l.wrapping_sub(1);
                        }
                        q = 0 as ::core::ffi::c_int as triepointer;
                        hc[0 as ::core::ffi::c_int as usize] = curlang as ::core::ffi::c_short;
                        while l as ::core::ffi::c_int <= k_0 as ::core::ffi::c_int {
                            c = hc[l as usize] as ASCIIcode;
                            l = l.wrapping_add(1);
                            p = *triel.offset(q as isize);
                            firstchild = true_0 as boolean;
                            while p > 0 as ::core::ffi::c_int
                                && c as ::core::ffi::c_int
                                    > *triec.offset(p as isize) as ::core::ffi::c_int
                            {
                                q = p;
                                p = *trier.offset(q as isize);
                                firstchild = false_0 as boolean;
                            }
                            if p == 0 as ::core::ffi::c_int
                                || (c as ::core::ffi::c_int)
                                    < *triec.offset(p as isize) as ::core::ffi::c_int
                            {
                                if trieptr == triesize {
                                    zoverflow(1378 as ::core::ffi::c_int, triesize);
                                }
                                trieptr += 1;
                                *trier.offset(trieptr as isize) = p;
                                p = trieptr;
                                *triel.offset(p as isize) = 0 as ::core::ffi::c_int as triepointer;
                                if firstchild != 0 {
                                    *triel.offset(q as isize) = p;
                                } else {
                                    *trier.offset(q as isize) = p;
                                }
                                *triec.offset(p as isize) = c as packedASCIIcode;
                                *trieo.offset(p as isize) = mintrieop as trieopcode;
                            }
                            q = p;
                        }
                        if *trieo.offset(q as isize) as ::core::ffi::c_int != mintrieop {
                            interaction as ::core::ffi::c_int == 3 as ::core::ffi::c_int;
                            if filelineerrorstylep != 0 {
                                printfileline();
                            } else {
                                zprintnl(264 as ::core::ffi::c_int);
                            }
                            zprint(1385 as ::core::ffi::c_int);
                            helpptr = 1 as ::core::ffi::c_uchar;
                            helpline[0 as ::core::ffi::c_int as usize] =
                                1383 as ::core::ffi::c_int as strnumber;
                            error();
                        }
                        *trieo.offset(q as isize) = v;
                    }
                    if curcmd as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                        break;
                    }
                    k_0 = 0 as ::core::ffi::c_uchar;
                    hyf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
                    digitsensed = false_0 as boolean;
                }
                _ => {
                    interaction as ::core::ffi::c_int == 3 as ::core::ffi::c_int;
                    if filelineerrorstylep != 0 {
                        printfileline();
                    } else {
                        zprintnl(264 as ::core::ffi::c_int);
                    }
                    zprint(1382 as ::core::ffi::c_int);
                    zprintesc(1380 as ::core::ffi::c_int);
                    helpptr = 1 as ::core::ffi::c_uchar;
                    helpline[0 as ::core::ffi::c_int as usize] =
                        1383 as ::core::ffi::c_int as strnumber;
                    error();
                }
            }
        }
        if (*eqtb.offset(29387 as ::core::ffi::c_int as isize)).u.CINT > 0 as ::core::ffi::c_int {
            c = curlang;
            firstchild = false_0 as boolean;
            p = 0 as ::core::ffi::c_int as triepointer;
            loop {
                q = p;
                p = *trier.offset(q as isize);
                if p == 0 as ::core::ffi::c_int
                    || c as ::core::ffi::c_int <= *triec.offset(p as isize) as ::core::ffi::c_int
                {
                    break;
                }
            }
            if p == 0 as ::core::ffi::c_int
                || (c as ::core::ffi::c_int) < *triec.offset(p as isize) as ::core::ffi::c_int
            {
                if trieptr == triesize {
                    zoverflow(1378 as ::core::ffi::c_int, triesize);
                }
                trieptr += 1;
                *trier.offset(trieptr as isize) = p;
                p = trieptr;
                *triel.offset(p as isize) = 0 as ::core::ffi::c_int as triepointer;
                if firstchild != 0 {
                    *triel.offset(q as isize) = p;
                } else {
                    *trier.offset(q as isize) = p;
                }
                *triec.offset(p as isize) = c as packedASCIIcode;
                *trieo.offset(p as isize) = mintrieop as trieopcode;
            }
            q = p;
            p = *triel.offset(q as isize);
            firstchild = true_0 as boolean;
            let mut for_end: integer = 0;
            c = 0 as ASCIIcode;
            for_end = 255 as ::core::ffi::c_int as integer;
            if c as ::core::ffi::c_int <= for_end {
                loop {
                    if (*eqtb
                        .offset((27997 as ::core::ffi::c_int + c as ::core::ffi::c_int) as isize))
                    .hh
                    .v
                    .RH > 0 as ::core::ffi::c_int
                        || c as ::core::ffi::c_int == 255 as ::core::ffi::c_int && firstchild != 0
                    {
                        if p == 0 as ::core::ffi::c_int {
                            if trieptr == triesize {
                                zoverflow(1378 as ::core::ffi::c_int, triesize);
                            }
                            trieptr += 1;
                            *trier.offset(trieptr as isize) = p;
                            p = trieptr;
                            *triel.offset(p as isize) = 0 as ::core::ffi::c_int as triepointer;
                            if firstchild != 0 {
                                *triel.offset(q as isize) = p;
                            } else {
                                *trier.offset(q as isize) = p;
                            }
                            *triec.offset(p as isize) = c as packedASCIIcode;
                            *trieo.offset(p as isize) = mintrieop as trieopcode;
                        } else {
                            *triec.offset(p as isize) = c as packedASCIIcode;
                        }
                        *trieo.offset(p as isize) = (*eqtb.offset(
                            (27997 as ::core::ffi::c_int + c as ::core::ffi::c_int) as isize,
                        ))
                        .hh
                        .v
                        .RH as trieopcode;
                        q = p;
                        p = *trier.offset(q as isize);
                        firstchild = false_0 as boolean;
                    }
                    let fresh85 = c;
                    c = c.wrapping_add(1);
                    if !((fresh85 as ::core::ffi::c_int) < for_end) {
                        break;
                    }
                }
            }
            if firstchild != 0 {
                *triel.offset(q as isize) = 0 as ::core::ffi::c_int as triepointer;
            } else {
                *trier.offset(q as isize) = 0 as ::core::ffi::c_int as triepointer;
            }
        }
    } else {
        interaction as ::core::ffi::c_int == 3 as ::core::ffi::c_int;
        if filelineerrorstylep != 0 {
            printfileline();
        } else {
            zprintnl(264 as ::core::ffi::c_int);
        }
        zprint(1379 as ::core::ffi::c_int);
        zprintesc(1380 as ::core::ffi::c_int);
        helpptr = 1 as ::core::ffi::c_uchar;
        helpline[0 as ::core::ffi::c_int as usize] = 1381 as ::core::ffi::c_int as strnumber;
        error();
        (*mem.offset((memtop as ::core::ffi::c_int - 12 as ::core::ffi::c_int) as isize))
            .hh
            .v
            .RH = zscantoks(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
        zflushlist(defref);
    };
}
#[no_mangle]
pub unsafe extern "C" fn inittrie() {
    let mut p: triepointer = 0;
    let mut j: integer = 0;
    let mut k_0: integer = 0;
    let mut t: integer = 0;
    let mut r: triepointer = 0;
    let mut s: triepointer = 0;
    opstart[0 as ::core::ffi::c_int as usize] = -mintrieop;
    let mut for_end: integer = 0;
    j = 1 as ::core::ffi::c_int as integer;
    for_end = 255 as ::core::ffi::c_int as integer;
    if j <= for_end {
        loop {
            opstart[j as usize] = (opstart
                [(j as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                + trieused[(j as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int) as integer;
            let fresh18 = j;
            j = j + 1;
            if !(fresh18 < for_end) {
                break;
            }
        }
    }
    let mut for_end_0: integer = 0;
    j = 1 as ::core::ffi::c_int as integer;
    for_end_0 = trieopptr;
    if j <= for_end_0 {
        loop {
            *(&raw mut zzzab as *mut integer)
                .offset(-(-(35111 as ::core::ffi::c_long) as ::core::ffi::c_int as isize))
                .offset(j as isize) = (opstart[trieoplang[j as usize] as usize]
                + trieopval[j as usize] as ::core::ffi::c_int)
                as integer;
            let fresh19 = j;
            j = j + 1;
            if !(fresh19 < for_end_0) {
                break;
            }
        }
    }
    let mut for_end_1: integer = 0;
    j = 1 as ::core::ffi::c_int as integer;
    for_end_1 = trieopptr;
    if j <= for_end_1 {
        loop {
            while *(&raw mut zzzab as *mut integer)
                .offset(-(-(35111 as ::core::ffi::c_long) as ::core::ffi::c_int as isize))
                .offset(j as isize)
                > j
            {
                k_0 = *(&raw mut zzzab as *mut integer)
                    .offset(-(-(35111 as ::core::ffi::c_long) as ::core::ffi::c_int as isize))
                    .offset(j as isize);
                t = hyfdistance[k_0 as usize] as integer;
                hyfdistance[k_0 as usize] = hyfdistance[j as usize];
                hyfdistance[j as usize] = t as smallnumber;
                t = hyfnum[k_0 as usize] as integer;
                hyfnum[k_0 as usize] = hyfnum[j as usize];
                hyfnum[j as usize] = t as smallnumber;
                t = hyfnext[k_0 as usize] as integer;
                hyfnext[k_0 as usize] = hyfnext[j as usize];
                hyfnext[j as usize] = t as trieopcode;
                *(&raw mut zzzab as *mut integer)
                    .offset(-(-(35111 as ::core::ffi::c_long) as ::core::ffi::c_int as isize))
                    .offset(j as isize) = *(&raw mut zzzab as *mut integer)
                    .offset(-(-(35111 as ::core::ffi::c_long) as ::core::ffi::c_int as isize))
                    .offset(k_0 as isize);
                *(&raw mut zzzab as *mut integer)
                    .offset(-(-(35111 as ::core::ffi::c_long) as ::core::ffi::c_int as isize))
                    .offset(k_0 as isize) = k_0;
            }
            let fresh20 = j;
            j = j + 1;
            if !(fresh20 < for_end_1) {
                break;
            }
        }
    }
    let mut for_end_2: integer = 0;
    p = 0 as ::core::ffi::c_int as triepointer;
    for_end_2 = triesize;
    if p <= for_end_2 {
        loop {
            *triehash.offset(p as isize) = 0 as ::core::ffi::c_int as triepointer;
            let fresh21 = p;
            p = p + 1;
            if !(fresh21 < for_end_2) {
                break;
            }
        }
    }
    *trier.offset(0 as ::core::ffi::c_int as isize) =
        zcompresstrie(*trier.offset(0 as ::core::ffi::c_int as isize));
    *triel.offset(0 as ::core::ffi::c_int as isize) =
        zcompresstrie(*triel.offset(0 as ::core::ffi::c_int as isize));
    let mut for_end_3: integer = 0;
    p = 0 as ::core::ffi::c_int as triepointer;
    for_end_3 = trieptr as integer;
    if p <= for_end_3 {
        loop {
            *triehash.offset(p as isize) = 0 as ::core::ffi::c_int as triepointer;
            let fresh22 = p;
            p = p + 1;
            if !(fresh22 < for_end_3) {
                break;
            }
        }
    }
    let mut for_end_4: integer = 0;
    p = 0 as ::core::ffi::c_int as triepointer;
    for_end_4 = 255 as ::core::ffi::c_int as integer;
    if p <= for_end_4 {
        loop {
            triemin[p as usize] =
                (p as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as triepointer;
            let fresh23 = p;
            p = p + 1;
            if !(fresh23 < for_end_4) {
                break;
            }
        }
    }
    *trietrl.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int as triepointer;
    triemax = 0 as ::core::ffi::c_int as triepointer;
    if *triel.offset(0 as ::core::ffi::c_int as isize) != 0 as ::core::ffi::c_int {
        zfirstfit(*triel.offset(0 as ::core::ffi::c_int as isize));
        ztriepack(*triel.offset(0 as ::core::ffi::c_int as isize));
    }
    if *trier.offset(0 as ::core::ffi::c_int as isize) != 0 as ::core::ffi::c_int {
        if *triel.offset(0 as ::core::ffi::c_int as isize) == 0 as ::core::ffi::c_int {
            let mut for_end_5: integer = 0;
            p = 0 as ::core::ffi::c_int as triepointer;
            for_end_5 = 255 as ::core::ffi::c_int as integer;
            if p <= for_end_5 {
                loop {
                    triemin[p as usize] =
                        (p as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as triepointer;
                    let fresh24 = p;
                    p = p + 1;
                    if !(fresh24 < for_end_5) {
                        break;
                    }
                }
            }
        }
        zfirstfit(*trier.offset(0 as ::core::ffi::c_int as isize));
        ztriepack(*trier.offset(0 as ::core::ffi::c_int as isize));
        hyphstart = *triehash.offset(*trier.offset(0 as ::core::ffi::c_int as isize) as isize);
    }
    if triemax == 0 as ::core::ffi::c_int {
        let mut for_end_6: integer = 0;
        r = 0 as ::core::ffi::c_int as triepointer;
        for_end_6 = 256 as ::core::ffi::c_int as integer;
        if r <= for_end_6 {
            loop {
                *trietrl.offset(r as isize) = 0 as ::core::ffi::c_int as triepointer;
                *trietro.offset(r as isize) = mintrieop as triepointer;
                *trietrc.offset(r as isize) = 0 as quarterword;
                let fresh25 = r;
                r = r + 1;
                if !(fresh25 < for_end_6) {
                    break;
                }
            }
        }
        triemax = 256 as ::core::ffi::c_int as triepointer;
    } else {
        if *trier.offset(0 as ::core::ffi::c_int as isize) > 0 as ::core::ffi::c_int {
            ztriefix(*trier.offset(0 as ::core::ffi::c_int as isize));
        }
        if *triel.offset(0 as ::core::ffi::c_int as isize) > 0 as ::core::ffi::c_int {
            ztriefix(*triel.offset(0 as ::core::ffi::c_int as isize));
        }
        r = 0 as ::core::ffi::c_int as triepointer;
        loop {
            s = *trietrl.offset(r as isize);
            *trietrl.offset(r as isize) = 0 as ::core::ffi::c_int as triepointer;
            *trietro.offset(r as isize) = mintrieop as triepointer;
            *trietrc.offset(r as isize) = 0 as quarterword;
            r = s;
            if r > triemax {
                break;
            }
        }
    }
    *trietrc.offset(0 as ::core::ffi::c_int as isize) = 63 as quarterword;
    trienotready = false_0 as boolean;
}
#[no_mangle]
pub unsafe extern "C" fn zlinebreak(mut d: boolean) {
    let mut current_block: u64;
    let mut mem: *mut memoryword = zmem;
    let mut eqtb: *mut memoryword = zeqtb;
    let mut q: halfword = 0;
    let mut r: halfword = 0;
    let mut s: halfword = 0;
    let mut prevs: halfword = 0;
    let mut f: internalfontnumber = 0;
    let mut j: smallnumber = 0;
    let mut c: ::core::ffi::c_uchar = 0;
    packbeginline = curlist.mlfield;
    (*mem.offset((memtop as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize))
        .hh
        .v
        .RH = (*mem.offset(curlist.headfield as isize)).hh.v.RH;
    if curlist.tailfield >= himemmin {
        prevtail = curlist.tailfield;
        (*mem.offset(curlist.tailfield as isize)).hh.v.RH =
            znewpenalty(10000 as ::core::ffi::c_int);
        curlist.tailfield = (*mem.offset(curlist.tailfield as isize)).hh.v.RH;
    } else if (*mem.offset(curlist.tailfield as isize)).hh.u.B0 as ::core::ffi::c_int
        != 10 as ::core::ffi::c_int
    {
        prevtail = curlist.tailfield;
        (*mem.offset(curlist.tailfield as isize)).hh.v.RH =
            znewpenalty(10000 as ::core::ffi::c_int);
        curlist.tailfield = (*mem.offset(curlist.tailfield as isize)).hh.v.RH;
    } else {
        (*mem.offset(curlist.tailfield as isize)).hh.u.B0 = 12 as ::core::ffi::c_short;
        zdeleteglueref(
            (*mem.offset(
                (curlist.tailfield as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
            ))
            .hh
            .v
            .LH,
        );
        zflushnodelist(
            (*mem.offset(
                (curlist.tailfield as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
            ))
            .hh
            .v
            .RH,
        );
        (*mem.offset(
            (curlist.tailfield as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        ))
        .u
        .CINT = 10000 as ::core::ffi::c_int as integer;
    }
    (*mem.offset(curlist.tailfield as isize)).hh.v.RH =
        znewparamglue(14 as ::core::ffi::c_int as smallnumber);
    lastlinefill = (*mem.offset(curlist.tailfield as isize)).hh.v.RH;
    initcurlang =
        (curlist.pgfield as ::core::ffi::c_long % 65536 as ::core::ffi::c_long) as ASCIIcode;
    initlhyf = (curlist.pgfield as ::core::ffi::c_long / 4194304 as ::core::ffi::c_long) as integer;
    initrhyf = (curlist.pgfield as ::core::ffi::c_long / 65536 as ::core::ffi::c_long
        % 64 as ::core::ffi::c_long) as integer;
    popnest();
    noshrinkerroryet = true_0 as boolean;
    if (*mem.offset((*eqtb.offset(26635 as ::core::ffi::c_int as isize)).hh.v.RH as isize))
        .hh
        .u
        .B1 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
        && (*mem.offset(
            ((*eqtb.offset(26635 as ::core::ffi::c_int as isize)).hh.v.RH as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) as isize,
        ))
        .u
        .CINT
            != 0 as ::core::ffi::c_int
    {
        (*eqtb.offset(26635 as ::core::ffi::c_int as isize)).hh.v.RH =
            zfiniteshrink((*eqtb.offset(26635 as ::core::ffi::c_int as isize)).hh.v.RH);
    }
    if (*mem.offset((*eqtb.offset(26636 as ::core::ffi::c_int as isize)).hh.v.RH as isize))
        .hh
        .u
        .B1 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
        && (*mem.offset(
            ((*eqtb.offset(26636 as ::core::ffi::c_int as isize)).hh.v.RH as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) as isize,
        ))
        .u
        .CINT
            != 0 as ::core::ffi::c_int
    {
        (*eqtb.offset(26636 as ::core::ffi::c_int as isize)).hh.v.RH =
            zfiniteshrink((*eqtb.offset(26636 as ::core::ffi::c_int as isize)).hh.v.RH);
    }
    q = (*eqtb.offset(26635 as ::core::ffi::c_int as isize)).hh.v.RH;
    r = (*eqtb.offset(26636 as ::core::ffi::c_int as isize)).hh.v.RH;
    background[1 as ::core::ffi::c_int as usize] = ((*mem
        .offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
    .u
    .CINT
        + (*mem.offset((r as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
            .u
            .CINT) as scaled;
    background[2 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int as scaled;
    background[3 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int as scaled;
    background[4 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int as scaled;
    background[5 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int as scaled;
    background[(2 as ::core::ffi::c_int + (*mem.offset(q as isize)).hh.u.B0 as ::core::ffi::c_int)
        as usize] = (*mem.offset((q as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
        .u
        .CINT as scaled;
    background[(2 as ::core::ffi::c_int + (*mem.offset(r as isize)).hh.u.B0 as ::core::ffi::c_int)
        as usize] = (background[(2 as ::core::ffi::c_int
        + (*mem.offset(r as isize)).hh.u.B0 as ::core::ffi::c_int)
        as usize]
        + (*mem.offset((r as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
            .u
            .CINT) as scaled;
    background[6 as ::core::ffi::c_int as usize] = ((*mem
        .offset((q as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize))
    .u
    .CINT
        + (*mem.offset((r as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize))
            .u
            .CINT) as scaled;
    if (*eqtb.offset(29360 as ::core::ffi::c_int as isize)).u.CINT > 1 as ::core::ffi::c_int {
        background[7 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int as scaled;
        background[8 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int as scaled;
        maxstretchratio = -(1 as ::core::ffi::c_int) as integer;
        maxshrinkratio = -(1 as ::core::ffi::c_int) as integer;
        curfontstep = -(1 as ::core::ffi::c_int) as integer;
        prevcharp = -(268435455 as ::core::ffi::c_long) as halfword;
    }
    dolastlinefit = false_0 as boolean;
    activenodesize = 3 as smallnumber;
    if (*eqtb.offset(29385 as ::core::ffi::c_int as isize)).u.CINT > 0 as ::core::ffi::c_int {
        q = (*mem.offset((lastlinefill as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
            .hh
            .v
            .LH;
        if (*mem.offset((q as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
            .u
            .CINT
            > 0 as ::core::ffi::c_int
            && (*mem.offset(q as isize)).hh.u.B0 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
        {
            if background[3 as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int
                && background[4 as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int
                && background[5 as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int
            {
                dolastlinefit = true_0 as boolean;
                activenodesize = 5 as smallnumber;
                fillwidth[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int as scaled;
                fillwidth[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int as scaled;
                fillwidth[2 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int as scaled;
                fillwidth[((*mem.offset(q as isize)).hh.u.B0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as usize] = (*mem
                    .offset((q as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
                .u
                .CINT as scaled;
            }
        }
    }
    minimumdemerits = 1073741823 as ::core::ffi::c_long as integer;
    minimaldemerits[3 as ::core::ffi::c_int as usize] =
        1073741823 as ::core::ffi::c_long as integer;
    minimaldemerits[2 as ::core::ffi::c_int as usize] =
        1073741823 as ::core::ffi::c_long as integer;
    minimaldemerits[1 as ::core::ffi::c_int as usize] =
        1073741823 as ::core::ffi::c_long as integer;
    minimaldemerits[0 as ::core::ffi::c_int as usize] =
        1073741823 as ::core::ffi::c_long as integer;
    if (*eqtb.offset(27158 as ::core::ffi::c_int as isize)).hh.v.RH as ::core::ffi::c_long
        == -(268435455 as ::core::ffi::c_long)
    {
        if (*eqtb.offset(29920 as ::core::ffi::c_int as isize)).u.CINT == 0 as ::core::ffi::c_int {
            lastspecialline = 0 as ::core::ffi::c_int as halfword;
            secondwidth = (*eqtb.offset(29906 as ::core::ffi::c_int as isize)).u.CINT as scaled;
            secondindent = 0 as ::core::ffi::c_int as scaled;
        } else {
            lastspecialline = (if (*eqtb.offset(29318 as ::core::ffi::c_int as isize)).u.CINT
                >= 0 as ::core::ffi::c_int
            {
                (*eqtb.offset(29318 as ::core::ffi::c_int as isize)).u.CINT
            } else {
                -(*eqtb.offset(29318 as ::core::ffi::c_int as isize)).u.CINT
            }) as halfword;
            if (*eqtb.offset(29318 as ::core::ffi::c_int as isize)).u.CINT < 0 as ::core::ffi::c_int
            {
                firstwidth = ((*eqtb.offset(29906 as ::core::ffi::c_int as isize)).u.CINT
                    - (if (*eqtb.offset(29920 as ::core::ffi::c_int as isize)).u.CINT
                        >= 0 as ::core::ffi::c_int
                    {
                        (*eqtb.offset(29920 as ::core::ffi::c_int as isize)).u.CINT
                    } else {
                        -(*eqtb.offset(29920 as ::core::ffi::c_int as isize)).u.CINT
                    })) as scaled;
                if (*eqtb.offset(29920 as ::core::ffi::c_int as isize)).u.CINT
                    >= 0 as ::core::ffi::c_int
                {
                    firstindent =
                        (*eqtb.offset(29920 as ::core::ffi::c_int as isize)).u.CINT as scaled;
                } else {
                    firstindent = 0 as ::core::ffi::c_int as scaled;
                }
                secondwidth = (*eqtb.offset(29906 as ::core::ffi::c_int as isize)).u.CINT as scaled;
                secondindent = 0 as ::core::ffi::c_int as scaled;
            } else {
                firstwidth = (*eqtb.offset(29906 as ::core::ffi::c_int as isize)).u.CINT as scaled;
                firstindent = 0 as ::core::ffi::c_int as scaled;
                secondwidth = ((*eqtb.offset(29906 as ::core::ffi::c_int as isize)).u.CINT
                    - (if (*eqtb.offset(29920 as ::core::ffi::c_int as isize)).u.CINT
                        >= 0 as ::core::ffi::c_int
                    {
                        (*eqtb.offset(29920 as ::core::ffi::c_int as isize)).u.CINT
                    } else {
                        -(*eqtb.offset(29920 as ::core::ffi::c_int as isize)).u.CINT
                    })) as scaled;
                if (*eqtb.offset(29920 as ::core::ffi::c_int as isize)).u.CINT
                    >= 0 as ::core::ffi::c_int
                {
                    secondindent =
                        (*eqtb.offset(29920 as ::core::ffi::c_int as isize)).u.CINT as scaled;
                } else {
                    secondindent = 0 as ::core::ffi::c_int as scaled;
                }
            }
        }
    } else {
        lastspecialline = ((*mem
            .offset((*eqtb.offset(27158 as ::core::ffi::c_int as isize)).hh.v.RH as isize))
        .hh
        .v
        .LH as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as halfword;
        secondwidth = (*mem.offset(
            ((*eqtb.offset(27158 as ::core::ffi::c_int as isize)).hh.v.RH as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * (lastspecialline as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                as isize,
        ))
        .u
        .CINT as scaled;
        secondindent = (*mem.offset(
            ((*eqtb.offset(27158 as ::core::ffi::c_int as isize)).hh.v.RH as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * lastspecialline as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as isize,
        ))
        .u
        .CINT as scaled;
    }
    if (*eqtb.offset(29296 as ::core::ffi::c_int as isize)).u.CINT == 0 as ::core::ffi::c_int {
        easyline = lastspecialline;
    } else {
        easyline = 268435455 as ::core::ffi::c_long as halfword;
    }
    threshold = (*eqtb.offset(29277 as ::core::ffi::c_int as isize)).u.CINT;
    if threshold >= 0 as ::core::ffi::c_int {
        if (*eqtb.offset(29309 as ::core::ffi::c_int as isize)).u.CINT > 0 as ::core::ffi::c_int {
            begindiagnostic();
            zprintnl(1360 as ::core::ffi::c_int);
        }
        secondpass = false_0 as boolean;
        finalpass = false_0 as boolean;
    } else {
        threshold = (*eqtb.offset(29278 as ::core::ffi::c_int as isize)).u.CINT;
        secondpass = true_0 as boolean;
        finalpass = ((*eqtb.offset(29923 as ::core::ffi::c_int as isize)).u.CINT
            <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int as boolean;
        if (*eqtb.offset(29309 as ::core::ffi::c_int as isize)).u.CINT > 0 as ::core::ffi::c_int {
            begindiagnostic();
        }
    }
    loop {
        if threshold > 10000 as ::core::ffi::c_int {
            threshold = 10000 as ::core::ffi::c_int as integer;
        }
        if secondpass != 0 {
            if trienotready != 0 {
                inittrie();
            }
            curlang = initcurlang;
            lhyf = initlhyf;
            rhyf = initrhyf;
            if *trietrc
                .offset((hyphstart as ::core::ffi::c_int + curlang as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                != curlang as ::core::ffi::c_int
            {
                hyphindex = 0 as ::core::ffi::c_int as triepointer;
            } else {
                hyphindex = *trietrl.offset(
                    (hyphstart as ::core::ffi::c_int + curlang as ::core::ffi::c_int) as isize,
                );
            }
        }
        q = zgetnode(activenodesize as integer);
        (*mem.offset(q as isize)).hh.u.B0 = 0 as ::core::ffi::c_short;
        (*mem.offset(q as isize)).hh.u.B1 = 2 as ::core::ffi::c_short;
        (*mem.offset(q as isize)).hh.v.RH =
            (memtop as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as halfword;
        (*mem.offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
            .hh
            .v
            .RH = -(268435455 as ::core::ffi::c_long) as halfword;
        (*mem.offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
            .hh
            .v
            .LH = (curlist.pgfield as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as halfword;
        (*mem.offset((q as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
            .u
            .CINT = 0 as ::core::ffi::c_int as integer;
        (*mem.offset((memtop as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as isize))
            .hh
            .v
            .RH = q;
        if dolastlinefit != 0 {
            (*mem.offset((q as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize))
                .u
                .CINT = 0 as ::core::ffi::c_int as integer;
            (*mem.offset((q as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize))
                .u
                .CINT = 0 as ::core::ffi::c_int as integer;
        }
        activewidth[1 as ::core::ffi::c_int as usize] =
            background[1 as ::core::ffi::c_int as usize];
        activewidth[2 as ::core::ffi::c_int as usize] =
            background[2 as ::core::ffi::c_int as usize];
        activewidth[3 as ::core::ffi::c_int as usize] =
            background[3 as ::core::ffi::c_int as usize];
        activewidth[4 as ::core::ffi::c_int as usize] =
            background[4 as ::core::ffi::c_int as usize];
        activewidth[5 as ::core::ffi::c_int as usize] =
            background[5 as ::core::ffi::c_int as usize];
        activewidth[6 as ::core::ffi::c_int as usize] =
            background[6 as ::core::ffi::c_int as usize];
        if (*eqtb.offset(29360 as ::core::ffi::c_int as isize)).u.CINT > 1 as ::core::ffi::c_int {
            activewidth[7 as ::core::ffi::c_int as usize] =
                background[7 as ::core::ffi::c_int as usize];
            activewidth[8 as ::core::ffi::c_int as usize] =
                background[8 as ::core::ffi::c_int as usize];
        }
        passive = -(268435455 as ::core::ffi::c_long) as halfword;
        printednode = (memtop as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as halfword;
        passnumber = 0 as ::core::ffi::c_int as halfword;
        fontinshortdisplay = 0 as ::core::ffi::c_int as integer;
        curp = (*mem.offset((memtop as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize))
            .hh
            .v
            .RH;
        autobreaking = true_0 as boolean;
        prevp = curp;
        prevcharp = -(268435455 as ::core::ffi::c_long) as halfword;
        prevlegal = -(268435455 as ::core::ffi::c_long) as halfword;
        rejectedcurp = -(268435455 as ::core::ffi::c_long) as halfword;
        tryprevbreak = false_0 as boolean;
        beforerejectedcurp = false_0 as boolean;
        firstp = curp;
        while curp as ::core::ffi::c_long != -(268435455 as ::core::ffi::c_long)
            && (*mem.offset((memtop as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as isize))
                .hh
                .v
                .RH
                != memtop as ::core::ffi::c_int - 7 as ::core::ffi::c_int
        {
            if curp >= himemmin {
                prevp = curp;
                loop {
                    f = (*mem.offset(curp as isize)).hh.u.B0 as internalfontnumber;
                    activewidth[1 as ::core::ffi::c_int as usize] = (activewidth
                        [1 as ::core::ffi::c_int as usize]
                        + (*fontinfo.offset(
                            (*widthbase.offset(f as isize) as ::core::ffi::c_int
                                + (*fontinfo.offset(
                                    (*charbase.offset(f as isize)
                                        + zeffectivechar(
                                            1 as ::core::ffi::c_int,
                                            f,
                                            (*mem.offset(curp as isize)).hh.u.B1 as quarterword,
                                        )) as isize,
                                ))
                                .v
                                .QQQQ
                                .u
                                .B0 as ::core::ffi::c_int) as isize,
                        ))
                        .u
                        .CINT)
                        as scaled;
                    if (*eqtb.offset(29360 as ::core::ffi::c_int as isize)).u.CINT
                        > 1 as ::core::ffi::c_int
                        && zcheckexpandpars(f) != 0
                    {
                        prevcharp = curp;
                        activewidth[7 as ::core::ffi::c_int as usize] = activewidth
                            [7 as ::core::ffi::c_int as usize]
                            + zcharstretch(f, (*mem.offset(curp as isize)).hh.u.B1 as eightbits);
                        activewidth[8 as ::core::ffi::c_int as usize] = activewidth
                            [8 as ::core::ffi::c_int as usize]
                            + zcharshrink(f, (*mem.offset(curp as isize)).hh.u.B1 as eightbits);
                    }
                    curp = (*mem.offset(curp as isize)).hh.v.RH;
                    if !(curp >= himemmin) {
                        break;
                    }
                }
            }
            match (*mem.offset(curp as isize)).hh.u.B0 as ::core::ffi::c_int {
                0 | 1 | 2 => {
                    activewidth[1 as ::core::ffi::c_int as usize] =
                        (activewidth[1 as ::core::ffi::c_int as usize]
                            + (*mem.offset(
                                (curp as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ))
                            .u
                            .CINT) as scaled;
                }
                8 => {
                    if (*mem.offset(curp as isize)).hh.u.B1 as ::core::ffi::c_int
                        == 5 as ::core::ffi::c_int
                    {
                        curlang = (*mem.offset(
                            (curp as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                        ))
                        .hh
                        .v
                        .RH as ASCIIcode;
                        lhyf = (*mem.offset(
                            (curp as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                        ))
                        .hh
                        .u
                        .B0 as integer;
                        rhyf = (*mem.offset(
                            (curp as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                        ))
                        .hh
                        .u
                        .B1 as integer;
                    }
                    if (*mem.offset(curp as isize)).hh.u.B1 as ::core::ffi::c_int
                        == 12 as ::core::ffi::c_int
                        || (*mem.offset(curp as isize)).hh.u.B1 as ::core::ffi::c_int
                            == 14 as ::core::ffi::c_int
                    {
                        activewidth[1 as ::core::ffi::c_int as usize] =
                            (activewidth[1 as ::core::ffi::c_int as usize]
                                + (*mem.offset(
                                    (curp as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                ))
                                .u
                                .CINT) as scaled;
                    }
                }
                10 => {
                    if autobreaking != 0 {
                        if prevp >= himemmin {
                            ztrybreak(
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int as smallnumber,
                            );
                        } else if ((*mem.offset(prevp as isize)).hh.u.B0 as ::core::ffi::c_int)
                            < 9 as ::core::ffi::c_int
                        {
                            ztrybreak(
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int as smallnumber,
                            );
                        } else if (*mem.offset(prevp as isize)).hh.u.B0 as ::core::ffi::c_int
                            == 11 as ::core::ffi::c_int
                            && (*mem.offset(prevp as isize)).hh.u.B1 as ::core::ffi::c_int
                                != 1 as ::core::ffi::c_int
                        {
                            ztrybreak(
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int as smallnumber,
                            );
                        }
                    }
                    if (*mem.offset(
                        (*mem.offset(
                            (curp as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                        ))
                        .hh
                        .v
                        .LH as isize,
                    ))
                    .hh
                    .u
                    .B1 as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                        && (*mem.offset(
                            ((*mem.offset(
                                (curp as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ))
                            .hh
                            .v
                            .LH as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int) as isize,
                        ))
                        .u
                        .CINT
                            != 0 as ::core::ffi::c_int
                    {
                        (*mem.offset(
                            (curp as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                        ))
                        .hh
                        .v
                        .LH = zfiniteshrink(
                            (*mem.offset(
                                (curp as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ))
                            .hh
                            .v
                            .LH,
                        );
                    }
                    q = (*mem
                        .offset((curp as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                    .hh
                    .v
                    .LH;
                    activewidth[1 as ::core::ffi::c_int as usize] = (activewidth
                        [1 as ::core::ffi::c_int as usize]
                        + (*mem
                            .offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                        .u
                        .CINT)
                        as scaled;
                    activewidth[(2 as ::core::ffi::c_int
                        + (*mem.offset(q as isize)).hh.u.B0 as ::core::ffi::c_int)
                        as usize] = (activewidth[(2 as ::core::ffi::c_int
                        + (*mem.offset(q as isize)).hh.u.B0 as ::core::ffi::c_int)
                        as usize]
                        + (*mem
                            .offset((q as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
                        .u
                        .CINT) as scaled;
                    activewidth[6 as ::core::ffi::c_int as usize] = (activewidth
                        [6 as ::core::ffi::c_int as usize]
                        + (*mem
                            .offset((q as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize))
                        .u
                        .CINT)
                        as scaled;
                    if secondpass != 0 && autobreaking != 0 {
                        prevs = curp;
                        s = (*mem.offset(prevs as isize)).hh.v.RH;
                        if s as ::core::ffi::c_long != -(268435455 as ::core::ffi::c_long) {
                            loop {
                                if s >= himemmin {
                                    c = (*mem.offset(s as isize)).hh.u.B1 as ::core::ffi::c_uchar;
                                    hf = (*mem.offset(s as isize)).hh.u.B0 as internalfontnumber;
                                    current_block = 9872982588115335912;
                                } else if (*mem.offset(s as isize)).hh.u.B0 as ::core::ffi::c_int
                                    == 6 as ::core::ffi::c_int
                                {
                                    if (*mem.offset(
                                        (s as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            as isize,
                                    ))
                                    .hh
                                    .v
                                    .RH
                                        as ::core::ffi::c_long
                                        == -(268435455 as ::core::ffi::c_long)
                                    {
                                        current_block = 8226276582369899889;
                                    } else {
                                        q = (*mem.offset(
                                            (s as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        ))
                                        .hh
                                        .v
                                        .RH;
                                        c = (*mem.offset(q as isize)).hh.u.B1
                                            as ::core::ffi::c_uchar;
                                        hf =
                                            (*mem.offset(q as isize)).hh.u.B0 as internalfontnumber;
                                        current_block = 9872982588115335912;
                                    }
                                } else if (*mem.offset(s as isize)).hh.u.B0 as ::core::ffi::c_int
                                    == 11 as ::core::ffi::c_int
                                    && (*mem.offset(s as isize)).hh.u.B1 as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int
                                {
                                    current_block = 8226276582369899889;
                                } else if (*mem.offset(s as isize)).hh.u.B0 as ::core::ffi::c_int
                                    == 9 as ::core::ffi::c_int
                                    && (*mem.offset(s as isize)).hh.u.B1 as ::core::ffi::c_int
                                        >= 4 as ::core::ffi::c_int
                                {
                                    current_block = 8226276582369899889;
                                } else {
                                    if !((*mem.offset(s as isize)).hh.u.B0 as ::core::ffi::c_int
                                        == 8 as ::core::ffi::c_int)
                                    {
                                        current_block = 309319537768397308;
                                        break;
                                    }
                                    if (*mem.offset(s as isize)).hh.u.B1 as ::core::ffi::c_int
                                        == 5 as ::core::ffi::c_int
                                    {
                                        curlang = (*mem.offset(
                                            (s as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        ))
                                        .hh
                                        .v
                                        .RH
                                            as ASCIIcode;
                                        lhyf = (*mem.offset(
                                            (s as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        ))
                                        .hh
                                        .u
                                        .B0
                                            as integer;
                                        rhyf = (*mem.offset(
                                            (s as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        ))
                                        .hh
                                        .u
                                        .B1
                                            as integer;
                                        if *trietrc.offset(
                                            (hyphstart as ::core::ffi::c_int
                                                + curlang as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            != curlang as ::core::ffi::c_int
                                        {
                                            hyphindex = 0 as ::core::ffi::c_int as triepointer;
                                        } else {
                                            hyphindex = *trietrl.offset(
                                                (hyphstart as ::core::ffi::c_int
                                                    + curlang as ::core::ffi::c_int)
                                                    as isize,
                                            );
                                        }
                                    }
                                    current_block = 8226276582369899889;
                                }
                                match current_block {
                                    9872982588115335912 => {
                                        if hyphindex == 0 as ::core::ffi::c_int {
                                            hc[0 as ::core::ffi::c_int as usize] = (*eqtb.offset(
                                                (27997 as ::core::ffi::c_int
                                                    + c as ::core::ffi::c_int)
                                                    as isize,
                                            ))
                                            .hh
                                            .v
                                            .RH
                                                as ::core::ffi::c_short;
                                        } else if *trietrc.offset(
                                            (hyphindex as ::core::ffi::c_int
                                                + c as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            != c as ::core::ffi::c_int
                                        {
                                            hc[0 as ::core::ffi::c_int as usize] =
                                                0 as ::core::ffi::c_short;
                                        } else {
                                            hc[0 as ::core::ffi::c_int as usize] = *trietro.offset(
                                                (hyphindex as ::core::ffi::c_int
                                                    + c as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as ::core::ffi::c_short;
                                        }
                                        if hc[0 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int
                                            != 0 as ::core::ffi::c_int
                                        {
                                            if hc[0 as ::core::ffi::c_int as usize]
                                                as ::core::ffi::c_int
                                                == c as ::core::ffi::c_int
                                                || (*eqtb
                                                    .offset(29315 as ::core::ffi::c_int as isize))
                                                .u
                                                .CINT
                                                    > 0 as ::core::ffi::c_int
                                            {
                                                current_block = 9009662752448988403;
                                                break;
                                            } else {
                                                current_block = 309319537768397308;
                                                break;
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                prevs = s;
                                s = (*mem.offset(prevs as isize)).hh.v.RH;
                            }
                            match current_block {
                                309319537768397308 => {}
                                _ => {
                                    hyfchar = *hyphenchar.offset(hf as isize);
                                    if !(hyfchar < 0 as ::core::ffi::c_int) {
                                        if !(hyfchar > 255 as ::core::ffi::c_int) {
                                            ha = prevs;
                                            if !(lhyf + rhyf > 63 as ::core::ffi::c_int) {
                                                hn = 0 as ::core::ffi::c_uchar;
                                                's_930: loop {
                                                    if s >= himemmin {
                                                        if (*mem.offset(s as isize)).hh.u.B0
                                                            as ::core::ffi::c_int
                                                            != hf
                                                        {
                                                            break;
                                                        }
                                                        hyfbchar = (*mem.offset(s as isize)).hh.u.B1
                                                            as halfword;
                                                        c = hyfbchar as ::core::ffi::c_uchar;
                                                        if hyphindex == 0 as ::core::ffi::c_int {
                                                            hc[0 as ::core::ffi::c_int as usize] =
                                                                (*eqtb.offset(
                                                                    (27997 as ::core::ffi::c_int
                                                                        + c as ::core::ffi::c_int)
                                                                        as isize,
                                                                ))
                                                                .hh
                                                                .v
                                                                .RH
                                                                    as ::core::ffi::c_short;
                                                        } else if *trietrc.offset(
                                                            (hyphindex as ::core::ffi::c_int
                                                                + c as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            != c as ::core::ffi::c_int
                                                        {
                                                            hc[0 as ::core::ffi::c_int as usize] =
                                                                0 as ::core::ffi::c_short;
                                                        } else {
                                                            hc[0 as ::core::ffi::c_int as usize] =
                                                                *trietro.offset(
                                                                    (hyphindex
                                                                        as ::core::ffi::c_int
                                                                        + c as ::core::ffi::c_int)
                                                                        as isize,
                                                                )
                                                                    as ::core::ffi::c_short;
                                                        }
                                                        if hc[0 as ::core::ffi::c_int as usize]
                                                            as ::core::ffi::c_int
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            break;
                                                        }
                                                        if hn as ::core::ffi::c_int
                                                            == 63 as ::core::ffi::c_int
                                                        {
                                                            break;
                                                        }
                                                        hb = s;
                                                        hn = hn.wrapping_add(1);
                                                        hu[hn as usize] = c as ::core::ffi::c_short;
                                                        hc[hn as usize] =
                                                            hc[0 as ::core::ffi::c_int as usize];
                                                        hyfbchar =
                                                            256 as ::core::ffi::c_int as halfword;
                                                    } else if (*mem.offset(s as isize)).hh.u.B0
                                                        as ::core::ffi::c_int
                                                        == 6 as ::core::ffi::c_int
                                                    {
                                                        if (*mem.offset(
                                                            (s as ::core::ffi::c_int
                                                                + 1 as ::core::ffi::c_int)
                                                                as isize,
                                                        ))
                                                        .hh
                                                        .u
                                                        .B0
                                                            as ::core::ffi::c_int
                                                            != hf
                                                        {
                                                            break;
                                                        }
                                                        j = hn as smallnumber;
                                                        q = (*mem.offset(
                                                            (s as ::core::ffi::c_int
                                                                + 1 as ::core::ffi::c_int)
                                                                as isize,
                                                        ))
                                                        .hh
                                                        .v
                                                        .RH;
                                                        if q as ::core::ffi::c_long
                                                            > -(268435455 as ::core::ffi::c_long)
                                                        {
                                                            hyfbchar =
                                                                (*mem.offset(q as isize)).hh.u.B1
                                                                    as halfword;
                                                        }
                                                        while q as ::core::ffi::c_long
                                                            > -(268435455 as ::core::ffi::c_long)
                                                        {
                                                            c = (*mem.offset(q as isize)).hh.u.B1
                                                                as ::core::ffi::c_uchar;
                                                            if hyphindex == 0 as ::core::ffi::c_int
                                                            {
                                                                hc[0 as ::core::ffi::c_int
                                                                    as usize] = (*eqtb.offset(
                                                                    (27997 as ::core::ffi::c_int
                                                                        + c as ::core::ffi::c_int)
                                                                        as isize,
                                                                ))
                                                                .hh
                                                                .v
                                                                .RH
                                                                    as ::core::ffi::c_short;
                                                            } else if *trietrc.offset(
                                                                (hyphindex as ::core::ffi::c_int
                                                                    + c as ::core::ffi::c_int)
                                                                    as isize,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != c as ::core::ffi::c_int
                                                            {
                                                                hc[0 as ::core::ffi::c_int
                                                                    as usize] =
                                                                    0 as ::core::ffi::c_short;
                                                            } else {
                                                                hc[0 as ::core::ffi::c_int
                                                                    as usize] = *trietro.offset(
                                                                    (hyphindex
                                                                        as ::core::ffi::c_int
                                                                        + c as ::core::ffi::c_int)
                                                                        as isize,
                                                                )
                                                                    as ::core::ffi::c_short;
                                                            }
                                                            if hc[0 as ::core::ffi::c_int as usize]
                                                                as ::core::ffi::c_int
                                                                == 0 as ::core::ffi::c_int
                                                            {
                                                                break 's_930;
                                                            }
                                                            if j as ::core::ffi::c_int
                                                                == 63 as ::core::ffi::c_int
                                                            {
                                                                break 's_930;
                                                            }
                                                            j = j.wrapping_add(1);
                                                            hu[j as usize] =
                                                                c as ::core::ffi::c_short;
                                                            hc[j as usize] = hc
                                                                [0 as ::core::ffi::c_int as usize];
                                                            q = (*mem.offset(q as isize)).hh.v.RH;
                                                        }
                                                        hb = s;
                                                        hn = j as ::core::ffi::c_uchar;
                                                        if (*mem.offset(s as isize)).hh.u.B1
                                                            as ::core::ffi::c_int
                                                            & 1 as ::core::ffi::c_int
                                                            != 0
                                                        {
                                                            hyfbchar = *fontbchar
                                                                .offset(hf as isize)
                                                                as halfword;
                                                        } else {
                                                            hyfbchar = 256 as ::core::ffi::c_int
                                                                as halfword;
                                                        }
                                                    } else {
                                                        if !((*mem.offset(s as isize)).hh.u.B0
                                                            as ::core::ffi::c_int
                                                            == 11 as ::core::ffi::c_int
                                                            && (*mem.offset(s as isize)).hh.u.B1
                                                                as ::core::ffi::c_int
                                                                == 0 as ::core::ffi::c_int)
                                                        {
                                                            break;
                                                        }
                                                        hb = s;
                                                        hyfbchar = *fontbchar.offset(hf as isize)
                                                            as halfword;
                                                    }
                                                    s = (*mem.offset(s as isize)).hh.v.RH;
                                                }
                                                if !((hn as ::core::ffi::c_int) < lhyf + rhyf) {
                                                    loop {
                                                        if !(s >= himemmin) {
                                                            match (*mem.offset(s as isize)).hh.u.B0
                                                                as ::core::ffi::c_int
                                                            {
                                                                6 => {}
                                                                11 => {
                                                                    current_block =
                                                                        5737028906218621319;
                                                                    match current_block {
                                                                        12869885497274271669 => {
                                                                            if (*mem.offset(s as isize)).hh.u.B1 as ::core::ffi::c_int
                                                                                >= 4 as ::core::ffi::c_int
                                                                            {
                                                                                current_block = 3531178331085578112;
                                                                                break;
                                                                            } else {
                                                                                current_block = 309319537768397308;
                                                                                break;
                                                                            }
                                                                        }
                                                                        _ => {
                                                                            if (*mem.offset(s as isize)).hh.u.B1 as ::core::ffi::c_int
                                                                                != 0 as ::core::ffi::c_int
                                                                            {
                                                                                current_block = 3531178331085578112;
                                                                                break;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                8 | 10 | 12 | 3 | 5 | 4 => {
                                                                    current_block =
                                                                        3531178331085578112;
                                                                    break;
                                                                }
                                                                9 => {
                                                                    current_block =
                                                                        12869885497274271669;
                                                                    match current_block {
                                                                        12869885497274271669 => {
                                                                            if (*mem.offset(s as isize)).hh.u.B1 as ::core::ffi::c_int
                                                                                >= 4 as ::core::ffi::c_int
                                                                            {
                                                                                current_block = 3531178331085578112;
                                                                                break;
                                                                            } else {
                                                                                current_block = 309319537768397308;
                                                                                break;
                                                                            }
                                                                        }
                                                                        _ => {
                                                                            if (*mem.offset(s as isize)).hh.u.B1 as ::core::ffi::c_int
                                                                                != 0 as ::core::ffi::c_int
                                                                            {
                                                                                current_block = 3531178331085578112;
                                                                                break;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                _ => {
                                                                    current_block =
                                                                        309319537768397308;
                                                                    break;
                                                                }
                                                            }
                                                        }
                                                        s = (*mem.offset(s as isize)).hh.v.RH;
                                                    }
                                                    match current_block {
                                                        309319537768397308 => {}
                                                        _ => {
                                                            hyphenate();
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                11 => {
                    if (*mem.offset(curp as isize)).hh.u.B1 as ::core::ffi::c_int
                        == 1 as ::core::ffi::c_int
                    {
                        if !((*mem.offset(curp as isize)).hh.v.RH >= himemmin) && autobreaking != 0
                        {
                            if (*mem.offset((*mem.offset(curp as isize)).hh.v.RH as isize))
                                .hh
                                .u
                                .B0 as ::core::ffi::c_int
                                == 10 as ::core::ffi::c_int
                            {
                                ztrybreak(
                                    0 as ::core::ffi::c_int,
                                    0 as ::core::ffi::c_int as smallnumber,
                                );
                            }
                        }
                        activewidth[1 as ::core::ffi::c_int as usize] =
                            (activewidth[1 as ::core::ffi::c_int as usize]
                                + (*mem.offset(
                                    (curp as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                ))
                                .u
                                .CINT) as scaled;
                    } else {
                        activewidth[1 as ::core::ffi::c_int as usize] =
                            (activewidth[1 as ::core::ffi::c_int as usize]
                                + (*mem.offset(
                                    (curp as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                ))
                                .u
                                .CINT) as scaled;
                        if (*eqtb.offset(29360 as ::core::ffi::c_int as isize)).u.CINT
                            > 1 as ::core::ffi::c_int
                            && (*mem.offset(curp as isize)).hh.u.B1 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                        {
                            activewidth[7 as ::core::ffi::c_int as usize] =
                                activewidth[7 as ::core::ffi::c_int as usize] + zkernstretch(curp);
                            activewidth[8 as ::core::ffi::c_int as usize] =
                                activewidth[8 as ::core::ffi::c_int as usize] + zkernshrink(curp);
                        }
                    }
                }
                6 => {
                    f = (*mem
                        .offset((curp as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                    .hh
                    .u
                    .B0 as internalfontnumber;
                    activewidth[1 as ::core::ffi::c_int as usize] = (activewidth
                        [1 as ::core::ffi::c_int as usize]
                        + (*fontinfo.offset(
                            (*widthbase.offset(f as isize) as ::core::ffi::c_int
                                + (*fontinfo.offset(
                                    (*charbase.offset(f as isize)
                                        + zeffectivechar(
                                            1 as ::core::ffi::c_int,
                                            f,
                                            (*mem.offset(
                                                (curp as ::core::ffi::c_int
                                                    + 1 as ::core::ffi::c_int)
                                                    as isize,
                                            ))
                                            .hh
                                            .u
                                            .B1
                                                as quarterword,
                                        )) as isize,
                                ))
                                .v
                                .QQQQ
                                .u
                                .B0 as ::core::ffi::c_int) as isize,
                        ))
                        .u
                        .CINT)
                        as scaled;
                    if (*eqtb.offset(29360 as ::core::ffi::c_int as isize)).u.CINT
                        > 1 as ::core::ffi::c_int
                        && zcheckexpandpars(f) != 0
                    {
                        prevcharp = curp;
                        activewidth[7 as ::core::ffi::c_int as usize] = activewidth
                            [7 as ::core::ffi::c_int as usize]
                            + zcharstretch(
                                f,
                                (*mem.offset(
                                    (curp as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                ))
                                .hh
                                .u
                                .B1 as eightbits,
                            );
                        activewidth[8 as ::core::ffi::c_int as usize] = activewidth
                            [8 as ::core::ffi::c_int as usize]
                            + zcharshrink(
                                f,
                                (*mem.offset(
                                    (curp as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                ))
                                .hh
                                .u
                                .B1 as eightbits,
                            );
                    }
                }
                7 => {
                    s = (*mem
                        .offset((curp as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                    .hh
                    .v
                    .LH;
                    discwidth[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int as scaled;
                    if (*eqtb.offset(29360 as ::core::ffi::c_int as isize)).u.CINT
                        > 1 as ::core::ffi::c_int
                    {
                        discwidth[7 as ::core::ffi::c_int as usize] =
                            0 as ::core::ffi::c_int as scaled;
                        discwidth[8 as ::core::ffi::c_int as usize] =
                            0 as ::core::ffi::c_int as scaled;
                    }
                    if s as ::core::ffi::c_long == -(268435455 as ::core::ffi::c_long) {
                        ztrybreak(
                            (*eqtb.offset(29281 as ::core::ffi::c_int as isize)).u.CINT,
                            1 as ::core::ffi::c_int as smallnumber,
                        );
                    } else {
                        loop {
                            if s >= himemmin {
                                f = (*mem.offset(s as isize)).hh.u.B0 as internalfontnumber;
                                discwidth[1 as ::core::ffi::c_int as usize] =
                                    (discwidth[1 as ::core::ffi::c_int as usize]
                                        + (*fontinfo.offset(
                                            (*widthbase.offset(f as isize) as ::core::ffi::c_int
                                                + (*fontinfo.offset(
                                                    (*charbase.offset(f as isize)
                                                        + zeffectivechar(
                                                            1 as ::core::ffi::c_int,
                                                            f,
                                                            (*mem.offset(s as isize)).hh.u.B1
                                                                as quarterword,
                                                        ))
                                                        as isize,
                                                ))
                                                .v
                                                .QQQQ
                                                .u
                                                .B0
                                                    as ::core::ffi::c_int)
                                                as isize,
                                        ))
                                        .u
                                        .CINT) as scaled;
                                if (*eqtb.offset(29360 as ::core::ffi::c_int as isize)).u.CINT
                                    > 1 as ::core::ffi::c_int
                                    && zcheckexpandpars(f) != 0
                                {
                                    prevcharp = s;
                                    discwidth[7 as ::core::ffi::c_int as usize] = discwidth
                                        [7 as ::core::ffi::c_int as usize]
                                        + zcharstretch(
                                            f,
                                            (*mem.offset(s as isize)).hh.u.B1 as eightbits,
                                        );
                                    discwidth[8 as ::core::ffi::c_int as usize] = discwidth
                                        [8 as ::core::ffi::c_int as usize]
                                        + zcharshrink(
                                            f,
                                            (*mem.offset(s as isize)).hh.u.B1 as eightbits,
                                        );
                                }
                            } else {
                                match (*mem.offset(s as isize)).hh.u.B0 as ::core::ffi::c_int {
                                    6 => {
                                        f = (*mem.offset(
                                            (s as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        ))
                                        .hh
                                        .u
                                        .B0
                                            as internalfontnumber;
                                        discwidth[1 as ::core::ffi::c_int as usize] = (discwidth
                                            [1 as ::core::ffi::c_int as usize]
                                            + (*fontinfo.offset(
                                                (*widthbase.offset(f as isize)
                                                    as ::core::ffi::c_int
                                                    + (*fontinfo.offset(
                                                        (*charbase.offset(f as isize)
                                                            + zeffectivechar(
                                                                1 as ::core::ffi::c_int,
                                                                f,
                                                                (*mem.offset(
                                                                    (s as ::core::ffi::c_int
                                                                        + 1 as ::core::ffi::c_int)
                                                                        as isize,
                                                                ))
                                                                .hh
                                                                .u
                                                                .B1
                                                                    as quarterword,
                                                            ))
                                                            as isize,
                                                    ))
                                                    .v
                                                    .QQQQ
                                                    .u
                                                    .B0
                                                        as ::core::ffi::c_int)
                                                    as isize,
                                            ))
                                            .u
                                            .CINT)
                                            as scaled;
                                        if (*eqtb.offset(29360 as ::core::ffi::c_int as isize))
                                            .u
                                            .CINT
                                            > 1 as ::core::ffi::c_int
                                            && zcheckexpandpars(f) != 0
                                        {
                                            prevcharp = s;
                                            discwidth[7 as ::core::ffi::c_int as usize] = discwidth
                                                [7 as ::core::ffi::c_int as usize]
                                                + zcharstretch(
                                                    f,
                                                    (*mem.offset(
                                                        (s as ::core::ffi::c_int
                                                            + 1 as ::core::ffi::c_int)
                                                            as isize,
                                                    ))
                                                    .hh
                                                    .u
                                                    .B1
                                                        as eightbits,
                                                );
                                            discwidth[8 as ::core::ffi::c_int as usize] = discwidth
                                                [8 as ::core::ffi::c_int as usize]
                                                + zcharshrink(
                                                    f,
                                                    (*mem.offset(
                                                        (s as ::core::ffi::c_int
                                                            + 1 as ::core::ffi::c_int)
                                                            as isize,
                                                    ))
                                                    .hh
                                                    .u
                                                    .B1
                                                        as eightbits,
                                                );
                                        }
                                    }
                                    0 | 1 | 2 | 11 => {
                                        discwidth[1 as ::core::ffi::c_int as usize] = (discwidth
                                            [1 as ::core::ffi::c_int as usize]
                                            + (*mem.offset(
                                                (s as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                    as isize,
                                            ))
                                            .u
                                            .CINT)
                                            as scaled;
                                        if (*mem.offset(s as isize)).hh.u.B0 as ::core::ffi::c_int
                                            == 11 as ::core::ffi::c_int
                                            && (*eqtb.offset(29360 as ::core::ffi::c_int as isize))
                                                .u
                                                .CINT
                                                > 1 as ::core::ffi::c_int
                                            && (*mem.offset(s as isize)).hh.u.B1
                                                as ::core::ffi::c_int
                                                == 0 as ::core::ffi::c_int
                                        {
                                            discwidth[7 as ::core::ffi::c_int as usize] = discwidth
                                                [7 as ::core::ffi::c_int as usize]
                                                + zkernstretch(s);
                                            discwidth[8 as ::core::ffi::c_int as usize] = discwidth
                                                [8 as ::core::ffi::c_int as usize]
                                                + zkernshrink(s);
                                        }
                                    }
                                    _ => {
                                        zconfusion(1364 as ::core::ffi::c_int);
                                    }
                                }
                            }
                            s = (*mem.offset(s as isize)).hh.v.RH;
                            if s as ::core::ffi::c_long == -(268435455 as ::core::ffi::c_long) {
                                break;
                            }
                        }
                        activewidth[1 as ::core::ffi::c_int as usize] = activewidth
                            [1 as ::core::ffi::c_int as usize]
                            + discwidth[1 as ::core::ffi::c_int as usize];
                        if (*eqtb.offset(29360 as ::core::ffi::c_int as isize)).u.CINT
                            > 1 as ::core::ffi::c_int
                        {
                            activewidth[7 as ::core::ffi::c_int as usize] = activewidth
                                [7 as ::core::ffi::c_int as usize]
                                + discwidth[7 as ::core::ffi::c_int as usize];
                            activewidth[8 as ::core::ffi::c_int as usize] = activewidth
                                [8 as ::core::ffi::c_int as usize]
                                + discwidth[8 as ::core::ffi::c_int as usize];
                        }
                        ztrybreak(
                            (*eqtb.offset(29280 as ::core::ffi::c_int as isize)).u.CINT,
                            1 as ::core::ffi::c_int as smallnumber,
                        );
                        activewidth[1 as ::core::ffi::c_int as usize] = activewidth
                            [1 as ::core::ffi::c_int as usize]
                            - discwidth[1 as ::core::ffi::c_int as usize];
                        if (*eqtb.offset(29360 as ::core::ffi::c_int as isize)).u.CINT
                            > 1 as ::core::ffi::c_int
                        {
                            activewidth[7 as ::core::ffi::c_int as usize] = activewidth
                                [7 as ::core::ffi::c_int as usize]
                                - discwidth[7 as ::core::ffi::c_int as usize];
                            activewidth[8 as ::core::ffi::c_int as usize] = activewidth
                                [8 as ::core::ffi::c_int as usize]
                                - discwidth[8 as ::core::ffi::c_int as usize];
                        }
                    }
                    r = (*mem.offset(curp as isize)).hh.u.B1 as halfword;
                    s = (*mem.offset(curp as isize)).hh.v.RH;
                    while r > 0 as ::core::ffi::c_int {
                        if s >= himemmin {
                            f = (*mem.offset(s as isize)).hh.u.B0 as internalfontnumber;
                            activewidth[1 as ::core::ffi::c_int as usize] =
                                (activewidth[1 as ::core::ffi::c_int as usize]
                                    + (*fontinfo.offset(
                                        (*widthbase.offset(f as isize) as ::core::ffi::c_int
                                            + (*fontinfo.offset(
                                                (*charbase.offset(f as isize)
                                                    + zeffectivechar(
                                                        1 as ::core::ffi::c_int,
                                                        f,
                                                        (*mem.offset(s as isize)).hh.u.B1
                                                            as quarterword,
                                                    ))
                                                    as isize,
                                            ))
                                            .v
                                            .QQQQ
                                            .u
                                            .B0
                                                as ::core::ffi::c_int)
                                            as isize,
                                    ))
                                    .u
                                    .CINT) as scaled;
                            if (*eqtb.offset(29360 as ::core::ffi::c_int as isize)).u.CINT
                                > 1 as ::core::ffi::c_int
                                && zcheckexpandpars(f) != 0
                            {
                                prevcharp = s;
                                activewidth[7 as ::core::ffi::c_int as usize] = activewidth
                                    [7 as ::core::ffi::c_int as usize]
                                    + zcharstretch(
                                        f,
                                        (*mem.offset(s as isize)).hh.u.B1 as eightbits,
                                    );
                                activewidth[8 as ::core::ffi::c_int as usize] = activewidth
                                    [8 as ::core::ffi::c_int as usize]
                                    + zcharshrink(
                                        f,
                                        (*mem.offset(s as isize)).hh.u.B1 as eightbits,
                                    );
                            }
                        } else {
                            match (*mem.offset(s as isize)).hh.u.B0 as ::core::ffi::c_int {
                                6 => {
                                    f = (*mem.offset(
                                        (s as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            as isize,
                                    ))
                                    .hh
                                    .u
                                    .B0
                                        as internalfontnumber;
                                    activewidth[1 as ::core::ffi::c_int as usize] = (activewidth
                                        [1 as ::core::ffi::c_int as usize]
                                        + (*fontinfo.offset(
                                            (*widthbase.offset(f as isize) as ::core::ffi::c_int
                                                + (*fontinfo.offset(
                                                    (*charbase.offset(f as isize)
                                                        + zeffectivechar(
                                                            1 as ::core::ffi::c_int,
                                                            f,
                                                            (*mem.offset(
                                                                (s as ::core::ffi::c_int
                                                                    + 1 as ::core::ffi::c_int)
                                                                    as isize,
                                                            ))
                                                            .hh
                                                            .u
                                                            .B1
                                                                as quarterword,
                                                        ))
                                                        as isize,
                                                ))
                                                .v
                                                .QQQQ
                                                .u
                                                .B0
                                                    as ::core::ffi::c_int)
                                                as isize,
                                        ))
                                        .u
                                        .CINT)
                                        as scaled;
                                    if (*eqtb.offset(29360 as ::core::ffi::c_int as isize)).u.CINT
                                        > 1 as ::core::ffi::c_int
                                        && zcheckexpandpars(f) != 0
                                    {
                                        prevcharp = s;
                                        activewidth[7 as ::core::ffi::c_int as usize] = activewidth
                                            [7 as ::core::ffi::c_int as usize]
                                            + zcharstretch(
                                                f,
                                                (*mem.offset(
                                                    (s as ::core::ffi::c_int
                                                        + 1 as ::core::ffi::c_int)
                                                        as isize,
                                                ))
                                                .hh
                                                .u
                                                .B1
                                                    as eightbits,
                                            );
                                        activewidth[8 as ::core::ffi::c_int as usize] = activewidth
                                            [8 as ::core::ffi::c_int as usize]
                                            + zcharshrink(
                                                f,
                                                (*mem.offset(
                                                    (s as ::core::ffi::c_int
                                                        + 1 as ::core::ffi::c_int)
                                                        as isize,
                                                ))
                                                .hh
                                                .u
                                                .B1
                                                    as eightbits,
                                            );
                                    }
                                }
                                0 | 1 | 2 | 11 => {
                                    activewidth[1 as ::core::ffi::c_int as usize] = (activewidth
                                        [1 as ::core::ffi::c_int as usize]
                                        + (*mem.offset(
                                            (s as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        ))
                                        .u
                                        .CINT)
                                        as scaled;
                                    if (*mem.offset(s as isize)).hh.u.B0 as ::core::ffi::c_int
                                        == 11 as ::core::ffi::c_int
                                        && (*eqtb.offset(29360 as ::core::ffi::c_int as isize))
                                            .u
                                            .CINT
                                            > 1 as ::core::ffi::c_int
                                        && (*mem.offset(s as isize)).hh.u.B1 as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int
                                    {
                                        activewidth[7 as ::core::ffi::c_int as usize] = activewidth
                                            [7 as ::core::ffi::c_int as usize]
                                            + zkernstretch(s);
                                        activewidth[8 as ::core::ffi::c_int as usize] = activewidth
                                            [8 as ::core::ffi::c_int as usize]
                                            + zkernshrink(s);
                                    }
                                }
                                _ => {
                                    zconfusion(1365 as ::core::ffi::c_int);
                                }
                            }
                        }
                        r -= 1;
                        s = (*mem.offset(s as isize)).hh.v.RH;
                    }
                    prevp = curp;
                    curp = s;
                    continue;
                }
                9 => {
                    if ((*mem.offset(curp as isize)).hh.u.B1 as ::core::ffi::c_int)
                        < 4 as ::core::ffi::c_int
                    {
                        autobreaking = ((*mem.offset(curp as isize)).hh.u.B1 as ::core::ffi::c_int
                            & 1 as ::core::ffi::c_int)
                            as boolean;
                    }
                    if !((*mem.offset(curp as isize)).hh.v.RH >= himemmin) && autobreaking != 0 {
                        if (*mem.offset((*mem.offset(curp as isize)).hh.v.RH as isize))
                            .hh
                            .u
                            .B0 as ::core::ffi::c_int
                            == 10 as ::core::ffi::c_int
                        {
                            ztrybreak(
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int as smallnumber,
                            );
                        }
                    }
                    activewidth[1 as ::core::ffi::c_int as usize] =
                        (activewidth[1 as ::core::ffi::c_int as usize]
                            + (*mem.offset(
                                (curp as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ))
                            .u
                            .CINT) as scaled;
                }
                12 => {
                    ztrybreak(
                        (*mem.offset(
                            (curp as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                        ))
                        .u
                        .CINT,
                        0 as ::core::ffi::c_int as smallnumber,
                    );
                }
                4 | 3 | 5 => {}
                _ => {
                    zconfusion(1363 as ::core::ffi::c_int);
                }
            }
            prevp = curp;
            curp = (*mem.offset(curp as isize)).hh.v.RH;
        }
        if curp as ::core::ffi::c_long == -(268435455 as ::core::ffi::c_long) {
            ztrybreak(
                -(10000 as ::core::ffi::c_int),
                1 as ::core::ffi::c_int as smallnumber,
            );
            if (*mem.offset((memtop as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as isize))
                .hh
                .v
                .RH
                != memtop as ::core::ffi::c_int - 7 as ::core::ffi::c_int
            {
                r = (*mem
                    .offset((memtop as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as isize))
                .hh
                .v
                .RH;
                fewestdemerits = 1073741823 as ::core::ffi::c_long as integer;
                loop {
                    if (*mem.offset(r as isize)).hh.u.B0 as ::core::ffi::c_int
                        != 2 as ::core::ffi::c_int
                    {
                        if (*mem
                            .offset((r as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
                        .u
                        .CINT
                            < fewestdemerits
                        {
                            fewestdemerits = (*mem.offset(
                                (r as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                            ))
                            .u
                            .CINT;
                            bestbet = r;
                        }
                    }
                    r = (*mem.offset(r as isize)).hh.v.RH;
                    if r == memtop as ::core::ffi::c_int - 7 as ::core::ffi::c_int {
                        break;
                    }
                }
                bestline = (*mem
                    .offset((bestbet as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                .hh
                .v
                .LH;
                if (*eqtb.offset(29296 as ::core::ffi::c_int as isize)).u.CINT
                    == 0 as ::core::ffi::c_int
                {
                    break;
                }
                r = (*mem
                    .offset((memtop as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as isize))
                .hh
                .v
                .RH;
                actuallooseness = 0 as ::core::ffi::c_int as integer;
                loop {
                    if (*mem.offset(r as isize)).hh.u.B0 as ::core::ffi::c_int
                        != 2 as ::core::ffi::c_int
                    {
                        linediff = ((*mem
                            .offset((r as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                        .hh
                        .v
                        .LH - bestline) as integer;
                        if linediff < actuallooseness
                            && (*eqtb.offset(29296 as ::core::ffi::c_int as isize)).u.CINT
                                <= linediff
                            || linediff > actuallooseness
                                && (*eqtb.offset(29296 as ::core::ffi::c_int as isize)).u.CINT
                                    >= linediff
                        {
                            bestbet = r;
                            actuallooseness = linediff;
                            fewestdemerits = (*mem.offset(
                                (r as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                            ))
                            .u
                            .CINT;
                        } else if linediff == actuallooseness
                            && (*mem.offset(
                                (r as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                            ))
                            .u
                            .CINT
                                < fewestdemerits
                        {
                            bestbet = r;
                            fewestdemerits = (*mem.offset(
                                (r as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                            ))
                            .u
                            .CINT;
                        }
                    }
                    r = (*mem.offset(r as isize)).hh.v.RH;
                    if r == memtop as ::core::ffi::c_int - 7 as ::core::ffi::c_int {
                        break;
                    }
                }
                bestline = (*mem
                    .offset((bestbet as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                .hh
                .v
                .LH;
                if actuallooseness == (*eqtb.offset(29296 as ::core::ffi::c_int as isize)).u.CINT
                    || finalpass != 0
                {
                    break;
                }
            }
        }
        q = (*mem.offset((memtop as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as isize))
            .hh
            .v
            .RH;
        while q != memtop as ::core::ffi::c_int - 7 as ::core::ffi::c_int {
            curp = (*mem.offset(q as isize)).hh.v.RH;
            if (*mem.offset(q as isize)).hh.u.B0 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                zfreenode(q, 9 as ::core::ffi::c_int);
            } else {
                zfreenode(q, activenodesize as halfword);
            }
            q = curp;
        }
        q = passive;
        while q as ::core::ffi::c_long != -(268435455 as ::core::ffi::c_long) {
            curp = (*mem.offset(q as isize)).hh.v.RH;
            zfreenode(q, 2 as ::core::ffi::c_int);
            q = curp;
        }
        if secondpass == 0 {
            if (*eqtb.offset(29309 as ::core::ffi::c_int as isize)).u.CINT > 0 as ::core::ffi::c_int
            {
                zprintnl(1361 as ::core::ffi::c_int);
            }
            threshold = (*eqtb.offset(29278 as ::core::ffi::c_int as isize)).u.CINT;
            secondpass = true_0 as boolean;
            finalpass = ((*eqtb.offset(29923 as ::core::ffi::c_int as isize)).u.CINT
                <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                as boolean;
        } else {
            if (*eqtb.offset(29309 as ::core::ffi::c_int as isize)).u.CINT > 0 as ::core::ffi::c_int
            {
                zprintnl(1362 as ::core::ffi::c_int);
            }
            background[2 as ::core::ffi::c_int as usize] = (background
                [2 as ::core::ffi::c_int as usize]
                + (*eqtb.offset(29923 as ::core::ffi::c_int as isize)).u.CINT)
                as scaled;
            finalpass = true_0 as boolean;
        }
    }
    if (*eqtb.offset(29309 as ::core::ffi::c_int as isize)).u.CINT > 0 as ::core::ffi::c_int {
        zenddiagnostic(1 as ::core::ffi::c_int);
        normalizeselector();
    }
    if dolastlinefit != 0 {
        if (*mem.offset((bestbet as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize))
            .u
            .CINT
            == 0 as ::core::ffi::c_int
        {
            dolastlinefit = false_0 as boolean;
        } else {
            q = znewspec(
                (*mem.offset(
                    (lastlinefill as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .hh
                .v
                .LH,
            );
            zdeleteglueref(
                (*mem.offset(
                    (lastlinefill as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ))
                .hh
                .v
                .LH,
            );
            (*mem.offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                .u
                .CINT = (*mem.offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                .u
                .CINT
                + (*mem.offset((bestbet as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize))
                    .u
                    .CINT
                - (*mem.offset((bestbet as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize))
                    .u
                    .CINT;
            (*mem.offset((q as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
                .u
                .CINT = 0 as ::core::ffi::c_int as integer;
            (*mem
                .offset((lastlinefill as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
            .hh
            .v
            .LH = q;
        }
    }
    zpostlinebreak(d);
    q = (*mem.offset((memtop as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as isize))
        .hh
        .v
        .RH;
    while q != memtop as ::core::ffi::c_int - 7 as ::core::ffi::c_int {
        curp = (*mem.offset(q as isize)).hh.v.RH;
        if (*mem.offset(q as isize)).hh.u.B0 as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            zfreenode(q, 9 as ::core::ffi::c_int);
        } else {
            zfreenode(q, activenodesize as halfword);
        }
        q = curp;
    }
    q = passive;
    while q as ::core::ffi::c_long != -(268435455 as ::core::ffi::c_long) {
        curp = (*mem.offset(q as isize)).hh.v.RH;
        zfreenode(q, 2 as ::core::ffi::c_int);
        q = curp;
    }
    packbeginline = 0 as ::core::ffi::c_int as integer;
}
#[no_mangle]
pub unsafe extern "C" fn newhyphexceptions() {
    let mut current_block: u64;
    let mut mem: *mut memoryword = zmem;
    let mut eqtb: *mut memoryword = zeqtb;
    let mut n: ::core::ffi::c_uchar = 0;
    let mut j: ::core::ffi::c_uchar = 0;
    let mut h: hyphpointer = 0;
    let mut k_0: strnumber = 0;
    let mut p: halfword = 0;
    let mut q: halfword = 0;
    let mut s: strnumber = 0;
    let mut u: poolpointer = 0;
    let mut v: poolpointer = 0;
    scanleftbrace();
    if (*eqtb.offset(29327 as ::core::ffi::c_int as isize)).u.CINT <= 0 as ::core::ffi::c_int {
        curlang = 0 as ASCIIcode;
    } else if (*eqtb.offset(29327 as ::core::ffi::c_int as isize)).u.CINT
        > 255 as ::core::ffi::c_int
    {
        curlang = 0 as ASCIIcode;
    } else {
        curlang = (*eqtb.offset(29327 as ::core::ffi::c_int as isize)).u.CINT as ASCIIcode;
    }
    if trienotready != 0 {
        hyphindex = 0 as ::core::ffi::c_int as triepointer;
    } else if *trietrc
        .offset((hyphstart as ::core::ffi::c_int + curlang as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int
        != curlang as ::core::ffi::c_int
    {
        hyphindex = 0 as ::core::ffi::c_int as triepointer;
    } else {
        hyphindex = *trietrl
            .offset((hyphstart as ::core::ffi::c_int + curlang as ::core::ffi::c_int) as isize);
    }
    n = 0 as ::core::ffi::c_uchar;
    p = -(268435455 as ::core::ffi::c_long) as halfword;
    's_79: loop {
        getxtoken();
        loop {
            match curcmd as ::core::ffi::c_int {
                11 | 12 | 68 => {
                    if curchr == 45 as ::core::ffi::c_int {
                        if (n as ::core::ffi::c_int) < 63 as ::core::ffi::c_int {
                            q = getavail();
                            (*mem.offset(q as isize)).hh.v.RH = p;
                            (*mem.offset(q as isize)).hh.v.LH = n as halfword;
                            p = q;
                        }
                    } else {
                        if hyphindex == 0 as ::core::ffi::c_int {
                            hc[0 as ::core::ffi::c_int as usize] =
                                (*eqtb.offset((27997 as halfword + curchr) as isize))
                                    .hh
                                    .v
                                    .RH as ::core::ffi::c_short;
                        } else if *trietrc.offset((hyphindex as halfword + curchr) as isize)
                            as ::core::ffi::c_int
                            != curchr
                        {
                            hc[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_short;
                        } else {
                            hc[0 as ::core::ffi::c_int as usize] = *trietro
                                .offset((hyphindex as halfword + curchr) as isize)
                                as ::core::ffi::c_short;
                        }
                        if hc[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            interaction as ::core::ffi::c_int == 3 as ::core::ffi::c_int;
                            if filelineerrorstylep != 0 {
                                printfileline();
                            } else {
                                zprintnl(264 as ::core::ffi::c_int);
                            }
                            zprint(1372 as ::core::ffi::c_int);
                            helpptr = 2 as ::core::ffi::c_uchar;
                            helpline[1 as ::core::ffi::c_int as usize] =
                                1373 as ::core::ffi::c_int as strnumber;
                            helpline[0 as ::core::ffi::c_int as usize] =
                                1374 as ::core::ffi::c_int as strnumber;
                            error();
                        } else if (n as ::core::ffi::c_int) < 63 as ::core::ffi::c_int {
                            n = n.wrapping_add(1);
                            hc[n as usize] = hc[0 as ::core::ffi::c_int as usize];
                        }
                    }
                    continue 's_79;
                }
                16 => {
                    scancharnum();
                    curchr = curval as halfword;
                    curcmd = 68 as eightbits;
                }
                10 | 2 => {
                    if n as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                        current_block = 10399321362245223758;
                        break;
                    } else {
                        current_block = 14329534724295951598;
                        break;
                    }
                }
                _ => {
                    interaction as ::core::ffi::c_int == 3 as ::core::ffi::c_int;
                    if filelineerrorstylep != 0 {
                        printfileline();
                    } else {
                        zprintnl(264 as ::core::ffi::c_int);
                    }
                    zprint(792 as ::core::ffi::c_int);
                    zprintesc(1368 as ::core::ffi::c_int);
                    zprint(1369 as ::core::ffi::c_int);
                    helpptr = 2 as ::core::ffi::c_uchar;
                    helpline[1 as ::core::ffi::c_int as usize] =
                        1370 as ::core::ffi::c_int as strnumber;
                    helpline[0 as ::core::ffi::c_int as usize] =
                        1371 as ::core::ffi::c_int as strnumber;
                    error();
                    continue 's_79;
                }
            }
        }
        match current_block {
            10399321362245223758 => {
                n = n.wrapping_add(1);
                hc[n as usize] = curlang as ::core::ffi::c_short;
                if poolptr as ::core::ffi::c_int + n as ::core::ffi::c_int > poolsize {
                    zoverflow(
                        259 as ::core::ffi::c_int,
                        poolsize as poolpointer - initpoolptr,
                    );
                }
                h = 0 as hyphpointer;
                let mut for_end: integer = 0;
                j = 1 as ::core::ffi::c_uchar;
                for_end = n as integer;
                if j as ::core::ffi::c_int <= for_end {
                    loop {
                        h = ((h as ::core::ffi::c_int
                            + h as ::core::ffi::c_int
                            + hc[j as usize] as ::core::ffi::c_int)
                            % 607 as ::core::ffi::c_int) as hyphpointer;
                        *strpool.offset(poolptr as isize) = hc[j as usize] as packedASCIIcode;
                        poolptr += 1;
                        let fresh86 = j;
                        j = j.wrapping_add(1);
                        if !((fresh86 as ::core::ffi::c_int) < for_end) {
                            break;
                        }
                    }
                }
                s = makestring();
                if hyphnext <= 607 as ::core::ffi::c_int {
                    while hyphnext > 0 as ::core::ffi::c_int
                        && *hyphword.offset(
                            (hyphnext as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
                        ) > 0 as ::core::ffi::c_int
                    {
                        hyphnext -= 1;
                    }
                }
                if hyphcount == hyphsize || hyphnext == 0 as ::core::ffi::c_int {
                    zoverflow(1375 as ::core::ffi::c_int, hyphsize);
                }
                hyphcount += 1;
                while *hyphword.offset(h as isize) != 0 as ::core::ffi::c_int {
                    k_0 = *hyphword.offset(h as isize);
                    if !(*strstart
                        .offset((k_0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        - *strstart.offset(k_0 as isize)
                        != *strstart
                            .offset((s as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                            - *strstart.offset(s as isize))
                    {
                        u = *strstart.offset(k_0 as isize);
                        v = *strstart.offset(s as isize);
                        loop {
                            if *strpool.offset(u as isize) as ::core::ffi::c_int
                                != *strpool.offset(v as isize) as ::core::ffi::c_int
                            {
                                current_block = 10435735846551762309;
                                break;
                            }
                            u += 1;
                            v += 1;
                            if u == *strstart.offset(
                                (k_0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ) {
                                current_block = 14298507163138330979;
                                break;
                            }
                        }
                        match current_block {
                            10435735846551762309 => {}
                            _ => {
                                strptr -= 1;
                                poolptr = *strstart.offset(strptr as isize) as integer;
                                s = *hyphword.offset(h as isize);
                                hyphcount -= 1;
                                break;
                            }
                        }
                    }
                    if *hyphlink.offset(h as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        *hyphlink.offset(h as isize) = hyphnext as hyphpointer;
                        if hyphnext >= hyphsize {
                            hyphnext = 607 as ::core::ffi::c_int as integer;
                        }
                        if hyphnext > 607 as ::core::ffi::c_int {
                            hyphnext += 1;
                        }
                    }
                    h = (*hyphlink.offset(h as isize) as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as hyphpointer;
                }
                *hyphword.offset(h as isize) = s;
                *hyphlist.offset(h as isize) = p;
            }
            _ => {}
        }
        if curcmd as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            return;
        }
        n = 0 as ::core::ffi::c_uchar;
        p = -(268435455 as ::core::ffi::c_long) as halfword;
    }
}
#[no_mangle]
pub unsafe extern "C" fn zdomarks(
    mut a: smallnumber,
    mut l: smallnumber,
    mut q: halfword,
) -> boolean {
    let mut Result: boolean = 0;
    let mut mem: *mut memoryword = zmem;
    let mut i: smallnumber = 0;
    if (l as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
        let mut for_end: integer = 0;
        i = 0 as smallnumber;
        for_end = 15 as ::core::ffi::c_int as integer;
        if i as ::core::ffi::c_int <= for_end {
            loop {
                if i as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                    curptr = (*mem.offset(
                        (q as ::core::ffi::c_int
                            + i as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as isize,
                    ))
                    .hh
                    .v
                    .RH;
                } else {
                    curptr = (*mem.offset(
                        (q as ::core::ffi::c_int
                            + i as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as isize,
                    ))
                    .hh
                    .v
                    .LH;
                }
                if curptr as ::core::ffi::c_long != -(268435455 as ::core::ffi::c_long) {
                    if zdomarks(
                        a,
                        (l as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as smallnumber,
                        curptr,
                    ) != 0
                    {
                        if i as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                            (*mem.offset(
                                (q as ::core::ffi::c_int
                                    + i as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int)
                                    as isize,
                            ))
                            .hh
                            .v
                            .RH = -(268435455 as ::core::ffi::c_long) as halfword;
                        } else {
                            (*mem.offset(
                                (q as ::core::ffi::c_int
                                    + i as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int)
                                    as isize,
                            ))
                            .hh
                            .v
                            .LH = -(268435455 as ::core::ffi::c_long) as halfword;
                        }
                        let ref mut fresh26 = (*mem.offset(q as isize)).hh.u.B1;
                        *fresh26 -= 1;
                    }
                }
                let fresh27 = i;
                i = i.wrapping_add(1);
                if !((fresh27 as ::core::ffi::c_int) < for_end) {
                    break;
                }
            }
        }
        if (*mem.offset(q as isize)).hh.u.B1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            zfreenode(q, 9 as ::core::ffi::c_int);
            q = -(268435455 as ::core::ffi::c_long) as halfword;
        }
    } else {
        match a as ::core::ffi::c_int {
            0 => {
                if (*mem.offset((q as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
                    .hh
                    .v
                    .RH as ::core::ffi::c_long
                    != -(268435455 as ::core::ffi::c_long)
                {
                    zdeletetokenref(
                        (*mem.offset((q as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
                            .hh
                            .v
                            .RH,
                    );
                    (*mem.offset((q as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
                        .hh
                        .v
                        .RH = -(268435455 as ::core::ffi::c_long) as halfword;
                    zdeletetokenref(
                        (*mem.offset((q as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize))
                            .hh
                            .v
                            .LH,
                    );
                    (*mem.offset((q as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize))
                        .hh
                        .v
                        .LH = -(268435455 as ::core::ffi::c_long) as halfword;
                }
            }
            1 => {
                if (*mem.offset((q as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
                    .hh
                    .v
                    .LH as ::core::ffi::c_long
                    != -(268435455 as ::core::ffi::c_long)
                {
                    if (*mem.offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                        .hh
                        .v
                        .LH as ::core::ffi::c_long
                        != -(268435455 as ::core::ffi::c_long)
                    {
                        zdeletetokenref(
                            (*mem.offset(
                                (q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ))
                            .hh
                            .v
                            .LH,
                        );
                    }
                    zdeletetokenref(
                        (*mem.offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                            .hh
                            .v
                            .RH,
                    );
                    (*mem.offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                        .hh
                        .v
                        .RH = -(268435455 as ::core::ffi::c_long) as halfword;
                    if (*mem.offset(
                        (*mem.offset((q as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
                            .hh
                            .v
                            .LH as isize,
                    ))
                    .hh
                    .v
                    .RH as ::core::ffi::c_long
                        == -(268435455 as ::core::ffi::c_long)
                    {
                        zdeletetokenref(
                            (*mem.offset(
                                (q as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                            ))
                            .hh
                            .v
                            .LH,
                        );
                        (*mem.offset(
                            (q as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                        ))
                        .hh
                        .v
                        .LH = -(268435455 as ::core::ffi::c_long) as halfword;
                    } else {
                        let ref mut fresh28 = (*mem.offset(
                            (*mem.offset(
                                (q as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                            ))
                            .hh
                            .v
                            .LH as isize,
                        ))
                        .hh
                        .v
                        .LH;
                        *fresh28 += 1;
                    }
                    (*mem.offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                        .hh
                        .v
                        .LH = (*mem
                        .offset((q as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
                    .hh
                    .v
                    .LH;
                }
            }
            2 => {
                if (*mem.offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                    .hh
                    .v
                    .LH as ::core::ffi::c_long
                    != -(268435455 as ::core::ffi::c_long)
                    && (*mem.offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                        .hh
                        .v
                        .RH as ::core::ffi::c_long
                        == -(268435455 as ::core::ffi::c_long)
                {
                    (*mem.offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                        .hh
                        .v
                        .RH = (*mem
                        .offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                    .hh
                    .v
                    .LH;
                    let ref mut fresh29 = (*mem.offset(
                        (*mem.offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                            .hh
                            .v
                            .LH as isize,
                    ))
                    .hh
                    .v
                    .LH;
                    *fresh29 += 1;
                }
            }
            3 => {
                let mut for_end_0: integer = 0;
                i = 0 as smallnumber;
                for_end_0 = 4 as ::core::ffi::c_int as integer;
                if i as ::core::ffi::c_int <= for_end_0 {
                    loop {
                        if i as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                            curptr = (*mem.offset(
                                (q as ::core::ffi::c_int
                                    + i as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int)
                                    as isize,
                            ))
                            .hh
                            .v
                            .RH;
                        } else {
                            curptr = (*mem.offset(
                                (q as ::core::ffi::c_int
                                    + i as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int)
                                    as isize,
                            ))
                            .hh
                            .v
                            .LH;
                        }
                        if curptr as ::core::ffi::c_long != -(268435455 as ::core::ffi::c_long) {
                            zdeletetokenref(curptr);
                            if i as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                                (*mem.offset(
                                    (q as ::core::ffi::c_int
                                        + i as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                                        + 1 as ::core::ffi::c_int)
                                        as isize,
                                ))
                                .hh
                                .v
                                .RH = -(268435455 as ::core::ffi::c_long) as halfword;
                            } else {
                                (*mem.offset(
                                    (q as ::core::ffi::c_int
                                        + i as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                                        + 1 as ::core::ffi::c_int)
                                        as isize,
                                ))
                                .hh
                                .v
                                .LH = -(268435455 as ::core::ffi::c_long) as halfword;
                            }
                        }
                        let fresh30 = i;
                        i = i.wrapping_add(1);
                        if !((fresh30 as ::core::ffi::c_int) < for_end_0) {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
        if (*mem.offset((q as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize))
            .hh
            .v
            .LH as ::core::ffi::c_long
            == -(268435455 as ::core::ffi::c_long)
        {
            if (*mem.offset((q as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize))
                .hh
                .v
                .LH as ::core::ffi::c_long
                == -(268435455 as ::core::ffi::c_long)
            {
                zfreenode(q, 4 as ::core::ffi::c_int);
                q = -(268435455 as ::core::ffi::c_long) as halfword;
            }
        }
    }
    Result = (q as ::core::ffi::c_long == -(268435455 as ::core::ffi::c_long)) as ::core::ffi::c_int
        as boolean;
    return Result;
}
#[no_mangle]
pub unsafe extern "C" fn prefixedcommand() {
    let mut current_block: u64;
    let mut mem: *mut memoryword = zmem;
    let mut eqtb: *mut memoryword = zeqtb;
    let mut a: smallnumber = 0;
    let mut f: internalfontnumber = 0;
    let mut j: halfword = 0;
    let mut k_0: fontindex = 0;
    let mut p: halfword = 0;
    let mut q: halfword = 0;
    let mut r: halfword = 0;
    let mut n: integer = 0;
    let mut e: boolean = 0;
    a = 0 as smallnumber;
    while curcmd as ::core::ffi::c_int == 93 as ::core::ffi::c_int {
        if a as ::core::ffi::c_int / curchr as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 {
            a = (a as halfword + curchr) as smallnumber;
        }
        loop {
            getxtoken();
            if curcmd as ::core::ffi::c_int != 10 as ::core::ffi::c_int
                && curcmd as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            {
                break;
            }
        }
        if curcmd as ::core::ffi::c_int <= 70 as ::core::ffi::c_int {
            interaction as ::core::ffi::c_int == 3 as ::core::ffi::c_int;
            if filelineerrorstylep != 0 {
                printfileline();
            } else {
                zprintnl(264 as ::core::ffi::c_int);
            }
            zprint(1609 as ::core::ffi::c_int);
            zprintcmdchr(curcmd, curchr);
            zprintchar(39 as ::core::ffi::c_int as ASCIIcode);
            helpptr = 1 as ::core::ffi::c_uchar;
            helpline[0 as ::core::ffi::c_int as usize] = 1610 as ::core::ffi::c_int as strnumber;
            if eTeXmode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                helpline[0 as ::core::ffi::c_int as usize] =
                    1611 as ::core::ffi::c_int as strnumber;
            }
            backerror();
            return;
        }
        if (*eqtb.offset(29313 as ::core::ffi::c_int as isize)).u.CINT > 2 as ::core::ffi::c_int {
            if eTeXmode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                showcurcmdchr();
            }
        }
    }
    if a as ::core::ffi::c_int >= 8 as ::core::ffi::c_int {
        j = 3585 as ::core::ffi::c_int as halfword;
        a = (a as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as smallnumber;
    } else {
        j = 0 as ::core::ffi::c_int as halfword;
    }
    if curcmd as ::core::ffi::c_int != 97 as ::core::ffi::c_int
        && (a as ::core::ffi::c_int % 4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            || j != 0 as ::core::ffi::c_int)
    {
        interaction as ::core::ffi::c_int == 3 as ::core::ffi::c_int;
        if filelineerrorstylep != 0 {
            printfileline();
        } else {
            zprintnl(264 as ::core::ffi::c_int);
        }
        zprint(797 as ::core::ffi::c_int);
        zprintesc(1601 as ::core::ffi::c_int);
        zprint(1612 as ::core::ffi::c_int);
        zprintesc(1602 as ::core::ffi::c_int);
        helpptr = 1 as ::core::ffi::c_uchar;
        helpline[0 as ::core::ffi::c_int as usize] = 1613 as ::core::ffi::c_int as strnumber;
        if eTeXmode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            helpline[0 as ::core::ffi::c_int as usize] = 1614 as ::core::ffi::c_int as strnumber;
            zprint(1612 as ::core::ffi::c_int);
            zprintesc(1615 as ::core::ffi::c_int);
        }
        zprint(1616 as ::core::ffi::c_int);
        zprintcmdchr(curcmd, curchr);
        zprintchar(39 as ::core::ffi::c_int as ASCIIcode);
        error();
    }
    if (*eqtb.offset(29320 as ::core::ffi::c_int as isize)).u.CINT != 0 as ::core::ffi::c_int {
        if (*eqtb.offset(29320 as ::core::ffi::c_int as isize)).u.CINT < 0 as ::core::ffi::c_int {
            if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                a = (a as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as smallnumber;
            }
        } else if !(a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int) {
            a = (a as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as smallnumber;
        }
    }
    match curcmd as ::core::ffi::c_int {
        87 => {
            if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                zgeqdefine(
                    27689 as ::core::ffi::c_int,
                    123 as ::core::ffi::c_int as quarterword,
                    curchr,
                );
            } else {
                zeqdefine(
                    27689 as ::core::ffi::c_int,
                    123 as ::core::ffi::c_int as quarterword,
                    curchr,
                );
            }
        }
        97 => {
            if curchr as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0
                && !(a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int)
                && (*eqtb.offset(29320 as ::core::ffi::c_int as isize)).u.CINT
                    >= 0 as ::core::ffi::c_int
            {
                a = (a as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as smallnumber;
            }
            e = (curchr >= 2 as ::core::ffi::c_int) as ::core::ffi::c_int as boolean;
            getrtoken();
            p = curcs;
            q = zscantoks(1 as ::core::ffi::c_int, e);
            if j != 0 as ::core::ffi::c_int {
                q = getavail();
                (*mem.offset(q as isize)).hh.v.LH = j;
                (*mem.offset(q as isize)).hh.v.RH = (*mem.offset(defref as isize)).hh.v.RH;
                (*mem.offset(defref as isize)).hh.v.RH = q;
            }
            if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                zgeqdefine(
                    p,
                    (114 as ::core::ffi::c_int + a as ::core::ffi::c_int % 4 as ::core::ffi::c_int)
                        as quarterword,
                    defref,
                );
            } else {
                zeqdefine(
                    p,
                    (114 as ::core::ffi::c_int + a as ::core::ffi::c_int % 4 as ::core::ffi::c_int)
                        as quarterword,
                    defref,
                );
            }
        }
        94 => {
            if !(curchr == 11 as ::core::ffi::c_int) {
                if curchr == 10 as ::core::ffi::c_int {
                    selector = 19 as ::core::ffi::c_uchar;
                    gettoken();
                    mubytestoken = curtok;
                    if curtok <= 4095 as ::core::ffi::c_int {
                        mubytestoken =
                            (curtok as ::core::ffi::c_int % 256 as ::core::ffi::c_int) as halfword;
                    }
                    mubyteprefix = 60 as ::core::ffi::c_int as integer;
                    mubyterelax = false_0 as boolean;
                    mubytetablein = true_0 as boolean;
                    mubytetableout = true_0 as boolean;
                    getxtoken();
                    if curcmd as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
                        getxtoken();
                    }
                    if curcmd as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                        mubytetableout = false_0 as boolean;
                        getxtoken();
                        if curcmd as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                            mubytetableout = true_0 as boolean;
                            mubytetablein = false_0 as boolean;
                            getxtoken();
                        }
                    } else if mubytestoken > 4095 as ::core::ffi::c_int
                        && curcmd as ::core::ffi::c_int == 6 as ::core::ffi::c_int
                    {
                        mubytetableout = false_0 as boolean;
                        scanint();
                        mubyteprefix = curval;
                        getxtoken();
                        if mubyteprefix > 50 as ::core::ffi::c_int {
                            mubyteprefix = 52 as ::core::ffi::c_int as integer;
                        }
                        if mubyteprefix <= 0 as ::core::ffi::c_int {
                            mubyteprefix = 51 as ::core::ffi::c_int as integer;
                        }
                    } else if mubytestoken > 4095 as ::core::ffi::c_int
                        && curcmd as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        mubytetableout = true_0 as boolean;
                        mubytetablein = false_0 as boolean;
                        mubyterelax = true_0 as boolean;
                        getxtoken();
                    }
                    r = getavail();
                    p = r;
                    while curcs == 0 as ::core::ffi::c_int {
                        q = getavail();
                        (*mem.offset(p as isize)).hh.v.RH = q;
                        (*mem.offset(q as isize)).hh.v.LH = curtok;
                        p = q;
                        getxtoken();
                    }
                    if curcmd as ::core::ffi::c_int != 67 as ::core::ffi::c_int
                        || curchr != 10 as ::core::ffi::c_int
                    {
                        interaction as ::core::ffi::c_int == 3 as ::core::ffi::c_int;
                        if filelineerrorstylep != 0 {
                            printfileline();
                        } else {
                            zprintnl(264 as ::core::ffi::c_int);
                        }
                        zprint(723 as ::core::ffi::c_int);
                        zprintesc(601 as ::core::ffi::c_int);
                        zprint(724 as ::core::ffi::c_int);
                        helpptr = 2 as ::core::ffi::c_uchar;
                        helpline[1 as ::core::ffi::c_int as usize] =
                            725 as ::core::ffi::c_int as strnumber;
                        helpline[0 as ::core::ffi::c_int as usize] =
                            1628 as ::core::ffi::c_int as strnumber;
                        backerror();
                    }
                    p = (*mem.offset(r as isize)).hh.v.RH;
                    if p as ::core::ffi::c_long == -(268435455 as ::core::ffi::c_long)
                        && mubytetablein != 0
                    {
                        interaction as ::core::ffi::c_int == 3 as ::core::ffi::c_int;
                        if filelineerrorstylep != 0 {
                            printfileline();
                        } else {
                            zprintnl(264 as ::core::ffi::c_int);
                        }
                        zprint(1629 as ::core::ffi::c_int);
                        zprintesc(1626 as ::core::ffi::c_int);
                        zprint(1630 as ::core::ffi::c_int);
                        helpptr = 2 as ::core::ffi::c_uchar;
                        helpline[1 as ::core::ffi::c_int as usize] =
                            1631 as ::core::ffi::c_int as strnumber;
                        helpline[0 as ::core::ffi::c_int as usize] =
                            1632 as ::core::ffi::c_int as strnumber;
                        error();
                    } else {
                        while p as ::core::ffi::c_long != -(268435455 as ::core::ffi::c_long) {
                            *strpool.offset(poolptr as isize) = ((*mem.offset(p as isize)).hh.v.LH
                                as ::core::ffi::c_int
                                % 256 as ::core::ffi::c_int)
                                as packedASCIIcode;
                            poolptr += 1;
                            p = (*mem.offset(p as isize)).hh.v.RH;
                        }
                        zflushlist(r);
                        if *strstart.offset(strptr as isize) as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            == poolptr
                            && *strpool.offset(
                                (poolptr as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_int
                                == mubytestoken
                        {
                            if mubyteread[mubytestoken as usize] as ::core::ffi::c_long
                                != -(268435455 as ::core::ffi::c_long)
                                && mubytetablein != 0
                            {
                                zdisposemunode(mubyteread[mubytestoken as usize]);
                            }
                            if mubytetablein != 0 {
                                mubyteread[mubytestoken as usize] =
                                    -(268435455 as ::core::ffi::c_long) as halfword;
                            }
                            if mubytetableout != 0 {
                                mubytewrite[mubytestoken as usize] =
                                    0 as ::core::ffi::c_int as strnumber;
                            }
                            poolptr = *strstart.offset(strptr as isize) as integer;
                        } else {
                            if mubytetablein != 0 {
                                mubyteupdate();
                            }
                            if mubytetableout != 0 {
                                if mubytestoken > 4095 as ::core::ffi::c_int {
                                    zdisposemutableout(
                                        mubytestoken as ::core::ffi::c_int
                                            - 4095 as ::core::ffi::c_int,
                                    );
                                    if *strstart.offset(strptr as isize) < poolptr
                                        || mubyterelax != 0
                                    {
                                        r = mubytecswrite[((mubytestoken as ::core::ffi::c_int
                                            - 4095 as ::core::ffi::c_int)
                                            % 128 as ::core::ffi::c_int)
                                            as usize];
                                        p = getavail();
                                        mubytecswrite[((mubytestoken as ::core::ffi::c_int
                                            - 4095 as ::core::ffi::c_int)
                                            % 128 as ::core::ffi::c_int)
                                            as usize] = p;
                                        (*mem.offset(p as isize)).hh.v.LH = (mubytestoken
                                            as ::core::ffi::c_int
                                            - 4095 as ::core::ffi::c_int)
                                            as halfword;
                                        (*mem.offset(p as isize)).hh.v.RH = getavail();
                                        p = (*mem.offset(p as isize)).hh.v.RH;
                                        if mubyterelax != 0 {
                                            (*mem.offset(p as isize)).hh.v.LH =
                                                0 as ::core::ffi::c_int as halfword;
                                            poolptr = *strstart.offset(strptr as isize) as integer;
                                        } else {
                                            (*mem.offset(p as isize)).hh.v.LH =
                                                slowmakestring() as halfword;
                                        }
                                        (*mem.offset(p as isize)).hh.v.RH = r;
                                    }
                                } else if *strstart.offset(strptr as isize) == poolptr {
                                    mubytewrite[mubytestoken as usize] =
                                        0 as ::core::ffi::c_int as strnumber;
                                } else {
                                    mubytewrite[mubytestoken as usize] = slowmakestring();
                                }
                            } else {
                                poolptr = *strstart.offset(strptr as isize) as integer;
                            }
                        }
                    }
                } else {
                    n = curchr as integer;
                    getrtoken();
                    p = curcs;
                    if n == 0 as ::core::ffi::c_int {
                        loop {
                            gettoken();
                            if curcmd as ::core::ffi::c_int != 10 as ::core::ffi::c_int {
                                break;
                            }
                        }
                        if curtok == 3133 as ::core::ffi::c_int {
                            gettoken();
                            if curcmd as ::core::ffi::c_int == 10 as ::core::ffi::c_int {
                                gettoken();
                            }
                        }
                    } else {
                        gettoken();
                        q = curtok;
                        gettoken();
                        backinput();
                        curtok = q;
                        backinput();
                    }
                    if curcmd as ::core::ffi::c_int >= 114 as ::core::ffi::c_int {
                        let ref mut fresh87 = (*mem.offset(curchr as isize)).hh.v.LH;
                        *fresh87 += 1;
                    } else if curcmd as ::core::ffi::c_int == 89 as ::core::ffi::c_int
                        || curcmd as ::core::ffi::c_int == 71 as ::core::ffi::c_int
                    {
                        if curchr < membot
                            || curchr > membot as ::core::ffi::c_int + 19 as ::core::ffi::c_int
                        {
                            let ref mut fresh88 = (*mem.offset(
                                (curchr as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ))
                            .hh
                            .v
                            .LH;
                            *fresh88 += 1;
                        }
                    }
                    if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                        zgeqdefine(p, curcmd, curchr);
                    } else {
                        zeqdefine(p, curcmd, curchr);
                    }
                }
            }
        }
        95 => {
            if curchr == 7 as ::core::ffi::c_int {
                scancharnum();
                p = (29021 as integer + curval) as halfword;
                scanoptionalequals();
                scancharnum();
                n = curval;
                scancharnum();
                if (*eqtb.offset(29334 as ::core::ffi::c_int as isize)).u.CINT
                    > 0 as ::core::ffi::c_int
                {
                    begindiagnostic();
                    zprintnl(1641 as ::core::ffi::c_int);
                    zprint(p as ::core::ffi::c_int - 29021 as ::core::ffi::c_int);
                    zprint(1642 as ::core::ffi::c_int);
                    zprint(n);
                    zprintchar(32 as ::core::ffi::c_int as ASCIIcode);
                    zprint(curval);
                    zenddiagnostic(0 as ::core::ffi::c_int);
                }
                n = n * 256 as integer + curval;
                if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                    zgeqdefine(p, 123 as ::core::ffi::c_int as quarterword, n);
                } else {
                    zeqdefine(p, 123 as ::core::ffi::c_int as quarterword, n);
                }
                if (p as ::core::ffi::c_int - 29021 as ::core::ffi::c_int)
                    < (*eqtb.offset(29332 as ::core::ffi::c_int as isize)).u.CINT
                {
                    if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                        zgeqworddefine(
                            29332 as ::core::ffi::c_int,
                            p as ::core::ffi::c_int - 29021 as ::core::ffi::c_int,
                        );
                    } else {
                        zeqworddefine(
                            29332 as ::core::ffi::c_int,
                            p as ::core::ffi::c_int - 29021 as ::core::ffi::c_int,
                        );
                    }
                }
                if p as ::core::ffi::c_int - 29021 as ::core::ffi::c_int
                    > (*eqtb.offset(29333 as ::core::ffi::c_int as isize)).u.CINT
                {
                    if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                        zgeqworddefine(
                            29333 as ::core::ffi::c_int,
                            p as ::core::ffi::c_int - 29021 as ::core::ffi::c_int,
                        );
                    } else {
                        zeqworddefine(
                            29333 as ::core::ffi::c_int,
                            p as ::core::ffi::c_int - 29021 as ::core::ffi::c_int,
                        );
                    }
                }
            } else {
                n = curchr as integer;
                getrtoken();
                p = curcs;
                if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                    zgeqdefine(
                        p,
                        0 as ::core::ffi::c_int as quarterword,
                        256 as ::core::ffi::c_int,
                    );
                } else {
                    zeqdefine(
                        p,
                        0 as ::core::ffi::c_int as quarterword,
                        256 as ::core::ffi::c_int,
                    );
                }
                scanoptionalequals();
                match n {
                    0 => {
                        scancharnum();
                        if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                            zgeqdefine(p, 68 as ::core::ffi::c_int as quarterword, curval);
                        } else {
                            zeqdefine(p, 68 as ::core::ffi::c_int as quarterword, curval);
                        }
                    }
                    1 => {
                        scanfifteenbitint();
                        if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                            zgeqdefine(p, 69 as ::core::ffi::c_int as quarterword, curval);
                        } else {
                            zeqdefine(p, 69 as ::core::ffi::c_int as quarterword, curval);
                        }
                    }
                    _ => {
                        scanregisternum();
                        if curval > 255 as ::core::ffi::c_int {
                            j = (n as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as halfword;
                            if j > 3 as ::core::ffi::c_int {
                                j = 5 as ::core::ffi::c_int as halfword;
                            }
                            zfindsaelement(j as smallnumber, curval, 1 as ::core::ffi::c_int);
                            let ref mut fresh89 = (*mem.offset(
                                (curptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ))
                            .hh
                            .v
                            .LH;
                            *fresh89 += 1;
                            if j == 5 as ::core::ffi::c_int {
                                j = 71 as ::core::ffi::c_int as halfword;
                            } else {
                                j = 89 as ::core::ffi::c_int as halfword;
                            }
                            if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                                zgeqdefine(p, j as quarterword, curptr);
                            } else {
                                zeqdefine(p, j as quarterword, curptr);
                            }
                        } else {
                            match n {
                                2 => {
                                    if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                                        zgeqdefine(
                                            p,
                                            73 as ::core::ffi::c_int as quarterword,
                                            29391 as integer + curval,
                                        );
                                    } else {
                                        zeqdefine(
                                            p,
                                            73 as ::core::ffi::c_int as quarterword,
                                            29391 as integer + curval,
                                        );
                                    }
                                }
                                3 => {
                                    if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                                        zgeqdefine(
                                            p,
                                            74 as ::core::ffi::c_int as quarterword,
                                            29937 as integer + curval,
                                        );
                                    } else {
                                        zeqdefine(
                                            p,
                                            74 as ::core::ffi::c_int as quarterword,
                                            29937 as integer + curval,
                                        );
                                    }
                                }
                                4 => {
                                    if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                                        zgeqdefine(
                                            p,
                                            75 as ::core::ffi::c_int as quarterword,
                                            26646 as integer + curval,
                                        );
                                    } else {
                                        zeqdefine(
                                            p,
                                            75 as ::core::ffi::c_int as quarterword,
                                            26646 as integer + curval,
                                        );
                                    }
                                }
                                5 => {
                                    if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                                        zgeqdefine(
                                            p,
                                            76 as ::core::ffi::c_int as quarterword,
                                            26902 as integer + curval,
                                        );
                                    } else {
                                        zeqdefine(
                                            p,
                                            76 as ::core::ffi::c_int as quarterword,
                                            26902 as integer + curval,
                                        );
                                    }
                                }
                                6 => {
                                    if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                                        zgeqdefine(
                                            p,
                                            72 as ::core::ffi::c_int as quarterword,
                                            27173 as integer + curval,
                                        );
                                    } else {
                                        zeqdefine(
                                            p,
                                            72 as ::core::ffi::c_int as quarterword,
                                            27173 as integer + curval,
                                        );
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        96 => {
            j = curchr;
            scanint();
            n = curval;
            if zscankeyword(1258 as ::core::ffi::c_int) == 0 {
                interaction as ::core::ffi::c_int == 3 as ::core::ffi::c_int;
                if filelineerrorstylep != 0 {
                    printfileline();
                } else {
                    zprintnl(264 as ::core::ffi::c_int);
                }
                zprint(1499 as ::core::ffi::c_int);
                helpptr = 2 as ::core::ffi::c_uchar;
                helpline[1 as ::core::ffi::c_int as usize] =
                    1643 as ::core::ffi::c_int as strnumber;
                helpline[0 as ::core::ffi::c_int as usize] =
                    1644 as ::core::ffi::c_int as strnumber;
                error();
            }
            getrtoken();
            p = curcs;
            zreadtoks(n, p, j);
            if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                zgeqdefine(p, 114 as ::core::ffi::c_int as quarterword, curval);
            } else {
                zeqdefine(p, 114 as ::core::ffi::c_int as quarterword, curval);
            }
        }
        71 | 72 => {
            q = curcs;
            e = false_0 as boolean;
            if curcmd as ::core::ffi::c_int == 71 as ::core::ffi::c_int {
                if curchr == membot {
                    scanregisternum();
                    if curval > 255 as ::core::ffi::c_int {
                        zfindsaelement(
                            5 as ::core::ffi::c_int as smallnumber,
                            curval,
                            1 as ::core::ffi::c_int,
                        );
                        curchr = curptr;
                        e = true_0 as boolean;
                    } else {
                        curchr = (27173 as integer + curval) as halfword;
                    }
                } else {
                    e = true_0 as boolean;
                }
            }
            p = curchr;
            scanoptionalequals();
            loop {
                getxtoken();
                if curcmd as ::core::ffi::c_int != 10 as ::core::ffi::c_int
                    && curcmd as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    break;
                }
            }
            if curcmd as ::core::ffi::c_int != 1 as ::core::ffi::c_int {
                if curcmd as ::core::ffi::c_int == 71 as ::core::ffi::c_int
                    || curcmd as ::core::ffi::c_int == 72 as ::core::ffi::c_int
                {
                    if curcmd as ::core::ffi::c_int == 71 as ::core::ffi::c_int {
                        if curchr == membot {
                            scanregisternum();
                            if curval < 256 as ::core::ffi::c_int {
                                q = (*eqtb.offset((27173 as integer + curval) as isize)).hh.v.RH;
                            } else {
                                zfindsaelement(
                                    5 as ::core::ffi::c_int as smallnumber,
                                    curval,
                                    0 as ::core::ffi::c_int,
                                );
                                if curptr as ::core::ffi::c_long
                                    == -(268435455 as ::core::ffi::c_long)
                                {
                                    q = -(268435455 as ::core::ffi::c_long) as halfword;
                                } else {
                                    q = (*mem.offset(
                                        (curptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            as isize,
                                    ))
                                    .hh
                                    .v
                                    .RH;
                                }
                            }
                        } else {
                            q = (*mem.offset(
                                (curchr as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ))
                            .hh
                            .v
                            .RH;
                        }
                    } else {
                        q = (*eqtb.offset(curchr as isize)).hh.v.RH;
                    }
                    if q as ::core::ffi::c_long == -(268435455 as ::core::ffi::c_long) {
                        if e != 0 {
                            if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                                zgsadef(p, -(268435455 as ::core::ffi::c_long) as halfword);
                            } else {
                                zsadef(p, -(268435455 as ::core::ffi::c_long) as halfword);
                            }
                        } else if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                            zgeqdefine(
                                p,
                                104 as ::core::ffi::c_int as quarterword,
                                -(268435455 as ::core::ffi::c_long) as halfword,
                            );
                        } else {
                            zeqdefine(
                                p,
                                104 as ::core::ffi::c_int as quarterword,
                                -(268435455 as ::core::ffi::c_long) as halfword,
                            );
                        }
                    } else {
                        let ref mut fresh90 = (*mem.offset(q as isize)).hh.v.LH;
                        *fresh90 += 1;
                        if e != 0 {
                            if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                                zgsadef(p, q);
                            } else {
                                zsadef(p, q);
                            }
                        } else if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                            zgeqdefine(p, 114 as ::core::ffi::c_int as quarterword, q);
                        } else {
                            zeqdefine(p, 114 as ::core::ffi::c_int as quarterword, q);
                        }
                    }
                    current_block = 7634702003649914622;
                } else {
                    current_block = 11208766757666257413;
                }
            } else {
                current_block = 11208766757666257413;
            }
            match current_block {
                7634702003649914622 => {}
                _ => {
                    backinput();
                    curcs = q;
                    q = zscantoks(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
                    if (*mem.offset(defref as isize)).hh.v.RH as ::core::ffi::c_long
                        == -(268435455 as ::core::ffi::c_long)
                    {
                        if e != 0 {
                            if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                                zgsadef(p, -(268435455 as ::core::ffi::c_long) as halfword);
                            } else {
                                zsadef(p, -(268435455 as ::core::ffi::c_long) as halfword);
                            }
                        } else if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                            zgeqdefine(
                                p,
                                104 as ::core::ffi::c_int as quarterword,
                                -(268435455 as ::core::ffi::c_long) as halfword,
                            );
                        } else {
                            zeqdefine(
                                p,
                                104 as ::core::ffi::c_int as quarterword,
                                -(268435455 as ::core::ffi::c_long) as halfword,
                            );
                        }
                        (*mem.offset(defref as isize)).hh.v.RH = avail;
                        avail = defref;
                        dynused -= 1;
                    } else {
                        if p == 27159 as ::core::ffi::c_int && e == 0 {
                            (*mem.offset(q as isize)).hh.v.RH = getavail();
                            q = (*mem.offset(q as isize)).hh.v.RH;
                            (*mem.offset(q as isize)).hh.v.LH =
                                637 as ::core::ffi::c_int as halfword;
                            q = getavail();
                            (*mem.offset(q as isize)).hh.v.LH =
                                379 as ::core::ffi::c_int as halfword;
                            (*mem.offset(q as isize)).hh.v.RH =
                                (*mem.offset(defref as isize)).hh.v.RH;
                            (*mem.offset(defref as isize)).hh.v.RH = q;
                        }
                        if e != 0 {
                            if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                                zgsadef(p, defref);
                            } else {
                                zsadef(p, defref);
                            }
                        } else if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                            zgeqdefine(p, 114 as ::core::ffi::c_int as quarterword, defref);
                        } else {
                            zeqdefine(p, 114 as ::core::ffi::c_int as quarterword, defref);
                        }
                    }
                }
            }
        }
        73 => {
            p = curchr;
            scanoptionalequals();
            scanint();
            if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                zgeqworddefine(p, curval);
            } else {
                zeqworddefine(p, curval);
            }
        }
        74 => {
            p = curchr;
            scanoptionalequals();
            zscandimen(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                zgeqworddefine(p, curval);
            } else {
                zeqworddefine(p, curval);
            }
        }
        75 | 76 => {
            p = curchr;
            n = curcmd as integer;
            scanoptionalequals();
            if n == 76 as ::core::ffi::c_int {
                zscanglue(3 as ::core::ffi::c_int as smallnumber);
            } else {
                zscanglue(2 as ::core::ffi::c_int as smallnumber);
            }
            trapzeroglue();
            if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                zgeqdefine(p, 120 as ::core::ffi::c_int as quarterword, curval);
            } else {
                zeqdefine(p, 120 as ::core::ffi::c_int as quarterword, curval);
            }
        }
        85 => {
            if curchr == 27741 as ::core::ffi::c_int {
                n = 15 as ::core::ffi::c_int as integer;
            } else if curchr == 28765 as ::core::ffi::c_int {
                n = 32768 as ::core::ffi::c_long as integer;
            } else if curchr == 28509 as ::core::ffi::c_int {
                n = 32767 as ::core::ffi::c_int as integer;
            } else if curchr == 29647 as ::core::ffi::c_int {
                n = 16777215 as ::core::ffi::c_long as integer;
            } else {
                n = 255 as ::core::ffi::c_int as integer;
            }
            p = curchr;
            scancharnum();
            if p == 27690 as ::core::ffi::c_int {
                p = curval as halfword;
            } else if p == 27691 as ::core::ffi::c_int {
                p = (curval as ::core::ffi::c_int + 256 as ::core::ffi::c_int) as halfword;
            } else if p == 27692 as ::core::ffi::c_int {
                p = (curval as ::core::ffi::c_int + 512 as ::core::ffi::c_int) as halfword;
            } else {
                p = (p as integer + curval) as halfword;
            }
            scanoptionalequals();
            scanint();
            if curval < 0 as ::core::ffi::c_int && p < 29647 as ::core::ffi::c_int || curval > n {
                interaction as ::core::ffi::c_int == 3 as ::core::ffi::c_int;
                if filelineerrorstylep != 0 {
                    printfileline();
                } else {
                    zprintnl(264 as ::core::ffi::c_int);
                }
                zprint(1648 as ::core::ffi::c_int);
                zprintint(curval as longinteger);
                if p < 29647 as ::core::ffi::c_int {
                    zprint(1649 as ::core::ffi::c_int);
                } else {
                    zprint(1650 as ::core::ffi::c_int);
                }
                zprintint(n as longinteger);
                helpptr = 1 as ::core::ffi::c_uchar;
                helpline[0 as ::core::ffi::c_int as usize] =
                    1651 as ::core::ffi::c_int as strnumber;
                error();
                curval = 0 as ::core::ffi::c_int as integer;
            }
            if p < 256 as ::core::ffi::c_int {
                xord[p as usize] = curval as ASCIIcode;
            } else if p < 512 as ::core::ffi::c_int {
                xchr[(p as ::core::ffi::c_int - 256 as ::core::ffi::c_int) as usize] =
                    curval as ASCIIcode;
            } else if p < 768 as ::core::ffi::c_int {
                xprn[(p as ::core::ffi::c_int - 512 as ::core::ffi::c_int) as usize] =
                    curval as ASCIIcode;
            } else if p < 28765 as ::core::ffi::c_int {
                if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                    zgeqdefine(p, 123 as ::core::ffi::c_int as quarterword, curval);
                } else {
                    zeqdefine(p, 123 as ::core::ffi::c_int as quarterword, curval);
                }
            } else if p < 29647 as ::core::ffi::c_int {
                if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                    zgeqdefine(p, 123 as ::core::ffi::c_int as quarterword, curval);
                } else {
                    zeqdefine(p, 123 as ::core::ffi::c_int as quarterword, curval);
                }
            } else if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                zgeqworddefine(p, curval);
            } else {
                zeqworddefine(p, curval);
            }
        }
        86 => {
            p = curchr;
            scanfourbitint();
            p = (p as integer + curval) as halfword;
            scanoptionalequals();
            scanfontident();
            if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                zgeqdefine(p, 123 as ::core::ffi::c_int as quarterword, curval);
            } else {
                zeqdefine(p, 123 as ::core::ffi::c_int as quarterword, curval);
            }
        }
        89 | 90 | 91 | 92 => {
            zdoregistercommand(a);
        }
        98 => {
            scanregisternum();
            if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                n = (1073774592 as ::core::ffi::c_long + curval as ::core::ffi::c_long) as integer;
            } else {
                n = (1073741824 as ::core::ffi::c_long + curval as ::core::ffi::c_long) as integer;
            }
            scanoptionalequals();
            if setboxallowed != 0 {
                zscanbox(n);
            } else {
                interaction as ::core::ffi::c_int == 3 as ::core::ffi::c_int;
                if filelineerrorstylep != 0 {
                    printfileline();
                } else {
                    zprintnl(264 as ::core::ffi::c_int);
                }
                zprint(792 as ::core::ffi::c_int);
                zprintesc(625 as ::core::ffi::c_int);
                helpptr = 2 as ::core::ffi::c_uchar;
                helpline[1 as ::core::ffi::c_int as usize] =
                    1657 as ::core::ffi::c_int as strnumber;
                helpline[0 as ::core::ffi::c_int as usize] =
                    1658 as ::core::ffi::c_int as strnumber;
                error();
            }
        }
        79 => {
            alteraux();
        }
        80 => {
            alterprevgraf();
        }
        81 => {
            alterpagesofar();
        }
        82 => {
            alterinteger();
        }
        83 => {
            alterboxdimen();
        }
        84 => {
            q = curchr;
            scanoptionalequals();
            scanint();
            n = curval;
            if n <= 0 as ::core::ffi::c_int {
                p = -(268435455 as ::core::ffi::c_long) as halfword;
            } else if q > 27158 as ::core::ffi::c_int {
                n = (curval as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as integer;
                p = zgetnode(
                    2 as ::core::ffi::c_int * n as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                );
                (*mem.offset(p as isize)).hh.v.LH = n as halfword;
                n = curval;
                (*mem.offset((p as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                    .u
                    .CINT = n;
                let mut for_end: integer = 0;
                j = (p as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as halfword;
                for_end = (p as ::core::ffi::c_int
                    + n as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as integer;
                if j <= for_end {
                    loop {
                        scanint();
                        (*mem.offset(j as isize)).u.CINT = curval;
                        let fresh91 = j;
                        j = j + 1;
                        if !(fresh91 < for_end) {
                            break;
                        }
                    }
                }
                if n as ::core::ffi::c_int & 1 as ::core::ffi::c_int == 0 {
                    (*mem.offset(
                        (p as ::core::ffi::c_int
                            + n as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int) as isize,
                    ))
                    .u
                    .CINT = 0 as ::core::ffi::c_int as integer;
                }
            } else {
                p = zgetnode(
                    2 as ::core::ffi::c_int * n as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                );
                (*mem.offset(p as isize)).hh.v.LH = n as halfword;
                let mut for_end_0: integer = 0;
                j = 1 as ::core::ffi::c_int as halfword;
                for_end_0 = n;
                if j <= for_end_0 {
                    loop {
                        zscandimen(
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                        );
                        (*mem.offset(
                            (p as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * j as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as isize,
                        ))
                        .u
                        .CINT = curval;
                        zscandimen(
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                        );
                        (*mem.offset((p + 2 as halfword * j) as isize)).u.CINT = curval;
                        let fresh92 = j;
                        j = j + 1;
                        if !(fresh92 < for_end_0) {
                            break;
                        }
                    }
                }
            }
            if a as ::core::ffi::c_int >= 4 as ::core::ffi::c_int {
                zgeqdefine(q, 121 as ::core::ffi::c_int as quarterword, p);
            } else {
                zeqdefine(q, 121 as ::core::ffi::c_int as quarterword, p);
            }
        }
        99 => {
            if curchr == 1 as ::core::ffi::c_int {
                if iniversion != 0 {
                    newpatterns();
                } else {
                    interaction as ::core::ffi::c_int == 3 as ::core::ffi::c_int;
                    if filelineerrorstylep != 0 {
                        printfileline();
                    } else {
                        zprintnl(264 as ::core::ffi::c_int);
                    }
                    zprint(1662 as ::core::ffi::c_int);
                    helpptr = 0 as ::core::ffi::c_uchar;
                    error();
                    loop {
                        gettoken();
                        if curcmd as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
                            break;
                        }
                    }
                    return;
                }
            } else {
                newhyphexceptions();
            }
        }
        77 => {
            zfindfontdimen(1 as ::core::ffi::c_int);
            k_0 = curval as fontindex;
            scanoptionalequals();
            zscandimen(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            (*fontinfo.offset(k_0 as isize)).u.CINT = curval;
        }
        78 => {
            n = curchr as integer;
            scanfontident();
            f = curval as internalfontnumber;
            if n == 6 as ::core::ffi::c_int {
                zsetnoligatures(f);
            } else if n < 2 as ::core::ffi::c_int {
                scanoptionalequals();
                scanint();
                if n == 0 as ::core::ffi::c_int {
                    *hyphenchar.offset(f as isize) = curval;
                } else {
                    *skewchar.offset(f as isize) = curval;
                }
            } else {
                scancharnum();
                p = curval as halfword;
                scanoptionalequals();
                scanint();
                match n {
                    2 => {
                        zsetlpcode(f, p as eightbits, curval);
                    }
                    3 => {
                        zsetrpcode(f, p as eightbits, curval);
                    }
                    4 => {
                        zsetefcode(f, p as eightbits, curval);
                    }
                    5 => {
                        zsettagcode(f, p as eightbits, curval);
                    }
                    7 => {
                        zsetknbscode(f, p as eightbits, curval);
                    }
                    8 => {
                        zsetstbscode(f, p as eightbits, curval);
                    }
                    9 => {
                        zsetshbscode(f, p as eightbits, curval);
                    }
                    10 => {
                        zsetknbccode(f, p as eightbits, curval);
                    }
                    11 => {
                        zsetknaccode(f, p as eightbits, curval);
                    }
                    _ => {}
                }
            }
        }
        88 => {
            znewfont(a);
        }
        101 => {
            znewletterspacedfont(a);
        }
        102 => {
            zmakefontcopy(a);
        }
        100 => {
            newinteraction();
        }
        _ => {
            zconfusion(1608 as ::core::ffi::c_int);
        }
    }
    if aftertoken != 0 as ::core::ffi::c_int {
        curtok = aftertoken;
        backinput();
        aftertoken = 0 as ::core::ffi::c_int as halfword;
    }
}
#[no_mangle]
pub unsafe extern "C" fn storefmtfile() {
    let mut current_block: u64;
    let mut mem: *mut memoryword = zmem;
    let mut eqtb: *mut memoryword = zeqtb;
    let mut j: integer = 0;
    let mut k_0: integer = 0;
    let mut l: integer = 0;
    let mut p: halfword = 0;
    let mut q: halfword = 0;
    let mut x: integer = 0;
    let mut formatengine: *mut ASCIIcode = ::core::ptr::null_mut::<ASCIIcode>();
    if saveptr != 0 as ::core::ffi::c_int {
        interaction as ::core::ffi::c_int == 3 as ::core::ffi::c_int;
        if filelineerrorstylep != 0 {
            printfileline();
        } else {
            zprintnl(264 as ::core::ffi::c_int);
        }
        zprint(1711 as ::core::ffi::c_int);
        helpptr = 1 as ::core::ffi::c_uchar;
        helpline[0 as ::core::ffi::c_int as usize] = 1712 as ::core::ffi::c_int as strnumber;
        if interaction as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            interaction = 2 as ::core::ffi::c_uchar;
        }
        if logopened != 0 {
            error();
        }
        history = 3 as ::core::ffi::c_uchar;
        jumpout();
    }
    selector = 21 as ::core::ffi::c_uchar;
    zprint(1730 as ::core::ffi::c_int);
    zprint(jobname);
    zprintchar(32 as ::core::ffi::c_int as ASCIIcode);
    zprintint((*eqtb.offset(29300 as ::core::ffi::c_int as isize)).u.CINT as longinteger);
    zprintchar(46 as ::core::ffi::c_int as ASCIIcode);
    zprintint((*eqtb.offset(29299 as ::core::ffi::c_int as isize)).u.CINT as longinteger);
    zprintchar(46 as ::core::ffi::c_int as ASCIIcode);
    zprintint((*eqtb.offset(29298 as ::core::ffi::c_int as isize)).u.CINT as longinteger);
    zprintchar(41 as ::core::ffi::c_int as ASCIIcode);
    if interaction as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        selector = 18 as ::core::ffi::c_uchar;
    } else {
        selector = 19 as ::core::ffi::c_uchar;
    }
    if poolptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int > poolsize {
        zoverflow(
            259 as ::core::ffi::c_int,
            poolsize as poolpointer - initpoolptr,
        );
    }
    formatident = makestring();
    zpackjobname(951 as ::core::ffi::c_int);
    while !(open_output(&raw mut fmtfile as *mut *mut FILE, FOPEN_WBIN_MODE.as_ptr()) != 0
        && {
            fmtfile = gzdopen(fileno(fmtfile as *mut FILE), FOPEN_WBIN_MODE.as_ptr()) as wordfile;
            !fmtfile.is_null()
        }
        && gzsetparams(
            fmtfile as gzFile,
            1 as ::core::ffi::c_int,
            Z_DEFAULT_STRATEGY,
        ) == Z_OK)
    {
        zpromptfilename(1731 as ::core::ffi::c_int, 951 as ::core::ffi::c_int);
    }
    zprintnl(1732 as ::core::ffi::c_int);
    zslowprint(zzwmakenamestring(&raw mut fmtfile));
    strptr -= 1;
    poolptr = *strstart.offset(strptr as isize) as integer;
    zprintnl(345 as ::core::ffi::c_int);
    zslowprint(formatident);
    let mut x_val: integer = 1462916184 as ::core::ffi::c_long as integer;
    do_dump(
        &raw mut x_val as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    x = strlen(enginename.as_ptr()) as integer;
    formatengine = xmalloc(
        ((x as ::core::ffi::c_int + 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<ASCIIcode>() as size_t),
    ) as *mut ASCIIcode;
    strcpy(
        formatengine as *mut ::core::ffi::c_char,
        enginename.as_ptr(),
    );
    let mut for_end: integer = 0;
    k_0 = x;
    for_end = (x as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as integer;
    if k_0 <= for_end {
        loop {
            *formatengine.offset(k_0 as isize) = 0 as ASCIIcode;
            let fresh8 = k_0;
            k_0 = k_0 + 1;
            if !(fresh8 < for_end) {
                break;
            }
        }
    }
    x = (x as ::core::ffi::c_int + 4 as ::core::ffi::c_int
        - x as ::core::ffi::c_int % 4 as ::core::ffi::c_int) as integer;
    let mut x_val_0: integer = x;
    do_dump(
        &raw mut x_val_0 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        formatengine.offset(0 as ::core::ffi::c_int as isize) as *mut ASCIIcode
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<ASCIIcode>() as ::core::ffi::c_int,
        x,
        fmtfile as gzFile,
    );
    free(formatengine as *mut ::core::ffi::c_void);
    let mut x_val_1: integer = 312141437 as ::core::ffi::c_long as integer;
    do_dump(
        &raw mut x_val_1 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        (&raw mut xord as *mut ASCIIcode).offset(0 as ::core::ffi::c_int as isize) as *mut ASCIIcode
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<ASCIIcode>() as ::core::ffi::c_int,
        256 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        (&raw mut xchr as *mut ASCIIcode).offset(0 as ::core::ffi::c_int as isize) as *mut ASCIIcode
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<ASCIIcode>() as ::core::ffi::c_int,
        256 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        (&raw mut xprn as *mut ASCIIcode).offset(0 as ::core::ffi::c_int as isize) as *mut ASCIIcode
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<ASCIIcode>() as ::core::ffi::c_int,
        256 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_2: integer = 268435455 as ::core::ffi::c_long as integer;
    do_dump(
        &raw mut x_val_2 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_3: integer = hashhigh as integer;
    do_dump(
        &raw mut x_val_3 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_4: integer = eTeXmode as integer;
    do_dump(
        &raw mut x_val_4 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut for_end_0: integer = 0;
    j = 0 as ::core::ffi::c_int as integer;
    for_end_0 = -(0 as ::core::ffi::c_int) as integer;
    if j <= for_end_0 {
        loop {
            (*eqtb.offset((29389 as integer + j) as isize)).u.CINT =
                0 as ::core::ffi::c_int as integer;
            let fresh9 = j;
            j = j + 1;
            if !(fresh9 < for_end_0) {
                break;
            }
        }
    }
    while pseudofiles as ::core::ffi::c_long != -(268435455 as ::core::ffi::c_long) {
        pseudoclose();
    }
    let mut x_val_5: integer = membot;
    do_dump(
        &raw mut x_val_5 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_6: integer = memtop;
    do_dump(
        &raw mut x_val_6 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_7: integer = 30192 as integer;
    do_dump(
        &raw mut x_val_7 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_8: integer = 8501 as integer;
    do_dump(
        &raw mut x_val_8 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_9: integer = 607 as integer;
    do_dump(
        &raw mut x_val_9 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_10: integer = 1296847960 as ::core::ffi::c_long as integer;
    do_dump(
        &raw mut x_val_10 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    if mltexp != 0 {
        let mut x_val_11: integer = 1 as integer;
        do_dump(
            &raw mut x_val_11 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
    } else {
        let mut x_val_12: integer = 0 as integer;
        do_dump(
            &raw mut x_val_12 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
    }
    let mut x_val_13: integer = 1162040408 as ::core::ffi::c_long as integer;
    do_dump(
        &raw mut x_val_13 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    if enctexp == 0 {
        let mut x_val_14: integer = 0 as integer;
        do_dump(
            &raw mut x_val_14 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
    } else {
        let mut x_val_15: integer = 1 as integer;
        do_dump(
            &raw mut x_val_15 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_dump(
            (&raw mut mubyteread as *mut halfword).offset(0 as ::core::ffi::c_int as isize)
                as *mut halfword as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<halfword>() as ::core::ffi::c_int,
            256 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_dump(
            (&raw mut mubytewrite as *mut strnumber).offset(0 as ::core::ffi::c_int as isize)
                as *mut strnumber as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<strnumber>() as ::core::ffi::c_int,
            256 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_dump(
            (&raw mut mubytecswrite as *mut halfword).offset(0 as ::core::ffi::c_int as isize)
                as *mut halfword as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<halfword>() as ::core::ffi::c_int,
            128 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
    }
    let mut x_val_16: integer = poolptr;
    do_dump(
        &raw mut x_val_16 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_17: integer = strptr as integer;
    do_dump(
        &raw mut x_val_17 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        strstart.offset(0 as ::core::ffi::c_int as isize) as *mut poolpointer
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<poolpointer>() as ::core::ffi::c_int,
        strptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        strpool.offset(0 as ::core::ffi::c_int as isize) as *mut packedASCIIcode
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<packedASCIIcode>() as ::core::ffi::c_int,
        poolptr,
        fmtfile as gzFile,
    );
    println();
    zprintint(strptr as longinteger);
    zprint(1713 as ::core::ffi::c_int);
    zprintint(poolptr as longinteger);
    sortavail();
    varused = 0 as ::core::ffi::c_int as integer;
    let mut x_val_18: integer = lomemmax as integer;
    do_dump(
        &raw mut x_val_18 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_19: integer = rover as integer;
    do_dump(
        &raw mut x_val_19 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    if eTeXmode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        let mut for_end_1: integer = 0;
        k_0 = 0 as ::core::ffi::c_int as integer;
        for_end_1 = 5 as ::core::ffi::c_int as integer;
        if k_0 <= for_end_1 {
            loop {
                let mut x_val_20: integer = saroot[k_0 as usize];
                do_dump(
                    &raw mut x_val_20 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    fmtfile as gzFile,
                );
                let fresh10 = k_0;
                k_0 = k_0 + 1;
                if !(fresh10 < for_end_1) {
                    break;
                }
            }
        }
    }
    p = membot as halfword;
    q = rover;
    x = 0 as ::core::ffi::c_int as integer;
    loop {
        do_dump(
            mem.offset(p as isize) as *mut memoryword as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<memoryword>() as ::core::ffi::c_int,
            q + 2 as halfword - p,
            fmtfile as gzFile,
        );
        x = (x as halfword + q + 2 as halfword - p) as integer;
        varused = (varused as halfword + q - p) as integer;
        p = q + (*mem.offset(q as isize)).hh.v.LH;
        q = (*mem.offset((q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
            .hh
            .v
            .RH;
        if q == rover {
            break;
        }
    }
    varused = (varused as halfword + lomemmax - p) as integer;
    dynused = (memend + 1 as halfword - himemmin) as integer;
    do_dump(
        mem.offset(p as isize) as *mut memoryword as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<memoryword>() as ::core::ffi::c_int,
        lomemmax + 1 as halfword - p,
        fmtfile as gzFile,
    );
    x = (x as halfword + lomemmax + 1 as halfword - p) as integer;
    let mut x_val_21: integer = himemmin as integer;
    do_dump(
        &raw mut x_val_21 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_22: integer = avail as integer;
    do_dump(
        &raw mut x_val_22 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        mem.offset(himemmin as isize) as *mut memoryword as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<memoryword>() as ::core::ffi::c_int,
        memend + 1 as halfword - himemmin,
        fmtfile as gzFile,
    );
    x = (x as halfword + memend + 1 as halfword - himemmin) as integer;
    p = avail;
    while p as ::core::ffi::c_long != -(268435455 as ::core::ffi::c_long) {
        dynused -= 1;
        p = (*mem.offset(p as isize)).hh.v.RH;
    }
    let mut x_val_23: integer = varused;
    do_dump(
        &raw mut x_val_23 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_24: integer = dynused;
    do_dump(
        &raw mut x_val_24 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    println();
    zprintint(x as longinteger);
    zprint(1714 as ::core::ffi::c_int);
    zprintint(varused as longinteger);
    zprintchar(38 as ::core::ffi::c_int as ASCIIcode);
    zprintint(dynused as longinteger);
    k_0 = 1 as ::core::ffi::c_int as integer;
    loop {
        j = k_0;
        loop {
            if !(j < 29276 as ::core::ffi::c_int) {
                current_block = 10282596542094995802;
                break;
            }
            if (*eqtb.offset(j as isize)).hh.v.RH
                == (*eqtb.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                    .hh
                    .v
                    .RH
                && (*eqtb.offset(j as isize)).hh.u.B0 as ::core::ffi::c_int
                    == (*eqtb.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                        .hh
                        .u
                        .B0 as ::core::ffi::c_int
                && (*eqtb.offset(j as isize)).hh.u.B1 as ::core::ffi::c_int
                    == (*eqtb.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                        .hh
                        .u
                        .B1 as ::core::ffi::c_int
            {
                current_block = 3361661777870869254;
                break;
            }
            j += 1;
        }
        match current_block {
            10282596542094995802 => {
                l = 29277 as ::core::ffi::c_int as integer;
            }
            _ => {
                j += 1;
                l = j;
                while j < 29276 as ::core::ffi::c_int {
                    if (*eqtb.offset(j as isize)).hh.v.RH
                        != (*eqtb
                            .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                        .hh
                        .v
                        .RH
                        || (*eqtb.offset(j as isize)).hh.u.B0 as ::core::ffi::c_int
                            != (*eqtb.offset(
                                (j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ))
                            .hh
                            .u
                            .B0 as ::core::ffi::c_int
                        || (*eqtb.offset(j as isize)).hh.u.B1 as ::core::ffi::c_int
                            != (*eqtb.offset(
                                (j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ))
                            .hh
                            .u
                            .B1 as ::core::ffi::c_int
                    {
                        break;
                    }
                    j += 1;
                }
            }
        }
        let mut x_val_25: integer = l - k_0;
        do_dump(
            &raw mut x_val_25 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_dump(
            eqtb.offset(k_0 as isize) as *mut memoryword as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<memoryword>() as ::core::ffi::c_int,
            l - k_0,
            fmtfile as gzFile,
        );
        k_0 = (j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as integer;
        let mut x_val_26: integer = k_0 - l;
        do_dump(
            &raw mut x_val_26 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        if k_0 == 29277 as ::core::ffi::c_int {
            break;
        }
    }
    loop {
        j = k_0;
        loop {
            if !(j < 30192 as ::core::ffi::c_int) {
                current_block = 11844752514624976770;
                break;
            }
            if (*eqtb.offset(j as isize)).u.CINT
                == (*eqtb.offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                    .u
                    .CINT
            {
                current_block = 7994893567089286288;
                break;
            }
            j += 1;
        }
        match current_block {
            11844752514624976770 => {
                l = 30193 as ::core::ffi::c_int as integer;
            }
            _ => {
                j += 1;
                l = j;
                while j < 30192 as ::core::ffi::c_int {
                    if (*eqtb.offset(j as isize)).u.CINT
                        != (*eqtb
                            .offset((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                        .u
                        .CINT
                    {
                        break;
                    }
                    j += 1;
                }
            }
        }
        let mut x_val_27: integer = l - k_0;
        do_dump(
            &raw mut x_val_27 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        do_dump(
            eqtb.offset(k_0 as isize) as *mut memoryword as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<memoryword>() as ::core::ffi::c_int,
            l - k_0,
            fmtfile as gzFile,
        );
        k_0 = (j as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as integer;
        let mut x_val_28: integer = k_0 - l;
        do_dump(
            &raw mut x_val_28 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        if k_0 > 30192 as ::core::ffi::c_int {
            break;
        }
    }
    if hashhigh > 0 as ::core::ffi::c_int {
        do_dump(
            eqtb.offset(30193 as ::core::ffi::c_int as isize) as *mut memoryword
                as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<memoryword>() as ::core::ffi::c_int,
            hashhigh,
            fmtfile as gzFile,
        );
    }
    let mut x_val_29: integer = parloc as integer;
    do_dump(
        &raw mut x_val_29 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_30: integer = writeloc as integer;
    do_dump(
        &raw mut x_val_30 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut for_end_2: integer = 0;
    p = 0 as ::core::ffi::c_int as halfword;
    for_end_2 = 2100 as ::core::ffi::c_int as integer;
    if p <= for_end_2 {
        loop {
            do_dump(
                (&raw mut prim as *mut twohalves).offset(p as isize) as *mut twohalves
                    as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<twohalves>() as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                fmtfile as gzFile,
            );
            let fresh11 = p;
            p = p + 1;
            if !(fresh11 < for_end_2) {
                break;
            }
        }
    }
    let mut x_val_31: integer = hashused as integer;
    do_dump(
        &raw mut x_val_31 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    cscount = (15513 as halfword - hashused + hashhigh) as integer;
    let mut for_end_3: integer = 0;
    p = 514 as ::core::ffi::c_int as halfword;
    for_end_3 = hashused as integer;
    if p <= for_end_3 {
        loop {
            if (*hash.offset(p as isize)).v.RH != 0 as ::core::ffi::c_int {
                let mut x_val_32: integer = p as integer;
                do_dump(
                    &raw mut x_val_32 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    fmtfile as gzFile,
                );
                do_dump(
                    hash.offset(p as isize) as *mut twohalves as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<twohalves>() as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    fmtfile as gzFile,
                );
                cscount += 1;
            }
            let fresh12 = p;
            p = p + 1;
            if !(fresh12 < for_end_3) {
                break;
            }
        }
    }
    do_dump(
        hash.offset((hashused as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
            as *mut twohalves as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<twohalves>() as ::core::ffi::c_int,
        26626 as halfword - hashused,
        fmtfile as gzFile,
    );
    if hashhigh > 0 as ::core::ffi::c_int {
        do_dump(
            hash.offset(30193 as ::core::ffi::c_int as isize) as *mut twohalves
                as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<twohalves>() as ::core::ffi::c_int,
            hashhigh,
            fmtfile as gzFile,
        );
    }
    let mut x_val_33: integer = cscount;
    do_dump(
        &raw mut x_val_33 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    println();
    zprintint(cscount as longinteger);
    zprint(1715 as ::core::ffi::c_int);
    let mut x_val_34: integer = fmemptr as integer;
    do_dump(
        &raw mut x_val_34 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        fontinfo.offset(0 as ::core::ffi::c_int as isize) as *mut fmemoryword
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<fmemoryword>() as ::core::ffi::c_int,
        fmemptr,
        fmtfile as gzFile,
    );
    let mut x_val_35: integer = fontptr as integer;
    do_dump(
        &raw mut x_val_35 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        fontcheck.offset(0 as ::core::ffi::c_int as isize) as *mut fourquarters
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<fourquarters>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        fontsize.offset(0 as ::core::ffi::c_int as isize) as *mut scaled
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<scaled>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        fontdsize.offset(0 as ::core::ffi::c_int as isize) as *mut scaled
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<scaled>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        fontparams.offset(0 as ::core::ffi::c_int as isize) as *mut fontindex
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<fontindex>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        hyphenchar.offset(0 as ::core::ffi::c_int as isize) as *mut integer
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        skewchar.offset(0 as ::core::ffi::c_int as isize) as *mut integer
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        fontname.offset(0 as ::core::ffi::c_int as isize) as *mut strnumber
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<strnumber>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        fontarea.offset(0 as ::core::ffi::c_int as isize) as *mut strnumber
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<strnumber>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        fontbc.offset(0 as ::core::ffi::c_int as isize) as *mut eightbits
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<eightbits>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        fontec.offset(0 as ::core::ffi::c_int as isize) as *mut eightbits
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<eightbits>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        charbase.offset(0 as ::core::ffi::c_int as isize) as *mut integer
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        widthbase.offset(0 as ::core::ffi::c_int as isize) as *mut integer
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        heightbase.offset(0 as ::core::ffi::c_int as isize) as *mut integer
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        depthbase.offset(0 as ::core::ffi::c_int as isize) as *mut integer
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        italicbase.offset(0 as ::core::ffi::c_int as isize) as *mut integer
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        ligkernbase.offset(0 as ::core::ffi::c_int as isize) as *mut integer
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        kernbase.offset(0 as ::core::ffi::c_int as isize) as *mut integer
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        extenbase.offset(0 as ::core::ffi::c_int as isize) as *mut integer
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        parambase.offset(0 as ::core::ffi::c_int as isize) as *mut integer
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        fontglue.offset(0 as ::core::ffi::c_int as isize) as *mut halfword
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<halfword>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        bcharlabel.offset(0 as ::core::ffi::c_int as isize) as *mut fontindex
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<fontindex>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        fontbchar.offset(0 as ::core::ffi::c_int as isize) as *mut ninebits
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<ninebits>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        fontfalsebchar.offset(0 as ::core::ffi::c_int as isize) as *mut ninebits
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<ninebits>() as ::core::ffi::c_int,
        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut for_end_4: integer = 0;
    k_0 = 0 as ::core::ffi::c_int as integer;
    for_end_4 = fontptr as integer;
    if k_0 <= for_end_4 {
        loop {
            zprintnl(1719 as ::core::ffi::c_int);
            zprintesc((*hash.offset((17626 as integer + k_0) as isize)).v.RH);
            zprintchar(61 as ::core::ffi::c_int as ASCIIcode);
            zprintfilename(
                *fontname.offset(k_0 as isize),
                *fontarea.offset(k_0 as isize),
                345 as ::core::ffi::c_int,
            );
            if *fontsize.offset(k_0 as isize) != *fontdsize.offset(k_0 as isize) {
                zprint(906 as ::core::ffi::c_int);
                zprintscaled(*fontsize.offset(k_0 as isize));
                zprint(312 as ::core::ffi::c_int);
            }
            let fresh13 = k_0;
            k_0 = k_0 + 1;
            if !(fresh13 < for_end_4) {
                break;
            }
        }
    }
    println();
    zprintint((fmemptr as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as longinteger);
    zprint(1716 as ::core::ffi::c_int);
    zprintint((fontptr as ::core::ffi::c_int - 0 as ::core::ffi::c_int) as longinteger);
    if fontptr != 1 as ::core::ffi::c_int {
        zprint(1717 as ::core::ffi::c_int);
    } else {
        zprint(1718 as ::core::ffi::c_int);
    }
    let mut x_val_36: integer = hyphcount;
    do_dump(
        &raw mut x_val_36 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    if hyphnext <= 607 as ::core::ffi::c_int {
        hyphnext = hyphsize;
    }
    let mut x_val_37: integer = hyphnext;
    do_dump(
        &raw mut x_val_37 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut for_end_5: integer = 0;
    k_0 = 0 as ::core::ffi::c_int as integer;
    for_end_5 = hyphsize;
    if k_0 <= for_end_5 {
        loop {
            if *hyphword.offset(k_0 as isize) != 0 as ::core::ffi::c_int {
                let mut x_val_38: integer = (k_0 as ::core::ffi::c_long
                    + 65536 as ::core::ffi::c_long
                        * *hyphlink.offset(k_0 as isize) as ::core::ffi::c_long)
                    as integer;
                do_dump(
                    &raw mut x_val_38 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    fmtfile as gzFile,
                );
                let mut x_val_39: integer = *hyphword.offset(k_0 as isize) as integer;
                do_dump(
                    &raw mut x_val_39 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    fmtfile as gzFile,
                );
                let mut x_val_40: integer = *hyphlist.offset(k_0 as isize) as integer;
                do_dump(
                    &raw mut x_val_40 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    fmtfile as gzFile,
                );
            }
            let fresh14 = k_0;
            k_0 = k_0 + 1;
            if !(fresh14 < for_end_5) {
                break;
            }
        }
    }
    println();
    zprintint(hyphcount as longinteger);
    if hyphcount != 1 as ::core::ffi::c_int {
        zprint(1720 as ::core::ffi::c_int);
    } else {
        zprint(1721 as ::core::ffi::c_int);
    }
    if trienotready != 0 {
        inittrie();
    }
    let mut x_val_41: integer = triemax as integer;
    do_dump(
        &raw mut x_val_41 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_42: integer = hyphstart as integer;
    do_dump(
        &raw mut x_val_42 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        trietrl.offset(0 as ::core::ffi::c_int as isize) as *mut triepointer
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<triepointer>() as ::core::ffi::c_int,
        triemax as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        trietro.offset(0 as ::core::ffi::c_int as isize) as *mut triepointer
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<triepointer>() as ::core::ffi::c_int,
        triemax as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        trietrc.offset(0 as ::core::ffi::c_int as isize) as *mut quarterword
            as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<quarterword>() as ::core::ffi::c_int,
        triemax as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_43: integer = trieopptr;
    do_dump(
        &raw mut x_val_43 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    do_dump(
        (&raw mut hyfdistance as *mut smallnumber).offset(1 as ::core::ffi::c_int as isize)
            as *mut smallnumber as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<smallnumber>() as ::core::ffi::c_int,
        trieopptr,
        fmtfile as gzFile,
    );
    do_dump(
        (&raw mut hyfnum as *mut smallnumber).offset(1 as ::core::ffi::c_int as isize)
            as *mut smallnumber as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<smallnumber>() as ::core::ffi::c_int,
        trieopptr,
        fmtfile as gzFile,
    );
    do_dump(
        (&raw mut hyfnext as *mut trieopcode).offset(1 as ::core::ffi::c_int as isize)
            as *mut trieopcode as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<trieopcode>() as ::core::ffi::c_int,
        trieopptr,
        fmtfile as gzFile,
    );
    zprintnl(1722 as ::core::ffi::c_int);
    zprintint(triemax as longinteger);
    zprint(1723 as ::core::ffi::c_int);
    zprintint(trieopptr as longinteger);
    if trieopptr != 1 as ::core::ffi::c_int {
        zprint(1724 as ::core::ffi::c_int);
    } else {
        zprint(1725 as ::core::ffi::c_int);
    }
    zprint(1726 as ::core::ffi::c_int);
    zprintint(35111 as ::core::ffi::c_long as longinteger);
    let mut for_end_6: integer = 0;
    k_0 = 255 as ::core::ffi::c_int as integer;
    for_end_6 = 0 as ::core::ffi::c_int as integer;
    if k_0 >= for_end_6 {
        loop {
            if trieused[k_0 as usize] as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                zprintnl(967 as ::core::ffi::c_int);
                zprintint(trieused[k_0 as usize] as longinteger);
                zprint(1727 as ::core::ffi::c_int);
                zprintint(k_0 as longinteger);
                let mut x_val_44: integer = k_0;
                do_dump(
                    &raw mut x_val_44 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    fmtfile as gzFile,
                );
                let mut x_val_45: integer = trieused[k_0 as usize] as integer;
                do_dump(
                    &raw mut x_val_45 as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    fmtfile as gzFile,
                );
            }
            let fresh15 = k_0;
            k_0 = k_0 - 1;
            if !(fresh15 > for_end_6) {
                break;
            }
        }
    }
    dumpimagemeta();
    let mut x_val_46: integer = pdfmemsize;
    do_dump(
        &raw mut x_val_46 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_47: integer = pdfmemptr;
    do_dump(
        &raw mut x_val_47 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut for_end_7: integer = 0;
    k_0 = 1 as ::core::ffi::c_int as integer;
    for_end_7 = (pdfmemptr as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as integer;
    if k_0 <= for_end_7 {
        loop {
            let mut x_val_48: integer = *pdfmem.offset(k_0 as isize);
            do_dump(
                &raw mut x_val_48 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                fmtfile as gzFile,
            );
            let fresh16 = k_0;
            k_0 = k_0 + 1;
            if !(fresh16 < for_end_7) {
                break;
            }
        }
    }
    println();
    zprintint((pdfmemptr as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as longinteger);
    zprint(1728 as ::core::ffi::c_int);
    let mut x_val_49: integer = objtabsize;
    do_dump(
        &raw mut x_val_49 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_50: integer = objptr;
    do_dump(
        &raw mut x_val_50 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_51: integer = sysobjptr;
    do_dump(
        &raw mut x_val_51 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut for_end_8: integer = 0;
    k_0 = 1 as ::core::ffi::c_int as integer;
    for_end_8 = sysobjptr;
    if k_0 <= for_end_8 {
        loop {
            let mut x_val_52: integer = (*objtab.offset(k_0 as isize)).int0;
            do_dump(
                &raw mut x_val_52 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                fmtfile as gzFile,
            );
            let mut x_val_53: integer = (*objtab.offset(k_0 as isize)).int1;
            do_dump(
                &raw mut x_val_53 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                fmtfile as gzFile,
            );
            let mut x_val_54: integer = (*objtab.offset(k_0 as isize)).int3;
            do_dump(
                &raw mut x_val_54 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                fmtfile as gzFile,
            );
            let mut x_val_55: integer = (*objtab.offset(k_0 as isize)).int4;
            do_dump(
                &raw mut x_val_55 as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                fmtfile as gzFile,
            );
            let fresh17 = k_0;
            k_0 = k_0 + 1;
            if !(fresh17 < for_end_8) {
                break;
            }
        }
    }
    println();
    zprintint(sysobjptr as longinteger);
    zprint(1729 as ::core::ffi::c_int);
    let mut x_val_56: integer = pdfobjcount;
    do_dump(
        &raw mut x_val_56 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_57: integer = pdfxformcount;
    do_dump(
        &raw mut x_val_57 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_58: integer = pdfximagecount;
    do_dump(
        &raw mut x_val_58 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_59: integer = headtab[7 as ::core::ffi::c_int as usize];
    do_dump(
        &raw mut x_val_59 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_60: integer = headtab[8 as ::core::ffi::c_int as usize];
    do_dump(
        &raw mut x_val_60 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_61: integer = headtab[9 as ::core::ffi::c_int as usize];
    do_dump(
        &raw mut x_val_61 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_62: integer = pdflastobj;
    do_dump(
        &raw mut x_val_62 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_63: integer = pdflastxform;
    do_dump(
        &raw mut x_val_63 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_64: integer = pdflastximage;
    do_dump(
        &raw mut x_val_64 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    dumptounicode();
    let mut x_val_65: integer = interaction as integer;
    do_dump(
        &raw mut x_val_65 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_66: integer = formatident as integer;
    do_dump(
        &raw mut x_val_66 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    let mut x_val_67: integer = 69069 as ::core::ffi::c_long as integer;
    do_dump(
        &raw mut x_val_67 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    (*eqtb.offset(29308 as ::core::ffi::c_int as isize)).u.CINT =
        0 as ::core::ffi::c_int as integer;
    gzclose(fmtfile as gzFile);
}
#[no_mangle]
pub unsafe extern "C" fn loadfmtfile() -> boolean {
    let mut current_block: u64;
    let mut Result: boolean = 0;
    let mut mem: *mut memoryword = zmem;
    let mut eqtb: *mut memoryword = zeqtb;
    let mut j: integer = 0;
    let mut k_0: integer = 0;
    let mut p: halfword = 0;
    let mut q: halfword = 0;
    let mut x: integer = 0;
    let mut formatengine: *mut ASCIIcode = ::core::ptr::null_mut::<ASCIIcode>();
    let mut dummyxord: ASCIIcode = 0;
    let mut dummyxchr: ASCIIcode = 0;
    let mut dummyxprn: ASCIIcode = 0;
    if iniversion != 0 {
        free(fontinfo as *mut ::core::ffi::c_void);
        free(strpool as *mut ::core::ffi::c_void);
        free(strstart as *mut ::core::ffi::c_void);
        free(yhash as *mut ::core::ffi::c_void);
        free(zeqtb as *mut ::core::ffi::c_void);
        free(yzmem as *mut ::core::ffi::c_void);
    }
    do_undump(
        &raw mut x as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        fmtfile as gzFile,
    );
    if debugformatfile != 0 {
        fprintf(
            __stderrp,
            b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
            b"fmtdebug:\0" as *const u8 as *const ::core::ffi::c_char,
            b"format magic number\0" as *const u8 as *const ::core::ffi::c_char,
        );
        fprintf(
            __stderrp,
            b"%s%ld\n\0" as *const u8 as *const ::core::ffi::c_char,
            b" = \0" as *const u8 as *const ::core::ffi::c_char,
            x as ::core::ffi::c_long,
        );
    }
    if !(x as ::core::ffi::c_long != 1462916184 as ::core::ffi::c_long) {
        do_undump(
            &raw mut x as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            fmtfile as gzFile,
        );
        if debugformatfile != 0 {
            fprintf(
                __stderrp,
                b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                b"fmtdebug:\0" as *const u8 as *const ::core::ffi::c_char,
                b"engine name size\0" as *const u8 as *const ::core::ffi::c_char,
            );
            fprintf(
                __stderrp,
                b"%s%ld\n\0" as *const u8 as *const ::core::ffi::c_char,
                b" = \0" as *const u8 as *const ::core::ffi::c_char,
                x as ::core::ffi::c_long,
            );
        }
        if !(x < 0 as ::core::ffi::c_int || x > 256 as ::core::ffi::c_int) {
            formatengine = xmalloc(
                ((x as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
                    .wrapping_mul(::core::mem::size_of::<ASCIIcode>() as size_t),
            ) as *mut ASCIIcode;
            do_undump(
                formatengine.offset(0 as ::core::ffi::c_int as isize) as *mut ASCIIcode
                    as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<ASCIIcode>() as ::core::ffi::c_int,
                x,
                fmtfile as gzFile,
            );
            *formatengine.offset((x as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                0 as ASCIIcode;
            if strcmp(
                enginename.as_ptr(),
                formatengine as string as *const ::core::ffi::c_char,
            ) != 0
            {
                fprintf(
                    __stdoutp,
                    b"%s%s%s%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    b"---! \0" as *const u8 as *const ::core::ffi::c_char,
                    nameoffile.offset(1 as ::core::ffi::c_int as isize) as string,
                    b" was written by \0" as *const u8 as *const ::core::ffi::c_char,
                    formatengine,
                );
                free(formatengine as *mut ::core::ffi::c_void);
            } else {
                free(formatengine as *mut ::core::ffi::c_void);
                do_undump(
                    &raw mut x as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    fmtfile as gzFile,
                );
                if debugformatfile != 0 {
                    fprintf(
                        __stderrp,
                        b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                        b"fmtdebug:\0" as *const u8 as *const ::core::ffi::c_char,
                        b"string pool checksum\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    fprintf(
                        __stderrp,
                        b"%s%ld\n\0" as *const u8 as *const ::core::ffi::c_char,
                        b" = \0" as *const u8 as *const ::core::ffi::c_char,
                        x as ::core::ffi::c_long,
                    );
                }
                if x as ::core::ffi::c_long != 312141437 as ::core::ffi::c_long {
                    fprintf(
                        __stdoutp,
                        b"%s%s%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                        b"---! \0" as *const u8 as *const ::core::ffi::c_char,
                        nameoffile.offset(1 as ::core::ffi::c_int as isize) as string,
                        b" made by different executable version, strings are different\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                    if !translate_filename.is_null() {
                        let mut for_end: integer = 0;
                        k_0 = 0 as ::core::ffi::c_int as integer;
                        for_end = 255 as ::core::ffi::c_int as integer;
                        if k_0 <= for_end {
                            loop {
                                do_undump(
                                    &raw mut dummyxord as *mut ::core::ffi::c_char,
                                    ::core::mem::size_of::<ASCIIcode>() as ::core::ffi::c_int,
                                    1 as ::core::ffi::c_int,
                                    fmtfile as gzFile,
                                );
                                let fresh31 = k_0;
                                k_0 = k_0 + 1;
                                if !(fresh31 < for_end) {
                                    break;
                                }
                            }
                        }
                        let mut for_end_0: integer = 0;
                        k_0 = 0 as ::core::ffi::c_int as integer;
                        for_end_0 = 255 as ::core::ffi::c_int as integer;
                        if k_0 <= for_end_0 {
                            loop {
                                do_undump(
                                    &raw mut dummyxchr as *mut ::core::ffi::c_char,
                                    ::core::mem::size_of::<ASCIIcode>() as ::core::ffi::c_int,
                                    1 as ::core::ffi::c_int,
                                    fmtfile as gzFile,
                                );
                                let fresh32 = k_0;
                                k_0 = k_0 + 1;
                                if !(fresh32 < for_end_0) {
                                    break;
                                }
                            }
                        }
                        let mut for_end_1: integer = 0;
                        k_0 = 0 as ::core::ffi::c_int as integer;
                        for_end_1 = 255 as ::core::ffi::c_int as integer;
                        if k_0 <= for_end_1 {
                            loop {
                                do_undump(
                                    &raw mut dummyxprn as *mut ::core::ffi::c_char,
                                    ::core::mem::size_of::<ASCIIcode>() as ::core::ffi::c_int,
                                    1 as ::core::ffi::c_int,
                                    fmtfile as gzFile,
                                );
                                let fresh33 = k_0;
                                k_0 = k_0 + 1;
                                if !(fresh33 < for_end_1) {
                                    break;
                                }
                            }
                        }
                    } else {
                        do_undump(
                            (&raw mut xord as *mut ASCIIcode)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *mut ASCIIcode
                                as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<ASCIIcode>() as ::core::ffi::c_int,
                            256 as ::core::ffi::c_int,
                            fmtfile as gzFile,
                        );
                        do_undump(
                            (&raw mut xchr as *mut ASCIIcode)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *mut ASCIIcode
                                as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<ASCIIcode>() as ::core::ffi::c_int,
                            256 as ::core::ffi::c_int,
                            fmtfile as gzFile,
                        );
                        do_undump(
                            (&raw mut xprn as *mut ASCIIcode)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as *mut ASCIIcode
                                as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<ASCIIcode>() as ::core::ffi::c_int,
                            256 as ::core::ffi::c_int,
                            fmtfile as gzFile,
                        );
                        if eightbitp != 0 {
                            let mut for_end_2: integer = 0;
                            k_0 = 0 as ::core::ffi::c_int as integer;
                            for_end_2 = 255 as ::core::ffi::c_int as integer;
                            if k_0 <= for_end_2 {
                                loop {
                                    xprn[k_0 as usize] = 1 as ASCIIcode;
                                    let fresh34 = k_0;
                                    k_0 = k_0 + 1;
                                    if !(fresh34 < for_end_2) {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    do_undump(
                        &raw mut x as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                        fmtfile as gzFile,
                    );
                    if !(x as ::core::ffi::c_long != 268435455 as ::core::ffi::c_long) {
                        do_undump(
                            &raw mut hashhigh as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<halfword>() as ::core::ffi::c_int,
                            1 as ::core::ffi::c_int,
                            fmtfile as gzFile,
                        );
                        if !(hashhigh < 0 as ::core::ffi::c_int
                            || hashhigh as ::core::ffi::c_long > suphashextra)
                        {
                            if hashextra < hashhigh {
                                hashextra = hashhigh;
                            }
                            eqtbtop = 30192 as halfword + hashextra;
                            if hashextra == 0 as ::core::ffi::c_int {
                                hashtop = 26627 as ::core::ffi::c_int as halfword;
                            } else {
                                hashtop = eqtbtop;
                            }
                            yhash = xmalloc(
                                ((1 as ::core::ffi::c_int + hashtop as ::core::ffi::c_int
                                    - 514 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<twohalves>() as size_t),
                            ) as *mut twohalves;
                            hash = yhash.offset(-(hashoffset as isize));
                            (*hash.offset(514 as ::core::ffi::c_int as isize)).v.LH =
                                0 as ::core::ffi::c_int as halfword;
                            (*hash.offset(514 as ::core::ffi::c_int as isize)).v.RH =
                                0 as ::core::ffi::c_int as halfword;
                            let mut for_end_3: integer = 0;
                            x = 515 as ::core::ffi::c_int as integer;
                            for_end_3 = hashtop as integer;
                            if x <= for_end_3 {
                                loop {
                                    *hash.offset(x as isize) =
                                        *hash.offset(514 as ::core::ffi::c_int as isize);
                                    let fresh35 = x;
                                    x = x + 1;
                                    if !(fresh35 < for_end_3) {
                                        break;
                                    }
                                }
                            }
                            zeqtb = xmalloc(
                                ((eqtbtop as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<memoryword>() as size_t),
                            ) as *mut memoryword;
                            eqtb = zeqtb;
                            (*eqtb.offset(26627 as ::core::ffi::c_int as isize)).hh.u.B0 =
                                104 as ::core::ffi::c_short;
                            (*eqtb.offset(26627 as ::core::ffi::c_int as isize)).hh.v.RH =
                                -(268435455 as ::core::ffi::c_long) as halfword;
                            (*eqtb.offset(26627 as ::core::ffi::c_int as isize)).hh.u.B1 =
                                0 as ::core::ffi::c_short;
                            let mut for_end_4: integer = 0;
                            x = 30193 as ::core::ffi::c_int as integer;
                            for_end_4 = eqtbtop as integer;
                            if x <= for_end_4 {
                                loop {
                                    *eqtb.offset(x as isize) =
                                        *eqtb.offset(26627 as ::core::ffi::c_int as isize);
                                    let fresh36 = x;
                                    x = x + 1;
                                    if !(fresh36 < for_end_4) {
                                        break;
                                    }
                                }
                            }
                            do_undump(
                                &raw mut x as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                1 as ::core::ffi::c_int,
                                fmtfile as gzFile,
                            );
                            if !(x < 0 as ::core::ffi::c_int || x > 1 as ::core::ffi::c_int) {
                                eTeXmode = x as ::core::ffi::c_uchar;
                                if eTeXmode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                                    maxregnum = 32767 as ::core::ffi::c_int as halfword;
                                    maxreghelpline = 2092 as ::core::ffi::c_int as strnumber;
                                } else {
                                    maxregnum = 255 as ::core::ffi::c_int as halfword;
                                    maxreghelpline = 800 as ::core::ffi::c_int as strnumber;
                                }
                                do_undump(
                                    &raw mut x as *mut ::core::ffi::c_char,
                                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                    1 as ::core::ffi::c_int,
                                    fmtfile as gzFile,
                                );
                                if debugformatfile != 0 {
                                    fprintf(
                                        __stderrp,
                                        b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                                        b"fmtdebug:\0" as *const u8 as *const ::core::ffi::c_char,
                                        b"mem_bot\0" as *const u8 as *const ::core::ffi::c_char,
                                    );
                                    fprintf(
                                        __stderrp,
                                        b"%s%ld\n\0" as *const u8 as *const ::core::ffi::c_char,
                                        b" = \0" as *const u8 as *const ::core::ffi::c_char,
                                        x as ::core::ffi::c_long,
                                    );
                                }
                                if !(x != membot) {
                                    do_undump(
                                        &raw mut memtop as *mut ::core::ffi::c_char,
                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                        1 as ::core::ffi::c_int,
                                        fmtfile as gzFile,
                                    );
                                    if debugformatfile != 0 {
                                        fprintf(
                                            __stderrp,
                                            b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                                            b"fmtdebug:\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            b"mem_top\0" as *const u8 as *const ::core::ffi::c_char,
                                        );
                                        fprintf(
                                            __stderrp,
                                            b"%s%ld\n\0" as *const u8 as *const ::core::ffi::c_char,
                                            b" = \0" as *const u8 as *const ::core::ffi::c_char,
                                            memtop as ::core::ffi::c_long,
                                        );
                                    }
                                    if !(membot as ::core::ffi::c_int + 1100 as ::core::ffi::c_int
                                        > memtop)
                                    {
                                        curlist.headfield = (memtop as ::core::ffi::c_int
                                            - 1 as ::core::ffi::c_int)
                                            as halfword;
                                        curlist.tailfield = (memtop as ::core::ffi::c_int
                                            - 1 as ::core::ffi::c_int)
                                            as halfword;
                                        pagetail = (memtop as ::core::ffi::c_int
                                            - 2 as ::core::ffi::c_int)
                                            as halfword;
                                        memmin = membot - extramembot;
                                        memmax = memtop + extramemtop;
                                        yzmem = xmalloc(
                                            ((memmax as ::core::ffi::c_int
                                                - memmin as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int
                                                + 1 as ::core::ffi::c_int)
                                                as size_t)
                                                .wrapping_mul(
                                                    ::core::mem::size_of::<memoryword>() as size_t
                                                ),
                                        )
                                            as *mut memoryword;
                                        zmem = yzmem.offset(-(memmin as isize));
                                        mem = zmem;
                                        do_undump(
                                            &raw mut x as *mut ::core::ffi::c_char,
                                            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                            1 as ::core::ffi::c_int,
                                            fmtfile as gzFile,
                                        );
                                        if !(x != 30192 as ::core::ffi::c_int) {
                                            do_undump(
                                                &raw mut x as *mut ::core::ffi::c_char,
                                                ::core::mem::size_of::<integer>()
                                                    as ::core::ffi::c_int,
                                                1 as ::core::ffi::c_int,
                                                fmtfile as gzFile,
                                            );
                                            if !(x != 8501 as ::core::ffi::c_int) {
                                                do_undump(
                                                    &raw mut x as *mut ::core::ffi::c_char,
                                                    ::core::mem::size_of::<integer>()
                                                        as ::core::ffi::c_int,
                                                    1 as ::core::ffi::c_int,
                                                    fmtfile as gzFile,
                                                );
                                                if !(x != 607 as ::core::ffi::c_int) {
                                                    do_undump(
                                                        &raw mut x as *mut ::core::ffi::c_char,
                                                        ::core::mem::size_of::<integer>()
                                                            as ::core::ffi::c_int,
                                                        1 as ::core::ffi::c_int,
                                                        fmtfile as gzFile,
                                                    );
                                                    if !(x as ::core::ffi::c_long
                                                        != 1296847960 as ::core::ffi::c_long)
                                                    {
                                                        do_undump(
                                                            &raw mut x as *mut ::core::ffi::c_char,
                                                            ::core::mem::size_of::<integer>()
                                                                as ::core::ffi::c_int,
                                                            1 as ::core::ffi::c_int,
                                                            fmtfile as gzFile,
                                                        );
                                                        if x == 1 as ::core::ffi::c_int {
                                                            mltexenabledp = true_0 as boolean;
                                                            current_block = 14913924298693586572;
                                                        } else if x != 0 as ::core::ffi::c_int {
                                                            current_block = 15581621980376734741;
                                                        } else {
                                                            current_block = 14913924298693586572;
                                                        }
                                                        match current_block {
                                                            15581621980376734741 => {}
                                                            _ => {
                                                                do_undump(
                                                                    &raw mut x
                                                                        as *mut ::core::ffi::c_char,
                                                                    ::core::mem::size_of::<integer>(
                                                                    )
                                                                        as ::core::ffi::c_int,
                                                                    1 as ::core::ffi::c_int,
                                                                    fmtfile as gzFile,
                                                                );
                                                                if !(x as ::core::ffi::c_long
                                                                    != 1162040408
                                                                        as ::core::ffi::c_long)
                                                                {
                                                                    do_undump(
                                                                        &raw mut x as *mut ::core::ffi::c_char,
                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                        1 as ::core::ffi::c_int,
                                                                        fmtfile as gzFile,
                                                                    );
                                                                    if x == 0 as ::core::ffi::c_int
                                                                    {
                                                                        enctexenabledp =
                                                                            false_0 as boolean;
                                                                        current_block =
                                                                            9073771928613846474;
                                                                    } else if x
                                                                        != 1 as ::core::ffi::c_int
                                                                    {
                                                                        current_block =
                                                                            15581621980376734741;
                                                                    } else {
                                                                        enctexenabledp =
                                                                            true_0 as boolean;
                                                                        do_undump(
                                                                            (&raw mut mubyteread as *mut halfword)
                                                                                .offset(0 as ::core::ffi::c_int as isize) as *mut halfword
                                                                                as *mut ::core::ffi::c_char,
                                                                            ::core::mem::size_of::<halfword>() as ::core::ffi::c_int,
                                                                            256 as ::core::ffi::c_int,
                                                                            fmtfile as gzFile,
                                                                        );
                                                                        do_undump(
                                                                            (&raw mut mubytewrite as *mut strnumber)
                                                                                .offset(0 as ::core::ffi::c_int as isize) as *mut strnumber
                                                                                as *mut ::core::ffi::c_char,
                                                                            ::core::mem::size_of::<strnumber>() as ::core::ffi::c_int,
                                                                            256 as ::core::ffi::c_int,
                                                                            fmtfile as gzFile,
                                                                        );
                                                                        do_undump(
                                                                            (&raw mut mubytecswrite as *mut halfword)
                                                                                .offset(0 as ::core::ffi::c_int as isize) as *mut halfword
                                                                                as *mut ::core::ffi::c_char,
                                                                            ::core::mem::size_of::<halfword>() as ::core::ffi::c_int,
                                                                            128 as ::core::ffi::c_int,
                                                                            fmtfile as gzFile,
                                                                        );
                                                                        current_block =
                                                                            9073771928613846474;
                                                                    }
                                                                    match current_block {
                                                                        15581621980376734741 => {}
                                                                        _ => {
                                                                            do_undump(
                                                                                &raw mut x as *mut ::core::ffi::c_char,
                                                                                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                1 as ::core::ffi::c_int,
                                                                                fmtfile as gzFile,
                                                                            );
                                                                            if !(x < 0 as ::core::ffi::c_int) {
                                                                                if x as ::core::ffi::c_long
                                                                                    > suppoolsize - poolfree as ::core::ffi::c_long
                                                                                {
                                                                                    fprintf(
                                                                                        __stdoutp,
                                                                                        b"%s%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                        b"---! Must increase the \0" as *const u8
                                                                                            as *const ::core::ffi::c_char,
                                                                                        b"string pool size\0" as *const u8
                                                                                            as *const ::core::ffi::c_char,
                                                                                    );
                                                                                } else {
                                                                                    if debugformatfile != 0 {
                                                                                        fprintf(
                                                                                            __stderrp,
                                                                                            b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                            b"fmtdebug:\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                            b"string pool size\0" as *const u8
                                                                                                as *const ::core::ffi::c_char,
                                                                                        );
                                                                                        fprintf(
                                                                                            __stderrp,
                                                                                            b"%s%ld\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                            b" = \0" as *const u8 as *const ::core::ffi::c_char,
                                                                                            x as ::core::ffi::c_long,
                                                                                        );
                                                                                    }
                                                                                    poolptr = x;
                                                                                    if poolsize < poolptr + poolfree {
                                                                                        poolsize = poolptr + poolfree;
                                                                                    }
                                                                                    do_undump(
                                                                                        &raw mut x as *mut ::core::ffi::c_char,
                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                        1 as ::core::ffi::c_int,
                                                                                        fmtfile as gzFile,
                                                                                    );
                                                                                    if !(x < 0 as ::core::ffi::c_int) {
                                                                                        if x as ::core::ffi::c_long
                                                                                            > supmaxstrings - stringsfree as ::core::ffi::c_long
                                                                                        {
                                                                                            fprintf(
                                                                                                __stdoutp,
                                                                                                b"%s%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                b"---! Must increase the \0" as *const u8
                                                                                                    as *const ::core::ffi::c_char,
                                                                                                b"sup strings\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                            );
                                                                                        } else {
                                                                                            if debugformatfile != 0 {
                                                                                                fprintf(
                                                                                                    __stderrp,
                                                                                                    b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                    b"fmtdebug:\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                    b"sup strings\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                );
                                                                                                fprintf(
                                                                                                    __stderrp,
                                                                                                    b"%s%ld\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                    b" = \0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                    x as ::core::ffi::c_long,
                                                                                                );
                                                                                            }
                                                                                            strptr = x as strnumber;
                                                                                            if maxstrings < strptr as integer + stringsfree {
                                                                                                maxstrings = strptr as integer + stringsfree;
                                                                                            }
                                                                                            strstart = xmalloc(
                                                                                                ((maxstrings as ::core::ffi::c_int
                                                                                                    + 1 as ::core::ffi::c_int) as size_t)
                                                                                                    .wrapping_mul(
                                                                                                        ::core::mem::size_of::<poolpointer>() as size_t,
                                                                                                    ),
                                                                                            ) as *mut poolpointer;
                                                                                            let mut i: ::core::ffi::c_uint = 0;
                                                                                            do_undump(
                                                                                                strstart.offset(0 as ::core::ffi::c_int as isize)
                                                                                                    as *mut poolpointer as *mut ::core::ffi::c_char,
                                                                                                ::core::mem::size_of::<poolpointer>() as ::core::ffi::c_int,
                                                                                                strptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                fmtfile as gzFile,
                                                                                            );
                                                                                            i = 0 as ::core::ffi::c_uint;
                                                                                            while i
                                                                                                < (strptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                    as ::core::ffi::c_uint
                                                                                            {
                                                                                                if *(strstart.offset(0 as ::core::ffi::c_int as isize)
                                                                                                    as *mut poolpointer)
                                                                                                    .offset(i as isize) < 0 as ::core::ffi::c_int
                                                                                                    || *(strstart.offset(0 as ::core::ffi::c_int as isize)
                                                                                                        as *mut poolpointer)
                                                                                                        .offset(i as isize) > poolptr
                                                                                                {
                                                                                                    fprintf(
                                                                                                        __stderrp,
                                                                                                        b"%s: fatal: (kpathsea) \0" as *const u8
                                                                                                            as *const ::core::ffi::c_char,
                                                                                                        (*kpse_def).invocation_name,
                                                                                                    );
                                                                                                    fprintf(
                                                                                                        __stderrp,
                                                                                                        b"Item %u (=%ld) of .fmt array at %lx <%ld or >%ld\0"
                                                                                                            as *const u8 as *const ::core::ffi::c_char,
                                                                                                        i,
                                                                                                        *(strstart.offset(0 as ::core::ffi::c_int as isize)
                                                                                                            as *mut poolpointer)
                                                                                                            .offset(i as isize) as uintptr_t,
                                                                                                        strstart.offset(0 as ::core::ffi::c_int as isize)
                                                                                                            as *mut poolpointer as uintptr_t,
                                                                                                        0 as ::core::ffi::c_int as uintptr_t,
                                                                                                        poolptr as uintptr_t,
                                                                                                    );
                                                                                                    fputs(
                                                                                                        b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                        __stderrp,
                                                                                                    );
                                                                                                    exit(1 as ::core::ffi::c_int);
                                                                                                }
                                                                                                i = i.wrapping_add(1);
                                                                                            }
                                                                                            strpool = xmalloc(
                                                                                                ((poolsize as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                    as size_t)
                                                                                                    .wrapping_mul(
                                                                                                        ::core::mem::size_of::<packedASCIIcode>() as size_t,
                                                                                                    ),
                                                                                            ) as *mut packedASCIIcode;
                                                                                            do_undump(
                                                                                                strpool.offset(0 as ::core::ffi::c_int as isize)
                                                                                                    as *mut packedASCIIcode as *mut ::core::ffi::c_char,
                                                                                                ::core::mem::size_of::<packedASCIIcode>()
                                                                                                    as ::core::ffi::c_int,
                                                                                                poolptr,
                                                                                                fmtfile as gzFile,
                                                                                            );
                                                                                            initstrptr = strptr;
                                                                                            initpoolptr = poolptr as poolpointer;
                                                                                            do_undump(
                                                                                                &raw mut x as *mut ::core::ffi::c_char,
                                                                                                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                1 as ::core::ffi::c_int,
                                                                                                fmtfile as gzFile,
                                                                                            );
                                                                                            if !(x
                                                                                                < membot as ::core::ffi::c_int + 1019 as ::core::ffi::c_int
                                                                                                || x
                                                                                                    > memtop as ::core::ffi::c_int - 15 as ::core::ffi::c_int)
                                                                                            {
                                                                                                lomemmax = x as halfword;
                                                                                                do_undump(
                                                                                                    &raw mut x as *mut ::core::ffi::c_char,
                                                                                                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                    1 as ::core::ffi::c_int,
                                                                                                    fmtfile as gzFile,
                                                                                                );
                                                                                                if !(x
                                                                                                    < membot as ::core::ffi::c_int + 20 as ::core::ffi::c_int
                                                                                                    || x > lomemmax)
                                                                                                {
                                                                                                    rover = x as halfword;
                                                                                                    if eTeXmode as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                                                                                                    {
                                                                                                        let mut for_end_5: integer = 0;
                                                                                                        k_0 = 0 as ::core::ffi::c_int as integer;
                                                                                                        for_end_5 = 5 as ::core::ffi::c_int as integer;
                                                                                                        if k_0 <= for_end_5 {
                                                                                                            loop {
                                                                                                                do_undump(
                                                                                                                    &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                    1 as ::core::ffi::c_int,
                                                                                                                    fmtfile as gzFile,
                                                                                                                );
                                                                                                                if (x as ::core::ffi::c_long)
                                                                                                                    < -(268435455 as ::core::ffi::c_long) || x > lomemmax
                                                                                                                {
                                                                                                                    current_block = 15581621980376734741;
                                                                                                                    break;
                                                                                                                }
                                                                                                                saroot[k_0 as usize] = x as halfword;
                                                                                                                let fresh37 = k_0;
                                                                                                                k_0 = k_0 + 1;
                                                                                                                if !(fresh37 < for_end_5) {
                                                                                                                    current_block = 13161952823003036500;
                                                                                                                    break;
                                                                                                                }
                                                                                                            }
                                                                                                        } else {
                                                                                                            current_block = 13161952823003036500;
                                                                                                        }
                                                                                                    } else {
                                                                                                        current_block = 13161952823003036500;
                                                                                                    }
                                                                                                    match current_block {
                                                                                                        15581621980376734741 => {}
                                                                                                        _ => {
                                                                                                            p = membot as halfword;
                                                                                                            q = rover;
                                                                                                            loop {
                                                                                                                do_undump(
                                                                                                                    mem.offset(p as isize) as *mut memoryword
                                                                                                                        as *mut ::core::ffi::c_char,
                                                                                                                    ::core::mem::size_of::<memoryword>() as ::core::ffi::c_int,
                                                                                                                    q + 2 as halfword - p,
                                                                                                                    fmtfile as gzFile,
                                                                                                                );
                                                                                                                if (*mem.offset(q as isize)).hh.v.LH > lomemmax - q
                                                                                                                    || (*mem
                                                                                                                        .offset(
                                                                                                                            (q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                                                                                                        ))
                                                                                                                        .hh
                                                                                                                        .v
                                                                                                                        .RH > lomemmax
                                                                                                                    || q
                                                                                                                        >= (*mem
                                                                                                                            .offset(
                                                                                                                                (q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                                                                                                            ))
                                                                                                                            .hh
                                                                                                                            .v
                                                                                                                            .RH
                                                                                                                        && (*mem
                                                                                                                            .offset(
                                                                                                                                (q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                                                                                                            ))
                                                                                                                            .hh
                                                                                                                            .v
                                                                                                                            .RH != rover
                                                                                                                {
                                                                                                                    current_block = 15581621980376734741;
                                                                                                                    break;
                                                                                                                }
                                                                                                                p = q + (*mem.offset(q as isize)).hh.v.LH;
                                                                                                                q = (*mem
                                                                                                                    .offset(
                                                                                                                        (q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                                                                                                    ))
                                                                                                                    .hh
                                                                                                                    .v
                                                                                                                    .RH;
                                                                                                                if q == rover {
                                                                                                                    current_block = 4127803603908737533;
                                                                                                                    break;
                                                                                                                }
                                                                                                            }
                                                                                                            match current_block {
                                                                                                                15581621980376734741 => {}
                                                                                                                _ => {
                                                                                                                    do_undump(
                                                                                                                        mem.offset(p as isize) as *mut memoryword
                                                                                                                            as *mut ::core::ffi::c_char,
                                                                                                                        ::core::mem::size_of::<memoryword>() as ::core::ffi::c_int,
                                                                                                                        lomemmax + 1 as halfword - p,
                                                                                                                        fmtfile as gzFile,
                                                                                                                    );
                                                                                                                    if memmin
                                                                                                                        < membot as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                                                                                                    {
                                                                                                                        p = (*mem
                                                                                                                            .offset(
                                                                                                                                (rover as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                    as isize,
                                                                                                                            ))
                                                                                                                            .hh
                                                                                                                            .v
                                                                                                                            .LH;
                                                                                                                        q = (memmin as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                            as halfword;
                                                                                                                        (*mem.offset(memmin as isize)).hh.v.RH = -(268435455
                                                                                                                            as ::core::ffi::c_long) as halfword;
                                                                                                                        (*mem.offset(memmin as isize)).hh.v.LH = -(268435455
                                                                                                                            as ::core::ffi::c_long) as halfword;
                                                                                                                        (*mem
                                                                                                                            .offset(
                                                                                                                                (p as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                                                                                                            ))
                                                                                                                            .hh
                                                                                                                            .v
                                                                                                                            .RH = q;
                                                                                                                        (*mem
                                                                                                                            .offset(
                                                                                                                                (rover as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                    as isize,
                                                                                                                            ))
                                                                                                                            .hh
                                                                                                                            .v
                                                                                                                            .LH = q;
                                                                                                                        (*mem
                                                                                                                            .offset(
                                                                                                                                (q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                                                                                                            ))
                                                                                                                            .hh
                                                                                                                            .v
                                                                                                                            .RH = rover;
                                                                                                                        (*mem
                                                                                                                            .offset(
                                                                                                                                (q as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                                                                                                            ))
                                                                                                                            .hh
                                                                                                                            .v
                                                                                                                            .LH = p;
                                                                                                                        (*mem.offset(q as isize)).hh.v.RH = 268435455
                                                                                                                            as ::core::ffi::c_long as halfword;
                                                                                                                        (*mem.offset(q as isize)).hh.v.LH = membot as halfword - q;
                                                                                                                    }
                                                                                                                    do_undump(
                                                                                                                        &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                        1 as ::core::ffi::c_int,
                                                                                                                        fmtfile as gzFile,
                                                                                                                    );
                                                                                                                    if !(x
                                                                                                                        < lomemmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                                                                                                        || x
                                                                                                                            > memtop as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                                                                                                                    {
                                                                                                                        himemmin = x as halfword;
                                                                                                                        do_undump(
                                                                                                                            &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                            1 as ::core::ffi::c_int,
                                                                                                                            fmtfile as gzFile,
                                                                                                                        );
                                                                                                                        if !((x as ::core::ffi::c_long)
                                                                                                                            < -(268435455 as ::core::ffi::c_long) || x > memtop)
                                                                                                                        {
                                                                                                                            avail = x as halfword;
                                                                                                                            memend = memtop as halfword;
                                                                                                                            do_undump(
                                                                                                                                mem.offset(himemmin as isize) as *mut memoryword
                                                                                                                                    as *mut ::core::ffi::c_char,
                                                                                                                                ::core::mem::size_of::<memoryword>() as ::core::ffi::c_int,
                                                                                                                                memend + 1 as halfword - himemmin,
                                                                                                                                fmtfile as gzFile,
                                                                                                                            );
                                                                                                                            do_undump(
                                                                                                                                &raw mut varused as *mut ::core::ffi::c_char,
                                                                                                                                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                1 as ::core::ffi::c_int,
                                                                                                                                fmtfile as gzFile,
                                                                                                                            );
                                                                                                                            do_undump(
                                                                                                                                &raw mut dynused as *mut ::core::ffi::c_char,
                                                                                                                                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                1 as ::core::ffi::c_int,
                                                                                                                                fmtfile as gzFile,
                                                                                                                            );
                                                                                                                            k_0 = 1 as ::core::ffi::c_int as integer;
                                                                                                                            loop {
                                                                                                                                do_undump(
                                                                                                                                    &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                    1 as ::core::ffi::c_int,
                                                                                                                                    fmtfile as gzFile,
                                                                                                                                );
                                                                                                                                if x < 1 as ::core::ffi::c_int
                                                                                                                                    || k_0 + x > 30193 as ::core::ffi::c_int
                                                                                                                                {
                                                                                                                                    current_block = 15581621980376734741;
                                                                                                                                    break;
                                                                                                                                }
                                                                                                                                do_undump(
                                                                                                                                    eqtb.offset(k_0 as isize) as *mut memoryword
                                                                                                                                        as *mut ::core::ffi::c_char,
                                                                                                                                    ::core::mem::size_of::<memoryword>() as ::core::ffi::c_int,
                                                                                                                                    x,
                                                                                                                                    fmtfile as gzFile,
                                                                                                                                );
                                                                                                                                k_0 = k_0 + x;
                                                                                                                                do_undump(
                                                                                                                                    &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                    1 as ::core::ffi::c_int,
                                                                                                                                    fmtfile as gzFile,
                                                                                                                                );
                                                                                                                                if x < 0 as ::core::ffi::c_int
                                                                                                                                    || k_0 + x > 30193 as ::core::ffi::c_int
                                                                                                                                {
                                                                                                                                    current_block = 15581621980376734741;
                                                                                                                                    break;
                                                                                                                                }
                                                                                                                                let mut for_end_6: integer = 0;
                                                                                                                                j = k_0;
                                                                                                                                for_end_6 = (k_0 as ::core::ffi::c_int
                                                                                                                                    + x as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                                                                                                                    as integer;
                                                                                                                                if j <= for_end_6 {
                                                                                                                                    loop {
                                                                                                                                        *eqtb.offset(j as isize) = *eqtb
                                                                                                                                            .offset(
                                                                                                                                                (k_0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                                                                                                                                    as isize,
                                                                                                                                            );
                                                                                                                                        let fresh38 = j;
                                                                                                                                        j = j + 1;
                                                                                                                                        if !(fresh38 < for_end_6) {
                                                                                                                                            break;
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                k_0 = k_0 + x;
                                                                                                                                if k_0 > 30192 as ::core::ffi::c_int {
                                                                                                                                    current_block = 13434751124187322381;
                                                                                                                                    break;
                                                                                                                                }
                                                                                                                            }
                                                                                                                            match current_block {
                                                                                                                                15581621980376734741 => {}
                                                                                                                                _ => {
                                                                                                                                    if hashhigh > 0 as ::core::ffi::c_int {
                                                                                                                                        do_undump(
                                                                                                                                            eqtb.offset(30193 as ::core::ffi::c_int as isize)
                                                                                                                                                as *mut memoryword as *mut ::core::ffi::c_char,
                                                                                                                                            ::core::mem::size_of::<memoryword>() as ::core::ffi::c_int,
                                                                                                                                            hashhigh,
                                                                                                                                            fmtfile as gzFile,
                                                                                                                                        );
                                                                                                                                    }
                                                                                                                                    do_undump(
                                                                                                                                        &raw mut parloc as *mut ::core::ffi::c_char,
                                                                                                                                        ::core::mem::size_of::<halfword>() as ::core::ffi::c_int,
                                                                                                                                        1 as ::core::ffi::c_int,
                                                                                                                                        fmtfile as gzFile,
                                                                                                                                    );
                                                                                                                                    partoken = 4095 as halfword + parloc;
                                                                                                                                    do_undump(
                                                                                                                                        &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                        1 as ::core::ffi::c_int,
                                                                                                                                        fmtfile as gzFile,
                                                                                                                                    );
                                                                                                                                    if !(x < 514 as ::core::ffi::c_int || x > hashtop) {
                                                                                                                                        writeloc = x as halfword;
                                                                                                                                        let mut for_end_7: integer = 0;
                                                                                                                                        p = 0 as ::core::ffi::c_int as halfword;
                                                                                                                                        for_end_7 = 2100 as ::core::ffi::c_int as integer;
                                                                                                                                        if p <= for_end_7 {
                                                                                                                                            loop {
                                                                                                                                                do_undump(
                                                                                                                                                    (&raw mut prim as *mut twohalves).offset(p as isize)
                                                                                                                                                        as *mut twohalves as *mut ::core::ffi::c_char,
                                                                                                                                                    ::core::mem::size_of::<twohalves>() as ::core::ffi::c_int,
                                                                                                                                                    1 as ::core::ffi::c_int,
                                                                                                                                                    fmtfile as gzFile,
                                                                                                                                                );
                                                                                                                                                let fresh39 = p;
                                                                                                                                                p = p + 1;
                                                                                                                                                if !(fresh39 < for_end_7) {
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                        do_undump(
                                                                                                                                            &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                                            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                            1 as ::core::ffi::c_int,
                                                                                                                                            fmtfile as gzFile,
                                                                                                                                        );
                                                                                                                                        if !(x < 514 as ::core::ffi::c_int
                                                                                                                                            || x > 15514 as ::core::ffi::c_int)
                                                                                                                                        {
                                                                                                                                            hashused = x as halfword;
                                                                                                                                            p = 513 as ::core::ffi::c_int as halfword;
                                                                                                                                            loop {
                                                                                                                                                do_undump(
                                                                                                                                                    &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                                                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                    1 as ::core::ffi::c_int,
                                                                                                                                                    fmtfile as gzFile,
                                                                                                                                                );
                                                                                                                                                if x < p as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                                                                                                                                    || x > hashused
                                                                                                                                                {
                                                                                                                                                    current_block = 15581621980376734741;
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                                p = x as halfword;
                                                                                                                                                do_undump(
                                                                                                                                                    hash.offset(p as isize) as *mut twohalves
                                                                                                                                                        as *mut ::core::ffi::c_char,
                                                                                                                                                    ::core::mem::size_of::<twohalves>() as ::core::ffi::c_int,
                                                                                                                                                    1 as ::core::ffi::c_int,
                                                                                                                                                    fmtfile as gzFile,
                                                                                                                                                );
                                                                                                                                                if p == hashused {
                                                                                                                                                    current_block = 9350489878244555550;
                                                                                                                                                    break;
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                            match current_block {
                                                                                                                                                15581621980376734741 => {}
                                                                                                                                                _ => {
                                                                                                                                                    do_undump(
                                                                                                                                                        hash
                                                                                                                                                            .offset(
                                                                                                                                                                (hashused as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                    as isize,
                                                                                                                                                            ) as *mut twohalves as *mut ::core::ffi::c_char,
                                                                                                                                                        ::core::mem::size_of::<twohalves>() as ::core::ffi::c_int,
                                                                                                                                                        26626 as halfword - hashused,
                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                    );
                                                                                                                                                    if debugformatfile != 0 {
                                                                                                                                                        zprintcsnames(
                                                                                                                                                            514 as ::core::ffi::c_int,
                                                                                                                                                            26626 as ::core::ffi::c_int,
                                                                                                                                                        );
                                                                                                                                                    }
                                                                                                                                                    if hashhigh > 0 as ::core::ffi::c_int {
                                                                                                                                                        do_undump(
                                                                                                                                                            hash.offset(30193 as ::core::ffi::c_int as isize)
                                                                                                                                                                as *mut twohalves as *mut ::core::ffi::c_char,
                                                                                                                                                            ::core::mem::size_of::<twohalves>() as ::core::ffi::c_int,
                                                                                                                                                            hashhigh,
                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                        );
                                                                                                                                                        if debugformatfile != 0 {
                                                                                                                                                            zprintcsnames(
                                                                                                                                                                30193 as ::core::ffi::c_int,
                                                                                                                                                                hashhigh as ::core::ffi::c_int - 30193 as ::core::ffi::c_int,
                                                                                                                                                            );
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                    do_undump(
                                                                                                                                                        &raw mut cscount as *mut ::core::ffi::c_char,
                                                                                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                        1 as ::core::ffi::c_int,
                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                    );
                                                                                                                                                    do_undump(
                                                                                                                                                        &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                        1 as ::core::ffi::c_int,
                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                    );
                                                                                                                                                    if !(x < 7 as ::core::ffi::c_int) {
                                                                                                                                                        if x as ::core::ffi::c_long > supfontmemsize {
                                                                                                                                                            fprintf(
                                                                                                                                                                __stdoutp,
                                                                                                                                                                b"%s%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                b"---! Must increase the \0" as *const u8
                                                                                                                                                                    as *const ::core::ffi::c_char,
                                                                                                                                                                b"font mem size\0" as *const u8
                                                                                                                                                                    as *const ::core::ffi::c_char,
                                                                                                                                                            );
                                                                                                                                                        } else {
                                                                                                                                                            if debugformatfile != 0 {
                                                                                                                                                                fprintf(
                                                                                                                                                                    __stderrp,
                                                                                                                                                                    b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                    b"fmtdebug:\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                    b"font mem size\0" as *const u8
                                                                                                                                                                        as *const ::core::ffi::c_char,
                                                                                                                                                                );
                                                                                                                                                                fprintf(
                                                                                                                                                                    __stderrp,
                                                                                                                                                                    b"%s%ld\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                    b" = \0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                    x as ::core::ffi::c_long,
                                                                                                                                                                );
                                                                                                                                                            }
                                                                                                                                                            fmemptr = x as fontindex;
                                                                                                                                                            if fmemptr > fontmemsize {
                                                                                                                                                                fontmemsize = fmemptr as integer;
                                                                                                                                                            }
                                                                                                                                                            fontinfo = xmalloc(
                                                                                                                                                                ((fontmemsize as ::core::ffi::c_int
                                                                                                                                                                    + 1 as ::core::ffi::c_int) as size_t)
                                                                                                                                                                    .wrapping_mul(
                                                                                                                                                                        ::core::mem::size_of::<fmemoryword>() as size_t,
                                                                                                                                                                    ),
                                                                                                                                                            ) as *mut fmemoryword;
                                                                                                                                                            do_undump(
                                                                                                                                                                fontinfo.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                    as *mut fmemoryword as *mut ::core::ffi::c_char,
                                                                                                                                                                ::core::mem::size_of::<fmemoryword>() as ::core::ffi::c_int,
                                                                                                                                                                fmemptr,
                                                                                                                                                                fmtfile as gzFile,
                                                                                                                                                            );
                                                                                                                                                            do_undump(
                                                                                                                                                                &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                                                                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                1 as ::core::ffi::c_int,
                                                                                                                                                                fmtfile as gzFile,
                                                                                                                                                            );
                                                                                                                                                            if !(x < 0 as ::core::ffi::c_int) {
                                                                                                                                                                if x > 9000 as ::core::ffi::c_int {
                                                                                                                                                                    fprintf(
                                                                                                                                                                        __stdoutp,
                                                                                                                                                                        b"%s%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                        b"---! Must increase the \0" as *const u8
                                                                                                                                                                            as *const ::core::ffi::c_char,
                                                                                                                                                                        b"font max\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                    );
                                                                                                                                                                } else {
                                                                                                                                                                    if debugformatfile != 0 {
                                                                                                                                                                        fprintf(
                                                                                                                                                                            __stderrp,
                                                                                                                                                                            b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                            b"fmtdebug:\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                            b"font max\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                        );
                                                                                                                                                                        fprintf(
                                                                                                                                                                            __stderrp,
                                                                                                                                                                            b"%s%ld\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                            b" = \0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                            x as ::core::ffi::c_long,
                                                                                                                                                                        );
                                                                                                                                                                    }
                                                                                                                                                                    fontptr = x as internalfontnumber;
                                                                                                                                                                    fontcheck = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(
                                                                                                                                                                                ::core::mem::size_of::<fourquarters>() as size_t,
                                                                                                                                                                            ),
                                                                                                                                                                    ) as *mut fourquarters;
                                                                                                                                                                    fontsize = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<scaled>() as size_t),
                                                                                                                                                                    ) as *mut scaled;
                                                                                                                                                                    fontdsize = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<scaled>() as size_t),
                                                                                                                                                                    ) as *mut scaled;
                                                                                                                                                                    fontparams = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<fontindex>() as size_t),
                                                                                                                                                                    ) as *mut fontindex;
                                                                                                                                                                    fontname = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<strnumber>() as size_t),
                                                                                                                                                                    ) as *mut strnumber;
                                                                                                                                                                    fontarea = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<strnumber>() as size_t),
                                                                                                                                                                    ) as *mut strnumber;
                                                                                                                                                                    fontbc = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<eightbits>() as size_t),
                                                                                                                                                                    ) as *mut eightbits;
                                                                                                                                                                    fontec = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<eightbits>() as size_t),
                                                                                                                                                                    ) as *mut eightbits;
                                                                                                                                                                    fontglue = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<halfword>() as size_t),
                                                                                                                                                                    ) as *mut halfword;
                                                                                                                                                                    hyphenchar = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    skewchar = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    bcharlabel = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<fontindex>() as size_t),
                                                                                                                                                                    ) as *mut fontindex;
                                                                                                                                                                    fontbchar = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<ninebits>() as size_t),
                                                                                                                                                                    ) as *mut ninebits;
                                                                                                                                                                    fontfalsebchar = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<ninebits>() as size_t),
                                                                                                                                                                    ) as *mut ninebits;
                                                                                                                                                                    charbase = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    widthbase = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    heightbase = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    depthbase = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    italicbase = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    ligkernbase = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    kernbase = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    extenbase = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    parambase = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    pdfcharused = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(
                                                                                                                                                                                ::core::mem::size_of::<charusedarray>() as size_t,
                                                                                                                                                                            ),
                                                                                                                                                                    ) as *mut charusedarray;
                                                                                                                                                                    pdffontsize = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<scaled>() as size_t),
                                                                                                                                                                    ) as *mut scaled;
                                                                                                                                                                    pdffontnum = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    pdffontmap = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(
                                                                                                                                                                                ::core::mem::size_of::<fmentryptr>() as size_t,
                                                                                                                                                                            ),
                                                                                                                                                                    ) as *mut fmentryptr;
                                                                                                                                                                    pdffonttype = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<eightbits>() as size_t),
                                                                                                                                                                    ) as *mut eightbits;
                                                                                                                                                                    pdffontattr = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<strnumber>() as size_t),
                                                                                                                                                                    ) as *mut strnumber;
                                                                                                                                                                    pdffontblink = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(
                                                                                                                                                                                ::core::mem::size_of::<internalfontnumber>() as size_t,
                                                                                                                                                                            ),
                                                                                                                                                                    ) as *mut internalfontnumber;
                                                                                                                                                                    pdffontelink = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(
                                                                                                                                                                                ::core::mem::size_of::<internalfontnumber>() as size_t,
                                                                                                                                                                            ),
                                                                                                                                                                    ) as *mut internalfontnumber;
                                                                                                                                                                    pdffonthasspacechar = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<boolean>() as size_t),
                                                                                                                                                                    ) as *mut boolean;
                                                                                                                                                                    pdffontstretch = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    pdffontshrink = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    pdffontstep = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    pdffontexpandratio = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    pdffontautoexpand = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<boolean>() as size_t),
                                                                                                                                                                    ) as *mut boolean;
                                                                                                                                                                    pdffontlpbase = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    pdffontrpbase = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    pdffontefbase = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    pdffontknbsbase = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    pdffontstbsbase = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    pdffontshbsbase = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    pdffontknbcbase = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    pdffontknacbase = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    vfpacketbase = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    vfdefaultfont = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(
                                                                                                                                                                                ::core::mem::size_of::<internalfontnumber>() as size_t,
                                                                                                                                                                            ),
                                                                                                                                                                    ) as *mut internalfontnumber;
                                                                                                                                                                    vflocalfontnum = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(
                                                                                                                                                                                ::core::mem::size_of::<internalfontnumber>() as size_t,
                                                                                                                                                                            ),
                                                                                                                                                                    ) as *mut internalfontnumber;
                                                                                                                                                                    vfefnts = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                    ) as *mut integer;
                                                                                                                                                                    vfifnts = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(
                                                                                                                                                                                ::core::mem::size_of::<internalfontnumber>() as size_t,
                                                                                                                                                                            ),
                                                                                                                                                                    ) as *mut internalfontnumber;
                                                                                                                                                                    pdffontnobuiltintounicode = xmalloc(
                                                                                                                                                                        ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as size_t)
                                                                                                                                                                            .wrapping_mul(::core::mem::size_of::<boolean>() as size_t),
                                                                                                                                                                    ) as *mut boolean;
                                                                                                                                                                    let mut for_end_8: integer = 0;
                                                                                                                                                                    fontk = 0 as ::core::ffi::c_int as integer;
                                                                                                                                                                    for_end_8 = fontmax;
                                                                                                                                                                    if fontk <= for_end_8 {
                                                                                                                                                                        loop {
                                                                                                                                                                            let mut for_end_9: integer = 0;
                                                                                                                                                                            k_0 = 0 as ::core::ffi::c_int as integer;
                                                                                                                                                                            for_end_9 = 31 as ::core::ffi::c_int as integer;
                                                                                                                                                                            if k_0 <= for_end_9 {
                                                                                                                                                                                loop {
                                                                                                                                                                                    (*pdfcharused.offset(fontk as isize))[k_0 as usize] = 0
                                                                                                                                                                                        as eightbits;
                                                                                                                                                                                    let fresh40 = k_0;
                                                                                                                                                                                    k_0 = k_0 + 1;
                                                                                                                                                                                    if !(fresh40 < for_end_9) {
                                                                                                                                                                                        break;
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                            *pdffontsize.offset(fontk as isize) = 0
                                                                                                                                                                                as ::core::ffi::c_int as scaled;
                                                                                                                                                                            *pdffontnum.offset(fontk as isize) = 0 as ::core::ffi::c_int
                                                                                                                                                                                as integer;
                                                                                                                                                                            let ref mut fresh41 = *pdffontmap.offset(fontk as isize);
                                                                                                                                                                            *fresh41 = ::core::ptr::null_mut::<integer>();
                                                                                                                                                                            *pdffonttype.offset(fontk as isize) = 0 as eightbits;
                                                                                                                                                                            *pdffontattr.offset(fontk as isize) = 345
                                                                                                                                                                                as ::core::ffi::c_int as strnumber;
                                                                                                                                                                            *pdffontblink.offset(fontk as isize) = 0
                                                                                                                                                                                as ::core::ffi::c_int as internalfontnumber;
                                                                                                                                                                            *pdffontelink.offset(fontk as isize) = 0
                                                                                                                                                                                as ::core::ffi::c_int as internalfontnumber;
                                                                                                                                                                            *pdffonthasspacechar.offset(fontk as isize) = false_0
                                                                                                                                                                                as boolean;
                                                                                                                                                                            *pdffontstretch.offset(fontk as isize) = 0
                                                                                                                                                                                as ::core::ffi::c_int as integer;
                                                                                                                                                                            *pdffontshrink.offset(fontk as isize) = 0
                                                                                                                                                                                as ::core::ffi::c_int as integer;
                                                                                                                                                                            *pdffontstep.offset(fontk as isize) = 0
                                                                                                                                                                                as ::core::ffi::c_int as integer;
                                                                                                                                                                            *pdffontexpandratio.offset(fontk as isize) = 0
                                                                                                                                                                                as ::core::ffi::c_int as integer;
                                                                                                                                                                            *pdffontautoexpand.offset(fontk as isize) = false_0
                                                                                                                                                                                as boolean;
                                                                                                                                                                            *pdffontlpbase.offset(fontk as isize) = 0
                                                                                                                                                                                as ::core::ffi::c_int as integer;
                                                                                                                                                                            *pdffontrpbase.offset(fontk as isize) = 0
                                                                                                                                                                                as ::core::ffi::c_int as integer;
                                                                                                                                                                            *pdffontefbase.offset(fontk as isize) = 0
                                                                                                                                                                                as ::core::ffi::c_int as integer;
                                                                                                                                                                            *pdffontknbsbase.offset(fontk as isize) = 0
                                                                                                                                                                                as ::core::ffi::c_int as integer;
                                                                                                                                                                            *pdffontstbsbase.offset(fontk as isize) = 0
                                                                                                                                                                                as ::core::ffi::c_int as integer;
                                                                                                                                                                            *pdffontshbsbase.offset(fontk as isize) = 0
                                                                                                                                                                                as ::core::ffi::c_int as integer;
                                                                                                                                                                            *pdffontknbcbase.offset(fontk as isize) = 0
                                                                                                                                                                                as ::core::ffi::c_int as integer;
                                                                                                                                                                            *pdffontknacbase.offset(fontk as isize) = 0
                                                                                                                                                                                as ::core::ffi::c_int as integer;
                                                                                                                                                                            *pdffontnobuiltintounicode.offset(fontk as isize) = false_0
                                                                                                                                                                                as boolean;
                                                                                                                                                                            let fresh42 = fontk;
                                                                                                                                                                            fontk = fontk + 1;
                                                                                                                                                                            if !(fresh42 < for_end_8) {
                                                                                                                                                                                break;
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                    makepdftexbanner();
                                                                                                                                                                    do_undump(
                                                                                                                                                                        fontcheck.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut fourquarters as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<fourquarters>()
                                                                                                                                                                            as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    do_undump(
                                                                                                                                                                        fontsize.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut scaled as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<scaled>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    do_undump(
                                                                                                                                                                        fontdsize.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut scaled as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<scaled>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    let mut i_0: ::core::ffi::c_uint = 0;
                                                                                                                                                                    do_undump(
                                                                                                                                                                        fontparams.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut fontindex as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<fontindex>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    i_0 = 0 as ::core::ffi::c_uint;
                                                                                                                                                                    while i_0
                                                                                                                                                                        < (fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as ::core::ffi::c_uint
                                                                                                                                                                    {
                                                                                                                                                                        if (*(fontparams.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut fontindex)
                                                                                                                                                                            .offset(i_0 as isize) as ::core::ffi::c_long)
                                                                                                                                                                            < -(268435455 as ::core::ffi::c_long)
                                                                                                                                                                            || *(fontparams.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                as *mut fontindex)
                                                                                                                                                                                .offset(i_0 as isize) as ::core::ffi::c_long
                                                                                                                                                                                > 268435455 as ::core::ffi::c_long
                                                                                                                                                                        {
                                                                                                                                                                            fprintf(
                                                                                                                                                                                __stderrp,
                                                                                                                                                                                b"%s: fatal: (kpathsea) \0" as *const u8
                                                                                                                                                                                    as *const ::core::ffi::c_char,
                                                                                                                                                                                (*kpse_def).invocation_name,
                                                                                                                                                                            );
                                                                                                                                                                            fprintf(
                                                                                                                                                                                __stderrp,
                                                                                                                                                                                b"Item %u (=%ld) of .fmt array at %lx <%ld or >%ld\0"
                                                                                                                                                                                    as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                i_0,
                                                                                                                                                                                *(fontparams.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                    as *mut fontindex)
                                                                                                                                                                                    .offset(i_0 as isize) as uintptr_t,
                                                                                                                                                                                fontparams.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                    as *mut fontindex as uintptr_t,
                                                                                                                                                                                -(268435455 as ::core::ffi::c_long) as uintptr_t,
                                                                                                                                                                                268435455 as ::core::ffi::c_long as uintptr_t,
                                                                                                                                                                            );
                                                                                                                                                                            fputs(
                                                                                                                                                                                b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                __stderrp,
                                                                                                                                                                            );
                                                                                                                                                                            exit(1 as ::core::ffi::c_int);
                                                                                                                                                                        }
                                                                                                                                                                        i_0 = i_0.wrapping_add(1);
                                                                                                                                                                    }
                                                                                                                                                                    do_undump(
                                                                                                                                                                        hyphenchar.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut integer as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    do_undump(
                                                                                                                                                                        skewchar.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut integer as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    let mut i_1: ::core::ffi::c_uint = 0;
                                                                                                                                                                    do_undump(
                                                                                                                                                                        fontname.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut strnumber as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<strnumber>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    i_1 = 0 as ::core::ffi::c_uint;
                                                                                                                                                                    while i_1
                                                                                                                                                                        < (fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as ::core::ffi::c_uint
                                                                                                                                                                    {
                                                                                                                                                                        if *(fontname.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut strnumber)
                                                                                                                                                                            .offset(i_1 as isize) > strptr
                                                                                                                                                                        {
                                                                                                                                                                            fprintf(
                                                                                                                                                                                __stderrp,
                                                                                                                                                                                b"%s: fatal: (kpathsea) \0" as *const u8
                                                                                                                                                                                    as *const ::core::ffi::c_char,
                                                                                                                                                                                (*kpse_def).invocation_name,
                                                                                                                                                                            );
                                                                                                                                                                            fprintf(
                                                                                                                                                                                __stderrp,
                                                                                                                                                                                b"Item %u (=%ld) of .fmt array at %lx >%ld\0" as *const u8
                                                                                                                                                                                    as *const ::core::ffi::c_char,
                                                                                                                                                                                i_1,
                                                                                                                                                                                *(fontname.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                    as *mut strnumber)
                                                                                                                                                                                    .offset(i_1 as isize) as uintptr_t,
                                                                                                                                                                                fontname.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                    as *mut strnumber as uintptr_t,
                                                                                                                                                                                strptr as uintptr_t,
                                                                                                                                                                            );
                                                                                                                                                                            fputs(
                                                                                                                                                                                b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                __stderrp,
                                                                                                                                                                            );
                                                                                                                                                                            exit(1 as ::core::ffi::c_int);
                                                                                                                                                                        }
                                                                                                                                                                        i_1 = i_1.wrapping_add(1);
                                                                                                                                                                    }
                                                                                                                                                                    let mut i_2: ::core::ffi::c_uint = 0;
                                                                                                                                                                    do_undump(
                                                                                                                                                                        fontarea.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut strnumber as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<strnumber>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    i_2 = 0 as ::core::ffi::c_uint;
                                                                                                                                                                    while i_2
                                                                                                                                                                        < (fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as ::core::ffi::c_uint
                                                                                                                                                                    {
                                                                                                                                                                        if *(fontarea.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut strnumber)
                                                                                                                                                                            .offset(i_2 as isize) > strptr
                                                                                                                                                                        {
                                                                                                                                                                            fprintf(
                                                                                                                                                                                __stderrp,
                                                                                                                                                                                b"%s: fatal: (kpathsea) \0" as *const u8
                                                                                                                                                                                    as *const ::core::ffi::c_char,
                                                                                                                                                                                (*kpse_def).invocation_name,
                                                                                                                                                                            );
                                                                                                                                                                            fprintf(
                                                                                                                                                                                __stderrp,
                                                                                                                                                                                b"Item %u (=%ld) of .fmt array at %lx >%ld\0" as *const u8
                                                                                                                                                                                    as *const ::core::ffi::c_char,
                                                                                                                                                                                i_2,
                                                                                                                                                                                *(fontarea.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                    as *mut strnumber)
                                                                                                                                                                                    .offset(i_2 as isize) as uintptr_t,
                                                                                                                                                                                fontarea.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                    as *mut strnumber as uintptr_t,
                                                                                                                                                                                strptr as uintptr_t,
                                                                                                                                                                            );
                                                                                                                                                                            fputs(
                                                                                                                                                                                b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                __stderrp,
                                                                                                                                                                            );
                                                                                                                                                                            exit(1 as ::core::ffi::c_int);
                                                                                                                                                                        }
                                                                                                                                                                        i_2 = i_2.wrapping_add(1);
                                                                                                                                                                    }
                                                                                                                                                                    do_undump(
                                                                                                                                                                        fontbc.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut eightbits as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<eightbits>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    do_undump(
                                                                                                                                                                        fontec.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut eightbits as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<eightbits>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    do_undump(
                                                                                                                                                                        charbase.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut integer as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    do_undump(
                                                                                                                                                                        widthbase.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut integer as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    do_undump(
                                                                                                                                                                        heightbase.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut integer as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    do_undump(
                                                                                                                                                                        depthbase.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut integer as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    do_undump(
                                                                                                                                                                        italicbase.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut integer as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    do_undump(
                                                                                                                                                                        ligkernbase.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut integer as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    do_undump(
                                                                                                                                                                        kernbase.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut integer as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    do_undump(
                                                                                                                                                                        extenbase.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut integer as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    do_undump(
                                                                                                                                                                        parambase.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut integer as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    let mut i_3: ::core::ffi::c_uint = 0;
                                                                                                                                                                    do_undump(
                                                                                                                                                                        fontglue.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut halfword as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<halfword>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    i_3 = 0 as ::core::ffi::c_uint;
                                                                                                                                                                    while i_3
                                                                                                                                                                        < (fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as ::core::ffi::c_uint
                                                                                                                                                                    {
                                                                                                                                                                        if (*(fontglue.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut halfword)
                                                                                                                                                                            .offset(i_3 as isize) as ::core::ffi::c_long)
                                                                                                                                                                            < -(268435455 as ::core::ffi::c_long)
                                                                                                                                                                            || *(fontglue.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                as *mut halfword)
                                                                                                                                                                                .offset(i_3 as isize) > lomemmax
                                                                                                                                                                        {
                                                                                                                                                                            fprintf(
                                                                                                                                                                                __stderrp,
                                                                                                                                                                                b"%s: fatal: (kpathsea) \0" as *const u8
                                                                                                                                                                                    as *const ::core::ffi::c_char,
                                                                                                                                                                                (*kpse_def).invocation_name,
                                                                                                                                                                            );
                                                                                                                                                                            fprintf(
                                                                                                                                                                                __stderrp,
                                                                                                                                                                                b"Item %u (=%ld) of .fmt array at %lx <%ld or >%ld\0"
                                                                                                                                                                                    as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                i_3,
                                                                                                                                                                                *(fontglue.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                    as *mut halfword)
                                                                                                                                                                                    .offset(i_3 as isize) as uintptr_t,
                                                                                                                                                                                fontglue.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                    as *mut halfword as uintptr_t,
                                                                                                                                                                                -(268435455 as ::core::ffi::c_long) as uintptr_t,
                                                                                                                                                                                lomemmax as uintptr_t,
                                                                                                                                                                            );
                                                                                                                                                                            fputs(
                                                                                                                                                                                b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                __stderrp,
                                                                                                                                                                            );
                                                                                                                                                                            exit(1 as ::core::ffi::c_int);
                                                                                                                                                                        }
                                                                                                                                                                        i_3 = i_3.wrapping_add(1);
                                                                                                                                                                    }
                                                                                                                                                                    let mut i_4: ::core::ffi::c_uint = 0;
                                                                                                                                                                    do_undump(
                                                                                                                                                                        bcharlabel.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut fontindex as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<fontindex>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    i_4 = 0 as ::core::ffi::c_uint;
                                                                                                                                                                    while i_4
                                                                                                                                                                        < (fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as ::core::ffi::c_uint
                                                                                                                                                                    {
                                                                                                                                                                        if *(bcharlabel.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut fontindex)
                                                                                                                                                                            .offset(i_4 as isize) < 0 as ::core::ffi::c_int
                                                                                                                                                                            || *(bcharlabel.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                as *mut fontindex)
                                                                                                                                                                                .offset(i_4 as isize)
                                                                                                                                                                                > fmemptr as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                                                                                                                                                        {
                                                                                                                                                                            fprintf(
                                                                                                                                                                                __stderrp,
                                                                                                                                                                                b"%s: fatal: (kpathsea) \0" as *const u8
                                                                                                                                                                                    as *const ::core::ffi::c_char,
                                                                                                                                                                                (*kpse_def).invocation_name,
                                                                                                                                                                            );
                                                                                                                                                                            fprintf(
                                                                                                                                                                                __stderrp,
                                                                                                                                                                                b"Item %u (=%ld) of .fmt array at %lx <%ld or >%ld\0"
                                                                                                                                                                                    as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                i_4,
                                                                                                                                                                                *(bcharlabel.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                    as *mut fontindex)
                                                                                                                                                                                    .offset(i_4 as isize) as uintptr_t,
                                                                                                                                                                                bcharlabel.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                    as *mut fontindex as uintptr_t,
                                                                                                                                                                                0 as ::core::ffi::c_int as uintptr_t,
                                                                                                                                                                                (fmemptr as uintptr_t)
                                                                                                                                                                                    .wrapping_sub(1 as ::core::ffi::c_int as uintptr_t),
                                                                                                                                                                            );
                                                                                                                                                                            fputs(
                                                                                                                                                                                b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                __stderrp,
                                                                                                                                                                            );
                                                                                                                                                                            exit(1 as ::core::ffi::c_int);
                                                                                                                                                                        }
                                                                                                                                                                        i_4 = i_4.wrapping_add(1);
                                                                                                                                                                    }
                                                                                                                                                                    let mut i_5: ::core::ffi::c_uint = 0;
                                                                                                                                                                    do_undump(
                                                                                                                                                                        fontbchar.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut ninebits as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<ninebits>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    i_5 = 0 as ::core::ffi::c_uint;
                                                                                                                                                                    while i_5
                                                                                                                                                                        < (fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as ::core::ffi::c_uint
                                                                                                                                                                    {
                                                                                                                                                                        if (*(fontbchar.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut ninebits)
                                                                                                                                                                            .offset(i_5 as isize) as ::core::ffi::c_int)
                                                                                                                                                                            < 0 as ::core::ffi::c_int
                                                                                                                                                                            || *(fontbchar.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                as *mut ninebits)
                                                                                                                                                                                .offset(i_5 as isize) as ::core::ffi::c_int
                                                                                                                                                                                > 256 as ::core::ffi::c_int
                                                                                                                                                                        {
                                                                                                                                                                            fprintf(
                                                                                                                                                                                __stderrp,
                                                                                                                                                                                b"%s: fatal: (kpathsea) \0" as *const u8
                                                                                                                                                                                    as *const ::core::ffi::c_char,
                                                                                                                                                                                (*kpse_def).invocation_name,
                                                                                                                                                                            );
                                                                                                                                                                            fprintf(
                                                                                                                                                                                __stderrp,
                                                                                                                                                                                b"Item %u (=%ld) of .fmt array at %lx <%ld or >%ld\0"
                                                                                                                                                                                    as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                i_5,
                                                                                                                                                                                *(fontbchar.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                    as *mut ninebits)
                                                                                                                                                                                    .offset(i_5 as isize) as uintptr_t,
                                                                                                                                                                                fontbchar.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                    as *mut ninebits as uintptr_t,
                                                                                                                                                                                0 as ::core::ffi::c_int as uintptr_t,
                                                                                                                                                                                256 as ::core::ffi::c_int as uintptr_t,
                                                                                                                                                                            );
                                                                                                                                                                            fputs(
                                                                                                                                                                                b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                __stderrp,
                                                                                                                                                                            );
                                                                                                                                                                            exit(1 as ::core::ffi::c_int);
                                                                                                                                                                        }
                                                                                                                                                                        i_5 = i_5.wrapping_add(1);
                                                                                                                                                                    }
                                                                                                                                                                    let mut i_6: ::core::ffi::c_uint = 0;
                                                                                                                                                                    do_undump(
                                                                                                                                                                        fontfalsebchar.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                            as *mut ninebits as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<ninebits>() as ::core::ffi::c_int,
                                                                                                                                                                        fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    i_6 = 0 as ::core::ffi::c_uint;
                                                                                                                                                                    while i_6
                                                                                                                                                                        < (fontptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                                                                                                                                            as ::core::ffi::c_uint
                                                                                                                                                                    {
                                                                                                                                                                        if (*(fontfalsebchar
                                                                                                                                                                            .offset(0 as ::core::ffi::c_int as isize) as *mut ninebits)
                                                                                                                                                                            .offset(i_6 as isize) as ::core::ffi::c_int)
                                                                                                                                                                            < 0 as ::core::ffi::c_int
                                                                                                                                                                            || *(fontfalsebchar.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                as *mut ninebits)
                                                                                                                                                                                .offset(i_6 as isize) as ::core::ffi::c_int
                                                                                                                                                                                > 256 as ::core::ffi::c_int
                                                                                                                                                                        {
                                                                                                                                                                            fprintf(
                                                                                                                                                                                __stderrp,
                                                                                                                                                                                b"%s: fatal: (kpathsea) \0" as *const u8
                                                                                                                                                                                    as *const ::core::ffi::c_char,
                                                                                                                                                                                (*kpse_def).invocation_name,
                                                                                                                                                                            );
                                                                                                                                                                            fprintf(
                                                                                                                                                                                __stderrp,
                                                                                                                                                                                b"Item %u (=%ld) of .fmt array at %lx <%ld or >%ld\0"
                                                                                                                                                                                    as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                i_6,
                                                                                                                                                                                *(fontfalsebchar.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                    as *mut ninebits)
                                                                                                                                                                                    .offset(i_6 as isize) as uintptr_t,
                                                                                                                                                                                fontfalsebchar.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                    as *mut ninebits as uintptr_t,
                                                                                                                                                                                0 as ::core::ffi::c_int as uintptr_t,
                                                                                                                                                                                256 as ::core::ffi::c_int as uintptr_t,
                                                                                                                                                                            );
                                                                                                                                                                            fputs(
                                                                                                                                                                                b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                __stderrp,
                                                                                                                                                                            );
                                                                                                                                                                            exit(1 as ::core::ffi::c_int);
                                                                                                                                                                        }
                                                                                                                                                                        i_6 = i_6.wrapping_add(1);
                                                                                                                                                                    }
                                                                                                                                                                    do_undump(
                                                                                                                                                                        &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                        1 as ::core::ffi::c_int,
                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                    );
                                                                                                                                                                    if !(x < 0 as ::core::ffi::c_int) {
                                                                                                                                                                        if x > hyphsize {
                                                                                                                                                                            fprintf(
                                                                                                                                                                                __stdoutp,
                                                                                                                                                                                b"%s%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                b"---! Must increase the \0" as *const u8
                                                                                                                                                                                    as *const ::core::ffi::c_char,
                                                                                                                                                                                b"hyph_size\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                            );
                                                                                                                                                                        } else {
                                                                                                                                                                            if debugformatfile != 0 {
                                                                                                                                                                                fprintf(
                                                                                                                                                                                    __stderrp,
                                                                                                                                                                                    b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                    b"fmtdebug:\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                    b"hyph_size\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                );
                                                                                                                                                                                fprintf(
                                                                                                                                                                                    __stderrp,
                                                                                                                                                                                    b"%s%ld\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                    b" = \0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                    x as ::core::ffi::c_long,
                                                                                                                                                                                );
                                                                                                                                                                            }
                                                                                                                                                                            hyphcount = x;
                                                                                                                                                                            do_undump(
                                                                                                                                                                                &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                                                                                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                1 as ::core::ffi::c_int,
                                                                                                                                                                                fmtfile as gzFile,
                                                                                                                                                                            );
                                                                                                                                                                            if !(x < 607 as ::core::ffi::c_int) {
                                                                                                                                                                                if x > hyphsize {
                                                                                                                                                                                    fprintf(
                                                                                                                                                                                        __stdoutp,
                                                                                                                                                                                        b"%s%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                        b"---! Must increase the \0" as *const u8
                                                                                                                                                                                            as *const ::core::ffi::c_char,
                                                                                                                                                                                        b"hyph_size\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                    );
                                                                                                                                                                                } else {
                                                                                                                                                                                    if debugformatfile != 0 {
                                                                                                                                                                                        fprintf(
                                                                                                                                                                                            __stderrp,
                                                                                                                                                                                            b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                            b"fmtdebug:\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                            b"hyph_size\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                        );
                                                                                                                                                                                        fprintf(
                                                                                                                                                                                            __stderrp,
                                                                                                                                                                                            b"%s%ld\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                            b" = \0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                            x as ::core::ffi::c_long,
                                                                                                                                                                                        );
                                                                                                                                                                                    }
                                                                                                                                                                                    hyphnext = x;
                                                                                                                                                                                    j = 0 as ::core::ffi::c_int as integer;
                                                                                                                                                                                    let mut for_end_10: integer = 0;
                                                                                                                                                                                    k_0 = 1 as ::core::ffi::c_int as integer;
                                                                                                                                                                                    for_end_10 = hyphcount;
                                                                                                                                                                                    if k_0 <= for_end_10 {
                                                                                                                                                                                        loop {
                                                                                                                                                                                            do_undump(
                                                                                                                                                                                                &raw mut j as *mut ::core::ffi::c_char,
                                                                                                                                                                                                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                1 as ::core::ffi::c_int,
                                                                                                                                                                                                fmtfile as gzFile,
                                                                                                                                                                                            );
                                                                                                                                                                                            if j < 0 as ::core::ffi::c_int {
                                                                                                                                                                                                current_block = 15581621980376734741;
                                                                                                                                                                                                break;
                                                                                                                                                                                            }
                                                                                                                                                                                            if j as ::core::ffi::c_long > 65535 as ::core::ffi::c_long {
                                                                                                                                                                                                hyphnext = (j as ::core::ffi::c_long
                                                                                                                                                                                                    / 65536 as ::core::ffi::c_long) as integer;
                                                                                                                                                                                                j = (j as ::core::ffi::c_long
                                                                                                                                                                                                    - hyphnext as ::core::ffi::c_long
                                                                                                                                                                                                        * 65536 as ::core::ffi::c_long) as integer;
                                                                                                                                                                                            } else {
                                                                                                                                                                                                hyphnext = 0 as ::core::ffi::c_int as integer;
                                                                                                                                                                                            }
                                                                                                                                                                                            if j >= hyphsize || hyphnext > hyphsize {
                                                                                                                                                                                                current_block = 15581621980376734741;
                                                                                                                                                                                                break;
                                                                                                                                                                                            }
                                                                                                                                                                                            *hyphlink.offset(j as isize) = hyphnext as hyphpointer;
                                                                                                                                                                                            do_undump(
                                                                                                                                                                                                &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                                                                                                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                1 as ::core::ffi::c_int,
                                                                                                                                                                                                fmtfile as gzFile,
                                                                                                                                                                                            );
                                                                                                                                                                                            if x < 0 as ::core::ffi::c_int || x > strptr {
                                                                                                                                                                                                current_block = 15581621980376734741;
                                                                                                                                                                                                break;
                                                                                                                                                                                            }
                                                                                                                                                                                            *hyphword.offset(j as isize) = x as strnumber;
                                                                                                                                                                                            do_undump(
                                                                                                                                                                                                &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                                                                                                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                1 as ::core::ffi::c_int,
                                                                                                                                                                                                fmtfile as gzFile,
                                                                                                                                                                                            );
                                                                                                                                                                                            if (x as ::core::ffi::c_long)
                                                                                                                                                                                                < -(268435455 as ::core::ffi::c_long)
                                                                                                                                                                                                || x as ::core::ffi::c_long
                                                                                                                                                                                                    > 268435455 as ::core::ffi::c_long
                                                                                                                                                                                            {
                                                                                                                                                                                                current_block = 15581621980376734741;
                                                                                                                                                                                                break;
                                                                                                                                                                                            }
                                                                                                                                                                                            *hyphlist.offset(j as isize) = x as halfword;
                                                                                                                                                                                            let fresh43 = k_0;
                                                                                                                                                                                            k_0 = k_0 + 1;
                                                                                                                                                                                            if !(fresh43 < for_end_10) {
                                                                                                                                                                                                current_block = 16816250088755962910;
                                                                                                                                                                                                break;
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    } else {
                                                                                                                                                                                        current_block = 16816250088755962910;
                                                                                                                                                                                    }
                                                                                                                                                                                    match current_block {
                                                                                                                                                                                        15581621980376734741 => {}
                                                                                                                                                                                        _ => {
                                                                                                                                                                                            j += 1;
                                                                                                                                                                                            if j < 607 as ::core::ffi::c_int {
                                                                                                                                                                                                j = 607 as ::core::ffi::c_int as integer;
                                                                                                                                                                                            }
                                                                                                                                                                                            hyphnext = j;
                                                                                                                                                                                            if hyphnext >= hyphsize {
                                                                                                                                                                                                hyphnext = 607 as ::core::ffi::c_int as integer;
                                                                                                                                                                                            } else if hyphnext >= 607 as ::core::ffi::c_int {
                                                                                                                                                                                                hyphnext += 1;
                                                                                                                                                                                            }
                                                                                                                                                                                            do_undump(
                                                                                                                                                                                                &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                                                                                                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                1 as ::core::ffi::c_int,
                                                                                                                                                                                                fmtfile as gzFile,
                                                                                                                                                                                            );
                                                                                                                                                                                            if !(x < 0 as ::core::ffi::c_int) {
                                                                                                                                                                                                if x > triesize {
                                                                                                                                                                                                    fprintf(
                                                                                                                                                                                                        __stdoutp,
                                                                                                                                                                                                        b"%s%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                                        b"---! Must increase the \0" as *const u8
                                                                                                                                                                                                            as *const ::core::ffi::c_char,
                                                                                                                                                                                                        b"trie size\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                                    );
                                                                                                                                                                                                } else {
                                                                                                                                                                                                    if debugformatfile != 0 {
                                                                                                                                                                                                        fprintf(
                                                                                                                                                                                                            __stderrp,
                                                                                                                                                                                                            b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                                            b"fmtdebug:\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                                            b"trie size\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                                        );
                                                                                                                                                                                                        fprintf(
                                                                                                                                                                                                            __stderrp,
                                                                                                                                                                                                            b"%s%ld\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                                            b" = \0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                                            x as ::core::ffi::c_long,
                                                                                                                                                                                                        );
                                                                                                                                                                                                    }
                                                                                                                                                                                                    j = x;
                                                                                                                                                                                                    triemax = j as triepointer;
                                                                                                                                                                                                    do_undump(
                                                                                                                                                                                                        &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                        1 as ::core::ffi::c_int,
                                                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                                                    );
                                                                                                                                                                                                    if !(x < 0 as ::core::ffi::c_int || x > j) {
                                                                                                                                                                                                        hyphstart = x as triepointer;
                                                                                                                                                                                                        if trietrl.is_null() {
                                                                                                                                                                                                            trietrl = xmalloc(
                                                                                                                                                                                                                ((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                                                                                                                                                                                                    + 1 as ::core::ffi::c_int) as size_t)
                                                                                                                                                                                                                    .wrapping_mul(
                                                                                                                                                                                                                        ::core::mem::size_of::<triepointer>() as size_t,
                                                                                                                                                                                                                    ),
                                                                                                                                                                                                            ) as *mut triepointer;
                                                                                                                                                                                                        }
                                                                                                                                                                                                        do_undump(
                                                                                                                                                                                                            trietrl.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                                                as *mut triepointer as *mut ::core::ffi::c_char,
                                                                                                                                                                                                            ::core::mem::size_of::<triepointer>() as ::core::ffi::c_int,
                                                                                                                                                                                                            j as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                                                                        );
                                                                                                                                                                                                        if trietro.is_null() {
                                                                                                                                                                                                            trietro = xmalloc(
                                                                                                                                                                                                                ((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                                                                                                                                                                                                    + 1 as ::core::ffi::c_int) as size_t)
                                                                                                                                                                                                                    .wrapping_mul(
                                                                                                                                                                                                                        ::core::mem::size_of::<triepointer>() as size_t,
                                                                                                                                                                                                                    ),
                                                                                                                                                                                                            ) as *mut triepointer;
                                                                                                                                                                                                        }
                                                                                                                                                                                                        do_undump(
                                                                                                                                                                                                            trietro.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                                                as *mut triepointer as *mut ::core::ffi::c_char,
                                                                                                                                                                                                            ::core::mem::size_of::<triepointer>() as ::core::ffi::c_int,
                                                                                                                                                                                                            j as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                                                                        );
                                                                                                                                                                                                        if trietrc.is_null() {
                                                                                                                                                                                                            trietrc = xmalloc(
                                                                                                                                                                                                                ((j as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                                                                                                                                                                                                    + 1 as ::core::ffi::c_int) as size_t)
                                                                                                                                                                                                                    .wrapping_mul(
                                                                                                                                                                                                                        ::core::mem::size_of::<quarterword>() as size_t,
                                                                                                                                                                                                                    ),
                                                                                                                                                                                                            ) as *mut quarterword;
                                                                                                                                                                                                        }
                                                                                                                                                                                                        do_undump(
                                                                                                                                                                                                            trietrc.offset(0 as ::core::ffi::c_int as isize)
                                                                                                                                                                                                                as *mut quarterword as *mut ::core::ffi::c_char,
                                                                                                                                                                                                            ::core::mem::size_of::<quarterword>() as ::core::ffi::c_int,
                                                                                                                                                                                                            j as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                                                                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                                                                        );
                                                                                                                                                                                                        do_undump(
                                                                                                                                                                                                            &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                                                                                                            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                            1 as ::core::ffi::c_int,
                                                                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                                                                        );
                                                                                                                                                                                                        if !(x < 0 as ::core::ffi::c_int) {
                                                                                                                                                                                                            if x as ::core::ffi::c_long > trieopsize {
                                                                                                                                                                                                                fprintf(
                                                                                                                                                                                                                    __stdoutp,
                                                                                                                                                                                                                    b"%s%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                                                    b"---! Must increase the \0" as *const u8
                                                                                                                                                                                                                        as *const ::core::ffi::c_char,
                                                                                                                                                                                                                    b"trie op size\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                                                );
                                                                                                                                                                                                            } else {
                                                                                                                                                                                                                if debugformatfile != 0 {
                                                                                                                                                                                                                    fprintf(
                                                                                                                                                                                                                        __stderrp,
                                                                                                                                                                                                                        b"%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                                                        b"fmtdebug:\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                                                        b"trie op size\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                                                    );
                                                                                                                                                                                                                    fprintf(
                                                                                                                                                                                                                        __stderrp,
                                                                                                                                                                                                                        b"%s%ld\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                                                        b" = \0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                                                        x as ::core::ffi::c_long,
                                                                                                                                                                                                                    );
                                                                                                                                                                                                                }
                                                                                                                                                                                                                j = x;
                                                                                                                                                                                                                trieopptr = j;
                                                                                                                                                                                                                do_undump(
                                                                                                                                                                                                                    (&raw mut hyfdistance as *mut smallnumber)
                                                                                                                                                                                                                        .offset(1 as ::core::ffi::c_int as isize)
                                                                                                                                                                                                                        as *mut smallnumber as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                    ::core::mem::size_of::<smallnumber>() as ::core::ffi::c_int,
                                                                                                                                                                                                                    j,
                                                                                                                                                                                                                    fmtfile as gzFile,
                                                                                                                                                                                                                );
                                                                                                                                                                                                                do_undump(
                                                                                                                                                                                                                    (&raw mut hyfnum as *mut smallnumber)
                                                                                                                                                                                                                        .offset(1 as ::core::ffi::c_int as isize)
                                                                                                                                                                                                                        as *mut smallnumber as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                    ::core::mem::size_of::<smallnumber>() as ::core::ffi::c_int,
                                                                                                                                                                                                                    j,
                                                                                                                                                                                                                    fmtfile as gzFile,
                                                                                                                                                                                                                );
                                                                                                                                                                                                                let mut i_7: ::core::ffi::c_uint = 0;
                                                                                                                                                                                                                do_undump(
                                                                                                                                                                                                                    (&raw mut hyfnext as *mut trieopcode)
                                                                                                                                                                                                                        .offset(1 as ::core::ffi::c_int as isize) as *mut trieopcode
                                                                                                                                                                                                                        as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                    ::core::mem::size_of::<trieopcode>() as ::core::ffi::c_int,
                                                                                                                                                                                                                    j,
                                                                                                                                                                                                                    fmtfile as gzFile,
                                                                                                                                                                                                                );
                                                                                                                                                                                                                i_7 = 0 as ::core::ffi::c_uint;
                                                                                                                                                                                                                while i_7 < j as ::core::ffi::c_uint {
                                                                                                                                                                                                                    if *((&raw mut hyfnext as *mut trieopcode)
                                                                                                                                                                                                                        .offset(1 as ::core::ffi::c_int as isize)
                                                                                                                                                                                                                        as *mut trieopcode)
                                                                                                                                                                                                                        .offset(i_7 as isize) as ::core::ffi::c_long
                                                                                                                                                                                                                        > 65535 as ::core::ffi::c_long
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        fprintf(
                                                                                                                                                                                                                            __stderrp,
                                                                                                                                                                                                                            b"%s: fatal: (kpathsea) \0" as *const u8
                                                                                                                                                                                                                                as *const ::core::ffi::c_char,
                                                                                                                                                                                                                            (*kpse_def).invocation_name,
                                                                                                                                                                                                                        );
                                                                                                                                                                                                                        fprintf(
                                                                                                                                                                                                                            __stderrp,
                                                                                                                                                                                                                            b"Item %u (=%ld) of .fmt array at %lx >%ld\0" as *const u8
                                                                                                                                                                                                                                as *const ::core::ffi::c_char,
                                                                                                                                                                                                                            i_7,
                                                                                                                                                                                                                            *((&raw mut hyfnext as *mut trieopcode)
                                                                                                                                                                                                                                .offset(1 as ::core::ffi::c_int as isize)
                                                                                                                                                                                                                                as *mut trieopcode)
                                                                                                                                                                                                                                .offset(i_7 as isize) as uintptr_t,
                                                                                                                                                                                                                            (&raw mut hyfnext as *mut trieopcode)
                                                                                                                                                                                                                                .offset(1 as ::core::ffi::c_int as isize) as *mut trieopcode
                                                                                                                                                                                                                                as uintptr_t,
                                                                                                                                                                                                                            65535 as ::core::ffi::c_long as uintptr_t,
                                                                                                                                                                                                                        );
                                                                                                                                                                                                                        fputs(
                                                                                                                                                                                                                            b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                                                                                                            __stderrp,
                                                                                                                                                                                                                        );
                                                                                                                                                                                                                        exit(1 as ::core::ffi::c_int);
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                    i_7 = i_7.wrapping_add(1);
                                                                                                                                                                                                                }
                                                                                                                                                                                                                let mut for_end_11: integer = 0;
                                                                                                                                                                                                                k_0 = 0 as ::core::ffi::c_int as integer;
                                                                                                                                                                                                                for_end_11 = 255 as ::core::ffi::c_int as integer;
                                                                                                                                                                                                                if k_0 <= for_end_11 {
                                                                                                                                                                                                                    loop {
                                                                                                                                                                                                                        trieused[k_0 as usize] = 0 as trieopcode;
                                                                                                                                                                                                                        let fresh44 = k_0;
                                                                                                                                                                                                                        k_0 = k_0 + 1;
                                                                                                                                                                                                                        if !(fresh44 < for_end_11) {
                                                                                                                                                                                                                            break;
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                }
                                                                                                                                                                                                                k_0 = 256 as ::core::ffi::c_int as integer;
                                                                                                                                                                                                                loop {
                                                                                                                                                                                                                    if !(j > 0 as ::core::ffi::c_int) {
                                                                                                                                                                                                                        current_block = 15420184218603695325;
                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                    do_undump(
                                                                                                                                                                                                                        &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                        1 as ::core::ffi::c_int,
                                                                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                                                                    );
                                                                                                                                                                                                                    if x < 0 as ::core::ffi::c_int
                                                                                                                                                                                                                        || x > k_0 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                                                                                                                                                                                                    {
                                                                                                                                                                                                                        current_block = 15581621980376734741;
                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                    k_0 = x;
                                                                                                                                                                                                                    do_undump(
                                                                                                                                                                                                                        &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                        ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                        1 as ::core::ffi::c_int,
                                                                                                                                                                                                                        fmtfile as gzFile,
                                                                                                                                                                                                                    );
                                                                                                                                                                                                                    if x < 1 as ::core::ffi::c_int || x > j {
                                                                                                                                                                                                                        current_block = 15581621980376734741;
                                                                                                                                                                                                                        break;
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                    x = x;
                                                                                                                                                                                                                    trieused[k_0 as usize] = x as trieopcode;
                                                                                                                                                                                                                    j = j - x;
                                                                                                                                                                                                                    opstart[k_0 as usize] = j;
                                                                                                                                                                                                                }
                                                                                                                                                                                                                match current_block {
                                                                                                                                                                                                                    15581621980376734741 => {}
                                                                                                                                                                                                                    _ => {
                                                                                                                                                                                                                        trienotready = false_0 as boolean;
                                                                                                                                                                                                                        undumpimagemeta(
                                                                                                                                                                                                                            (*eqtb.offset(29351 as ::core::ffi::c_int as isize)).u.CINT,
                                                                                                                                                                                                                            (*eqtb.offset(29352 as ::core::ffi::c_int as isize)).u.CINT,
                                                                                                                                                                                                                            (*eqtb.offset(29355 as ::core::ffi::c_int as isize)).u.CINT,
                                                                                                                                                                                                                        );
                                                                                                                                                                                                                        do_undump(
                                                                                                                                                                                                                            &raw mut pdfmemsize as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                            1 as ::core::ffi::c_int,
                                                                                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                                                                                        );
                                                                                                                                                                                                                        pdfmem = xrealloc(
                                                                                                                                                                                                                            pdfmem as address,
                                                                                                                                                                                                                            ((pdfmemsize as ::core::ffi::c_int
                                                                                                                                                                                                                                + 1 as ::core::ffi::c_int) as size_t)
                                                                                                                                                                                                                                .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                                                                                                                                                                                                                        ) as *mut integer;
                                                                                                                                                                                                                        do_undump(
                                                                                                                                                                                                                            &raw mut pdfmemptr as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                            1 as ::core::ffi::c_int,
                                                                                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                                                                                        );
                                                                                                                                                                                                                        let mut for_end_12: integer = 0;
                                                                                                                                                                                                                        k_0 = 1 as ::core::ffi::c_int as integer;
                                                                                                                                                                                                                        for_end_12 = (pdfmemptr as ::core::ffi::c_int
                                                                                                                                                                                                                            - 1 as ::core::ffi::c_int) as integer;
                                                                                                                                                                                                                        if k_0 <= for_end_12 {
                                                                                                                                                                                                                            loop {
                                                                                                                                                                                                                                do_undump(
                                                                                                                                                                                                                                    pdfmem.offset(k_0 as isize) as *mut integer
                                                                                                                                                                                                                                        as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                                    1 as ::core::ffi::c_int,
                                                                                                                                                                                                                                    fmtfile as gzFile,
                                                                                                                                                                                                                                );
                                                                                                                                                                                                                                let fresh45 = k_0;
                                                                                                                                                                                                                                k_0 = k_0 + 1;
                                                                                                                                                                                                                                if !(fresh45 < for_end_12) {
                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                            }
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                        do_undump(
                                                                                                                                                                                                                            &raw mut objtabsize as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                            1 as ::core::ffi::c_int,
                                                                                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                                                                                        );
                                                                                                                                                                                                                        do_undump(
                                                                                                                                                                                                                            &raw mut objptr as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                            1 as ::core::ffi::c_int,
                                                                                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                                                                                        );
                                                                                                                                                                                                                        do_undump(
                                                                                                                                                                                                                            &raw mut sysobjptr as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                            1 as ::core::ffi::c_int,
                                                                                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                                                                                        );
                                                                                                                                                                                                                        let mut for_end_13: integer = 0;
                                                                                                                                                                                                                        k_0 = 1 as ::core::ffi::c_int as integer;
                                                                                                                                                                                                                        for_end_13 = sysobjptr;
                                                                                                                                                                                                                        if k_0 <= for_end_13 {
                                                                                                                                                                                                                            loop {
                                                                                                                                                                                                                                do_undump(
                                                                                                                                                                                                                                    &raw mut (*objtab.offset(k_0 as isize)).int0
                                                                                                                                                                                                                                        as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                                    1 as ::core::ffi::c_int,
                                                                                                                                                                                                                                    fmtfile as gzFile,
                                                                                                                                                                                                                                );
                                                                                                                                                                                                                                do_undump(
                                                                                                                                                                                                                                    &raw mut (*objtab.offset(k_0 as isize)).int1
                                                                                                                                                                                                                                        as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                                    1 as ::core::ffi::c_int,
                                                                                                                                                                                                                                    fmtfile as gzFile,
                                                                                                                                                                                                                                );
                                                                                                                                                                                                                                (*objtab.offset(k_0 as isize)).int2 = -(1
                                                                                                                                                                                                                                    as ::core::ffi::c_int) as longinteger;
                                                                                                                                                                                                                                do_undump(
                                                                                                                                                                                                                                    &raw mut (*objtab.offset(k_0 as isize)).int3
                                                                                                                                                                                                                                        as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                                    1 as ::core::ffi::c_int,
                                                                                                                                                                                                                                    fmtfile as gzFile,
                                                                                                                                                                                                                                );
                                                                                                                                                                                                                                do_undump(
                                                                                                                                                                                                                                    &raw mut (*objtab.offset(k_0 as isize)).int4
                                                                                                                                                                                                                                        as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                                    1 as ::core::ffi::c_int,
                                                                                                                                                                                                                                    fmtfile as gzFile,
                                                                                                                                                                                                                                );
                                                                                                                                                                                                                                let fresh46 = k_0;
                                                                                                                                                                                                                                k_0 = k_0 + 1;
                                                                                                                                                                                                                                if !(fresh46 < for_end_13) {
                                                                                                                                                                                                                                    break;
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                            }
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                        do_undump(
                                                                                                                                                                                                                            &raw mut pdfobjcount as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                            1 as ::core::ffi::c_int,
                                                                                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                                                                                        );
                                                                                                                                                                                                                        do_undump(
                                                                                                                                                                                                                            &raw mut pdfxformcount as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                            1 as ::core::ffi::c_int,
                                                                                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                                                                                        );
                                                                                                                                                                                                                        do_undump(
                                                                                                                                                                                                                            &raw mut pdfximagecount as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                            1 as ::core::ffi::c_int,
                                                                                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                                                                                        );
                                                                                                                                                                                                                        do_undump(
                                                                                                                                                                                                                            (&raw mut headtab as *mut integer)
                                                                                                                                                                                                                                .offset(7 as ::core::ffi::c_int as isize) as *mut integer
                                                                                                                                                                                                                                as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                            1 as ::core::ffi::c_int,
                                                                                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                                                                                        );
                                                                                                                                                                                                                        do_undump(
                                                                                                                                                                                                                            (&raw mut headtab as *mut integer)
                                                                                                                                                                                                                                .offset(8 as ::core::ffi::c_int as isize) as *mut integer
                                                                                                                                                                                                                                as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                            1 as ::core::ffi::c_int,
                                                                                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                                                                                        );
                                                                                                                                                                                                                        do_undump(
                                                                                                                                                                                                                            (&raw mut headtab as *mut integer)
                                                                                                                                                                                                                                .offset(9 as ::core::ffi::c_int as isize) as *mut integer
                                                                                                                                                                                                                                as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                            1 as ::core::ffi::c_int,
                                                                                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                                                                                        );
                                                                                                                                                                                                                        do_undump(
                                                                                                                                                                                                                            &raw mut pdflastobj as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                            1 as ::core::ffi::c_int,
                                                                                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                                                                                        );
                                                                                                                                                                                                                        do_undump(
                                                                                                                                                                                                                            &raw mut pdflastxform as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                            1 as ::core::ffi::c_int,
                                                                                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                                                                                        );
                                                                                                                                                                                                                        do_undump(
                                                                                                                                                                                                                            &raw mut pdflastximage as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                            1 as ::core::ffi::c_int,
                                                                                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                                                                                        );
                                                                                                                                                                                                                        undumptounicode();
                                                                                                                                                                                                                        do_undump(
                                                                                                                                                                                                                            &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                            ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                            1 as ::core::ffi::c_int,
                                                                                                                                                                                                                            fmtfile as gzFile,
                                                                                                                                                                                                                        );
                                                                                                                                                                                                                        if !(x < 0 as ::core::ffi::c_int
                                                                                                                                                                                                                            || x > 3 as ::core::ffi::c_int)
                                                                                                                                                                                                                        {
                                                                                                                                                                                                                            interaction = x as ::core::ffi::c_uchar;
                                                                                                                                                                                                                            if interactionoption as ::core::ffi::c_int
                                                                                                                                                                                                                                != 4 as ::core::ffi::c_int
                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                interaction = interactionoption;
                                                                                                                                                                                                                            }
                                                                                                                                                                                                                            do_undump(
                                                                                                                                                                                                                                &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                                ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                                1 as ::core::ffi::c_int,
                                                                                                                                                                                                                                fmtfile as gzFile,
                                                                                                                                                                                                                            );
                                                                                                                                                                                                                            if !(x < 0 as ::core::ffi::c_int || x > strptr) {
                                                                                                                                                                                                                                formatident = x as strnumber;
                                                                                                                                                                                                                                do_undump(
                                                                                                                                                                                                                                    &raw mut x as *mut ::core::ffi::c_char,
                                                                                                                                                                                                                                    ::core::mem::size_of::<integer>() as ::core::ffi::c_int,
                                                                                                                                                                                                                                    1 as ::core::ffi::c_int,
                                                                                                                                                                                                                                    fmtfile as gzFile,
                                                                                                                                                                                                                                );
                                                                                                                                                                                                                                if !(x as ::core::ffi::c_long
                                                                                                                                                                                                                                    != 69069 as ::core::ffi::c_long)
                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                    curlist.auxfield.u.CINT = (*eqtb
                                                                                                                                                                                                                                        .offset(29935 as ::core::ffi::c_int as isize))
                                                                                                                                                                                                                                        .u
                                                                                                                                                                                                                                        .CINT;
                                                                                                                                                                                                                                    Result = true_0 as boolean;
                                                                                                                                                                                                                                    return Result;
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                            }
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                }
                                                                                                                                                                                                            }
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fprintf(
        __stdoutp,
        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        b"(Fatal format file error; I'm stymied)\0" as *const u8 as *const ::core::ffi::c_char,
    );
    Result = false_0 as boolean;
    return Result;
}
#[no_mangle]
pub unsafe extern "C" fn finalcleanup() {
    let mut mem: *mut memoryword = zmem;
    let mut eqtb: *mut memoryword = zeqtb;
    let mut c: smallnumber = 0;
    c = curchr as smallnumber;
    if c as ::core::ffi::c_int != 1 as ::core::ffi::c_int {
        (*eqtb.offset(29326 as ::core::ffi::c_int as isize)).u.CINT =
            -(1 as ::core::ffi::c_int) as integer;
    }
    if jobname == 0 as ::core::ffi::c_int {
        openlogfile();
    }
    while inputptr > 0 as ::core::ffi::c_int {
        if curinput.statefield as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            endtokenlist();
        } else {
            endfilereading();
        }
    }
    while openparens > 0 as ::core::ffi::c_int {
        zprint(1735 as ::core::ffi::c_int);
        openparens -= 1;
    }
    if curlevel as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
        zprintnl(40 as ::core::ffi::c_int);
        zprintesc(1736 as ::core::ffi::c_int);
        zprint(1737 as ::core::ffi::c_int);
        zprintint((curlevel as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as longinteger);
        zprintchar(41 as ::core::ffi::c_int as ASCIIcode);
        if eTeXmode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
            showsavegroups();
        }
    }
    while condptr as ::core::ffi::c_long != -(268435455 as ::core::ffi::c_long) {
        zprintnl(40 as ::core::ffi::c_int);
        zprintesc(1736 as ::core::ffi::c_int);
        zprint(1738 as ::core::ffi::c_int);
        zprintcmdchr(108 as ::core::ffi::c_int as quarterword, curif as halfword);
        if ifline != 0 as ::core::ffi::c_int {
            zprint(1739 as ::core::ffi::c_int);
            zprintint(ifline as longinteger);
        }
        zprint(1740 as ::core::ffi::c_int);
        ifline = (*mem.offset((condptr as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
            .u
            .CINT;
        curif = (*mem.offset(condptr as isize)).hh.u.B1 as smallnumber;
        tempptr = condptr;
        condptr = (*mem.offset(condptr as isize)).hh.v.RH;
        zfreenode(tempptr, 2 as ::core::ffi::c_int);
    }
    if history as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if history as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            || (interaction as ::core::ffi::c_int) < 3 as ::core::ffi::c_int
        {
            if selector as ::core::ffi::c_int == 19 as ::core::ffi::c_int {
                selector = 17 as ::core::ffi::c_uchar;
                zprintnl(1741 as ::core::ffi::c_int);
                selector = 19 as ::core::ffi::c_uchar;
            }
        }
    }
    if c as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        if iniversion != 0 {
            let mut for_end: integer = 0;
            c = 0 as smallnumber;
            for_end = 4 as ::core::ffi::c_int as integer;
            if c as ::core::ffi::c_int <= for_end {
                loop {
                    if curmark[c as usize] as ::core::ffi::c_long
                        != -(268435455 as ::core::ffi::c_long)
                    {
                        zdeletetokenref(curmark[c as usize]);
                    }
                    let fresh6 = c;
                    c = c.wrapping_add(1);
                    if !((fresh6 as ::core::ffi::c_int) < for_end) {
                        break;
                    }
                }
            }
            if saroot[6 as ::core::ffi::c_int as usize] as ::core::ffi::c_long
                != -(268435455 as ::core::ffi::c_long)
            {
                if zdomarks(
                    3 as ::core::ffi::c_int as smallnumber,
                    0 as ::core::ffi::c_int as smallnumber,
                    saroot[6 as ::core::ffi::c_int as usize],
                ) != 0
                {
                    saroot[6 as ::core::ffi::c_int as usize] =
                        -(268435455 as ::core::ffi::c_long) as halfword;
                }
            }
            let mut for_end_0: integer = 0;
            c = 2 as smallnumber;
            for_end_0 = 3 as ::core::ffi::c_int as integer;
            if c as ::core::ffi::c_int <= for_end_0 {
                loop {
                    zflushnodelist(discptr[c as usize]);
                    let fresh7 = c;
                    c = c.wrapping_add(1);
                    if !((fresh7 as ::core::ffi::c_int) < for_end_0) {
                        break;
                    }
                }
            }
            if lastglue as ::core::ffi::c_long != 268435455 as ::core::ffi::c_long {
                zdeleteglueref(lastglue);
            }
            storefmtfile();
            return;
        }
        zprintnl(1742 as ::core::ffi::c_int);
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn initprim() {
    let mut eqtb: *mut memoryword = zeqtb;
    nonewcontrolsequence = false_0 as boolean;
    first = 0 as ::core::ffi::c_int as integer;
    zprimitive(
        394 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int as quarterword,
        26628 as ::core::ffi::c_int,
    );
    zprimitive(
        395 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int as quarterword,
        26629 as ::core::ffi::c_int,
    );
    zprimitive(
        396 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int as quarterword,
        26630 as ::core::ffi::c_int,
    );
    zprimitive(
        397 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int as quarterword,
        26631 as ::core::ffi::c_int,
    );
    zprimitive(
        398 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int as quarterword,
        26632 as ::core::ffi::c_int,
    );
    zprimitive(
        399 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int as quarterword,
        26633 as ::core::ffi::c_int,
    );
    zprimitive(
        400 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int as quarterword,
        26634 as ::core::ffi::c_int,
    );
    zprimitive(
        401 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int as quarterword,
        26635 as ::core::ffi::c_int,
    );
    zprimitive(
        402 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int as quarterword,
        26636 as ::core::ffi::c_int,
    );
    zprimitive(
        403 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int as quarterword,
        26637 as ::core::ffi::c_int,
    );
    zprimitive(
        404 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int as quarterword,
        26638 as ::core::ffi::c_int,
    );
    zprimitive(
        405 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int as quarterword,
        26639 as ::core::ffi::c_int,
    );
    zprimitive(
        406 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int as quarterword,
        26640 as ::core::ffi::c_int,
    );
    zprimitive(
        407 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int as quarterword,
        26641 as ::core::ffi::c_int,
    );
    zprimitive(
        408 as ::core::ffi::c_int,
        75 as ::core::ffi::c_int as quarterword,
        26642 as ::core::ffi::c_int,
    );
    zprimitive(
        409 as ::core::ffi::c_int,
        76 as ::core::ffi::c_int as quarterword,
        26643 as ::core::ffi::c_int,
    );
    zprimitive(
        410 as ::core::ffi::c_int,
        76 as ::core::ffi::c_int as quarterword,
        26644 as ::core::ffi::c_int,
    );
    zprimitive(
        411 as ::core::ffi::c_int,
        76 as ::core::ffi::c_int as quarterword,
        26645 as ::core::ffi::c_int,
    );
    zprimitive(
        415 as ::core::ffi::c_int,
        72 as ::core::ffi::c_int as quarterword,
        27159 as ::core::ffi::c_int,
    );
    zprimitive(
        416 as ::core::ffi::c_int,
        72 as ::core::ffi::c_int as quarterword,
        27160 as ::core::ffi::c_int,
    );
    zprimitive(
        417 as ::core::ffi::c_int,
        72 as ::core::ffi::c_int as quarterword,
        27161 as ::core::ffi::c_int,
    );
    zprimitive(
        418 as ::core::ffi::c_int,
        72 as ::core::ffi::c_int as quarterword,
        27162 as ::core::ffi::c_int,
    );
    zprimitive(
        419 as ::core::ffi::c_int,
        72 as ::core::ffi::c_int as quarterword,
        27163 as ::core::ffi::c_int,
    );
    zprimitive(
        420 as ::core::ffi::c_int,
        72 as ::core::ffi::c_int as quarterword,
        27164 as ::core::ffi::c_int,
    );
    zprimitive(
        421 as ::core::ffi::c_int,
        72 as ::core::ffi::c_int as quarterword,
        27165 as ::core::ffi::c_int,
    );
    zprimitive(
        422 as ::core::ffi::c_int,
        72 as ::core::ffi::c_int as quarterword,
        27166 as ::core::ffi::c_int,
    );
    zprimitive(
        423 as ::core::ffi::c_int,
        72 as ::core::ffi::c_int as quarterword,
        27167 as ::core::ffi::c_int,
    );
    zprimitive(
        424 as ::core::ffi::c_int,
        72 as ::core::ffi::c_int as quarterword,
        27168 as ::core::ffi::c_int,
    );
    zprimitive(
        425 as ::core::ffi::c_int,
        72 as ::core::ffi::c_int as quarterword,
        27169 as ::core::ffi::c_int,
    );
    zprimitive(
        426 as ::core::ffi::c_int,
        72 as ::core::ffi::c_int as quarterword,
        27170 as ::core::ffi::c_int,
    );
    zprimitive(
        427 as ::core::ffi::c_int,
        72 as ::core::ffi::c_int as quarterword,
        27171 as ::core::ffi::c_int,
    );
    zprimitive(
        441 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29277 as ::core::ffi::c_int,
    );
    zprimitive(
        442 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29278 as ::core::ffi::c_int,
    );
    zprimitive(
        443 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29279 as ::core::ffi::c_int,
    );
    zprimitive(
        444 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29280 as ::core::ffi::c_int,
    );
    zprimitive(
        445 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29281 as ::core::ffi::c_int,
    );
    zprimitive(
        446 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29282 as ::core::ffi::c_int,
    );
    zprimitive(
        447 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29283 as ::core::ffi::c_int,
    );
    zprimitive(
        448 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29284 as ::core::ffi::c_int,
    );
    zprimitive(
        449 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29285 as ::core::ffi::c_int,
    );
    zprimitive(
        450 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29286 as ::core::ffi::c_int,
    );
    zprimitive(
        451 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29287 as ::core::ffi::c_int,
    );
    zprimitive(
        452 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29288 as ::core::ffi::c_int,
    );
    zprimitive(
        453 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29289 as ::core::ffi::c_int,
    );
    zprimitive(
        454 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29290 as ::core::ffi::c_int,
    );
    zprimitive(
        455 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29291 as ::core::ffi::c_int,
    );
    zprimitive(
        456 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29292 as ::core::ffi::c_int,
    );
    zprimitive(
        457 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29293 as ::core::ffi::c_int,
    );
    zprimitive(
        458 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29294 as ::core::ffi::c_int,
    );
    zprimitive(
        459 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29295 as ::core::ffi::c_int,
    );
    zprimitive(
        460 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29296 as ::core::ffi::c_int,
    );
    zprimitive(
        461 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29297 as ::core::ffi::c_int,
    );
    zprimitive(
        462 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29298 as ::core::ffi::c_int,
    );
    zprimitive(
        463 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29299 as ::core::ffi::c_int,
    );
    zprimitive(
        464 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29300 as ::core::ffi::c_int,
    );
    zprimitive(
        465 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29301 as ::core::ffi::c_int,
    );
    zprimitive(
        466 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29302 as ::core::ffi::c_int,
    );
    zprimitive(
        467 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29303 as ::core::ffi::c_int,
    );
    zprimitive(
        468 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29304 as ::core::ffi::c_int,
    );
    zprimitive(
        469 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29305 as ::core::ffi::c_int,
    );
    zprimitive(
        470 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29306 as ::core::ffi::c_int,
    );
    zprimitive(
        471 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29307 as ::core::ffi::c_int,
    );
    zprimitive(
        472 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29308 as ::core::ffi::c_int,
    );
    zprimitive(
        473 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29309 as ::core::ffi::c_int,
    );
    zprimitive(
        474 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29310 as ::core::ffi::c_int,
    );
    zprimitive(
        475 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29311 as ::core::ffi::c_int,
    );
    zprimitive(
        476 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29312 as ::core::ffi::c_int,
    );
    zprimitive(
        477 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29313 as ::core::ffi::c_int,
    );
    zprimitive(
        478 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29314 as ::core::ffi::c_int,
    );
    zprimitive(
        479 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29315 as ::core::ffi::c_int,
    );
    zprimitive(
        480 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29316 as ::core::ffi::c_int,
    );
    zprimitive(
        481 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29317 as ::core::ffi::c_int,
    );
    zprimitive(
        482 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29318 as ::core::ffi::c_int,
    );
    zprimitive(
        483 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29319 as ::core::ffi::c_int,
    );
    zprimitive(
        484 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29320 as ::core::ffi::c_int,
    );
    zprimitive(
        485 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29321 as ::core::ffi::c_int,
    );
    zprimitive(
        486 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29322 as ::core::ffi::c_int,
    );
    zprimitive(
        487 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29323 as ::core::ffi::c_int,
    );
    zprimitive(
        488 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29324 as ::core::ffi::c_int,
    );
    zprimitive(
        489 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29325 as ::core::ffi::c_int,
    );
    zprimitive(
        490 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29326 as ::core::ffi::c_int,
    );
    zprimitive(
        491 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29327 as ::core::ffi::c_int,
    );
    zprimitive(
        492 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29328 as ::core::ffi::c_int,
    );
    zprimitive(
        493 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29329 as ::core::ffi::c_int,
    );
    zprimitive(
        494 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29330 as ::core::ffi::c_int,
    );
    zprimitive(
        495 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29331 as ::core::ffi::c_int,
    );
    if mltexp != 0 {
        mltexenabledp = true_0 as boolean;
        zprimitive(
            497 as ::core::ffi::c_int,
            73 as ::core::ffi::c_int as quarterword,
            29333 as ::core::ffi::c_int,
        );
        zprimitive(
            498 as ::core::ffi::c_int,
            73 as ::core::ffi::c_int as quarterword,
            29334 as ::core::ffi::c_int,
        );
    }
    if enctexp != 0 {
        enctexenabledp = true_0 as boolean;
        zprimitive(
            499 as ::core::ffi::c_int,
            73 as ::core::ffi::c_int as quarterword,
            29338 as ::core::ffi::c_int,
        );
        zprimitive(
            500 as ::core::ffi::c_int,
            73 as ::core::ffi::c_int as quarterword,
            29339 as ::core::ffi::c_int,
        );
        zprimitive(
            501 as ::core::ffi::c_int,
            73 as ::core::ffi::c_int as quarterword,
            29340 as ::core::ffi::c_int,
        );
        zprimitive(
            502 as ::core::ffi::c_int,
            73 as ::core::ffi::c_int as quarterword,
            29341 as ::core::ffi::c_int,
        );
    }
    zprimitive(
        503 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29335 as ::core::ffi::c_int,
    );
    zprimitive(
        544 as ::core::ffi::c_int,
        103 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        504 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29336 as ::core::ffi::c_int,
    );
    zprimitive(
        505 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29337 as ::core::ffi::c_int,
    );
    zprimitive(
        506 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29342 as ::core::ffi::c_int,
    );
    zprimitive(
        507 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29343 as ::core::ffi::c_int,
    );
    zprimitive(
        508 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29363 as ::core::ffi::c_int,
    );
    zprimitive(
        509 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29344 as ::core::ffi::c_int,
    );
    zprimitive(
        510 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29345 as ::core::ffi::c_int,
    );
    zprimitive(
        511 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29346 as ::core::ffi::c_int,
    );
    zprimitive(
        512 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29347 as ::core::ffi::c_int,
    );
    zprimitive(
        513 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29348 as ::core::ffi::c_int,
    );
    zprimitive(
        545 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29352 as ::core::ffi::c_int,
    );
    zprimitive(
        514 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29349 as ::core::ffi::c_int,
    );
    zprimitive(
        515 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29350 as ::core::ffi::c_int,
    );
    zprimitive(
        516 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29351 as ::core::ffi::c_int,
    );
    zprimitive(
        517 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29352 as ::core::ffi::c_int,
    );
    zprimitive(
        518 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29353 as ::core::ffi::c_int,
    );
    zprimitive(
        519 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29354 as ::core::ffi::c_int,
    );
    zprimitive(
        520 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29355 as ::core::ffi::c_int,
    );
    zprimitive(
        521 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29356 as ::core::ffi::c_int,
    );
    zprimitive(
        522 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29357 as ::core::ffi::c_int,
    );
    zprimitive(
        523 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29358 as ::core::ffi::c_int,
    );
    zprimitive(
        524 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29359 as ::core::ffi::c_int,
    );
    zprimitive(
        525 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29360 as ::core::ffi::c_int,
    );
    zprimitive(
        526 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29361 as ::core::ffi::c_int,
    );
    zprimitive(
        527 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29362 as ::core::ffi::c_int,
    );
    zprimitive(
        528 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29364 as ::core::ffi::c_int,
    );
    zprimitive(
        529 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29365 as ::core::ffi::c_int,
    );
    zprimitive(
        530 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29366 as ::core::ffi::c_int,
    );
    zprimitive(
        531 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29367 as ::core::ffi::c_int,
    );
    zprimitive(
        532 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29368 as ::core::ffi::c_int,
    );
    zprimitive(
        533 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29369 as ::core::ffi::c_int,
    );
    zprimitive(
        534 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29370 as ::core::ffi::c_int,
    );
    zprimitive(
        535 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29371 as ::core::ffi::c_int,
    );
    zprimitive(
        536 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29372 as ::core::ffi::c_int,
    );
    zprimitive(
        537 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29373 as ::core::ffi::c_int,
    );
    zprimitive(
        538 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29374 as ::core::ffi::c_int,
    );
    zprimitive(
        539 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29375 as ::core::ffi::c_int,
    );
    zprimitive(
        540 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29376 as ::core::ffi::c_int,
    );
    zprimitive(
        541 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29377 as ::core::ffi::c_int,
    );
    zprimitive(
        542 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29378 as ::core::ffi::c_int,
    );
    zprimitive(
        548 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29903 as ::core::ffi::c_int,
    );
    zprimitive(
        549 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29904 as ::core::ffi::c_int,
    );
    zprimitive(
        550 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29905 as ::core::ffi::c_int,
    );
    zprimitive(
        551 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29906 as ::core::ffi::c_int,
    );
    zprimitive(
        552 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29907 as ::core::ffi::c_int,
    );
    zprimitive(
        553 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29908 as ::core::ffi::c_int,
    );
    zprimitive(
        554 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29909 as ::core::ffi::c_int,
    );
    zprimitive(
        555 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29910 as ::core::ffi::c_int,
    );
    zprimitive(
        556 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29911 as ::core::ffi::c_int,
    );
    zprimitive(
        557 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29912 as ::core::ffi::c_int,
    );
    zprimitive(
        558 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29913 as ::core::ffi::c_int,
    );
    zprimitive(
        559 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29914 as ::core::ffi::c_int,
    );
    zprimitive(
        560 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29915 as ::core::ffi::c_int,
    );
    zprimitive(
        561 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29916 as ::core::ffi::c_int,
    );
    zprimitive(
        562 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29917 as ::core::ffi::c_int,
    );
    zprimitive(
        563 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29918 as ::core::ffi::c_int,
    );
    zprimitive(
        564 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29919 as ::core::ffi::c_int,
    );
    zprimitive(
        565 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29920 as ::core::ffi::c_int,
    );
    zprimitive(
        566 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29921 as ::core::ffi::c_int,
    );
    zprimitive(
        567 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29922 as ::core::ffi::c_int,
    );
    zprimitive(
        568 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29923 as ::core::ffi::c_int,
    );
    zprimitive(
        569 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29924 as ::core::ffi::c_int,
    );
    zprimitive(
        570 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29925 as ::core::ffi::c_int,
    );
    zprimitive(
        571 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29926 as ::core::ffi::c_int,
    );
    zprimitive(
        572 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29927 as ::core::ffi::c_int,
    );
    zprimitive(
        573 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29928 as ::core::ffi::c_int,
    );
    zprimitive(
        574 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29929 as ::core::ffi::c_int,
    );
    zprimitive(
        575 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29930 as ::core::ffi::c_int,
    );
    zprimitive(
        576 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29931 as ::core::ffi::c_int,
    );
    zprimitive(
        577 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29932 as ::core::ffi::c_int,
    );
    zprimitive(
        578 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29933 as ::core::ffi::c_int,
    );
    zprimitive(
        579 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29934 as ::core::ffi::c_int,
    );
    zprimitive(
        580 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29935 as ::core::ffi::c_int,
    );
    zprimitive(
        581 as ::core::ffi::c_int,
        74 as ::core::ffi::c_int as quarterword,
        29936 as ::core::ffi::c_int,
    );
    zprimitive(
        32 as ::core::ffi::c_int,
        64 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        47 as ::core::ffi::c_int,
        44 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        593 as ::core::ffi::c_int,
        45 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        594 as ::core::ffi::c_int,
        90 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        595 as ::core::ffi::c_int,
        40 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        596 as ::core::ffi::c_int,
        41 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        597 as ::core::ffi::c_int,
        61 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        598 as ::core::ffi::c_int,
        16 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        589 as ::core::ffi::c_int,
        110 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        599 as ::core::ffi::c_int,
        15 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        600 as ::core::ffi::c_int,
        92 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        590 as ::core::ffi::c_int,
        67 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    if enctexp != 0 {
        zprimitive(
            601 as ::core::ffi::c_int,
            67 as ::core::ffi::c_int as quarterword,
            10 as ::core::ffi::c_int,
        );
    }
    zprimitive(
        602 as ::core::ffi::c_int,
        62 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    (*hash.offset(15516 as ::core::ffi::c_int as isize)).v.RH =
        602 as ::core::ffi::c_int as halfword;
    *eqtb.offset(15516 as ::core::ffi::c_int as isize) = *eqtb.offset(curval as isize);
    zprimitive(
        603 as ::core::ffi::c_int,
        105 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        604 as ::core::ffi::c_int,
        88 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        605 as ::core::ffi::c_int,
        101 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        606 as ::core::ffi::c_int,
        102 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        607 as ::core::ffi::c_int,
        77 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        608 as ::core::ffi::c_int,
        32 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        609 as ::core::ffi::c_int,
        36 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        610 as ::core::ffi::c_int,
        39 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        337 as ::core::ffi::c_int,
        37 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        360 as ::core::ffi::c_int,
        18 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        611 as ::core::ffi::c_int,
        46 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        612 as ::core::ffi::c_int,
        17 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        613 as ::core::ffi::c_int,
        54 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        614 as ::core::ffi::c_int,
        91 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        615 as ::core::ffi::c_int,
        34 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        616 as ::core::ffi::c_int,
        65 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        617 as ::core::ffi::c_int,
        106 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        586 as ::core::ffi::c_int,
        106 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        342 as ::core::ffi::c_int,
        55 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        618 as ::core::ffi::c_int,
        63 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        619 as ::core::ffi::c_int,
        84 as ::core::ffi::c_int as quarterword,
        27158 as ::core::ffi::c_int,
    );
    zprimitive(
        620 as ::core::ffi::c_int,
        42 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        621 as ::core::ffi::c_int,
        80 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        622 as ::core::ffi::c_int,
        66 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        623 as ::core::ffi::c_int,
        96 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        624 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int as quarterword,
        256 as ::core::ffi::c_int,
    );
    (*hash.offset(15521 as ::core::ffi::c_int as isize)).v.RH =
        624 as ::core::ffi::c_int as halfword;
    *eqtb.offset(15521 as ::core::ffi::c_int as isize) = *eqtb.offset(curval as isize);
    zprimitive(
        625 as ::core::ffi::c_int,
        98 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        626 as ::core::ffi::c_int,
        112 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        428 as ::core::ffi::c_int,
        71 as ::core::ffi::c_int as quarterword,
        membot,
    );
    zprimitive(
        361 as ::core::ffi::c_int,
        38 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        627 as ::core::ffi::c_int,
        33 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        628 as ::core::ffi::c_int,
        56 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        629 as ::core::ffi::c_int,
        35 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        691 as ::core::ffi::c_int,
        13 as ::core::ffi::c_int as quarterword,
        256 as ::core::ffi::c_int,
    );
    parloc = curval as halfword;
    partoken = 4095 as halfword + parloc;
    zprimitive(
        727 as ::core::ffi::c_int,
        107 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        728 as ::core::ffi::c_int,
        107 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        729 as ::core::ffi::c_int,
        113 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        730 as ::core::ffi::c_int,
        113 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        731 as ::core::ffi::c_int,
        113 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        732 as ::core::ffi::c_int,
        113 as ::core::ffi::c_int as quarterword,
        3 as ::core::ffi::c_int,
    );
    zprimitive(
        733 as ::core::ffi::c_int,
        113 as ::core::ffi::c_int as quarterword,
        4 as ::core::ffi::c_int,
    );
    zprimitive(
        546 as ::core::ffi::c_int,
        89 as ::core::ffi::c_int as quarterword,
        membot as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
    );
    zprimitive(
        583 as ::core::ffi::c_int,
        89 as ::core::ffi::c_int as quarterword,
        membot as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
    );
    zprimitive(
        413 as ::core::ffi::c_int,
        89 as ::core::ffi::c_int as quarterword,
        membot as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
    );
    zprimitive(
        414 as ::core::ffi::c_int,
        89 as ::core::ffi::c_int as quarterword,
        membot as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
    );
    zprimitive(
        766 as ::core::ffi::c_int,
        79 as ::core::ffi::c_int as quarterword,
        105 as ::core::ffi::c_int,
    );
    zprimitive(
        767 as ::core::ffi::c_int,
        79 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        768 as ::core::ffi::c_int,
        82 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        769 as ::core::ffi::c_int,
        82 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        770 as ::core::ffi::c_int,
        83 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        771 as ::core::ffi::c_int,
        83 as ::core::ffi::c_int as quarterword,
        3 as ::core::ffi::c_int,
    );
    zprimitive(
        772 as ::core::ffi::c_int,
        83 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        773 as ::core::ffi::c_int,
        70 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        774 as ::core::ffi::c_int,
        70 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        775 as ::core::ffi::c_int,
        70 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        776 as ::core::ffi::c_int,
        70 as ::core::ffi::c_int as quarterword,
        4 as ::core::ffi::c_int,
    );
    zprimitive(
        777 as ::core::ffi::c_int,
        70 as ::core::ffi::c_int as quarterword,
        5 as ::core::ffi::c_int,
    );
    zprimitive(
        778 as ::core::ffi::c_int,
        70 as ::core::ffi::c_int as quarterword,
        6 as ::core::ffi::c_int,
    );
    zprimitive(
        779 as ::core::ffi::c_int,
        70 as ::core::ffi::c_int as quarterword,
        7 as ::core::ffi::c_int,
    );
    zprimitive(
        780 as ::core::ffi::c_int,
        70 as ::core::ffi::c_int as quarterword,
        8 as ::core::ffi::c_int,
    );
    zprimitive(
        781 as ::core::ffi::c_int,
        70 as ::core::ffi::c_int as quarterword,
        9 as ::core::ffi::c_int,
    );
    zprimitive(
        782 as ::core::ffi::c_int,
        70 as ::core::ffi::c_int as quarterword,
        10 as ::core::ffi::c_int,
    );
    zprimitive(
        783 as ::core::ffi::c_int,
        70 as ::core::ffi::c_int as quarterword,
        11 as ::core::ffi::c_int,
    );
    zprimitive(
        784 as ::core::ffi::c_int,
        70 as ::core::ffi::c_int as quarterword,
        12 as ::core::ffi::c_int,
    );
    zprimitive(
        785 as ::core::ffi::c_int,
        70 as ::core::ffi::c_int as quarterword,
        13 as ::core::ffi::c_int,
    );
    zprimitive(
        786 as ::core::ffi::c_int,
        70 as ::core::ffi::c_int as quarterword,
        14 as ::core::ffi::c_int,
    );
    zprimitive(
        787 as ::core::ffi::c_int,
        70 as ::core::ffi::c_int as quarterword,
        15 as ::core::ffi::c_int,
    );
    zprimitive(
        788 as ::core::ffi::c_int,
        70 as ::core::ffi::c_int as quarterword,
        16 as ::core::ffi::c_int,
    );
    zprimitive(
        789 as ::core::ffi::c_int,
        70 as ::core::ffi::c_int as quarterword,
        17 as ::core::ffi::c_int,
    );
    zprimitive(
        790 as ::core::ffi::c_int,
        70 as ::core::ffi::c_int as quarterword,
        18 as ::core::ffi::c_int,
    );
    zprimitive(
        791 as ::core::ffi::c_int,
        70 as ::core::ffi::c_int as quarterword,
        19 as ::core::ffi::c_int,
    );
    zprimitive(
        850 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        851 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        852 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        853 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        3 as ::core::ffi::c_int,
    );
    zprimitive(
        854 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        4 as ::core::ffi::c_int,
    );
    zprimitive(
        855 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        6 as ::core::ffi::c_int,
    );
    zprimitive(
        856 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        7 as ::core::ffi::c_int,
    );
    zprimitive(
        857 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        8 as ::core::ffi::c_int,
    );
    zprimitive(
        858 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        9 as ::core::ffi::c_int,
    );
    zprimitive(
        859 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        10 as ::core::ffi::c_int,
    );
    zprimitive(
        860 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        11 as ::core::ffi::c_int,
    );
    zprimitive(
        861 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        12 as ::core::ffi::c_int,
    );
    zprimitive(
        862 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        16 as ::core::ffi::c_int,
    );
    zprimitive(
        863 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        17 as ::core::ffi::c_int,
    );
    zprimitive(
        864 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        13 as ::core::ffi::c_int,
    );
    zprimitive(
        865 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        14 as ::core::ffi::c_int,
    );
    zprimitive(
        866 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        15 as ::core::ffi::c_int,
    );
    zprimitive(
        867 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        20 as ::core::ffi::c_int,
    );
    zprimitive(
        868 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        21 as ::core::ffi::c_int,
    );
    zprimitive(
        869 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        22 as ::core::ffi::c_int,
    );
    zprimitive(
        870 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        23 as ::core::ffi::c_int,
    );
    zprimitive(
        871 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        24 as ::core::ffi::c_int,
    );
    zprimitive(
        872 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        25 as ::core::ffi::c_int,
    );
    zprimitive(
        873 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        26 as ::core::ffi::c_int,
    );
    zprimitive(
        874 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        27 as ::core::ffi::c_int,
    );
    zprimitive(
        875 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        28 as ::core::ffi::c_int,
    );
    zprimitive(
        876 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        18 as ::core::ffi::c_int,
    );
    zprimitive(
        877 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        19 as ::core::ffi::c_int,
    );
    zprimitive(
        878 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        29 as ::core::ffi::c_int,
    );
    zprimitive(
        879 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        30 as ::core::ffi::c_int,
    );
    zprimitive(
        880 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        33 as ::core::ffi::c_int,
    );
    zprimitive(
        881 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        31 as ::core::ffi::c_int,
    );
    zprimitive(
        882 as ::core::ffi::c_int,
        111 as ::core::ffi::c_int as quarterword,
        32 as ::core::ffi::c_int,
    );
    zprimitive(
        922 as ::core::ffi::c_int,
        108 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        923 as ::core::ffi::c_int,
        108 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        924 as ::core::ffi::c_int,
        108 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        925 as ::core::ffi::c_int,
        108 as ::core::ffi::c_int as quarterword,
        3 as ::core::ffi::c_int,
    );
    zprimitive(
        926 as ::core::ffi::c_int,
        108 as ::core::ffi::c_int as quarterword,
        4 as ::core::ffi::c_int,
    );
    zprimitive(
        927 as ::core::ffi::c_int,
        108 as ::core::ffi::c_int as quarterword,
        5 as ::core::ffi::c_int,
    );
    zprimitive(
        928 as ::core::ffi::c_int,
        108 as ::core::ffi::c_int as quarterword,
        6 as ::core::ffi::c_int,
    );
    zprimitive(
        929 as ::core::ffi::c_int,
        108 as ::core::ffi::c_int as quarterword,
        7 as ::core::ffi::c_int,
    );
    zprimitive(
        930 as ::core::ffi::c_int,
        108 as ::core::ffi::c_int as quarterword,
        8 as ::core::ffi::c_int,
    );
    zprimitive(
        931 as ::core::ffi::c_int,
        108 as ::core::ffi::c_int as quarterword,
        9 as ::core::ffi::c_int,
    );
    zprimitive(
        932 as ::core::ffi::c_int,
        108 as ::core::ffi::c_int as quarterword,
        10 as ::core::ffi::c_int,
    );
    zprimitive(
        933 as ::core::ffi::c_int,
        108 as ::core::ffi::c_int as quarterword,
        11 as ::core::ffi::c_int,
    );
    zprimitive(
        934 as ::core::ffi::c_int,
        108 as ::core::ffi::c_int as quarterword,
        12 as ::core::ffi::c_int,
    );
    zprimitive(
        935 as ::core::ffi::c_int,
        108 as ::core::ffi::c_int as quarterword,
        13 as ::core::ffi::c_int,
    );
    zprimitive(
        936 as ::core::ffi::c_int,
        108 as ::core::ffi::c_int as quarterword,
        14 as ::core::ffi::c_int,
    );
    zprimitive(
        937 as ::core::ffi::c_int,
        108 as ::core::ffi::c_int as quarterword,
        15 as ::core::ffi::c_int,
    );
    zprimitive(
        938 as ::core::ffi::c_int,
        108 as ::core::ffi::c_int as quarterword,
        16 as ::core::ffi::c_int,
    );
    zprimitive(
        939 as ::core::ffi::c_int,
        108 as ::core::ffi::c_int as quarterword,
        21 as ::core::ffi::c_int,
    );
    zprimitive(
        941 as ::core::ffi::c_int,
        109 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    (*hash.offset(15518 as ::core::ffi::c_int as isize)).v.RH =
        941 as ::core::ffi::c_int as halfword;
    *eqtb.offset(15518 as ::core::ffi::c_int as isize) = *eqtb.offset(curval as isize);
    zprimitive(
        942 as ::core::ffi::c_int,
        109 as ::core::ffi::c_int as quarterword,
        4 as ::core::ffi::c_int,
    );
    zprimitive(
        943 as ::core::ffi::c_int,
        109 as ::core::ffi::c_int as quarterword,
        3 as ::core::ffi::c_int,
    );
    zprimitive(
        969 as ::core::ffi::c_int,
        87 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    (*hash.offset(17626 as ::core::ffi::c_int as isize)).v.RH =
        969 as ::core::ffi::c_int as halfword;
    *eqtb.offset(17626 as ::core::ffi::c_int as isize) = *eqtb.offset(curval as isize);
    zprimitive(
        1321 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int as quarterword,
        256 as ::core::ffi::c_int,
    );
    zprimitive(
        1322 as ::core::ffi::c_int,
        5 as ::core::ffi::c_int as quarterword,
        257 as ::core::ffi::c_int,
    );
    (*hash.offset(15515 as ::core::ffi::c_int as isize)).v.RH =
        1322 as ::core::ffi::c_int as halfword;
    *eqtb.offset(15515 as ::core::ffi::c_int as isize) = *eqtb.offset(curval as isize);
    zprimitive(
        1323 as ::core::ffi::c_int,
        5 as ::core::ffi::c_int as quarterword,
        258 as ::core::ffi::c_int,
    );
    (*hash.offset(15519 as ::core::ffi::c_int as isize)).v.RH =
        1324 as ::core::ffi::c_int as halfword;
    (*hash.offset(15520 as ::core::ffi::c_int as isize)).v.RH =
        1324 as ::core::ffi::c_int as halfword;
    (*eqtb.offset(15520 as ::core::ffi::c_int as isize)).hh.u.B0 = 9 as ::core::ffi::c_short;
    (*eqtb.offset(15520 as ::core::ffi::c_int as isize)).hh.v.RH =
        (memtop as ::core::ffi::c_int - 11 as ::core::ffi::c_int) as halfword;
    (*eqtb.offset(15520 as ::core::ffi::c_int as isize)).hh.u.B1 = 1 as ::core::ffi::c_short;
    *eqtb.offset(15519 as ::core::ffi::c_int as isize) =
        *eqtb.offset(15520 as ::core::ffi::c_int as isize);
    (*eqtb.offset(15519 as ::core::ffi::c_int as isize)).hh.u.B0 = 118 as ::core::ffi::c_short;
    zprimitive(
        1399 as ::core::ffi::c_int,
        81 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1400 as ::core::ffi::c_int,
        81 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1401 as ::core::ffi::c_int,
        81 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        1402 as ::core::ffi::c_int,
        81 as ::core::ffi::c_int as quarterword,
        3 as ::core::ffi::c_int,
    );
    zprimitive(
        1403 as ::core::ffi::c_int,
        81 as ::core::ffi::c_int as quarterword,
        4 as ::core::ffi::c_int,
    );
    zprimitive(
        1404 as ::core::ffi::c_int,
        81 as ::core::ffi::c_int as quarterword,
        5 as ::core::ffi::c_int,
    );
    zprimitive(
        1405 as ::core::ffi::c_int,
        81 as ::core::ffi::c_int as quarterword,
        6 as ::core::ffi::c_int,
    );
    zprimitive(
        1406 as ::core::ffi::c_int,
        81 as ::core::ffi::c_int as quarterword,
        7 as ::core::ffi::c_int,
    );
    zprimitive(
        350 as ::core::ffi::c_int,
        14 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1451 as ::core::ffi::c_int,
        14 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1452 as ::core::ffi::c_int,
        26 as ::core::ffi::c_int as quarterword,
        4 as ::core::ffi::c_int,
    );
    zprimitive(
        1453 as ::core::ffi::c_int,
        26 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1454 as ::core::ffi::c_int,
        26 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1455 as ::core::ffi::c_int,
        26 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        1456 as ::core::ffi::c_int,
        26 as ::core::ffi::c_int as quarterword,
        3 as ::core::ffi::c_int,
    );
    zprimitive(
        1457 as ::core::ffi::c_int,
        27 as ::core::ffi::c_int as quarterword,
        4 as ::core::ffi::c_int,
    );
    zprimitive(
        1458 as ::core::ffi::c_int,
        27 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1459 as ::core::ffi::c_int,
        27 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1460 as ::core::ffi::c_int,
        27 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        1461 as ::core::ffi::c_int,
        27 as ::core::ffi::c_int as quarterword,
        3 as ::core::ffi::c_int,
    );
    zprimitive(
        343 as ::core::ffi::c_int,
        28 as ::core::ffi::c_int as quarterword,
        5 as ::core::ffi::c_int,
    );
    zprimitive(
        322 as ::core::ffi::c_int,
        29 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        349 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int as quarterword,
        99 as ::core::ffi::c_int,
    );
    zprimitive(
        1479 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1480 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1481 as ::core::ffi::c_int,
        22 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1482 as ::core::ffi::c_int,
        22 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        430 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1483 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1484 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        1394 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int as quarterword,
        3 as ::core::ffi::c_int,
    );
    zprimitive(
        1485 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int as quarterword,
        4 as ::core::ffi::c_int,
    );
    zprimitive(
        1396 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int as quarterword,
        5 as ::core::ffi::c_int,
    );
    zprimitive(
        1486 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int as quarterword,
        109 as ::core::ffi::c_int,
    );
    zprimitive(
        1487 as ::core::ffi::c_int,
        31 as ::core::ffi::c_int as quarterword,
        99 as ::core::ffi::c_int,
    );
    zprimitive(
        1488 as ::core::ffi::c_int,
        31 as ::core::ffi::c_int as quarterword,
        100 as ::core::ffi::c_int,
    );
    zprimitive(
        1489 as ::core::ffi::c_int,
        31 as ::core::ffi::c_int as quarterword,
        101 as ::core::ffi::c_int,
    );
    zprimitive(
        1490 as ::core::ffi::c_int,
        31 as ::core::ffi::c_int as quarterword,
        102 as ::core::ffi::c_int,
    );
    zprimitive(
        1506 as ::core::ffi::c_int,
        43 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1507 as ::core::ffi::c_int,
        43 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1508 as ::core::ffi::c_int,
        43 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        1518 as ::core::ffi::c_int,
        25 as ::core::ffi::c_int as quarterword,
        12 as ::core::ffi::c_int,
    );
    zprimitive(
        1519 as ::core::ffi::c_int,
        25 as ::core::ffi::c_int as quarterword,
        11 as ::core::ffi::c_int,
    );
    zprimitive(
        1520 as ::core::ffi::c_int,
        25 as ::core::ffi::c_int as quarterword,
        10 as ::core::ffi::c_int,
    );
    zprimitive(
        1521 as ::core::ffi::c_int,
        23 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1522 as ::core::ffi::c_int,
        23 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1523 as ::core::ffi::c_int,
        24 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1524 as ::core::ffi::c_int,
        24 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        45 as ::core::ffi::c_int,
        47 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        358 as ::core::ffi::c_int,
        47 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1556 as ::core::ffi::c_int,
        48 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1557 as ::core::ffi::c_int,
        48 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1288 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int as quarterword,
        16 as ::core::ffi::c_int,
    );
    zprimitive(
        1289 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int as quarterword,
        17 as ::core::ffi::c_int,
    );
    zprimitive(
        1290 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int as quarterword,
        18 as ::core::ffi::c_int,
    );
    zprimitive(
        1291 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int as quarterword,
        19 as ::core::ffi::c_int,
    );
    zprimitive(
        1292 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int as quarterword,
        20 as ::core::ffi::c_int,
    );
    zprimitive(
        1293 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int as quarterword,
        21 as ::core::ffi::c_int,
    );
    zprimitive(
        1294 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int as quarterword,
        22 as ::core::ffi::c_int,
    );
    zprimitive(
        1295 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int as quarterword,
        23 as ::core::ffi::c_int,
    );
    zprimitive(
        1297 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int as quarterword,
        26 as ::core::ffi::c_int,
    );
    zprimitive(
        1296 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int as quarterword,
        27 as ::core::ffi::c_int,
    );
    zprimitive(
        1558 as ::core::ffi::c_int,
        51 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1301 as ::core::ffi::c_int,
        51 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1302 as ::core::ffi::c_int,
        51 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        1283 as ::core::ffi::c_int,
        53 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1284 as ::core::ffi::c_int,
        53 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        1285 as ::core::ffi::c_int,
        53 as ::core::ffi::c_int as quarterword,
        4 as ::core::ffi::c_int,
    );
    zprimitive(
        1286 as ::core::ffi::c_int,
        53 as ::core::ffi::c_int as quarterword,
        6 as ::core::ffi::c_int,
    );
    zprimitive(
        1576 as ::core::ffi::c_int,
        52 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1577 as ::core::ffi::c_int,
        52 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1578 as ::core::ffi::c_int,
        52 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        1579 as ::core::ffi::c_int,
        52 as ::core::ffi::c_int as quarterword,
        3 as ::core::ffi::c_int,
    );
    zprimitive(
        1580 as ::core::ffi::c_int,
        52 as ::core::ffi::c_int as quarterword,
        4 as ::core::ffi::c_int,
    );
    zprimitive(
        1581 as ::core::ffi::c_int,
        52 as ::core::ffi::c_int as quarterword,
        5 as ::core::ffi::c_int,
    );
    zprimitive(
        1298 as ::core::ffi::c_int,
        49 as ::core::ffi::c_int as quarterword,
        30 as ::core::ffi::c_int,
    );
    zprimitive(
        1299 as ::core::ffi::c_int,
        49 as ::core::ffi::c_int as quarterword,
        31 as ::core::ffi::c_int,
    );
    (*hash.offset(15517 as ::core::ffi::c_int as isize)).v.RH =
        1299 as ::core::ffi::c_int as halfword;
    *eqtb.offset(15517 as ::core::ffi::c_int as isize) = *eqtb.offset(curval as isize);
    zprimitive(
        1601 as ::core::ffi::c_int,
        93 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1602 as ::core::ffi::c_int,
        93 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        1603 as ::core::ffi::c_int,
        93 as ::core::ffi::c_int as quarterword,
        4 as ::core::ffi::c_int,
    );
    zprimitive(
        1604 as ::core::ffi::c_int,
        97 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1605 as ::core::ffi::c_int,
        97 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1606 as ::core::ffi::c_int,
        97 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        1607 as ::core::ffi::c_int,
        97 as ::core::ffi::c_int as quarterword,
        3 as ::core::ffi::c_int,
    );
    zprimitive(
        1624 as ::core::ffi::c_int,
        94 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1625 as ::core::ffi::c_int,
        94 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    if enctexp != 0 {
        zprimitive(
            1626 as ::core::ffi::c_int,
            94 as ::core::ffi::c_int as quarterword,
            10 as ::core::ffi::c_int,
        );
        zprimitive(
            1627 as ::core::ffi::c_int,
            94 as ::core::ffi::c_int as quarterword,
            11 as ::core::ffi::c_int,
        );
    }
    zprimitive(
        1633 as ::core::ffi::c_int,
        95 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1634 as ::core::ffi::c_int,
        95 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1635 as ::core::ffi::c_int,
        95 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        1636 as ::core::ffi::c_int,
        95 as ::core::ffi::c_int as quarterword,
        3 as ::core::ffi::c_int,
    );
    zprimitive(
        1637 as ::core::ffi::c_int,
        95 as ::core::ffi::c_int as quarterword,
        4 as ::core::ffi::c_int,
    );
    zprimitive(
        1638 as ::core::ffi::c_int,
        95 as ::core::ffi::c_int as quarterword,
        5 as ::core::ffi::c_int,
    );
    zprimitive(
        1639 as ::core::ffi::c_int,
        95 as ::core::ffi::c_int as quarterword,
        6 as ::core::ffi::c_int,
    );
    if mltexp != 0 {
        zprimitive(
            1640 as ::core::ffi::c_int,
            95 as ::core::ffi::c_int as quarterword,
            7 as ::core::ffi::c_int,
        );
    }
    zprimitive(
        436 as ::core::ffi::c_int,
        85 as ::core::ffi::c_int as quarterword,
        27741 as ::core::ffi::c_int,
    );
    if enctexp != 0 {
        zprimitive(
            1645 as ::core::ffi::c_int,
            85 as ::core::ffi::c_int as quarterword,
            27690 as ::core::ffi::c_int,
        );
        zprimitive(
            1646 as ::core::ffi::c_int,
            85 as ::core::ffi::c_int as quarterword,
            27691 as ::core::ffi::c_int,
        );
        zprimitive(
            1647 as ::core::ffi::c_int,
            85 as ::core::ffi::c_int as quarterword,
            27692 as ::core::ffi::c_int,
        );
    }
    zprimitive(
        440 as ::core::ffi::c_int,
        85 as ::core::ffi::c_int as quarterword,
        28765 as ::core::ffi::c_int,
    );
    zprimitive(
        437 as ::core::ffi::c_int,
        85 as ::core::ffi::c_int as quarterword,
        27997 as ::core::ffi::c_int,
    );
    zprimitive(
        438 as ::core::ffi::c_int,
        85 as ::core::ffi::c_int as quarterword,
        28253 as ::core::ffi::c_int,
    );
    zprimitive(
        439 as ::core::ffi::c_int,
        85 as ::core::ffi::c_int as quarterword,
        28509 as ::core::ffi::c_int,
    );
    zprimitive(
        547 as ::core::ffi::c_int,
        85 as ::core::ffi::c_int as quarterword,
        29647 as ::core::ffi::c_int,
    );
    zprimitive(
        433 as ::core::ffi::c_int,
        86 as ::core::ffi::c_int as quarterword,
        27693 as ::core::ffi::c_int,
    );
    zprimitive(
        434 as ::core::ffi::c_int,
        86 as ::core::ffi::c_int as quarterword,
        27709 as ::core::ffi::c_int,
    );
    zprimitive(
        435 as ::core::ffi::c_int,
        86 as ::core::ffi::c_int as quarterword,
        27725 as ::core::ffi::c_int,
    );
    zprimitive(
        1368 as ::core::ffi::c_int,
        99 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1380 as ::core::ffi::c_int,
        99 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1663 as ::core::ffi::c_int,
        78 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1664 as ::core::ffi::c_int,
        78 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1665 as ::core::ffi::c_int,
        78 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        1666 as ::core::ffi::c_int,
        78 as ::core::ffi::c_int as quarterword,
        3 as ::core::ffi::c_int,
    );
    zprimitive(
        1667 as ::core::ffi::c_int,
        78 as ::core::ffi::c_int as quarterword,
        4 as ::core::ffi::c_int,
    );
    zprimitive(
        1668 as ::core::ffi::c_int,
        78 as ::core::ffi::c_int as quarterword,
        5 as ::core::ffi::c_int,
    );
    zprimitive(
        1669 as ::core::ffi::c_int,
        78 as ::core::ffi::c_int as quarterword,
        7 as ::core::ffi::c_int,
    );
    zprimitive(
        1670 as ::core::ffi::c_int,
        78 as ::core::ffi::c_int as quarterword,
        8 as ::core::ffi::c_int,
    );
    zprimitive(
        1671 as ::core::ffi::c_int,
        78 as ::core::ffi::c_int as quarterword,
        9 as ::core::ffi::c_int,
    );
    zprimitive(
        1672 as ::core::ffi::c_int,
        78 as ::core::ffi::c_int as quarterword,
        10 as ::core::ffi::c_int,
    );
    zprimitive(
        1673 as ::core::ffi::c_int,
        78 as ::core::ffi::c_int as quarterword,
        11 as ::core::ffi::c_int,
    );
    zprimitive(
        1674 as ::core::ffi::c_int,
        78 as ::core::ffi::c_int as quarterword,
        6 as ::core::ffi::c_int,
    );
    zprimitive(
        274 as ::core::ffi::c_int,
        100 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        275 as ::core::ffi::c_int,
        100 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        276 as ::core::ffi::c_int,
        100 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        1682 as ::core::ffi::c_int,
        100 as ::core::ffi::c_int as quarterword,
        3 as ::core::ffi::c_int,
    );
    zprimitive(
        1683 as ::core::ffi::c_int,
        60 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1684 as ::core::ffi::c_int,
        60 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1685 as ::core::ffi::c_int,
        58 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1686 as ::core::ffi::c_int,
        58 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1692 as ::core::ffi::c_int,
        57 as ::core::ffi::c_int as quarterword,
        27997 as ::core::ffi::c_int,
    );
    zprimitive(
        1693 as ::core::ffi::c_int,
        57 as ::core::ffi::c_int as quarterword,
        28253 as ::core::ffi::c_int,
    );
    zprimitive(
        1694 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        1695 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    zprimitive(
        1696 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        1697 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int as quarterword,
        3 as ::core::ffi::c_int,
    );
    zprimitive(
        1744 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        0 as ::core::ffi::c_int,
    );
    zprimitive(
        687 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        1 as ::core::ffi::c_int,
    );
    writeloc = curval as halfword;
    zprimitive(
        1745 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        2 as ::core::ffi::c_int,
    );
    zprimitive(
        1746 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        3 as ::core::ffi::c_int,
    );
    (*hash.offset(15524 as ::core::ffi::c_int as isize)).v.RH =
        1746 as ::core::ffi::c_int as halfword;
    *eqtb.offset(15524 as ::core::ffi::c_int as isize) = *eqtb.offset(curval as isize);
    zprimitive(
        1747 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        5 as ::core::ffi::c_int,
    );
    zprimitive(
        1748 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        6 as ::core::ffi::c_int,
    );
    zprimitive(
        1749 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        7 as ::core::ffi::c_int,
    );
    zprimitive(
        1145 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        40 as ::core::ffi::c_int,
    );
    zprimitive(
        1750 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        41 as ::core::ffi::c_int,
    );
    zprimitive(
        1751 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        42 as ::core::ffi::c_int,
    );
    zprimitive(
        1752 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        43 as ::core::ffi::c_int,
    );
    zprimitive(
        1753 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        9 as ::core::ffi::c_int,
    );
    zprimitive(
        1754 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        10 as ::core::ffi::c_int,
    );
    zprimitive(
        1755 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        11 as ::core::ffi::c_int,
    );
    zprimitive(
        1756 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        12 as ::core::ffi::c_int,
    );
    zprimitive(
        1757 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        13 as ::core::ffi::c_int,
    );
    zprimitive(
        1758 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        14 as ::core::ffi::c_int,
    );
    zprimitive(
        1759 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        15 as ::core::ffi::c_int,
    );
    zprimitive(
        1760 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        16 as ::core::ffi::c_int,
    );
    zprimitive(
        1761 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        17 as ::core::ffi::c_int,
    );
    zprimitive(
        1762 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        18 as ::core::ffi::c_int,
    );
    zprimitive(
        1763 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        19 as ::core::ffi::c_int,
    );
    zprimitive(
        1764 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        20 as ::core::ffi::c_int,
    );
    zprimitive(
        1765 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        21 as ::core::ffi::c_int,
    );
    zprimitive(
        1766 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        22 as ::core::ffi::c_int,
    );
    zprimitive(
        1767 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        23 as ::core::ffi::c_int,
    );
    zprimitive(
        1768 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        36 as ::core::ffi::c_int,
    );
    zprimitive(
        1769 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        37 as ::core::ffi::c_int,
    );
    zprimitive(
        1770 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        38 as ::core::ffi::c_int,
    );
    zprimitive(
        1771 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        24 as ::core::ffi::c_int,
    );
    zprimitive(
        1772 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        25 as ::core::ffi::c_int,
    );
    zprimitive(
        1773 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        26 as ::core::ffi::c_int,
    );
    zprimitive(
        1774 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        28 as ::core::ffi::c_int,
    );
    zprimitive(
        1775 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        27 as ::core::ffi::c_int,
    );
    zprimitive(
        1776 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        29 as ::core::ffi::c_int,
    );
    zprimitive(
        1777 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        30 as ::core::ffi::c_int,
    );
    zprimitive(
        1778 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        31 as ::core::ffi::c_int,
    );
    zprimitive(
        1779 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        32 as ::core::ffi::c_int,
    );
    zprimitive(
        1780 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        33 as ::core::ffi::c_int,
    );
    zprimitive(
        1781 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        35 as ::core::ffi::c_int,
    );
    zprimitive(
        1782 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        34 as ::core::ffi::c_int,
    );
    zprimitive(
        1783 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        39 as ::core::ffi::c_int,
    );
    zprimitive(
        1784 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        44 as ::core::ffi::c_int,
    );
    zprimitive(
        1785 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        45 as ::core::ffi::c_int,
    );
    zprimitive(
        1786 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        46 as ::core::ffi::c_int,
    );
    zprimitive(
        1787 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        47 as ::core::ffi::c_int,
    );
    zprimitive(
        1788 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        48 as ::core::ffi::c_int,
    );
    zprimitive(
        1789 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        49 as ::core::ffi::c_int,
    );
    zprimitive(
        1790 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int as quarterword,
        50 as ::core::ffi::c_int,
    );
    zprimitive(
        2104 as ::core::ffi::c_int,
        73 as ::core::ffi::c_int as quarterword,
        29390 as ::core::ffi::c_int,
    );
    nonewcontrolsequence = true_0 as boolean;
}
#[no_mangle]
pub unsafe extern "C" fn mainbody() {
    let mut current_block: u64;
    let mut eqtb: *mut memoryword = zeqtb;
    bounddefault = 0 as ::core::ffi::c_int as integer;
    boundname = b"mem_bot\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut membot, boundname, bounddefault);
    bounddefault = 250000 as ::core::ffi::c_long as integer;
    boundname = b"main_memory\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut mainmemory, boundname, bounddefault);
    bounddefault = 0 as ::core::ffi::c_int as integer;
    boundname = b"extra_mem_top\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut extramemtop, boundname, bounddefault);
    bounddefault = 0 as ::core::ffi::c_int as integer;
    boundname = b"extra_mem_bot\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut extramembot, boundname, bounddefault);
    bounddefault = 200000 as ::core::ffi::c_long as integer;
    boundname = b"pool_size\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut poolsize, boundname, bounddefault);
    bounddefault = 75000 as ::core::ffi::c_long as integer;
    boundname = b"string_vacancies\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut stringvacancies, boundname, bounddefault);
    bounddefault = 5000 as ::core::ffi::c_int as integer;
    boundname = b"pool_free\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut poolfree, boundname, bounddefault);
    bounddefault = 15000 as ::core::ffi::c_int as integer;
    boundname = b"max_strings\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut maxstrings, boundname, bounddefault);
    bounddefault = 100 as ::core::ffi::c_int as integer;
    boundname = b"strings_free\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut stringsfree, boundname, bounddefault);
    bounddefault = 100000 as ::core::ffi::c_long as integer;
    boundname = b"font_mem_size\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut fontmemsize, boundname, bounddefault);
    bounddefault = 500 as ::core::ffi::c_int as integer;
    boundname = b"font_max\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut fontmax, boundname, bounddefault);
    bounddefault = 20000 as ::core::ffi::c_int as integer;
    boundname = b"trie_size\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut triesize, boundname, bounddefault);
    bounddefault = 659 as ::core::ffi::c_int as integer;
    boundname = b"hyph_size\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut hyphsize, boundname, bounddefault);
    bounddefault = 3000 as ::core::ffi::c_int as integer;
    boundname = b"buf_size\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut bufsize, boundname, bounddefault);
    bounddefault = 50 as ::core::ffi::c_int as integer;
    boundname = b"nest_size\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut nestsize, boundname, bounddefault);
    bounddefault = 15 as ::core::ffi::c_int as integer;
    boundname = b"max_in_open\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut maxinopen, boundname, bounddefault);
    bounddefault = 60 as ::core::ffi::c_int as integer;
    boundname = b"param_size\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut paramsize, boundname, bounddefault);
    bounddefault = 4000 as ::core::ffi::c_int as integer;
    boundname = b"save_size\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut savesize, boundname, bounddefault);
    bounddefault = 300 as ::core::ffi::c_int as integer;
    boundname = b"stack_size\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut stacksize, boundname, bounddefault);
    bounddefault = 16384 as ::core::ffi::c_int as integer;
    boundname = b"dvi_buf_size\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut dvibufsize, boundname, bounddefault);
    bounddefault = 79 as ::core::ffi::c_int as integer;
    boundname = b"error_line\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut errorline, boundname, bounddefault);
    bounddefault = 50 as ::core::ffi::c_int as integer;
    boundname = b"half_error_line\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut halferrorline, boundname, bounddefault);
    bounddefault = 79 as ::core::ffi::c_int as integer;
    boundname = b"max_print_line\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut maxprintline, boundname, bounddefault);
    bounddefault = 0 as ::core::ffi::c_int as integer;
    boundname = b"hash_extra\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut hashextra, boundname, bounddefault);
    bounddefault = 10000 as ::core::ffi::c_int as integer;
    boundname = b"expand_depth\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut expanddepth, boundname, bounddefault);
    bounddefault = 72 as ::core::ffi::c_int as integer;
    boundname = b"pk_dpi\0" as *const u8 as *const ::core::ffi::c_char as const_string;
    setupboundvariable(&raw mut pkdpi, boundname, bounddefault);
    if membot < infmembot {
        membot = infmembot as integer;
    } else if membot > supmembot {
        membot = supmembot as integer;
    }
    if mainmemory < infmainmemory {
        mainmemory = infmainmemory as integer;
    } else if mainmemory as ::core::ffi::c_long > supmainmemory {
        mainmemory = supmainmemory as integer;
    }
    if iniversion != 0 {
        extramemtop = 0 as ::core::ffi::c_int as integer;
        extramembot = 0 as ::core::ffi::c_int as integer;
    }
    if extramembot as ::core::ffi::c_long > supmainmemory {
        extramembot = supmainmemory as integer;
    }
    if extramemtop as ::core::ffi::c_long > supmainmemory {
        extramemtop = supmainmemory as integer;
    }
    memtop = (membot as ::core::ffi::c_int + mainmemory as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as integer;
    memmin = membot;
    memmax = memtop;
    if triesize < inftriesize {
        triesize = inftriesize as integer;
    } else if triesize as ::core::ffi::c_long > suptriesize {
        triesize = suptriesize as integer;
    }
    if hyphsize < infhyphsize {
        hyphsize = infhyphsize as integer;
    } else if hyphsize as ::core::ffi::c_long > suphyphsize {
        hyphsize = suphyphsize as integer;
    }
    if bufsize < infbufsize {
        bufsize = infbufsize as integer;
    } else if bufsize as ::core::ffi::c_long > supbufsize {
        bufsize = supbufsize as integer;
    }
    if nestsize < infnestsize {
        nestsize = infnestsize as integer;
    } else if nestsize > supnestsize {
        nestsize = supnestsize as integer;
    }
    if maxinopen < infmaxinopen {
        maxinopen = infmaxinopen as integer;
    } else if maxinopen > supmaxinopen {
        maxinopen = supmaxinopen as integer;
    }
    if paramsize < infparamsize {
        paramsize = infparamsize as integer;
    } else if paramsize > supparamsize {
        paramsize = supparamsize as integer;
    }
    if savesize < infsavesize {
        savesize = infsavesize as integer;
    } else if savesize as ::core::ffi::c_long > supsavesize {
        savesize = supsavesize as integer;
    }
    if stacksize < infstacksize {
        stacksize = infstacksize as integer;
    } else if stacksize > supstacksize {
        stacksize = supstacksize as integer;
    }
    if dvibufsize < infdvibufsize {
        dvibufsize = infdvibufsize as integer;
    } else if dvibufsize as ::core::ffi::c_long > supdvibufsize {
        dvibufsize = supdvibufsize as integer;
    }
    if poolsize < infpoolsize {
        poolsize = infpoolsize as integer;
    } else if poolsize as ::core::ffi::c_long > suppoolsize {
        poolsize = suppoolsize as integer;
    }
    if stringvacancies < infstringvacancies {
        stringvacancies = infstringvacancies as integer;
    } else if stringvacancies as ::core::ffi::c_long > supstringvacancies {
        stringvacancies = supstringvacancies as integer;
    }
    if poolfree < infpoolfree {
        poolfree = infpoolfree as integer;
    } else if poolfree as ::core::ffi::c_long > suppoolfree {
        poolfree = suppoolfree as integer;
    }
    if maxstrings < infmaxstrings {
        maxstrings = infmaxstrings as integer;
    } else if maxstrings as ::core::ffi::c_long > supmaxstrings {
        maxstrings = supmaxstrings as integer;
    }
    if stringsfree < infstringsfree {
        stringsfree = infstringsfree as integer;
    } else if stringsfree as ::core::ffi::c_long > supstringsfree {
        stringsfree = supstringsfree as integer;
    }
    if fontmemsize < inffontmemsize {
        fontmemsize = inffontmemsize as integer;
    } else if fontmemsize as ::core::ffi::c_long > supfontmemsize {
        fontmemsize = supfontmemsize as integer;
    }
    if fontmax < inffontmax {
        fontmax = inffontmax as integer;
    } else if fontmax > supfontmax {
        fontmax = supfontmax as integer;
    }
    if hashextra < infhashextra {
        hashextra = infhashextra as halfword;
    } else if hashextra as ::core::ffi::c_long > suphashextra {
        hashextra = suphashextra as halfword;
    }
    if objtabsize < infobjtabsize {
        objtabsize = infobjtabsize as integer;
    } else if objtabsize as ::core::ffi::c_long > supobjtabsize {
        objtabsize = supobjtabsize as integer;
    }
    if pdfmemsize < infpdfmemsize {
        pdfmemsize = infpdfmemsize as integer;
    } else if pdfmemsize as ::core::ffi::c_long > suppdfmemsize {
        pdfmemsize = suppdfmemsize as integer;
    }
    if destnamessize < infdestnamessize {
        destnamessize = infdestnamessize as integer;
    } else if destnamessize as ::core::ffi::c_long > supdestnamessize {
        destnamessize = supdestnamessize as integer;
    }
    if pkdpi < infpkdpi {
        pkdpi = infpkdpi as integer;
    } else if pkdpi > suppkdpi {
        pkdpi = suppkdpi as integer;
    }
    if errorline > 255 as ::core::ffi::c_int {
        errorline = 255 as ::core::ffi::c_int as integer;
    }
    buffer = xmalloc(
        ((bufsize as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<ASCIIcode>() as size_t),
    ) as *mut ASCIIcode;
    nest = xmalloc(
        ((nestsize as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<liststaterecord>() as size_t),
    ) as *mut liststaterecord;
    savestack = xmalloc(
        ((savesize as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<memoryword>() as size_t),
    ) as *mut memoryword;
    inputstack = xmalloc(
        ((stacksize as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<instaterecord>() as size_t),
    ) as *mut instaterecord;
    inputfile = xmalloc(
        ((maxinopen as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<alphafile>() as size_t),
    ) as *mut alphafile;
    linestack = xmalloc(
        ((maxinopen as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
    ) as *mut integer;
    eofseen = xmalloc(
        ((maxinopen as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<boolean>() as size_t),
    ) as *mut boolean;
    grpstack = xmalloc(
        ((maxinopen as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<savepointer>() as size_t),
    ) as *mut savepointer;
    ifstack = xmalloc(
        ((maxinopen as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<halfword>() as size_t),
    ) as *mut halfword;
    sourcefilenamestack = xmalloc(
        ((maxinopen as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<strnumber>() as size_t),
    ) as *mut strnumber;
    fullsourcefilenamestack = xmalloc(
        ((maxinopen as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<strnumber>() as size_t),
    ) as *mut strnumber;
    paramstack = xmalloc(
        ((paramsize as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<halfword>() as size_t),
    ) as *mut halfword;
    dvibuf = xmalloc(
        ((dvibufsize as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<eightbits>() as size_t),
    ) as *mut eightbits;
    hyphword = xmalloc(
        ((hyphsize as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<strnumber>() as size_t),
    ) as *mut strnumber;
    hyphlist = xmalloc(
        ((hyphsize as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<halfword>() as size_t),
    ) as *mut halfword;
    hyphlink = xmalloc(
        ((hyphsize as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<hyphpointer>() as size_t),
    ) as *mut hyphpointer;
    objtab = xmalloc(
        ((1000 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<objentry>() as size_t),
    ) as *mut objentry;
    pdfmem = xmalloc(
        ((10000 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
    ) as *mut integer;
    destnames = xmalloc(
        ((1000 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<destnameentry>() as size_t),
    ) as *mut destnameentry;
    pdfopbuf = xmalloc(
        ((16384 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<eightbits>() as size_t),
    ) as *mut eightbits;
    pdfosbuf = xmalloc(
        ((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<eightbits>() as size_t),
    ) as *mut eightbits;
    pdfosobjnum = xmalloc(
        ((100 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
    ) as *mut integer;
    pdfosobjoff = xmalloc(
        ((100 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
    ) as *mut integer;
    if iniversion != 0 {
        yzmem = xmalloc(
            ((memtop as ::core::ffi::c_int - membot as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<memoryword>() as size_t),
        ) as *mut memoryword;
        zmem = yzmem.offset(-(membot as isize));
        eqtbtop = 30192 as halfword + hashextra;
        if hashextra == 0 as ::core::ffi::c_int {
            hashtop = 26627 as ::core::ffi::c_int as halfword;
        } else {
            hashtop = eqtbtop;
        }
        yhash = xmalloc(
            ((1 as ::core::ffi::c_int + hashtop as ::core::ffi::c_int - 514 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<twohalves>() as size_t),
        ) as *mut twohalves;
        hash = yhash.offset(-(hashoffset as isize));
        (*hash.offset(514 as ::core::ffi::c_int as isize)).v.LH =
            0 as ::core::ffi::c_int as halfword;
        (*hash.offset(514 as ::core::ffi::c_int as isize)).v.RH =
            0 as ::core::ffi::c_int as halfword;
        let mut for_end: integer = 0;
        hashused = 515 as ::core::ffi::c_int as halfword;
        for_end = hashtop as integer;
        if hashused <= for_end {
            loop {
                *hash.offset(hashused as isize) = *hash.offset(514 as ::core::ffi::c_int as isize);
                let fresh0 = hashused;
                hashused = hashused + 1;
                if !(fresh0 < for_end) {
                    break;
                }
            }
        }
        zeqtb = xmalloc(
            ((eqtbtop as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<memoryword>() as size_t),
        ) as *mut memoryword;
        eqtb = zeqtb;
        strstart = xmalloc(
            ((maxstrings as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<poolpointer>() as size_t),
        ) as *mut poolpointer;
        strpool = xmalloc(
            ((poolsize as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<packedASCIIcode>() as size_t),
        ) as *mut packedASCIIcode;
        fontinfo = xmalloc(
            ((fontmemsize as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<fmemoryword>() as size_t),
        ) as *mut fmemoryword;
    }
    history = 3 as ::core::ffi::c_uchar;
    if readyalready as ::core::ffi::c_long == 314159 as ::core::ffi::c_long {
        current_block = 11301457205589090766;
    } else {
        bad = 0 as ::core::ffi::c_int as integer;
        if halferrorline < 30 as ::core::ffi::c_int
            || halferrorline > errorline as ::core::ffi::c_int - 15 as ::core::ffi::c_int
        {
            bad = 1 as ::core::ffi::c_int as integer;
        }
        if maxprintline < 60 as ::core::ffi::c_int {
            bad = 2 as ::core::ffi::c_int as integer;
        }
        if dvibufsize as ::core::ffi::c_int % 8 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            bad = 3 as ::core::ffi::c_int as integer;
        }
        if membot as ::core::ffi::c_int + 1100 as ::core::ffi::c_int > memtop {
            bad = 4 as ::core::ffi::c_int as integer;
        }
        if 8501 as ::core::ffi::c_int > 15000 as ::core::ffi::c_int {
            bad = 5 as ::core::ffi::c_int as integer;
        }
        if maxinopen >= 128 as ::core::ffi::c_int {
            bad = 6 as ::core::ffi::c_int as integer;
        }
        if memtop < 267 as ::core::ffi::c_int {
            bad = 7 as ::core::ffi::c_int as integer;
        }
        if memmin != membot || memmax != memtop {
            bad = 10 as ::core::ffi::c_int as integer;
        }
        if memmin > membot || memmax < memtop {
            bad = 10 as ::core::ffi::c_int as integer;
        }
        if 0 as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            || (255 as ::core::ffi::c_int) < 127 as ::core::ffi::c_int
        {
            bad = 11 as ::core::ffi::c_int as integer;
        }
        if -(268435455 as ::core::ffi::c_long) > 0 as ::core::ffi::c_long
            || (268435455 as ::core::ffi::c_long) < 32767 as ::core::ffi::c_long
        {
            bad = 12 as ::core::ffi::c_int as integer;
        }
        if (0 as ::core::ffi::c_long) < -(268435455 as ::core::ffi::c_long)
            || 255 as ::core::ffi::c_long > 268435455 as ::core::ffi::c_long
        {
            bad = 13 as ::core::ffi::c_int as integer;
        }
        if membot as ::core::ffi::c_long - supmainmemory < -(268435455 as ::core::ffi::c_long)
            || memtop as ::core::ffi::c_long + supmainmemory >= 268435455 as ::core::ffi::c_long
        {
            bad = 14 as ::core::ffi::c_int as integer;
        }
        if (9000 as ::core::ffi::c_long) < -(268435455 as ::core::ffi::c_long)
            || 9000 as ::core::ffi::c_long > 268435455 as ::core::ffi::c_long
        {
            bad = 15 as ::core::ffi::c_int as integer;
        }
        if fontmax > 9000 as ::core::ffi::c_int {
            bad = 16 as ::core::ffi::c_int as integer;
        }
        if savesize as ::core::ffi::c_long > 268435455 as ::core::ffi::c_long
            || maxstrings as ::core::ffi::c_long > 268435455 as ::core::ffi::c_long
        {
            bad = 17 as ::core::ffi::c_int as integer;
        }
        if bufsize as ::core::ffi::c_long > 268435455 as ::core::ffi::c_long {
            bad = 18 as ::core::ffi::c_int as integer;
        }
        if (255 as ::core::ffi::c_int) < 255 as ::core::ffi::c_int {
            bad = 19 as ::core::ffi::c_int as integer;
        }
        if 34287 as ::core::ffi::c_long + hashextra as ::core::ffi::c_long
            > 268435455 as ::core::ffi::c_long
        {
            bad = 21 as ::core::ffi::c_int as integer;
        }
        if hashoffset < 0 as ::core::ffi::c_int || hashoffset > 514 as ::core::ffi::c_int {
            bad = 42 as ::core::ffi::c_int as integer;
        }
        if formatdefaultlength > maxint {
            bad = 31 as ::core::ffi::c_int as integer;
        }
        if (2 as ::core::ffi::c_long * 268435455 as ::core::ffi::c_long)
            < (memtop - memmin) as ::core::ffi::c_long
        {
            bad = 41 as ::core::ffi::c_int as integer;
        }
        if bad > 0 as ::core::ffi::c_int {
            fprintf(
                __stdoutp,
                b"%s%s%ld\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"Ouch---my internal constants have been clobbered!\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"---case \0" as *const u8 as *const ::core::ffi::c_char,
                bad as ::core::ffi::c_long,
            );
            current_block = 10212330169448209891;
        } else {
            initialize();
            if iniversion != 0 {
                if getstringsstarted() == 0 {
                    current_block = 10212330169448209891;
                } else {
                    initprim();
                    initstrptr = strptr;
                    initpoolptr = poolptr as poolpointer;
                    fixdateandtime();
                    current_block = 16286683003977321678;
                }
            } else {
                current_block = 16286683003977321678;
            }
            match current_block {
                10212330169448209891 => {}
                _ => {
                    readyalready = 314159 as ::core::ffi::c_long as integer;
                    current_block = 11301457205589090766;
                }
            }
        }
    }
    match current_block {
        11301457205589090766 => {
            selector = 17 as ::core::ffi::c_uchar;
            tally = 0 as ::core::ffi::c_int as integer;
            termoffset = 0 as ::core::ffi::c_int as integer;
            fileoffset = 0 as ::core::ffi::c_int as integer;
            if srcspecialsp != 0 || filelineerrorstylep != 0 || parsefirstlinep != 0 {
                fprintf(
                    __stdoutp,
                    b"%s%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"This is pdfTeX, Version 3.141592653\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"-2.6\0" as *const u8 as *const ::core::ffi::c_char,
                    b"-1.40.29\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
                fprintf(
                    __stdoutp,
                    b"%s%s%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"This is pdfTeX, Version 3.141592653\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"-2.6\0" as *const u8 as *const ::core::ffi::c_char,
                    b"-1.40.29\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            fputs(versionstring, __stdoutp);
            if formatident == 0 as ::core::ffi::c_int {
                fprintf(
                    __stdoutp,
                    b"%s%s%c\n\0" as *const u8 as *const ::core::ffi::c_char,
                    b" (preloaded format=\0" as *const u8 as *const ::core::ffi::c_char,
                    dump_name,
                    ')' as i32,
                );
            } else {
                zslowprint(formatident);
                println();
            }
            if shellenabledp != 0 {
                putc(' ' as i32, __stdoutp);
                if restrictedshell != 0 {
                    fputs(
                        b"restricted \0" as *const u8 as *const ::core::ffi::c_char,
                        __stdoutp,
                    );
                }
                fprintf(
                    __stdoutp,
                    b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    b"\\write18 enabled.\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if srcspecialsp != 0 {
                fprintf(
                    __stdoutp,
                    b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    b" Source specials enabled.\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            if !translate_filename.is_null() {
                fputs(
                    b" (\0" as *const u8 as *const ::core::ffi::c_char,
                    __stdoutp,
                );
                fputs(translate_filename as *const ::core::ffi::c_char, __stdoutp);
                putc(')' as i32, __stdoutp);
                putc('\n' as i32, __stdoutp);
            }
            fflush(__stdoutp);
            jobname = 0 as ::core::ffi::c_int as strnumber;
            nameinprogress = false_0 as boolean;
            logopened = false_0 as boolean;
            outputfilename = 0 as ::core::ffi::c_int as strnumber;
            inputptr = 0 as ::core::ffi::c_int as integer;
            maxinstack = 0 as ::core::ffi::c_int as integer;
            *sourcefilenamestack.offset(0 as ::core::ffi::c_int as isize) =
                0 as ::core::ffi::c_int as strnumber;
            *fullsourcefilenamestack.offset(0 as ::core::ffi::c_int as isize) =
                0 as ::core::ffi::c_int as strnumber;
            inopen = 0 as ::core::ffi::c_int as integer;
            openparens = 0 as ::core::ffi::c_int as integer;
            maxbufstack = 0 as ::core::ffi::c_int as integer;
            *grpstack.offset(0 as ::core::ffi::c_int as isize) =
                0 as ::core::ffi::c_int as savepointer;
            *ifstack.offset(0 as ::core::ffi::c_int as isize) =
                -(268435455 as ::core::ffi::c_long) as halfword;
            paramptr = 0 as ::core::ffi::c_int as integer;
            maxparamstack = 0 as ::core::ffi::c_int as integer;
            first = bufsize;
            loop {
                *buffer.offset(first as isize) = 0 as ASCIIcode;
                first -= 1;
                if first == 0 as ::core::ffi::c_int {
                    break;
                }
            }
            *buffer.offset(0 as ::core::ffi::c_int as isize) = 0 as ASCIIcode;
            scannerstatus = 0 as ::core::ffi::c_uchar;
            warningindex = -(268435455 as ::core::ffi::c_long) as halfword;
            first = 1 as ::core::ffi::c_int as integer;
            curinput.statefield = 33 as quarterword;
            curinput.startfield = 1 as ::core::ffi::c_int as halfword;
            curinput.indexfield = 0 as quarterword;
            line = 0 as ::core::ffi::c_int as integer;
            curinput.namefield = 0 as ::core::ffi::c_int as halfword;
            forceeof = false_0 as boolean;
            alignstate = 1000000 as ::core::ffi::c_long as integer;
            if !(initterminal() == 0) {
                curinput.limitfield = last as halfword;
                first = (last as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as integer;
                if (etexp != 0
                    || *buffer.offset(curinput.locfield as isize) as ::core::ffi::c_int
                        == 42 as ::core::ffi::c_int)
                    && formatident == 1710 as ::core::ffi::c_int
                {
                    nonewcontrolsequence = false_0 as boolean;
                    zprimitive(
                        1993 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        3 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        1994 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        20 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        883 as ::core::ffi::c_int,
                        111 as ::core::ffi::c_int as quarterword,
                        5 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        1996 as ::core::ffi::c_int,
                        72 as ::core::ffi::c_int as quarterword,
                        27172 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        1997 as ::core::ffi::c_int,
                        73 as ::core::ffi::c_int as quarterword,
                        29379 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        1998 as ::core::ffi::c_int,
                        73 as ::core::ffi::c_int as quarterword,
                        29380 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        1999 as ::core::ffi::c_int,
                        73 as ::core::ffi::c_int as quarterword,
                        29381 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2000 as ::core::ffi::c_int,
                        73 as ::core::ffi::c_int as quarterword,
                        29382 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2001 as ::core::ffi::c_int,
                        73 as ::core::ffi::c_int as quarterword,
                        29383 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2002 as ::core::ffi::c_int,
                        73 as ::core::ffi::c_int as quarterword,
                        29384 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2003 as ::core::ffi::c_int,
                        73 as ::core::ffi::c_int as quarterword,
                        29385 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2004 as ::core::ffi::c_int,
                        73 as ::core::ffi::c_int as quarterword,
                        29386 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2005 as ::core::ffi::c_int,
                        73 as ::core::ffi::c_int as quarterword,
                        29387 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2006 as ::core::ffi::c_int,
                        73 as ::core::ffi::c_int as quarterword,
                        29388 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2021 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        21 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2022 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        22 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2023 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        23 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2024 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        24 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2025 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        25 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2026 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        28 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2027 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        29 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2028 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        30 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2029 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        31 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2030 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        32 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2031 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        33 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2032 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        34 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2033 as ::core::ffi::c_int,
                        19 as ::core::ffi::c_int as quarterword,
                        4 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2035 as ::core::ffi::c_int,
                        19 as ::core::ffi::c_int as quarterword,
                        5 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2036 as ::core::ffi::c_int,
                        112 as ::core::ffi::c_int as quarterword,
                        1 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2037 as ::core::ffi::c_int,
                        112 as ::core::ffi::c_int as quarterword,
                        5 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2038 as ::core::ffi::c_int,
                        19 as ::core::ffi::c_int as quarterword,
                        6 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2042 as ::core::ffi::c_int,
                        82 as ::core::ffi::c_int as quarterword,
                        2 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        1300 as ::core::ffi::c_int,
                        49 as ::core::ffi::c_int as quarterword,
                        1 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2046 as ::core::ffi::c_int,
                        73 as ::core::ffi::c_int as quarterword,
                        29389 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2047 as ::core::ffi::c_int,
                        33 as ::core::ffi::c_int as quarterword,
                        6 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2048 as ::core::ffi::c_int,
                        33 as ::core::ffi::c_int as quarterword,
                        7 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2049 as ::core::ffi::c_int,
                        33 as ::core::ffi::c_int as quarterword,
                        10 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2050 as ::core::ffi::c_int,
                        33 as ::core::ffi::c_int as quarterword,
                        11 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2059 as ::core::ffi::c_int,
                        107 as ::core::ffi::c_int as quarterword,
                        2 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2061 as ::core::ffi::c_int,
                        96 as ::core::ffi::c_int as quarterword,
                        1 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        940 as ::core::ffi::c_int,
                        105 as ::core::ffi::c_int as quarterword,
                        1 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2062 as ::core::ffi::c_int,
                        108 as ::core::ffi::c_int as quarterword,
                        17 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2063 as ::core::ffi::c_int,
                        108 as ::core::ffi::c_int as quarterword,
                        18 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2064 as ::core::ffi::c_int,
                        108 as ::core::ffi::c_int as quarterword,
                        19 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2065 as ::core::ffi::c_int,
                        108 as ::core::ffi::c_int as quarterword,
                        20 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2066 as ::core::ffi::c_int,
                        108 as ::core::ffi::c_int as quarterword,
                        22 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2067 as ::core::ffi::c_int,
                        108 as ::core::ffi::c_int as quarterword,
                        23 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        1615 as ::core::ffi::c_int,
                        93 as ::core::ffi::c_int as quarterword,
                        8 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2073 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        39 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2074 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        40 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2075 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        41 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2076 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        42 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2080 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        26 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2081 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        27 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2082 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        35 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2083 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        36 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2084 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        37 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2085 as ::core::ffi::c_int,
                        70 as ::core::ffi::c_int as quarterword,
                        38 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2086 as ::core::ffi::c_int,
                        18 as ::core::ffi::c_int as quarterword,
                        5 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2087 as ::core::ffi::c_int,
                        113 as ::core::ffi::c_int as quarterword,
                        5 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2088 as ::core::ffi::c_int,
                        113 as ::core::ffi::c_int as quarterword,
                        6 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2089 as ::core::ffi::c_int,
                        113 as ::core::ffi::c_int as quarterword,
                        7 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2090 as ::core::ffi::c_int,
                        113 as ::core::ffi::c_int as quarterword,
                        8 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2091 as ::core::ffi::c_int,
                        113 as ::core::ffi::c_int as quarterword,
                        9 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2095 as ::core::ffi::c_int,
                        24 as ::core::ffi::c_int as quarterword,
                        2 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2096 as ::core::ffi::c_int,
                        24 as ::core::ffi::c_int as quarterword,
                        3 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2097 as ::core::ffi::c_int,
                        84 as ::core::ffi::c_int as quarterword,
                        27429 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2098 as ::core::ffi::c_int,
                        84 as ::core::ffi::c_int as quarterword,
                        27430 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2099 as ::core::ffi::c_int,
                        84 as ::core::ffi::c_int as quarterword,
                        27431 as ::core::ffi::c_int,
                    );
                    zprimitive(
                        2100 as ::core::ffi::c_int,
                        84 as ::core::ffi::c_int as quarterword,
                        27432 as ::core::ffi::c_int,
                    );
                    if *buffer.offset(curinput.locfield as isize) as ::core::ffi::c_int
                        == 42 as ::core::ffi::c_int
                    {
                        curinput.locfield += 1;
                    }
                    eTeXmode = 1 as ::core::ffi::c_uchar;
                    maxregnum = 32767 as ::core::ffi::c_int as halfword;
                    maxreghelpline = 2092 as ::core::ffi::c_int as strnumber;
                }
                if nonewcontrolsequence == 0 {
                    nonewcontrolsequence = true_0 as boolean;
                    current_block = 6762054512782224738;
                } else if formatident == 0 as ::core::ffi::c_int
                    || *buffer.offset(curinput.locfield as isize) as ::core::ffi::c_int
                        == 38 as ::core::ffi::c_int
                    || dumpline != 0
                {
                    if formatident != 0 as ::core::ffi::c_int {
                        initialize();
                    }
                    if openfmtfile() == 0 {
                        current_block = 10212330169448209891;
                    } else if loadfmtfile() == 0 {
                        gzclose(fmtfile as gzFile);
                        current_block = 10212330169448209891;
                    } else {
                        gzclose(fmtfile as gzFile);
                        eqtb = zeqtb;
                        while curinput.locfield < curinput.limitfield
                            && *buffer.offset(curinput.locfield as isize) as ::core::ffi::c_int
                                == 32 as ::core::ffi::c_int
                        {
                            curinput.locfield += 1;
                        }
                        current_block = 6762054512782224738;
                    }
                } else {
                    current_block = 6762054512782224738;
                }
                match current_block {
                    10212330169448209891 => {}
                    _ => {
                        if pdfoutputoption != 0 as ::core::ffi::c_int {
                            (*eqtb.offset(29342 as ::core::ffi::c_int as isize)).u.CINT =
                                pdfoutputvalue;
                        }
                        if pdfdraftmodeoption != 0 as ::core::ffi::c_int {
                            (*eqtb.offset(29368 as ::core::ffi::c_int as isize)).u.CINT =
                                pdfdraftmodevalue;
                        }
                        pdfinitmapfile(b"pdftex.map\0" as *const u8 as const_string);
                        if eTeXmode as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
                            fprintf(
                                __stdoutp,
                                b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                                b"entering extended mode\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        }
                        if (*eqtb.offset(29325 as ::core::ffi::c_int as isize)).u.CINT
                            < 0 as ::core::ffi::c_int
                            || (*eqtb.offset(29325 as ::core::ffi::c_int as isize)).u.CINT
                                > 255 as ::core::ffi::c_int
                        {
                            curinput.limitfield -= 1;
                        } else {
                            *buffer.offset(curinput.limitfield as isize) =
                                (*eqtb.offset(29325 as ::core::ffi::c_int as isize)).u.CINT
                                    as ASCIIcode;
                        }
                        if mltexenabledp != 0 {
                            fprintf(
                                __stdoutp,
                                b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                                b"MLTeX v2.2 enabled\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                        }
                        if enctexenabledp != 0 {
                            fputs(
                                b" encTeX v. Jun. 2004\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                __stdoutp,
                            );
                            fprintf(
                                __stdoutp,
                                b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                                b", reencoding enabled.\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            if !translate_filename.is_null() {
                                fprintf(
                                    __stdoutp,
                                    b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                                    b" (\\xordcode, \\xchrcode, \\xprncode overridden by TCX)\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                            }
                        }
                        fixdateandtime();
                        if trienotready != 0 {
                            trietrl = xmalloc(
                                ((triesize as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<triepointer>() as size_t),
                            ) as *mut triepointer;
                            trietro = xmalloc(
                                ((triesize as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<triepointer>() as size_t),
                            ) as *mut triepointer;
                            trietrc = xmalloc(
                                ((triesize as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<quarterword>() as size_t),
                            ) as *mut quarterword;
                            triec = xmalloc(
                                ((triesize as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<packedASCIIcode>() as size_t
                                    ),
                            ) as *mut packedASCIIcode;
                            trieo = xmalloc(
                                ((triesize as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<trieopcode>() as size_t),
                            ) as *mut trieopcode;
                            triel = xmalloc(
                                ((triesize as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<triepointer>() as size_t),
                            ) as *mut triepointer;
                            trier = xmalloc(
                                ((triesize as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<triepointer>() as size_t),
                            ) as *mut triepointer;
                            triehash = xmalloc(
                                ((triesize as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<triepointer>() as size_t),
                            ) as *mut triepointer;
                            trietaken = xmalloc(
                                ((triesize as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<boolean>() as size_t),
                            ) as *mut boolean;
                            *triel.offset(0 as ::core::ffi::c_int as isize) =
                                0 as ::core::ffi::c_int as triepointer;
                            *triec.offset(0 as ::core::ffi::c_int as isize) = 0 as packedASCIIcode;
                            trieptr = 0 as ::core::ffi::c_int as triepointer;
                            *trier.offset(0 as ::core::ffi::c_int as isize) =
                                0 as ::core::ffi::c_int as triepointer;
                            hyphstart = 0 as ::core::ffi::c_int as triepointer;
                            fontcheck = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<fourquarters>() as size_t),
                            ) as *mut fourquarters;
                            fontsize = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<scaled>() as size_t),
                            ) as *mut scaled;
                            fontdsize = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<scaled>() as size_t),
                            ) as *mut scaled;
                            fontparams = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<fontindex>() as size_t),
                            ) as *mut fontindex;
                            fontname = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<strnumber>() as size_t),
                            ) as *mut strnumber;
                            fontarea = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<strnumber>() as size_t),
                            ) as *mut strnumber;
                            fontbc = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<eightbits>() as size_t),
                            ) as *mut eightbits;
                            fontec = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<eightbits>() as size_t),
                            ) as *mut eightbits;
                            fontglue = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<halfword>() as size_t),
                            ) as *mut halfword;
                            hyphenchar = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            skewchar = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            bcharlabel = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<fontindex>() as size_t),
                            ) as *mut fontindex;
                            fontbchar = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<ninebits>() as size_t),
                            ) as *mut ninebits;
                            fontfalsebchar = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<ninebits>() as size_t),
                            ) as *mut ninebits;
                            charbase = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            widthbase = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            heightbase = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            depthbase = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            italicbase = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            ligkernbase = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            kernbase = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            extenbase = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            parambase = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            pdfcharused =
                                xmalloc(
                                    ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                        as size_t)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<charusedarray>() as size_t
                                        ),
                                ) as *mut charusedarray;
                            pdffontsize = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<scaled>() as size_t),
                            ) as *mut scaled;
                            pdffontnum = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            pdffontmap = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<fmentryptr>() as size_t),
                            ) as *mut fmentryptr;
                            pdffonttype = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<eightbits>() as size_t),
                            ) as *mut eightbits;
                            pdffontattr = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<strnumber>() as size_t),
                            ) as *mut strnumber;
                            pdffontblink = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<internalfontnumber>() as size_t
                                    ),
                            ) as *mut internalfontnumber;
                            pdffontelink = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<internalfontnumber>() as size_t
                                    ),
                            ) as *mut internalfontnumber;
                            pdffonthasspacechar = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<boolean>() as size_t),
                            ) as *mut boolean;
                            pdffontstretch = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            pdffontshrink = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            pdffontstep = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            pdffontexpandratio = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            pdffontautoexpand = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<boolean>() as size_t),
                            ) as *mut boolean;
                            pdffontlpbase = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            pdffontrpbase = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            pdffontefbase = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            pdffontknbsbase = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            pdffontstbsbase = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            pdffontshbsbase = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            pdffontknbcbase = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            pdffontknacbase = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            vfpacketbase = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            vfdefaultfont = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<internalfontnumber>() as size_t
                                    ),
                            )
                                as *mut internalfontnumber;
                            vflocalfontnum = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<internalfontnumber>() as size_t
                                    ),
                            )
                                as *mut internalfontnumber;
                            vfefnts = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<integer>() as size_t),
                            ) as *mut integer;
                            vfifnts = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<internalfontnumber>() as size_t
                                    ),
                            ) as *mut internalfontnumber;
                            pdffontnobuiltintounicode = xmalloc(
                                ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                    as size_t)
                                    .wrapping_mul(::core::mem::size_of::<boolean>() as size_t),
                            )
                                as *mut boolean;
                            let mut for_end_0: integer = 0;
                            fontk = 0 as ::core::ffi::c_int as integer;
                            for_end_0 = fontmax;
                            if fontk <= for_end_0 {
                                loop {
                                    let mut for_end_1: integer = 0;
                                    k = 0 as ::core::ffi::c_uchar;
                                    for_end_1 = 31 as ::core::ffi::c_int as integer;
                                    if k as ::core::ffi::c_int <= for_end_1 {
                                        loop {
                                            (*pdfcharused.offset(fontk as isize))[k as usize] =
                                                0 as eightbits;
                                            let fresh1 = k;
                                            k = k.wrapping_add(1);
                                            if !((fresh1 as ::core::ffi::c_int) < for_end_1) {
                                                break;
                                            }
                                        }
                                    }
                                    *pdffontsize.offset(fontk as isize) =
                                        0 as ::core::ffi::c_int as scaled;
                                    *pdffontnum.offset(fontk as isize) =
                                        0 as ::core::ffi::c_int as integer;
                                    let ref mut fresh2 = *pdffontmap.offset(fontk as isize);
                                    *fresh2 = ::core::ptr::null_mut::<integer>();
                                    *pdffonttype.offset(fontk as isize) = 0 as eightbits;
                                    *pdffontattr.offset(fontk as isize) =
                                        345 as ::core::ffi::c_int as strnumber;
                                    *pdffontblink.offset(fontk as isize) =
                                        0 as ::core::ffi::c_int as internalfontnumber;
                                    *pdffontelink.offset(fontk as isize) =
                                        0 as ::core::ffi::c_int as internalfontnumber;
                                    *pdffonthasspacechar.offset(fontk as isize) =
                                        false_0 as boolean;
                                    *pdffontstretch.offset(fontk as isize) =
                                        0 as ::core::ffi::c_int as integer;
                                    *pdffontshrink.offset(fontk as isize) =
                                        0 as ::core::ffi::c_int as integer;
                                    *pdffontstep.offset(fontk as isize) =
                                        0 as ::core::ffi::c_int as integer;
                                    *pdffontexpandratio.offset(fontk as isize) =
                                        0 as ::core::ffi::c_int as integer;
                                    *pdffontautoexpand.offset(fontk as isize) = false_0 as boolean;
                                    *pdffontlpbase.offset(fontk as isize) =
                                        0 as ::core::ffi::c_int as integer;
                                    *pdffontrpbase.offset(fontk as isize) =
                                        0 as ::core::ffi::c_int as integer;
                                    *pdffontefbase.offset(fontk as isize) =
                                        0 as ::core::ffi::c_int as integer;
                                    *pdffontknbsbase.offset(fontk as isize) =
                                        0 as ::core::ffi::c_int as integer;
                                    *pdffontstbsbase.offset(fontk as isize) =
                                        0 as ::core::ffi::c_int as integer;
                                    *pdffontshbsbase.offset(fontk as isize) =
                                        0 as ::core::ffi::c_int as integer;
                                    *pdffontknbcbase.offset(fontk as isize) =
                                        0 as ::core::ffi::c_int as integer;
                                    *pdffontknacbase.offset(fontk as isize) =
                                        0 as ::core::ffi::c_int as integer;
                                    *pdffontnobuiltintounicode.offset(fontk as isize) =
                                        false_0 as boolean;
                                    let fresh3 = fontk;
                                    fontk = fontk + 1;
                                    if !(fresh3 < for_end_0) {
                                        break;
                                    }
                                }
                            }
                            fontptr = 0 as ::core::ffi::c_int as internalfontnumber;
                            fmemptr = 7 as ::core::ffi::c_int as fontindex;
                            makepdftexbanner();
                            *fontname.offset(0 as ::core::ffi::c_int as isize) =
                                969 as ::core::ffi::c_int as strnumber;
                            *fontarea.offset(0 as ::core::ffi::c_int as isize) =
                                345 as ::core::ffi::c_int as strnumber;
                            *hyphenchar.offset(0 as ::core::ffi::c_int as isize) =
                                45 as ::core::ffi::c_int as integer;
                            *skewchar.offset(0 as ::core::ffi::c_int as isize) =
                                -(1 as ::core::ffi::c_int) as integer;
                            *bcharlabel.offset(0 as ::core::ffi::c_int as isize) =
                                0 as ::core::ffi::c_int as fontindex;
                            *fontbchar.offset(0 as ::core::ffi::c_int as isize) = 256 as ninebits;
                            *fontfalsebchar.offset(0 as ::core::ffi::c_int as isize) =
                                256 as ninebits;
                            *fontbc.offset(0 as ::core::ffi::c_int as isize) = 1 as eightbits;
                            *fontec.offset(0 as ::core::ffi::c_int as isize) = 0 as eightbits;
                            *fontsize.offset(0 as ::core::ffi::c_int as isize) =
                                0 as ::core::ffi::c_int as scaled;
                            *fontdsize.offset(0 as ::core::ffi::c_int as isize) =
                                0 as ::core::ffi::c_int as scaled;
                            *charbase.offset(0 as ::core::ffi::c_int as isize) =
                                0 as ::core::ffi::c_int as integer;
                            *widthbase.offset(0 as ::core::ffi::c_int as isize) =
                                0 as ::core::ffi::c_int as integer;
                            *heightbase.offset(0 as ::core::ffi::c_int as isize) =
                                0 as ::core::ffi::c_int as integer;
                            *depthbase.offset(0 as ::core::ffi::c_int as isize) =
                                0 as ::core::ffi::c_int as integer;
                            *italicbase.offset(0 as ::core::ffi::c_int as isize) =
                                0 as ::core::ffi::c_int as integer;
                            *ligkernbase.offset(0 as ::core::ffi::c_int as isize) =
                                0 as ::core::ffi::c_int as integer;
                            *kernbase.offset(0 as ::core::ffi::c_int as isize) =
                                0 as ::core::ffi::c_int as integer;
                            *extenbase.offset(0 as ::core::ffi::c_int as isize) =
                                0 as ::core::ffi::c_int as integer;
                            *fontglue.offset(0 as ::core::ffi::c_int as isize) =
                                -(268435455 as ::core::ffi::c_long) as halfword;
                            *fontparams.offset(0 as ::core::ffi::c_int as isize) =
                                7 as ::core::ffi::c_int as fontindex;
                            *parambase.offset(0 as ::core::ffi::c_int as isize) =
                                -(1 as ::core::ffi::c_int) as integer;
                            let mut for_end_2: integer = 0;
                            fontk = 0 as ::core::ffi::c_int as integer;
                            for_end_2 = 6 as ::core::ffi::c_int as integer;
                            if fontk <= for_end_2 {
                                loop {
                                    (*fontinfo.offset(fontk as isize)).u.CINT =
                                        0 as ::core::ffi::c_int as integer;
                                    let fresh4 = fontk;
                                    fontk = fontk + 1;
                                    if !(fresh4 < for_end_2) {
                                        break;
                                    }
                                }
                            }
                        }
                        fontused = xmalloc(
                            ((fontmax as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
                                .wrapping_mul(::core::mem::size_of::<boolean>() as size_t),
                        ) as *mut boolean;
                        let mut for_end_3: integer = 0;
                        fontk = 0 as ::core::ffi::c_int as integer;
                        for_end_3 = fontmax;
                        if fontk <= for_end_3 {
                            loop {
                                *fontused.offset(fontk as isize) = false_0 as boolean;
                                let fresh5 = fontk;
                                fontk = fontk + 1;
                                if !(fresh5 < for_end_3) {
                                    break;
                                }
                            }
                        }
                        randomseed = ((microseconds as ::core::ffi::c_int
                            * 1000 as ::core::ffi::c_int)
                            as ::core::ffi::c_long
                            + epochseconds as ::core::ffi::c_long % 1000000 as ::core::ffi::c_long)
                            as scaled;
                        zinitrandoms(randomseed);
                        magicoffset = (*strstart.offset(1315 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            - 9 as ::core::ffi::c_int * 16 as ::core::ffi::c_int)
                            as integer;
                        if interaction as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            selector = 16 as ::core::ffi::c_uchar;
                        } else {
                            selector = 17 as ::core::ffi::c_uchar;
                        }
                        if curinput.locfield < curinput.limitfield
                            && (*eqtb.offset(
                                (27741 as ::core::ffi::c_int
                                    + *buffer.offset(curinput.locfield as isize)
                                        as ::core::ffi::c_int)
                                    as isize,
                            ))
                            .hh
                            .v
                            .RH != 0 as ::core::ffi::c_int
                        {
                            startinput();
                        }
                        history = 0 as ::core::ffi::c_uchar;
                        synctexinitcommand();
                        maincontrol();
                        finalcleanup();
                        closefilesandterminate();
                    }
                }
            }
        }
        _ => {}
    }
    fflush(__stdoutp);
    readyalready = 0 as ::core::ffi::c_int as integer;
    if history as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        && history as ::core::ffi::c_int != 1 as ::core::ffi::c_int
    {
        uexit(1 as ::core::ffi::c_int);
    } else {
        uexit(0 as ::core::ffi::c_int);
    };
}
pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
