use super::*;

pub const cJSON_False: i32 = 0;

pub const cJSON_True: i32 = 1;

pub const cJSON_NULL: i32 = 2;

pub const cJSON_Number: i32 = 3;

pub const cJSON_String: i32 = 4;

pub const cJSON_Array: i32 = 5;

pub const cJSON_Object: i32 = 6;

pub const cJSON_IsReference: i32 = 256;

pub const cJSON_StringIsConst: i32 = 512;

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
