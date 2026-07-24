use super::*;

/// The cJSON structure:
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct CJSON {
    pub(crate) next: *mut CJSON,
    pub(crate) prev: *mut CJSON,
    pub(crate) child: *mut CJSON,
    pub(crate) type_: i32,
    pub(crate) valuestring: *mut i8,
    pub(crate) valueint: i32,
    pub(crate) valuedouble: f64,
    pub(crate) string: *mut i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct CJSONHooks {
    pub(crate) malloc_fn: Option<unsafe extern "C" fn(u64) -> *mut ()>,
    pub(crate) free_fn: Option<unsafe extern "C" fn(*mut ()) -> ()>,
}
