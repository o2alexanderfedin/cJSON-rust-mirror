#![allow(unused_imports, dead_code)]

mod src;
mod tests;
use crate::tests::main::__main_inner;

pub(crate) type DarwinSizeT = u64;

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *const i8) -> i32 {
    let __r: Result<(), i32> = __main_inner(argc, argv);
    if __r.is_ok() {
        return 0;
    }
    return __r.unwrap_err();
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub(crate) struct SFILE {
    pub(crate) _opaque: [u8; 0],
}

pub(crate) type FILE = SFILE;

extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32) -> bool;
    fn __builtin_object_size(_: *const (), _: i32) -> u64;
    fn __builtin___memset_chk(_: *mut (), _: i32, _: u64, _: u64) -> *mut ();
    fn pow(_: f64, _: f64) -> f64;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64) -> i32;
    fn __builtin___memcpy_chk(_: *mut (), _: *const (), _: u64, _: u64) -> *mut ();
    fn __builtin___strcpy_chk(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn fabs(_: f64) -> f64;
    fn __builtin___sprintf_chk(_: *mut i8, _: i32, _: u64, _: *const i8, ...) -> i32;
    fn floor(_: f64) -> f64;
    fn strchr(__s: *const i8, __c: i32) -> *mut i8;
    fn strlen(__s: *const i8) -> u64;
    fn tolower(_c: i32) -> i32;
    fn __assert_rtn(_: *const i8, _: *const i8, _: i32, _: *const i8) -> ();
    fn printf(_: *const i8, ...) -> i32;
    fn free(_: *mut ()) -> ();
    fn fopen(__filename: *const i8, __mode: *const i8) -> *mut FILE;
    fn fseek(_: *mut FILE, _: i64, _: i32) -> i32;
    fn ftell(_: *mut FILE) -> i64;
    fn malloc(__size: u64) -> *mut ();
    fn fread(__ptr: *mut (), __size: u64, __nitems: u64, __stream: *mut FILE) -> u64;
    fn fclose(_: *mut FILE) -> i32;
    fn __builtin_expect(_: i64, _: i64) -> i64;
}
