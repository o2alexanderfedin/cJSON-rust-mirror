use super::*;
use crate::src::c_json_h::{
    cJSON_Array, cJSON_False, cJSON_IsReference, cJSON_NULL, cJSON_Number, cJSON_Object,
    cJSON_String, cJSON_StringIsConst, cJSON_True, CJSONHooks, CJSON,
};
use crate::{
    __builtin___memcpy_chk, __builtin___memset_chk, __builtin___sprintf_chk,
    __builtin___strcpy_chk, __builtin_object_size, fabs, floor, free, malloc, pow, strchr, strlen,
    strncmp, tolower,
};

static mut c_json_malloc: Option<unsafe extern "C" fn(u64) -> *mut ()> = Some(malloc);

static mut c_json_free: Option<unsafe extern "C" fn(*mut ()) -> ()> = Some(free);

/// Supply malloc, realloc and free functions to cJSON
#[allow(unused_doc_comments)]
pub(crate) extern "C" fn c_json_init_hooks(hooks: *mut CJSONHooks) -> () {
    unsafe {
        if (hooks).is_null() as i32 != 0 {
            /// Reset hooks
            (c_json_malloc = Some(malloc));
            c_json_free = Some(free);
            return;
        }
        c_json_malloc = if unsafe { (*hooks).malloc_fn.is_some() } {
            unsafe { (*hooks).malloc_fn }
        } else {
            Some(malloc)
        };
        c_json_free = if unsafe { (*hooks).free_fn.is_some() } {
            unsafe { (*hooks).free_fn }
        } else {
            Some(free)
        };
    }
}

/// Internal constructor.
extern "C" fn c_json_new_item() -> *mut CJSON {
    unsafe {
        let node: *mut CJSON =
            unsafe { c_json_malloc.unwrap()(core::mem::size_of::<CJSON>()) } as *mut CJSON;
        if !(node).is_null() {
            unsafe {
                __builtin___memset_chk(
                    node as *mut (),
                    0,
                    core::mem::size_of::<CJSON>() as u64,
                    unsafe { __builtin_object_size(node as *const (), 0) },
                )
            };
        }
        return node;
    }
}

static mut ep: *const i8 = unsafe { core::mem::zeroed() };

extern "C" fn parse_hex4(mut str: *const i8) -> u32 {
    let mut h: u32 = 0 as u32;
    if unsafe { *str } as i32 >= '0' as i32 && unsafe { *str } as i32 <= '9' as i32 {
        h = h.wrapping_add((unsafe { *str } as i32 - '0' as i32) as u32);
    } else if unsafe { *str } as i32 >= 'A' as i32 && unsafe { *str } as i32 <= 'F' as i32 {
        h = h.wrapping_add((10 + unsafe { *str } as i32 - 'A' as i32) as u32);
    } else if unsafe { *str } as i32 >= 'a' as i32 && unsafe { *str } as i32 <= 'f' as i32 {
        h = h.wrapping_add((10 + unsafe { *str } as i32 - 'a' as i32) as u32);
    } else {
        return 0 as u32;
    }
    h = h << 4;
    {
        let __n = 1;
        let __p = &mut str;
        *__p = unsafe { (*__p).offset(__n as isize) };
    };
    if unsafe { *str } as i32 >= '0' as i32 && unsafe { *str } as i32 <= '9' as i32 {
        h = h.wrapping_add((unsafe { *str } as i32 - '0' as i32) as u32);
    } else if unsafe { *str } as i32 >= 'A' as i32 && unsafe { *str } as i32 <= 'F' as i32 {
        h = h.wrapping_add((10 + unsafe { *str } as i32 - 'A' as i32) as u32);
    } else if unsafe { *str } as i32 >= 'a' as i32 && unsafe { *str } as i32 <= 'f' as i32 {
        h = h.wrapping_add((10 + unsafe { *str } as i32 - 'a' as i32) as u32);
    } else {
        return 0 as u32;
    }
    h = h << 4;
    {
        let __n = 1;
        let __p = &mut str;
        *__p = unsafe { (*__p).offset(__n as isize) };
    };
    if unsafe { *str } as i32 >= '0' as i32 && unsafe { *str } as i32 <= '9' as i32 {
        h = h.wrapping_add((unsafe { *str } as i32 - '0' as i32) as u32);
    } else if unsafe { *str } as i32 >= 'A' as i32 && unsafe { *str } as i32 <= 'F' as i32 {
        h = h.wrapping_add((10 + unsafe { *str } as i32 - 'A' as i32) as u32);
    } else if unsafe { *str } as i32 >= 'a' as i32 && unsafe { *str } as i32 <= 'f' as i32 {
        h = h.wrapping_add((10 + unsafe { *str } as i32 - 'a' as i32) as u32);
    } else {
        return 0 as u32;
    }
    h = h << 4;
    {
        let __n = 1;
        let __p = &mut str;
        *__p = unsafe { (*__p).offset(__n as isize) };
    };
    if unsafe { *str } as i32 >= '0' as i32 && unsafe { *str } as i32 <= '9' as i32 {
        h = h.wrapping_add((unsafe { *str } as i32 - '0' as i32) as u32);
    } else if unsafe { *str } as i32 >= 'A' as i32 && unsafe { *str } as i32 <= 'F' as i32 {
        h = h.wrapping_add((10 + unsafe { *str } as i32 - 'A' as i32) as u32);
    } else if unsafe { *str } as i32 >= 'a' as i32 && unsafe { *str } as i32 <= 'f' as i32 {
        h = h.wrapping_add((10 + unsafe { *str } as i32 - 'a' as i32) as u32);
    } else {
        return 0 as u32;
    }
    return h;
}

/// Parse the input text into an unescaped cstring, and populate item.
static first_byte_mark: [u8; 7] = [
    0 as u8, 0 as u8, 192 as u8, 224 as u8, 240 as u8, 248 as u8, 252 as u8,
];

#[allow(unused_doc_comments)]
extern "C" fn parse_string(item: &mut CJSON, str: *const i8) -> *const i8 {
    unsafe {
        let mut ptr: *const i8 = unsafe { str.offset(1 as isize) };
        let mut ptr2: *mut i8 = core::ptr::null_mut();
        let mut out: *mut i8 = core::ptr::null_mut();
        let mut len: i32 = 0;
        let mut uc: u32 = 0 as u32;
        let mut uc2: u32 = 0 as u32;
        if unsafe { *str } as i32 != '\"' as i32 {
            ep = str;
            return core::ptr::null();
        }
        while unsafe { *ptr } as i32 != '\"' as i32 && unsafe { *ptr } != 0 && {
            len += 1;
            len
        } != 0
        {
            if unsafe {
                *{
                    let __old = ptr;
                    ptr = unsafe { ptr.offset(1) };
                    __old
                }
            } as i32
                == '\\' as i32
            {
                {
                    let __old = ptr;
                    ptr = unsafe { ptr.offset(1) };
                    __old
                };
            }
        }

        /// Skip escaped quotes.
        (out = unsafe { c_json_malloc.unwrap()((len + 1) as u64) } as *mut i8);
        if (out).is_null() as i32 != 0 {
            return core::ptr::null();
        }
        ptr = unsafe { str.offset(1 as isize) };
        ptr2 = out;
        while unsafe { *ptr } as i32 != '\"' as i32 && unsafe { *ptr } != 0 {
            if unsafe { *ptr } as i32 != '\\' as i32 {
                unsafe {
                    *{
                        let __old = ptr2;
                        ptr2 = unsafe { ptr2.offset(1) };
                        __old
                    } = unsafe {
                        *{
                            let __old = ptr;
                            ptr = unsafe { ptr.offset(1) };
                            __old
                        }
                    } as i8
                };
            } else {
                {
                    let __n = 1;
                    let __p = &mut ptr;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
                '__s2: {
                    match unsafe { *ptr } {
                        98 => {
                            unsafe {
                                *{
                                    let __old = ptr2;
                                    ptr2 = unsafe { ptr2.offset(1) };
                                    __old
                                } = '\u{8}' as i32 as i8
                            };
                        }
                        102 => {
                            unsafe {
                                *{
                                    let __old = ptr2;
                                    ptr2 = unsafe { ptr2.offset(1) };
                                    __old
                                } = '\u{c}' as i32 as i8
                            };
                        }
                        110 => {
                            unsafe {
                                *{
                                    let __old = ptr2;
                                    ptr2 = unsafe { ptr2.offset(1) };
                                    __old
                                } = '\n' as i32 as i8
                            };
                        }
                        114 => {
                            unsafe {
                                *{
                                    let __old = ptr2;
                                    ptr2 = unsafe { ptr2.offset(1) };
                                    __old
                                } = '\r' as i32 as i8
                            };
                        }
                        116 => {
                            unsafe {
                                *{
                                    let __old = ptr2;
                                    ptr2 = unsafe { ptr2.offset(1) };
                                    __old
                                } = '\t' as i32 as i8
                            };
                        }
                        117 => {
                            uc = parse_hex4(unsafe { ptr.offset(1 as isize) });
                            {
                                let __n = 4;
                                let __p = &mut ptr;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            if uc >= 56320 as u32 && uc <= 57343 as u32 || uc == 0 as u32 {
                                break '__s2;
                            }
                            if uc >= 55296 as u32 && uc <= 56319 as u32 {
                                if unsafe { *ptr.offset(1 as isize) } as i32 != '\\' as i32
                                    || unsafe { *ptr.offset(2 as isize) } as i32 != 'u' as i32
                                {
                                    break '__s2;
                                }

                                /// missing second-half of surrogate.
                                (uc2 = parse_hex4(unsafe { ptr.offset(3 as isize) }));
                                {
                                    let __n = 6;
                                    let __p = &mut ptr;
                                    *__p = unsafe { (*__p).offset(__n as isize) };
                                };
                                if uc2 < 56320 as u32 || uc2 > 57343 as u32 {
                                    break '__s2;
                                }

                                /// invalid second-half of surrogate.
                                (uc = (65536 as u32)
                                    .wrapping_add((uc & 1023 as u32) << 10 | uc2 & 1023 as u32));
                            }
                            len = 4;
                            if uc < 128 as u32 {
                                len = 1;
                            } else if uc < 2048 as u32 {
                                len = 2;
                            } else if uc < 65536 as u32 {
                                len = 3;
                            }
                            {
                                let __n = len;
                                let __p = &mut ptr2;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                            '__s3: {
                                match len {
                                    4 => {
                                        unsafe {
                                            *{
                                                ptr2 = unsafe { ptr2.offset(-1) };
                                                ptr2
                                            } = ((uc | 128 as u32) & 191 as u32) as i8
                                        };
                                        uc >>= 6 as u32;
                                        unsafe {
                                            *{
                                                ptr2 = unsafe { ptr2.offset(-1) };
                                                ptr2
                                            } = ((uc | 128 as u32) & 191 as u32) as i8
                                        };
                                        uc >>= 6 as u32;
                                        unsafe {
                                            *{
                                                ptr2 = unsafe { ptr2.offset(-1) };
                                                ptr2
                                            } = ((uc | 128 as u32) & 191 as u32) as i8
                                        };
                                        uc >>= 6 as u32;
                                        unsafe {
                                            *{
                                                ptr2 = unsafe { ptr2.offset(-1) };
                                                ptr2
                                            } = (uc | first_byte_mark[len as usize] as u32) as i8
                                        };
                                    }
                                    3 => {
                                        unsafe {
                                            *{
                                                ptr2 = unsafe { ptr2.offset(-1) };
                                                ptr2
                                            } = ((uc | 128 as u32) & 191 as u32) as i8
                                        };
                                        uc >>= 6 as u32;
                                        unsafe {
                                            *{
                                                ptr2 = unsafe { ptr2.offset(-1) };
                                                ptr2
                                            } = ((uc | 128 as u32) & 191 as u32) as i8
                                        };
                                        uc >>= 6 as u32;
                                        unsafe {
                                            *{
                                                ptr2 = unsafe { ptr2.offset(-1) };
                                                ptr2
                                            } = (uc | first_byte_mark[len as usize] as u32) as i8
                                        };
                                    }
                                    2 => {
                                        unsafe {
                                            *{
                                                ptr2 = unsafe { ptr2.offset(-1) };
                                                ptr2
                                            } = ((uc | 128 as u32) & 191 as u32) as i8
                                        };
                                        uc >>= 6 as u32;
                                        unsafe {
                                            *{
                                                ptr2 = unsafe { ptr2.offset(-1) };
                                                ptr2
                                            } = (uc | first_byte_mark[len as usize] as u32) as i8
                                        };
                                    }
                                    1 => {
                                        unsafe {
                                            *{
                                                ptr2 = unsafe { ptr2.offset(-1) };
                                                ptr2
                                            } = (uc | first_byte_mark[len as usize] as u32) as i8
                                        };
                                    }
                                    _ => {}
                                }
                            }
                            {
                                let __n = len;
                                let __p = &mut ptr2;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                        }
                        _ => {
                            unsafe {
                                *{
                                    let __old = ptr2;
                                    ptr2 = unsafe { ptr2.offset(1) };
                                    __old
                                } = unsafe { *ptr } as i8
                            };
                        }
                    }
                }
                {
                    let __n = 1;
                    let __p = &mut ptr;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
            }
        }
        unsafe { *ptr2 = 0 as i8 };
        if unsafe { *ptr } as i32 == '\"' as i32 {
            {
                let __old = ptr;
                ptr = unsafe { ptr.offset(1) };
                __old
            };
        }
        (*item).valuestring = out;
        (*item).type_ = cJSON_String as i32;
        return ptr;
    }
}

/// Parse the input text to generate a number, and populate the result into item.
#[allow(unused_doc_comments)]
extern "C" fn parse_number(item: &mut CJSON, mut num: *const i8) -> *const i8 {
    let mut n: f64 = 0 as f64;
    let mut sign: f64 = 1 as f64;
    let mut scale: f64 = 0 as f64;
    let mut subscale: i32 = 0;
    let mut signsubscale: i32 = 1;
    if unsafe { *num } as i32 == '-' as i32 {
        {
            sign = -1 as f64;
            {
                let __old = num;
                num = unsafe { num.offset(1) };
                __old
            }
        };
    }
    if unsafe { *num } as i32 == '0' as i32 {
        {
            let __old = num;
            num = unsafe { num.offset(1) };
            __old
        };
    }
    if unsafe { *num } as i32 >= '1' as i32 && unsafe { *num } as i32 <= '9' as i32 {
        '__b4: loop {
            '__c4: loop {
                n = n * 10.0
                    + (unsafe {
                        *{
                            let __old = num;
                            num = unsafe { num.offset(1) };
                            __old
                        }
                    } as i32
                        - '0' as i32) as f64;
                break '__c4;
            }
            if !(unsafe { *num } as i32 >= '0' as i32 && unsafe { *num } as i32 <= '9' as i32) {
                break '__b4;
            }
        }
    }
    if unsafe { *num } as i32 == '.' as i32
        && unsafe { *num.offset(1 as isize) } as i32 >= '0' as i32
        && unsafe { *num.offset(1 as isize) } as i32 <= '9' as i32
    {
        {
            let __n = 1;
            let __p = &mut num;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        '__b5: loop {
            '__c5: loop {
                {
                    n = n * 10.0
                        + (unsafe {
                            *{
                                let __old = num;
                                num = unsafe { num.offset(1) };
                                __old
                            }
                        } as i32
                            - '0' as i32) as f64;
                    {
                        let __old = scale;
                        scale -= 1;
                        __old
                    }
                };
                break '__c5;
            }
            if !(unsafe { *num } as i32 >= '0' as i32 && unsafe { *num } as i32 <= '9' as i32) {
                break '__b5;
            }
        }
    }
    if unsafe { *num } as i32 == 'e' as i32 || unsafe { *num } as i32 == 'E' as i32 {
        {
            let __n = 1;
            let __p = &mut num;
            *__p = unsafe { (*__p).offset(__n as isize) };
        };
        if unsafe { *num } as i32 == '+' as i32 {
            {
                let __old = num;
                num = unsafe { num.offset(1) };
                __old
            };
        } else if unsafe { *num } as i32 == '-' as i32 {
            {
                signsubscale = -1;
                {
                    let __old = num;
                    num = unsafe { num.offset(1) };
                    __old
                }
            };
        }
        while unsafe { *num } as i32 >= '0' as i32 && unsafe { *num } as i32 <= '9' as i32 {
            subscale = subscale * 10
                + (unsafe {
                    *{
                        let __old = num;
                        num = unsafe { num.offset(1) };
                        __old
                    }
                } as i32
                    - '0' as i32);
        }
    }
    n = sign * n * unsafe { pow(10.0, scale + (subscale * signsubscale) as f64) };

    /// number = +/- number.fraction * 10^+/- exponent
    ((*item).valuedouble = n);
    (*item).valueint = n as i32;
    (*item).type_ = cJSON_Number as i32;
    return num;
}

/// Utility to jump whitespace and cr/lf
extern "C" fn skip(mut in__1: *const i8) -> *const i8 {
    while !(in__1).is_null() && unsafe { *in__1 } != 0 && unsafe { *in__1 } as u8 as i32 <= 32 {
        {
            let __old = in__1;
            in__1 = unsafe { in__1.offset(1) };
            __old
        };
    }
    return in__1;
}

/// Build an array from input text.
#[allow(unused_doc_comments)]
extern "C" fn parse_array(item: &mut CJSON, mut value: *const i8) -> *const i8 {
    unsafe {
        let mut child: *mut CJSON = core::ptr::null_mut();
        if unsafe { *value } as i32 != '[' as i32 {
            ep = value;
            return core::ptr::null();
        }

        /// not an array!
        ((*item).type_ = cJSON_Array as i32);
        value = skip(unsafe { value.offset(1 as isize) });
        if unsafe { *value } as i32 == ']' as i32 {
            return unsafe { value.offset(1 as isize) };
        }

        /// empty array.
        ((*item).child = {
            child = c_json_new_item();
            child
        });
        if ((*item).child).is_null() as i32 != 0 {
            return core::ptr::null();
        }

        /// memory fail
        (value = skip(parse_value(child, skip(value))));
        if (value).is_null() as i32 != 0 {
            return core::ptr::null();
        }
        while unsafe { *value } as i32 == ',' as i32 {
            let new_item: *mut CJSON = c_json_new_item();
            if (new_item).is_null() as i32 != 0 {
                return core::ptr::null();
            }

            /// memory fail
            unsafe {
                (*child).next = new_item
            };
            unsafe { (*new_item).prev = child };
            child = new_item;
            value = skip(parse_value(
                child,
                skip(unsafe { value.offset(1 as isize) }),
            ));
            if (value).is_null() as i32 != 0 {
                return core::ptr::null();
            }
        }
        if unsafe { *value } as i32 == ']' as i32 {
            return unsafe { value.offset(1 as isize) };
        }

        /// end of array
        (ep = value);
        return core::ptr::null();
    }
}

/// Build an object from the text.
#[allow(unused_doc_comments)]
extern "C" fn parse_object(item: &mut CJSON, mut value: *const i8) -> *const i8 {
    unsafe {
        let mut child: *mut CJSON = core::ptr::null_mut();
        if unsafe { *value } as i32 != '{' as i32 {
            ep = value;
            return core::ptr::null();
        }

        /// not an object!
        ((*item).type_ = cJSON_Object as i32);
        value = skip(unsafe { value.offset(1 as isize) });
        if unsafe { *value } as i32 == '}' as i32 {
            return unsafe { value.offset(1 as isize) };
        }

        /// empty array.
        ((*item).child = {
            child = c_json_new_item();
            child
        });
        if ((*item).child).is_null() as i32 != 0 {
            return core::ptr::null();
        }
        value = skip(parse_string(unsafe { &mut *child }, skip(value)));
        if (value).is_null() as i32 != 0 {
            return core::ptr::null();
        }
        unsafe { (*child).string = unsafe { (*child).valuestring } };
        unsafe { (*child).valuestring = core::ptr::null_mut() };
        if unsafe { *value } as i32 != ':' as i32 {
            ep = value;
            return core::ptr::null();
        }

        /// fail!
        (value = skip(parse_value(
            child,
            skip(unsafe { value.offset(1 as isize) }),
        )));
        if (value).is_null() as i32 != 0 {
            return core::ptr::null();
        }
        while unsafe { *value } as i32 == ',' as i32 {
            let new_item: *mut CJSON = c_json_new_item();
            if (new_item).is_null() as i32 != 0 {
                return core::ptr::null();
            }

            /// memory fail
            unsafe {
                (*child).next = new_item
            };
            unsafe { (*new_item).prev = child };
            child = new_item;
            value = skip(parse_string(
                unsafe { &mut *child },
                skip(unsafe { value.offset(1 as isize) }),
            ));
            if (value).is_null() as i32 != 0 {
                return core::ptr::null();
            }
            unsafe { (*child).string = unsafe { (*child).valuestring } };
            unsafe { (*child).valuestring = core::ptr::null_mut() };
            if unsafe { *value } as i32 != ':' as i32 {
                ep = value;
                return core::ptr::null();
            }

            /// fail!
            (value = skip(parse_value(
                child,
                skip(unsafe { value.offset(1 as isize) }),
            )));
            if (value).is_null() as i32 != 0 {
                return core::ptr::null();
            }
        }
        if unsafe { *value } as i32 == '}' as i32 {
            return unsafe { value.offset(1 as isize) };
        }

        /// end of array
        (ep = value);
        return core::ptr::null();
    }
}

/// Predeclare these prototypes.
extern "C" fn parse_value(item: *mut CJSON, value: *const i8) -> *const i8 {
    unsafe {
        if (value).is_null() as i32 != 0 {
            return core::ptr::null();
        }
        if (unsafe { strncmp(value, c"null".as_ptr() as *mut i8 as *const i8, 4 as u64) } == 0)
            as i32
            != 0
        {
            unsafe { (*item).type_ = cJSON_NULL as i32 };
            return unsafe { value.offset(4 as isize) };
        }
        if (unsafe { strncmp(value, c"false".as_ptr() as *mut i8 as *const i8, 5 as u64) } == 0)
            as i32
            != 0
        {
            unsafe { (*item).type_ = cJSON_False as i32 };
            return unsafe { value.offset(5 as isize) };
        }
        if (unsafe { strncmp(value, c"true".as_ptr() as *mut i8 as *const i8, 4 as u64) } == 0)
            as i32
            != 0
        {
            unsafe { (*item).type_ = cJSON_True as i32 };
            unsafe { (*item).valueint = 1 };
            return unsafe { value.offset(4 as isize) };
        }
        if unsafe { *value } as i32 == '\"' as i32 {
            return parse_string(unsafe { &mut *item }, value);
        }
        if unsafe { *value } as i32 == '-' as i32
            || unsafe { *value } as i32 >= '0' as i32 && unsafe { *value } as i32 <= '9' as i32
        {
            return parse_number(unsafe { &mut *item }, value);
        }
        if unsafe { *value } as i32 == '[' as i32 {
            return parse_array(unsafe { &mut *item }, value);
        }
        if unsafe { *value } as i32 == '{' as i32 {
            return parse_object(unsafe { &mut *item }, value);
        }
        ep = value;
        return core::ptr::null();
    }
}

/// Delete a cJSON entity and all subentities.
pub(crate) extern "C" fn c_json_delete(mut c: *mut CJSON) -> () {
    unsafe {
        let mut next: *mut CJSON = core::ptr::null_mut();
        while !(c).is_null() {
            next = unsafe { (*c).next };
            if (unsafe { (*c).type_ } & cJSON_IsReference as i32 == 0) as i32 != 0
                && !(unsafe { (*c).child }).is_null()
            {
                c_json_delete(unsafe { (*c).child });
            }
            if (unsafe { (*c).type_ } & cJSON_IsReference as i32 == 0) as i32 != 0
                && !(unsafe { (*c).valuestring }).is_null()
            {
                unsafe { c_json_free.unwrap()(unsafe { (*c).valuestring } as *mut ()) };
            }
            if (unsafe { (*c).type_ } & cJSON_StringIsConst as i32 == 0) as i32 != 0
                && !(unsafe { (*c).string }).is_null()
            {
                unsafe { c_json_free.unwrap()(unsafe { (*c).string } as *mut ()) };
            }
            unsafe { c_json_free.unwrap()(c as *mut ()) };
            c = next;
        }
    }
}

/// ParseWithOpts allows you to require (and check) that the JSON is null terminated, and to retrieve the pointer to the final byte parsed.
#[allow(unused_doc_comments)]
pub(crate) extern "C" fn c_json_parse_with_opts(
    value: *const i8,
    return_parse_end: *mut *const i8,
    require_null_terminated: i32,
) -> *mut CJSON {
    unsafe {
        let mut end: *const i8 = core::ptr::null();
        let c: *mut CJSON = c_json_new_item();
        ep = core::ptr::null();
        if (c).is_null() as i32 != 0 {
            return core::ptr::null_mut();
        }

        /// memory fail
        (end = parse_value(c, skip(value)));
        if (end).is_null() as i32 != 0 {
            c_json_delete(c);
            return core::ptr::null_mut();
        }
        if require_null_terminated != 0 {
            end = skip(end);
            if unsafe { *end } != 0 {
                c_json_delete(c);
                ep = end;
                return core::ptr::null_mut();
            }
        }
        if !(return_parse_end).is_null() {
            unsafe { *return_parse_end = end };
        }
        return c;
    }
}

/// Supply a block of JSON, and this returns a cJSON object you can interrogate. Call cJSON_Delete when finished.
pub(crate) extern "C" fn c_json_parse(value: *const i8) -> *mut CJSON {
    return c_json_parse_with_opts(value, core::ptr::null_mut(), 0);
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub(crate) struct Printbuffer {
    pub(crate) buffer: *mut i8,
    pub(crate) length: i32,
    pub(crate) offset: i32,
}

extern "C" fn pow2gt(mut x: i32) -> i32 {
    x -= 1;
    x |= x >> 1;
    x |= x >> 2;
    x |= x >> 4;
    x |= x >> 8;
    x |= x >> 16;
    return x + 1;
}

extern "C" fn ensure(p: *mut Printbuffer, mut needed: i32) -> *mut i8 {
    unsafe {
        let mut newbuffer: *mut i8 = core::ptr::null_mut();
        let mut newsize: i32 = 0;
        if (p).is_null() as i32 != 0 || (unsafe { (*p).buffer }).is_null() as i32 != 0 {
            return core::ptr::null_mut();
        }
        needed += unsafe { (*p).offset };
        if needed <= unsafe { (*p).length } {
            return unsafe { unsafe { (*p).buffer.offset(unsafe { (*p).offset } as isize) } };
        }
        newsize = pow2gt(needed);
        newbuffer = unsafe { c_json_malloc.unwrap()(newsize as u64) } as *mut i8;
        if (newbuffer).is_null() as i32 != 0 {
            unsafe { c_json_free.unwrap()(unsafe { (*p).buffer } as *mut ()) };
            {
                unsafe { (*p).length = 0 };
                unsafe { (*p).buffer = core::ptr::null_mut() }
            };
            return core::ptr::null_mut();
        }
        if !(newbuffer).is_null() {
            unsafe {
                __builtin___memcpy_chk(
                    newbuffer as *mut (),
                    unsafe { (*p).buffer } as *const (),
                    unsafe { (*p).length } as u64,
                    unsafe { __builtin_object_size(newbuffer as *const (), 0) },
                )
            };
        }
        unsafe { c_json_free.unwrap()(unsafe { (*p).buffer } as *mut ()) };
        unsafe { (*p).length = newsize };
        unsafe { (*p).buffer = newbuffer };
        return unsafe { newbuffer.offset(unsafe { (*p).offset } as isize) };
    }
}

/// Render the number nicely from the given item into a string.
extern "C" fn print_number(item: &CJSON, p: *mut Printbuffer) -> *mut i8 {
    unsafe {
        let mut str: *mut i8 = core::ptr::null_mut();
        let d: f64 = (*item).valuedouble;
        if d == 0 as f64 {
            if !(p).is_null() {
                str = ensure(p, 2);
            } else {
                str = unsafe { c_json_malloc.unwrap()(2) } as *mut i8;
            }
            if !(str).is_null() {
                unsafe {
                    __builtin___strcpy_chk(str, c"0".as_ptr() as *mut i8 as *const i8, unsafe {
                        __builtin_object_size(str as *const (), if 2 > 1 { 1 } else { 0 })
                    })
                };
            }
        } else if unsafe { fabs((*item).valueint as f64 - d) } <= 2.220446049250313e-16
            && d <= i32::MAX as f64
            && d >= i32::MIN as f64
        {
            if !(p).is_null() {
                str = ensure(p, 21);
            } else {
                str = unsafe { c_json_malloc.unwrap()(21) } as *mut i8;
            }
            if !(str).is_null() {
                unsafe {
                    __builtin___sprintf_chk(
                        str,
                        0,
                        unsafe {
                            __builtin_object_size(str as *const (), if 2 > 1 { 1 } else { 0 })
                        },
                        c"%d".as_ptr() as *mut i8 as *const i8,
                        (*item).valueint,
                    )
                };
            }
        } else {
            if !(p).is_null() {
                str = ensure(p, 64);
            } else {
                str = unsafe { c_json_malloc.unwrap()(64) } as *mut i8;
            }
            if !(str).is_null() {
                if unsafe { fabs(unsafe { floor(d) } - d) } <= 2.220446049250313e-16
                    && unsafe { fabs(d) } < 1e60
                {
                    unsafe {
                        __builtin___sprintf_chk(
                            str,
                            0,
                            unsafe {
                                __builtin_object_size(str as *const (), if 2 > 1 { 1 } else { 0 })
                            },
                            c"%.0f".as_ptr() as *mut i8 as *const i8,
                            d,
                        )
                    };
                } else if unsafe { fabs(d) } < 1e-6 || unsafe { fabs(d) } > 1000000000.0 {
                    unsafe {
                        __builtin___sprintf_chk(
                            str,
                            0,
                            unsafe {
                                __builtin_object_size(str as *const (), if 2 > 1 { 1 } else { 0 })
                            },
                            c"%e".as_ptr() as *mut i8 as *const i8,
                            d,
                        )
                    };
                } else {
                    unsafe {
                        __builtin___sprintf_chk(
                            str,
                            0,
                            unsafe {
                                __builtin_object_size(str as *const (), if 2 > 1 { 1 } else { 0 })
                            },
                            c"%f".as_ptr() as *mut i8 as *const i8,
                            d,
                        )
                    };
                }
            }
        }
        return str;
    }
}

/// Render the cstring provided to an escaped version that can be printed.
extern "C" fn print_string_ptr(str: *const i8, p: *mut Printbuffer) -> *mut i8 {
    unsafe {
        let mut ptr: *const i8 = core::ptr::null();
        let mut ptr2: *mut i8 = core::ptr::null_mut();
        let mut out: *mut i8 = core::ptr::null_mut();
        let mut len: i32 = 0;
        let mut flag: i32 = 0;
        let mut token: u8 = 0 as u8;
        {
            ptr = str;
            '__b11: loop {
                if !(unsafe { *ptr } != 0) {
                    break '__b11;
                }
                '__c11: loop {
                    flag |= if unsafe { *ptr } as i32 > 0 && (unsafe { *ptr } as i32) < 32
                        || unsafe { *ptr } as i32 == '\"' as i32
                        || unsafe { *ptr } as i32 == '\\' as i32
                    {
                        1
                    } else {
                        0
                    };
                    break '__c11;
                }
                {
                    let __n = 1;
                    let __p = &mut ptr;
                    *__p = unsafe { (*__p).offset(__n as isize) };
                };
            }
        }
        if (flag == 0) as i32 != 0 {
            len = unsafe { ptr.offset_from(str) } as i64 as i32;
            if !(p).is_null() {
                out = ensure(p, len + 3);
            } else {
                out = unsafe { c_json_malloc.unwrap()((len + 3) as u64) } as *mut i8;
            }
            if (out).is_null() as i32 != 0 {
                return core::ptr::null_mut();
            }
            ptr2 = out;
            unsafe {
                *{
                    let __old = ptr2;
                    ptr2 = unsafe { ptr2.offset(1) };
                    __old
                } = '\"' as i32 as i8
            };
            unsafe {
                __builtin___strcpy_chk(ptr2, str, unsafe {
                    __builtin_object_size(ptr2 as *const (), if 2 > 1 { 1 } else { 0 })
                })
            };
            unsafe { *ptr2.offset(len as isize) = '\"' as i32 as i8 };
            unsafe { *ptr2.offset((len + 1) as isize) = 0 as i8 };
            return out;
        }
        if (str).is_null() as i32 != 0 {
            if !(p).is_null() {
                out = ensure(p, 3);
            } else {
                out = unsafe { c_json_malloc.unwrap()(3) } as *mut i8;
            }
            if (out).is_null() as i32 != 0 {
                return core::ptr::null_mut();
            }
            unsafe {
                __builtin___strcpy_chk(out, c"\"\"".as_ptr() as *mut i8 as *const i8, unsafe {
                    __builtin_object_size(out as *const (), if 2 > 1 { 1 } else { 0 })
                })
            };
            return out;
        }
        ptr = str;
        while {
            token = unsafe { *ptr } as u8;
            token
        } != 0
            && {
                len += 1;
                len
            } != 0
        {
            if !(unsafe {
                strchr(
                    c"\"\\\u{8}\u{c}\n\r\t".as_ptr() as *mut i8 as *const i8,
                    token as i32,
                )
            })
            .is_null()
            {
                {
                    let __old = len;
                    len += 1;
                    __old
                };
            } else if (token as i32) < 32 {
                len += 5;
            }
            {
                let __n = 1;
                let __p = &mut ptr;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
        }
        if !(p).is_null() {
            out = ensure(p, len + 3);
        } else {
            out = unsafe { c_json_malloc.unwrap()((len + 3) as u64) } as *mut i8;
        }
        if (out).is_null() as i32 != 0 {
            return core::ptr::null_mut();
        }
        ptr2 = out;
        ptr = str;
        unsafe {
            *{
                let __old = ptr2;
                ptr2 = unsafe { ptr2.offset(1) };
                __old
            } = '\"' as i32 as i8
        };
        while unsafe { *ptr } != 0 {
            if unsafe { *ptr } as u8 as i32 > 31
                && unsafe { *ptr } as i32 != '\"' as i32
                && unsafe { *ptr } as i32 != '\\' as i32
            {
                unsafe {
                    *{
                        let __old = ptr2;
                        ptr2 = unsafe { ptr2.offset(1) };
                        __old
                    } = unsafe {
                        *{
                            let __old = ptr;
                            ptr = unsafe { ptr.offset(1) };
                            __old
                        }
                    } as i8
                };
            } else {
                unsafe {
                    *{
                        let __old = ptr2;
                        ptr2 = unsafe { ptr2.offset(1) };
                        __old
                    } = '\\' as i32 as i8
                };
                '__s14: {
                    match {
                        token = unsafe {
                            *{
                                let __old = ptr;
                                ptr = unsafe { ptr.offset(1) };
                                __old
                            }
                        } as u8;
                        token
                    } {
                        92 => {
                            unsafe {
                                *{
                                    let __old = ptr2;
                                    ptr2 = unsafe { ptr2.offset(1) };
                                    __old
                                } = '\\' as i32 as i8
                            };
                        }
                        34 => {
                            unsafe {
                                *{
                                    let __old = ptr2;
                                    ptr2 = unsafe { ptr2.offset(1) };
                                    __old
                                } = '\"' as i32 as i8
                            };
                        }
                        8 => {
                            unsafe {
                                *{
                                    let __old = ptr2;
                                    ptr2 = unsafe { ptr2.offset(1) };
                                    __old
                                } = 'b' as i32 as i8
                            };
                        }
                        12 => {
                            unsafe {
                                *{
                                    let __old = ptr2;
                                    ptr2 = unsafe { ptr2.offset(1) };
                                    __old
                                } = 'f' as i32 as i8
                            };
                        }
                        10 => {
                            unsafe {
                                *{
                                    let __old = ptr2;
                                    ptr2 = unsafe { ptr2.offset(1) };
                                    __old
                                } = 'n' as i32 as i8
                            };
                        }
                        13 => {
                            unsafe {
                                *{
                                    let __old = ptr2;
                                    ptr2 = unsafe { ptr2.offset(1) };
                                    __old
                                } = 'r' as i32 as i8
                            };
                        }
                        9 => {
                            unsafe {
                                *{
                                    let __old = ptr2;
                                    ptr2 = unsafe { ptr2.offset(1) };
                                    __old
                                } = 't' as i32 as i8
                            };
                        }
                        _ => {
                            unsafe {
                                __builtin___sprintf_chk(
                                    ptr2,
                                    0,
                                    unsafe {
                                        __builtin_object_size(
                                            ptr2 as *const (),
                                            if 2 > 1 { 1 } else { 0 },
                                        )
                                    },
                                    c"u%04x".as_ptr() as *mut i8 as *const i8,
                                    token as i32,
                                )
                            };
                            {
                                let __n = 5;
                                let __p = &mut ptr2;
                                *__p = unsafe { (*__p).offset(__n as isize) };
                            };
                        }
                    }
                }
            }
        }
        unsafe {
            *{
                let __old = ptr2;
                ptr2 = unsafe { ptr2.offset(1) };
                __old
            } = '\"' as i32 as i8
        };
        unsafe {
            *{
                let __old = ptr2;
                ptr2 = unsafe { ptr2.offset(1) };
                __old
            } = 0 as i8
        };
        return out;
    }
}

/// Invote print_string_ptr (which is useful) on an item.
extern "C" fn print_string(item: &CJSON, p: *mut Printbuffer) -> *mut i8 {
    return print_string_ptr((*item).valuestring as *const i8, p);
}

extern "C" fn update(p: *const Printbuffer) -> i32 {
    let mut str: *const i8 = core::ptr::null();
    if (p).is_null() as i32 != 0 || (unsafe { (*p).buffer }).is_null() as i32 != 0 {
        return 0;
    }
    str = unsafe { unsafe { (*p).buffer.offset(unsafe { (*p).offset } as isize) } };
    return (unsafe { (*p).offset } as u64).wrapping_add(unsafe { strlen(str as *const i8) })
        as i32;
}

/// Render an array to text
#[allow(unused_doc_comments)]
extern "C" fn print_array(item: &CJSON, depth: i32, fmt: i32, p: *mut Printbuffer) -> *mut i8 {
    unsafe {
        let mut entries: *mut *mut i8 = core::ptr::null_mut();
        let mut out: *mut i8 = core::ptr::null_mut();
        let mut ptr: *mut i8 = core::ptr::null_mut();
        let mut ret: *mut i8 = core::ptr::null_mut();
        let mut len: i32 = 5;
        let mut child: *mut CJSON = (*item).child;
        let mut numentries: i32 = 0;
        let mut i: i32 = 0;
        let mut fail: i32 = 0;
        let mut tmplen: u64 = 0 as u64;
        while !(child).is_null() {
            {
                {
                    let __old = numentries;
                    numentries += 1;
                    __old
                };
                child = unsafe { (*child).next }
            };
        }
        if (numentries == 0) as i32 != 0 {
            if !(p).is_null() {
                out = ensure(p, 3);
            } else {
                out = unsafe { c_json_malloc.unwrap()(3) } as *mut i8;
            }
            if !(out).is_null() {
                unsafe {
                    __builtin___strcpy_chk(out, c"[]".as_ptr() as *mut i8 as *const i8, unsafe {
                        __builtin_object_size(out as *const (), if 2 > 1 { 1 } else { 0 })
                    })
                };
            }
            return out;
        }
        if !(p).is_null() {
            /// Compose the output array.
            (i = unsafe { (*p).offset });
            ptr = ensure(p, 1);
            if (ptr).is_null() as i32 != 0 {
                return core::ptr::null_mut();
            }
            unsafe { *ptr = '[' as i32 as i8 };
            {
                let __p = unsafe { &mut (*p).offset };
                let __t = *__p;
                *__p += 1;
                __t
            };
            child = (*item).child;
            while !(child).is_null() && (fail == 0) as i32 != 0 {
                print_value(child, depth + 1, fmt, p);
                unsafe { (*p).offset = update(p as *const Printbuffer) };
                if !(unsafe { (*child).next }).is_null() {
                    len = if fmt != 0 { 2 } else { 1 };
                    ptr = ensure(p, len + 1);
                    if (ptr).is_null() as i32 != 0 {
                        return core::ptr::null_mut();
                    }
                    unsafe {
                        *{
                            let __old = ptr;
                            ptr = unsafe { ptr.offset(1) };
                            __old
                        } = ',' as i32 as i8
                    };
                    if fmt != 0 {
                        unsafe {
                            *{
                                let __old = ptr;
                                ptr = unsafe { ptr.offset(1) };
                                __old
                            } = ' ' as i32 as i8
                        };
                    }
                    unsafe { *ptr = 0 as i8 };
                    unsafe { (*p).offset += len };
                }
                child = unsafe { (*child).next };
            }
            ptr = ensure(p, 2);
            if (ptr).is_null() as i32 != 0 {
                return core::ptr::null_mut();
            }
            unsafe {
                *{
                    let __old = ptr;
                    ptr = unsafe { ptr.offset(1) };
                    __old
                } = ']' as i32 as i8
            };
            unsafe { *ptr = 0 as i8 };
            out = unsafe { unsafe { (*p).buffer.offset(i as isize) } };
        } else {
            /// Allocate an array to hold the values for each
            (entries = unsafe {
                c_json_malloc.unwrap()(
                    (numentries as u64).wrapping_mul(core::mem::size_of::<*mut i8>() as u64),
                )
            } as *mut *mut i8);
            if (entries).is_null() as i32 != 0 {
                return core::ptr::null_mut();
            }
            unsafe {
                __builtin___memset_chk(
                    entries as *mut (),
                    0,
                    (numentries as u64).wrapping_mul(core::mem::size_of::<*mut i8>() as u64),
                    unsafe { __builtin_object_size(entries as *const (), 0) },
                )
            };

            /// Retrieve all the results:
            (child = (*item).child);
            while !(child).is_null() && (fail == 0) as i32 != 0 {
                ret = print_value(child, depth + 1, fmt, core::ptr::null_mut());
                unsafe {
                    *entries.offset({
                        let __old = i;
                        i += 1;
                        __old
                    } as isize) = ret
                };
                if !(ret).is_null() {
                    len += unsafe {
                        strlen(ret as *const i8)
                            .wrapping_add(2 as u64)
                            .wrapping_add(if fmt != 0 { 1 } else { 0 } as u64)
                    } as i32;
                } else {
                    fail = 1;
                }
                child = unsafe { (*child).next };
            }
            if (fail == 0) as i32 != 0 {
                out = unsafe { c_json_malloc.unwrap()(len as u64) } as *mut i8;
            }
            if (out).is_null() as i32 != 0 {
                fail = 1;
            }
            if fail != 0 {
                {
                    i = 0;
                    '__b18: loop {
                        if !(i < numentries) {
                            break '__b18;
                        }
                        '__c18: loop {
                            if !(unsafe { *entries.offset(i as isize) }).is_null() {
                                unsafe {
                                    c_json_free.unwrap()(
                                        unsafe { *entries.offset(i as isize) } as *mut ()
                                    )
                                };
                            }
                            break '__c18;
                        }
                        i += 1;
                    }
                }
                unsafe { c_json_free.unwrap()(entries as *mut ()) };
                return core::ptr::null_mut();
            }

            /// Compose the output array.
            unsafe {
                *out = '[' as i32 as i8
            };
            ptr = unsafe { out.offset(1 as isize) };
            unsafe { *ptr = 0 as i8 };
            {
                i = 0;
                '__b19: loop {
                    if !(i < numentries) {
                        break '__b19;
                    }
                    '__c19: loop {
                        tmplen =
                            unsafe { strlen(unsafe { *entries.offset(i as isize) } as *const i8) };
                        unsafe {
                            __builtin___memcpy_chk(
                                ptr as *mut (),
                                unsafe { *entries.offset(i as isize) } as *const (),
                                tmplen,
                                unsafe { __builtin_object_size(ptr as *const (), 0) },
                            )
                        };
                        {
                            let __n = tmplen;
                            let __p = &mut ptr;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                        if i != numentries - 1 {
                            unsafe {
                                *{
                                    let __old = ptr;
                                    ptr = unsafe { ptr.offset(1) };
                                    __old
                                } = ',' as i32 as i8
                            };
                            if fmt != 0 {
                                unsafe {
                                    *{
                                        let __old = ptr;
                                        ptr = unsafe { ptr.offset(1) };
                                        __old
                                    } = ' ' as i32 as i8
                                };
                            }
                            unsafe { *ptr = 0 as i8 };
                        }
                        unsafe {
                            c_json_free.unwrap()(unsafe { *entries.offset(i as isize) } as *mut ())
                        };
                        break '__c19;
                    }
                    i += 1;
                }
            }
            unsafe { c_json_free.unwrap()(entries as *mut ()) };
            unsafe {
                *{
                    let __old = ptr;
                    ptr = unsafe { ptr.offset(1) };
                    __old
                } = ']' as i32 as i8
            };
            unsafe {
                *{
                    let __old = ptr;
                    ptr = unsafe { ptr.offset(1) };
                    __old
                } = 0 as i8
            };
        }
        return out;
    }
}

/// Render an object to text.
#[allow(unused_doc_comments)]
extern "C" fn print_object(item: &CJSON, mut depth: i32, fmt: i32, p: *mut Printbuffer) -> *mut i8 {
    unsafe {
        let mut entries: *mut *mut i8 = core::ptr::null_mut();
        let mut names: *mut *mut i8 = core::ptr::null_mut();
        let mut out: *mut i8 = core::ptr::null_mut();
        let mut ptr: *mut i8 = core::ptr::null_mut();
        let mut ret: *mut i8 = core::ptr::null_mut();
        let mut str: *mut i8 = core::ptr::null_mut();
        let mut len: i32 = 7;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut child: *mut CJSON = (*item).child;
        let mut numentries: i32 = 0;
        let mut fail: i32 = 0;
        let mut tmplen: u64 = 0 as u64;
        while !(child).is_null() {
            {
                {
                    let __old = numentries;
                    numentries += 1;
                    __old
                };
                child = unsafe { (*child).next }
            };
        }
        if (numentries == 0) as i32 != 0 {
            if !(p).is_null() {
                out = ensure(p, if fmt != 0 { depth + 4 } else { 3 });
            } else {
                out = unsafe { c_json_malloc.unwrap()(if fmt != 0 { depth + 4 } else { 3 } as u64) }
                    as *mut i8;
            }
            if (out).is_null() as i32 != 0 {
                return core::ptr::null_mut();
            }
            ptr = out;
            unsafe {
                *{
                    let __old = ptr;
                    ptr = unsafe { ptr.offset(1) };
                    __old
                } = '{' as i32 as i8
            };
            if fmt != 0 {
                unsafe {
                    *{
                        let __old = ptr;
                        ptr = unsafe { ptr.offset(1) };
                        __old
                    } = '\n' as i32 as i8
                };
                {
                    i = 0;
                    '__b21: loop {
                        if !(i < depth - 1) {
                            break '__b21;
                        }
                        '__c21: loop {
                            unsafe {
                                *{
                                    let __old = ptr;
                                    ptr = unsafe { ptr.offset(1) };
                                    __old
                                } = '\t' as i32 as i8
                            };
                            break '__c21;
                        }
                        i += 1;
                    }
                }
            }
            unsafe {
                *{
                    let __old = ptr;
                    ptr = unsafe { ptr.offset(1) };
                    __old
                } = '}' as i32 as i8
            };
            unsafe {
                *{
                    let __old = ptr;
                    ptr = unsafe { ptr.offset(1) };
                    __old
                } = 0 as i8
            };
            return out;
        }
        if !(p).is_null() {
            /// Compose the output:
            (i = unsafe { (*p).offset });
            len = if fmt != 0 { 2 } else { 1 };
            ptr = ensure(p, len + 1);
            if (ptr).is_null() as i32 != 0 {
                return core::ptr::null_mut();
            }
            unsafe {
                *{
                    let __old = ptr;
                    ptr = unsafe { ptr.offset(1) };
                    __old
                } = '{' as i32 as i8
            };
            if fmt != 0 {
                unsafe {
                    *{
                        let __old = ptr;
                        ptr = unsafe { ptr.offset(1) };
                        __old
                    } = '\n' as i32 as i8
                };
            }
            unsafe { *ptr = 0 as i8 };
            unsafe { (*p).offset += len };
            child = (*item).child;
            depth += 1;
            while !(child).is_null() {
                if fmt != 0 {
                    ptr = ensure(p, depth);
                    if (ptr).is_null() as i32 != 0 {
                        return core::ptr::null_mut();
                    }
                    {
                        j = 0;
                        '__b23: loop {
                            if !(j < depth) {
                                break '__b23;
                            }
                            '__c23: loop {
                                unsafe {
                                    *{
                                        let __old = ptr;
                                        ptr = unsafe { ptr.offset(1) };
                                        __old
                                    } = '\t' as i32 as i8
                                };
                                break '__c23;
                            }
                            j += 1;
                        }
                    }
                    unsafe { (*p).offset += depth };
                }
                print_string_ptr(unsafe { (*child).string } as *const i8, p);
                unsafe { (*p).offset = update(p as *const Printbuffer) };
                len = if fmt != 0 { 2 } else { 1 };
                ptr = ensure(p, len);
                if (ptr).is_null() as i32 != 0 {
                    return core::ptr::null_mut();
                }
                unsafe {
                    *{
                        let __old = ptr;
                        ptr = unsafe { ptr.offset(1) };
                        __old
                    } = ':' as i32 as i8
                };
                if fmt != 0 {
                    unsafe {
                        *{
                            let __old = ptr;
                            ptr = unsafe { ptr.offset(1) };
                            __old
                        } = '\t' as i32 as i8
                    };
                }
                unsafe { (*p).offset += len };
                print_value(child, depth, fmt, p);
                unsafe { (*p).offset = update(p as *const Printbuffer) };
                len = if fmt != 0 { 1 } else { 0 }
                    + if !(unsafe { (*child).next }).is_null() {
                        1
                    } else {
                        0
                    };
                ptr = ensure(p, len + 1);
                if (ptr).is_null() as i32 != 0 {
                    return core::ptr::null_mut();
                }
                if !(unsafe { (*child).next }).is_null() {
                    unsafe {
                        *{
                            let __old = ptr;
                            ptr = unsafe { ptr.offset(1) };
                            __old
                        } = ',' as i32 as i8
                    };
                }
                if fmt != 0 {
                    unsafe {
                        *{
                            let __old = ptr;
                            ptr = unsafe { ptr.offset(1) };
                            __old
                        } = '\n' as i32 as i8
                    };
                }
                unsafe { *ptr = 0 as i8 };
                unsafe { (*p).offset += len };
                child = unsafe { (*child).next };
            }
            ptr = ensure(p, if fmt != 0 { depth + 1 } else { 2 });
            if (ptr).is_null() as i32 != 0 {
                return core::ptr::null_mut();
            }
            if fmt != 0 {
                {
                    i = 0;
                    '__b24: loop {
                        if !(i < depth - 1) {
                            break '__b24;
                        }
                        '__c24: loop {
                            unsafe {
                                *{
                                    let __old = ptr;
                                    ptr = unsafe { ptr.offset(1) };
                                    __old
                                } = '\t' as i32 as i8
                            };
                            break '__c24;
                        }
                        i += 1;
                    }
                }
            }
            unsafe {
                *{
                    let __old = ptr;
                    ptr = unsafe { ptr.offset(1) };
                    __old
                } = '}' as i32 as i8
            };
            unsafe { *ptr = 0 as i8 };
            out = unsafe { unsafe { (*p).buffer.offset(i as isize) } };
        } else {
            /// Allocate space for the names and the objects
            (entries = unsafe {
                c_json_malloc.unwrap()(
                    (numentries as u64).wrapping_mul(core::mem::size_of::<*mut i8>() as u64),
                )
            } as *mut *mut i8);
            if (entries).is_null() as i32 != 0 {
                return core::ptr::null_mut();
            }
            names = unsafe {
                c_json_malloc.unwrap()(
                    (numentries as u64).wrapping_mul(core::mem::size_of::<*mut i8>() as u64),
                )
            } as *mut *mut i8;
            if (names).is_null() as i32 != 0 {
                unsafe { c_json_free.unwrap()(entries as *mut ()) };
                return core::ptr::null_mut();
            }
            unsafe {
                __builtin___memset_chk(
                    entries as *mut (),
                    0,
                    (core::mem::size_of::<*mut i8>() as u64).wrapping_mul(numentries as u64),
                    unsafe { __builtin_object_size(entries as *const (), 0) },
                )
            };
            unsafe {
                __builtin___memset_chk(
                    names as *mut (),
                    0,
                    (core::mem::size_of::<*mut i8>() as u64).wrapping_mul(numentries as u64),
                    unsafe { __builtin_object_size(names as *const (), 0) },
                )
            };

            /// Collect all the results into our arrays:
            (child = (*item).child);
            depth += 1;
            if fmt != 0 {
                len += depth;
            }
            while !(child).is_null() {
                unsafe {
                    *names.offset(i as isize) = {
                        str = print_string_ptr(
                            unsafe { (*child).string } as *const i8,
                            core::ptr::null_mut(),
                        );
                        str
                    }
                };
                unsafe {
                    *entries.offset({
                        let __old = i;
                        i += 1;
                        __old
                    } as isize) = {
                        ret = print_value(child, depth, fmt, core::ptr::null_mut());
                        ret
                    }
                };
                if !(str).is_null() && !(ret).is_null() {
                    len += unsafe {
                        strlen(ret as *const i8)
                            .wrapping_add(unsafe { strlen(str as *const i8) })
                            .wrapping_add(2 as u64)
                            .wrapping_add(if fmt != 0 { 2 + depth } else { 0 } as u64)
                    } as i32;
                } else {
                    fail = 1;
                }
                child = unsafe { (*child).next };
            }
            if (fail == 0) as i32 != 0 {
                out = unsafe { c_json_malloc.unwrap()(len as u64) } as *mut i8;
            }
            if (out).is_null() as i32 != 0 {
                fail = 1;
            }
            if fail != 0 {
                {
                    i = 0;
                    '__b26: loop {
                        if !(i < numentries) {
                            break '__b26;
                        }
                        '__c26: loop {
                            if !(unsafe { *names.offset(i as isize) }).is_null() {
                                unsafe {
                                    c_json_free.unwrap()(
                                        unsafe { *names.offset(i as isize) } as *mut ()
                                    )
                                };
                            }
                            if !(unsafe { *entries.offset(i as isize) }).is_null() {
                                unsafe {
                                    c_json_free.unwrap()(
                                        unsafe { *entries.offset(i as isize) } as *mut ()
                                    )
                                };
                            }
                            break '__c26;
                        }
                        i += 1;
                    }
                }
                unsafe { c_json_free.unwrap()(names as *mut ()) };
                unsafe { c_json_free.unwrap()(entries as *mut ()) };
                return core::ptr::null_mut();
            }

            /// Compose the output:
            unsafe {
                *out = '{' as i32 as i8
            };
            ptr = unsafe { out.offset(1 as isize) };
            if fmt != 0 {
                unsafe {
                    *{
                        let __old = ptr;
                        ptr = unsafe { ptr.offset(1) };
                        __old
                    } = '\n' as i32 as i8
                };
            }
            unsafe { *ptr = 0 as i8 };
            {
                i = 0;
                '__b27: loop {
                    if !(i < numentries) {
                        break '__b27;
                    }
                    '__c27: loop {
                        if fmt != 0 {
                            {
                                j = 0;
                                '__b28: loop {
                                    if !(j < depth) {
                                        break '__b28;
                                    }
                                    '__c28: loop {
                                        unsafe {
                                            *{
                                                let __old = ptr;
                                                ptr = unsafe { ptr.offset(1) };
                                                __old
                                            } = '\t' as i32 as i8
                                        };
                                        break '__c28;
                                    }
                                    j += 1;
                                }
                            }
                        }
                        tmplen =
                            unsafe { strlen(unsafe { *names.offset(i as isize) } as *const i8) };
                        unsafe {
                            __builtin___memcpy_chk(
                                ptr as *mut (),
                                unsafe { *names.offset(i as isize) } as *const (),
                                tmplen,
                                unsafe { __builtin_object_size(ptr as *const (), 0) },
                            )
                        };
                        {
                            let __n = tmplen;
                            let __p = &mut ptr;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                        unsafe {
                            *{
                                let __old = ptr;
                                ptr = unsafe { ptr.offset(1) };
                                __old
                            } = ':' as i32 as i8
                        };
                        if fmt != 0 {
                            unsafe {
                                *{
                                    let __old = ptr;
                                    ptr = unsafe { ptr.offset(1) };
                                    __old
                                } = '\t' as i32 as i8
                            };
                        }
                        unsafe {
                            __builtin___strcpy_chk(
                                ptr,
                                unsafe { *entries.offset(i as isize) } as *const i8,
                                unsafe {
                                    __builtin_object_size(
                                        ptr as *const (),
                                        if 2 > 1 { 1 } else { 0 },
                                    )
                                },
                            )
                        };
                        {
                            let __n = unsafe {
                                strlen(unsafe { *entries.offset(i as isize) } as *const i8)
                            };
                            let __p = &mut ptr;
                            *__p = unsafe { (*__p).add(__n as usize) };
                        };
                        if i != numentries - 1 {
                            unsafe {
                                *{
                                    let __old = ptr;
                                    ptr = unsafe { ptr.offset(1) };
                                    __old
                                } = ',' as i32 as i8
                            };
                        }
                        if fmt != 0 {
                            unsafe {
                                *{
                                    let __old = ptr;
                                    ptr = unsafe { ptr.offset(1) };
                                    __old
                                } = '\n' as i32 as i8
                            };
                        }
                        unsafe { *ptr = 0 as i8 };
                        unsafe {
                            c_json_free.unwrap()(unsafe { *names.offset(i as isize) } as *mut ())
                        };
                        unsafe {
                            c_json_free.unwrap()(unsafe { *entries.offset(i as isize) } as *mut ())
                        };
                        break '__c27;
                    }
                    i += 1;
                }
            }
            unsafe { c_json_free.unwrap()(names as *mut ()) };
            unsafe { c_json_free.unwrap()(entries as *mut ()) };
            if fmt != 0 {
                {
                    i = 0;
                    '__b29: loop {
                        if !(i < depth - 1) {
                            break '__b29;
                        }
                        '__c29: loop {
                            unsafe {
                                *{
                                    let __old = ptr;
                                    ptr = unsafe { ptr.offset(1) };
                                    __old
                                } = '\t' as i32 as i8
                            };
                            break '__c29;
                        }
                        i += 1;
                    }
                }
            }
            unsafe {
                *{
                    let __old = ptr;
                    ptr = unsafe { ptr.offset(1) };
                    __old
                } = '}' as i32 as i8
            };
            unsafe {
                *{
                    let __old = ptr;
                    ptr = unsafe { ptr.offset(1) };
                    __old
                } = 0 as i8
            };
        }
        return out;
    }
}

extern "C" fn c_json_strdup(str: *const i8) -> *mut i8 {
    unsafe {
        let mut len: u64 = 0 as u64;
        let mut copy: *mut i8 = core::ptr::null_mut();
        len = unsafe { strlen(str).wrapping_add(1 as u64) };
        if ({
            copy = unsafe { c_json_malloc.unwrap()(len) } as *mut i8;
            copy
        })
        .is_null() as i32
            != 0
        {
            return core::ptr::null_mut();
        }
        unsafe {
            __builtin___memcpy_chk(copy as *mut (), str as *const (), len, unsafe {
                __builtin_object_size(copy as *const (), 0)
            })
        };
        return copy;
    }
}

/// Render a value to text.
extern "C" fn print_value(item: *mut CJSON, depth: i32, fmt: i32, p: *mut Printbuffer) -> *mut i8 {
    let mut out: *mut i8 = core::ptr::null_mut();
    if (item).is_null() as i32 != 0 {
        return core::ptr::null_mut();
    }
    if !(p).is_null() {
        '__s30: {
            match unsafe { (*item).type_ } & 255 {
                2 => {
                    {
                        out = ensure(p, 5);
                        if !(out).is_null() {
                            unsafe {
                                __builtin___strcpy_chk(
                                    out,
                                    c"null".as_ptr() as *mut i8 as *const i8,
                                    unsafe {
                                        __builtin_object_size(
                                            out as *const (),
                                            if 2 > 1 { 1 } else { 0 },
                                        )
                                    },
                                )
                            };
                        }
                        break '__s30;
                    }
                    {
                        out = ensure(p, 6);
                        if !(out).is_null() {
                            unsafe {
                                __builtin___strcpy_chk(
                                    out,
                                    c"false".as_ptr() as *mut i8 as *const i8,
                                    unsafe {
                                        __builtin_object_size(
                                            out as *const (),
                                            if 2 > 1 { 1 } else { 0 },
                                        )
                                    },
                                )
                            };
                        }
                        break '__s30;
                    }
                    {
                        out = ensure(p, 5);
                        if !(out).is_null() {
                            unsafe {
                                __builtin___strcpy_chk(
                                    out,
                                    c"true".as_ptr() as *mut i8 as *const i8,
                                    unsafe {
                                        __builtin_object_size(
                                            out as *const (),
                                            if 2 > 1 { 1 } else { 0 },
                                        )
                                    },
                                )
                            };
                        }
                        break '__s30;
                    }
                    out = print_number(unsafe { &*item }, p);
                }
                0 => {
                    {
                        out = ensure(p, 6);
                        if !(out).is_null() {
                            unsafe {
                                __builtin___strcpy_chk(
                                    out,
                                    c"false".as_ptr() as *mut i8 as *const i8,
                                    unsafe {
                                        __builtin_object_size(
                                            out as *const (),
                                            if 2 > 1 { 1 } else { 0 },
                                        )
                                    },
                                )
                            };
                        }
                        break '__s30;
                    }
                    {
                        out = ensure(p, 5);
                        if !(out).is_null() {
                            unsafe {
                                __builtin___strcpy_chk(
                                    out,
                                    c"true".as_ptr() as *mut i8 as *const i8,
                                    unsafe {
                                        __builtin_object_size(
                                            out as *const (),
                                            if 2 > 1 { 1 } else { 0 },
                                        )
                                    },
                                )
                            };
                        }
                        break '__s30;
                    }
                    out = print_number(unsafe { &*item }, p);
                }
                1 => {
                    {
                        out = ensure(p, 5);
                        if !(out).is_null() {
                            unsafe {
                                __builtin___strcpy_chk(
                                    out,
                                    c"true".as_ptr() as *mut i8 as *const i8,
                                    unsafe {
                                        __builtin_object_size(
                                            out as *const (),
                                            if 2 > 1 { 1 } else { 0 },
                                        )
                                    },
                                )
                            };
                        }
                        break '__s30;
                    }
                    out = print_number(unsafe { &*item }, p);
                }
                3 => {
                    out = print_number(unsafe { &*item }, p);
                }
                4 => {
                    out = print_string(unsafe { &*item }, p);
                }
                5 => {
                    out = unsafe { print_array(unsafe { &*item }, depth, fmt, p) };
                }
                6 => {
                    out = unsafe { print_object(unsafe { &*item }, depth, fmt, p) };
                }
                _ => {}
            }
        }
    } else {
        '__s31: {
            match unsafe { (*item).type_ } & 255 {
                2 => {
                    out = c_json_strdup(c"null".as_ptr() as *mut i8 as *const i8);
                }
                0 => {
                    out = c_json_strdup(c"false".as_ptr() as *mut i8 as *const i8);
                }
                1 => {
                    out = c_json_strdup(c"true".as_ptr() as *mut i8 as *const i8);
                }
                3 => {
                    out = print_number(unsafe { &*item }, core::ptr::null_mut());
                }
                4 => {
                    out = print_string(unsafe { &*item }, core::ptr::null_mut());
                }
                5 => {
                    out = unsafe {
                        print_array(unsafe { &*item }, depth, fmt, core::ptr::null_mut())
                    };
                }
                6 => {
                    out = unsafe {
                        print_object(unsafe { &*item }, depth, fmt, core::ptr::null_mut())
                    };
                }
                _ => {}
            }
        }
    }
    return out;
}

/// Render a cJSON entity to text for transfer/storage. Free the char* when finished.
pub(crate) extern "C" fn c_json_print(item: *mut CJSON) -> *mut i8 {
    return unsafe { print_value(item, 0, 1, core::ptr::null_mut()) };
}

/// Render a cJSON entity to text for transfer/storage without any formatting. Free the char* when finished.
pub(crate) extern "C" fn c_json_print_unformatted(item: *mut CJSON) -> *mut i8 {
    return unsafe { print_value(item, 0, 0, core::ptr::null_mut()) };
}

/// Render a cJSON entity to text using a buffered strategy. prebuffer is a guess at the final size. guessing well reduces reallocation. fmt=0 gives unformatted, =1 gives formatted
pub(crate) extern "C" fn c_json_print_buffered(
    item: *mut CJSON,
    prebuffer: i32,
    fmt: i32,
) -> *mut i8 {
    unsafe {
        let mut p: Printbuffer = Printbuffer::default();
        p.buffer = unsafe { c_json_malloc.unwrap()(prebuffer as u64) } as *mut i8;
        p.length = prebuffer;
        p.offset = 0;
        return unsafe { print_value(item, 0, fmt, &mut p) };
        return p.buffer;
    }
}

/// Returns the number of items in an array (or object).
pub(crate) extern "C" fn c_json_get_array_size(array: &CJSON) -> i32 {
    let mut c: *const CJSON = (*array).child as *const CJSON;
    let mut i: i32 = 0;
    while !(c).is_null() {
        {
            {
                let __old = i;
                i += 1;
                __old
            };
            c = unsafe { (*c).next }
        };
    }
    return i;
}

/// Retrieve item number "item" from array "array". Returns NULL if unsuccessful.
pub(crate) extern "C" fn c_json_get_array_item(array: &CJSON, mut item: i32) -> *mut CJSON {
    let mut c: *mut CJSON = (*array).child;
    while !(c).is_null() && item > 0 {
        {
            {
                let __old = item;
                item -= 1;
                __old
            };
            c = unsafe { (*c).next }
        };
    }
    return c;
}

extern "C" fn c_json_strcasecmp(mut s1: *const i8, mut s2: *const i8) -> i32 {
    if (s1).is_null() as i32 != 0 {
        return if s1 == s2 { 0 } else { 1 };
    }
    if (s2).is_null() as i32 != 0 {
        return 1;
    }
    {
        '__b34: loop {
            if !(unsafe { tolower(unsafe { *s1 } as i32) }
                == unsafe { tolower(unsafe { *s2 } as i32) })
            {
                break '__b34;
            }
            '__c34: loop {
                if unsafe { *s1 } as i32 == 0 {
                    return 0;
                }
                break '__c34;
            }
            {
                {
                    s1 = unsafe { s1.offset(1) };
                    s1
                };
                {
                    s2 = unsafe { s2.offset(1) };
                    s2
                }
            };
        }
    }
    return unsafe { tolower(unsafe { *(s1 as *const u8) } as i32) }
        - unsafe { tolower(unsafe { *(s2 as *const u8) } as i32) };
}

/// Get item "string" from object. Case insensitive.
pub(crate) extern "C" fn c_json_get_object_item(object: &CJSON, string: *const i8) -> *mut CJSON {
    let mut c: *mut CJSON = (*object).child;
    while !(c).is_null() && c_json_strcasecmp(unsafe { (*c).string } as *const i8, string) != 0 {
        c = unsafe { (*c).next };
    }
    return c;
}

/// For analysing failed parses. This returns a pointer to the parse error. You'll probably need to look a few chars back to make sense of it. Defined when cJSON_Parse() returns 0. 0 when cJSON_Parse() succeeds.
pub(crate) extern "C" fn c_json_get_error_ptr() -> *const i8 {
    unsafe {
        return ep;
    }
}

/// These calls create a cJSON item of the appropriate type.
pub(crate) extern "C" fn c_json_create_null() -> *mut CJSON {
    let item: *mut CJSON = c_json_new_item();
    if !(item).is_null() {
        unsafe { (*item).type_ = cJSON_NULL as i32 };
    }
    return item;
}

pub(crate) extern "C" fn c_json_create_true() -> *mut CJSON {
    let item: *mut CJSON = c_json_new_item();
    if !(item).is_null() {
        unsafe { (*item).type_ = cJSON_True as i32 };
    }
    return item;
}

pub(crate) extern "C" fn c_json_create_false() -> *mut CJSON {
    let item: *mut CJSON = c_json_new_item();
    if !(item).is_null() {
        unsafe { (*item).type_ = cJSON_False as i32 };
    }
    return item;
}

pub(crate) extern "C" fn c_json_create_bool(b: i32) -> *mut CJSON {
    let item: *mut CJSON = c_json_new_item();
    if !(item).is_null() {
        unsafe { (*item).type_ = if b != 0 { cJSON_True } else { cJSON_False } };
    }
    return item;
}

pub(crate) extern "C" fn c_json_create_number(num: f64) -> *mut CJSON {
    let item: *mut CJSON = c_json_new_item();
    if !(item).is_null() {
        unsafe { (*item).type_ = cJSON_Number as i32 };
        unsafe { (*item).valuedouble = num };
        unsafe { (*item).valueint = num as i32 };
    }
    return item;
}

pub(crate) extern "C" fn c_json_create_string(string: *const i8) -> *mut CJSON {
    let item: *mut CJSON = c_json_new_item();
    if !(item).is_null() {
        unsafe { (*item).type_ = cJSON_String as i32 };
        unsafe { (*item).valuestring = c_json_strdup(string) };
    }
    return item;
}

pub(crate) extern "C" fn c_json_create_array() -> *mut CJSON {
    let item: *mut CJSON = c_json_new_item();
    if !(item).is_null() {
        unsafe { (*item).type_ = cJSON_Array as i32 };
    }
    return item;
}

pub(crate) extern "C" fn c_json_create_object() -> *mut CJSON {
    let item: *mut CJSON = c_json_new_item();
    if !(item).is_null() {
        unsafe { (*item).type_ = cJSON_Object as i32 };
    }
    return item;
}

/// Utility for array list handling.
extern "C" fn suffix_object(prev: *mut CJSON, item: *mut CJSON) -> () {
    unsafe { (*prev).next = item };
    unsafe { (*item).prev = prev };
}

/// These utilities create an Array of count items.
pub(crate) extern "C" fn c_json_create_int_array(numbers: *const i32, count: i32) -> *mut CJSON {
    let mut i: i32 = 0;
    let mut n: *mut CJSON = core::ptr::null_mut();
    let mut p: *mut CJSON = core::ptr::null_mut();
    let a: *mut CJSON = c_json_create_array();
    {
        i = 0;
        '__b36: loop {
            if !(!(a).is_null() && i < count) {
                break '__b36;
            }
            '__c36: loop {
                n = c_json_create_number(unsafe { *numbers.offset(i as isize) } as f64);
                if (i == 0) as i32 != 0 {
                    unsafe { (*a).child = n };
                } else {
                    suffix_object(p, n);
                }
                p = n;
                break '__c36;
            }
            i += 1;
        }
    }
    return a;
}

pub(crate) extern "C" fn c_json_create_float_array(numbers: *const f32, count: i32) -> *mut CJSON {
    let mut i: i32 = 0;
    let mut n: *mut CJSON = core::ptr::null_mut();
    let mut p: *mut CJSON = core::ptr::null_mut();
    let a: *mut CJSON = c_json_create_array();
    {
        i = 0;
        '__b37: loop {
            if !(!(a).is_null() && i < count) {
                break '__b37;
            }
            '__c37: loop {
                n = c_json_create_number(unsafe { *numbers.offset(i as isize) } as f64);
                if (i == 0) as i32 != 0 {
                    unsafe { (*a).child = n };
                } else {
                    suffix_object(p, n);
                }
                p = n;
                break '__c37;
            }
            i += 1;
        }
    }
    return a;
}

pub(crate) extern "C" fn c_json_create_double_array(numbers: *const f64, count: i32) -> *mut CJSON {
    let mut i: i32 = 0;
    let mut n: *mut CJSON = core::ptr::null_mut();
    let mut p: *mut CJSON = core::ptr::null_mut();
    let a: *mut CJSON = c_json_create_array();
    {
        i = 0;
        '__b38: loop {
            if !(!(a).is_null() && i < count) {
                break '__b38;
            }
            '__c38: loop {
                n = c_json_create_number(unsafe { *numbers.offset(i as isize) });
                if (i == 0) as i32 != 0 {
                    unsafe { (*a).child = n };
                } else {
                    suffix_object(p, n);
                }
                p = n;
                break '__c38;
            }
            i += 1;
        }
    }
    return a;
}

pub(crate) extern "C" fn c_json_create_string_array(
    strings: *mut *const i8,
    count: i32,
) -> *mut CJSON {
    let mut i: i32 = 0;
    let mut n: *mut CJSON = core::ptr::null_mut();
    let mut p: *mut CJSON = core::ptr::null_mut();
    let a: *mut CJSON = c_json_create_array();
    {
        i = 0;
        '__b39: loop {
            if !(!(a).is_null() && i < count) {
                break '__b39;
            }
            '__c39: loop {
                n = c_json_create_string(unsafe { *strings.offset(i as isize) });
                if (i == 0) as i32 != 0 {
                    unsafe { (*a).child = n };
                } else {
                    suffix_object(p, n);
                }
                p = n;
                break '__c39;
            }
            i += 1;
        }
    }
    return a;
}

/// Append item to the specified array/object.
pub(crate) extern "C" fn c_json_add_item_to_array(array: &mut CJSON, item: *mut CJSON) -> () {
    let mut c: *mut CJSON = (*array).child;
    if (item).is_null() as i32 != 0 {
        return;
    }
    if (c).is_null() as i32 != 0 {
        (*array).child = item;
    } else {
        while !(c).is_null() && !(unsafe { (*c).next }).is_null() {
            c = unsafe { (*c).next };
        }
        suffix_object(c, item);
    }
}

pub(crate) extern "C" fn c_json_add_item_to_object(
    object: *mut CJSON,
    string: *const i8,
    item: *mut CJSON,
) -> () {
    unsafe {
        if (item).is_null() as i32 != 0 {
            return;
        }
        if !(unsafe { (*item).string }).is_null() {
            unsafe { c_json_free.unwrap()(unsafe { (*item).string } as *mut ()) };
        }
        unsafe { (*item).string = c_json_strdup(string) };
        c_json_add_item_to_array(unsafe { &mut *object }, item);
    }
}

pub(crate) extern "C" fn c_json_add_item_to_object_cs(
    object: *mut CJSON,
    string: *const i8,
    item: *mut CJSON,
) -> () {
    unsafe {
        if (item).is_null() as i32 != 0 {
            return;
        }
        if (unsafe { (*item).type_ } & cJSON_StringIsConst as i32 == 0) as i32 != 0
            && !(unsafe { (*item).string }).is_null()
        {
            unsafe { c_json_free.unwrap()(unsafe { (*item).string } as *mut ()) };
        }
        unsafe { (*item).string = string as *mut i8 };
        unsafe { (*item).type_ |= cJSON_StringIsConst as i32 };
        c_json_add_item_to_array(unsafe { &mut *object }, item);
    }
}

/// Utility for handling references.
extern "C" fn create_reference(item: *const CJSON) -> *mut CJSON {
    let ref_: *mut CJSON = c_json_new_item();
    if (ref_).is_null() as i32 != 0 {
        return core::ptr::null_mut();
    }
    unsafe {
        __builtin___memcpy_chk(
            ref_ as *mut (),
            item as *const (),
            core::mem::size_of::<CJSON>() as u64,
            unsafe { __builtin_object_size(ref_ as *const (), 0) },
        )
    };
    unsafe { (*ref_).string = core::ptr::null_mut() };
    unsafe { (*ref_).type_ |= cJSON_IsReference as i32 };
    unsafe {
        (*ref_).next = {
            unsafe { (*ref_).prev = core::ptr::null_mut() };
            unsafe { (*ref_).prev }
        }
    };
    return ref_;
}

/// Append reference to item to the specified array/object. Use this when you want to add an existing cJSON to a new cJSON, but don't want to corrupt your existing cJSON.
pub(crate) extern "C" fn c_json_add_item_reference_to_array(
    array: *mut CJSON,
    item: *mut CJSON,
) -> () {
    c_json_add_item_to_array(
        unsafe { &mut *array },
        create_reference(item as *const CJSON),
    );
}

pub(crate) extern "C" fn c_json_add_item_reference_to_object(
    object: *mut CJSON,
    string: *const i8,
    item: *mut CJSON,
) -> () {
    c_json_add_item_to_object(object, string, create_reference(item as *const CJSON));
}

/// Remove/Detatch items from Arrays/Objects.
pub(crate) extern "C" fn c_json_detach_item_from_array(
    array: &mut CJSON,
    mut which: i32,
) -> *mut CJSON {
    let mut c: *mut CJSON = (*array).child;
    while !(c).is_null() && which > 0 {
        {
            c = unsafe { (*c).next };
            {
                let __old = which;
                which -= 1;
                __old
            }
        };
    }
    if (c).is_null() as i32 != 0 {
        return core::ptr::null_mut();
    }
    if !(unsafe { (*c).prev }).is_null() {
        unsafe { (*unsafe { (*c).prev }).next = unsafe { (*c).next } };
    }
    if !(unsafe { (*c).next }).is_null() {
        unsafe { (*unsafe { (*c).next }).prev = unsafe { (*c).prev } };
    }
    if c == (*array).child {
        (*array).child = unsafe { (*c).next };
    }
    unsafe {
        (*c).prev = {
            unsafe { (*c).next = core::ptr::null_mut() };
            unsafe { (*c).next }
        }
    };
    return c;
}

pub(crate) extern "C" fn c_json_delete_item_from_array(array: *mut CJSON, which: i32) -> () {
    c_json_delete(c_json_detach_item_from_array(unsafe { &mut *array }, which));
}

pub(crate) extern "C" fn c_json_detach_item_from_object(
    object: *mut CJSON,
    string: *const i8,
) -> *mut CJSON {
    let mut i: i32 = 0;
    let mut c: *const CJSON = unsafe { (*object).child } as *const CJSON;
    while !(c).is_null() && c_json_strcasecmp(unsafe { (*c).string } as *const i8, string) != 0 {
        {
            {
                let __old = i;
                i += 1;
                __old
            };
            c = unsafe { (*c).next }
        };
    }
    if !(c).is_null() {
        return c_json_detach_item_from_array(unsafe { &mut *object }, i);
    }
    return core::ptr::null_mut();
}

pub(crate) extern "C" fn c_json_delete_item_from_object(
    object: *mut CJSON,
    string: *const i8,
) -> () {
    c_json_delete(c_json_detach_item_from_object(object, string));
}

/// Update array items.
pub(crate) extern "C" fn c_json_insert_item_in_array(
    array: *mut CJSON,
    mut which: i32,
    newitem: *mut CJSON,
) -> () {
    let mut c: *mut CJSON = unsafe { (*array).child };
    while !(c).is_null() && which > 0 {
        {
            c = unsafe { (*c).next };
            {
                let __old = which;
                which -= 1;
                __old
            }
        };
    }
    if (c).is_null() as i32 != 0 {
        c_json_add_item_to_array(unsafe { &mut *array }, newitem);
        return;
    }
    unsafe { (*newitem).next = c };
    unsafe { (*newitem).prev = unsafe { (*c).prev } };
    unsafe { (*c).prev = newitem };
    if c == unsafe { (*array).child } {
        unsafe { (*array).child = newitem };
    } else {
        unsafe { (*unsafe { (*newitem).prev }).next = newitem };
    }
}

pub(crate) extern "C" fn c_json_replace_item_in_array(
    array: &mut CJSON,
    mut which: i32,
    newitem: *mut CJSON,
) -> () {
    let mut c: *mut CJSON = (*array).child;
    while !(c).is_null() && which > 0 {
        {
            c = unsafe { (*c).next };
            {
                let __old = which;
                which -= 1;
                __old
            }
        };
    }
    if (c).is_null() as i32 != 0 {
        return;
    }
    unsafe { (*newitem).next = unsafe { (*c).next } };
    unsafe { (*newitem).prev = unsafe { (*c).prev } };
    if !(unsafe { (*newitem).next }).is_null() {
        unsafe { (*unsafe { (*newitem).next }).prev = newitem };
    }
    if c == (*array).child {
        (*array).child = newitem;
    } else {
        unsafe { (*unsafe { (*newitem).prev }).next = newitem };
    }
    unsafe {
        (*c).next = {
            unsafe { (*c).prev = core::ptr::null_mut() };
            unsafe { (*c).prev }
        }
    };
    c_json_delete(c);
}

pub(crate) extern "C" fn c_json_replace_item_in_object(
    object: *mut CJSON,
    string: *const i8,
    newitem: *mut CJSON,
) -> () {
    let mut i: i32 = 0;
    let mut c: *const CJSON = unsafe { (*object).child } as *const CJSON;
    while !(c).is_null() && c_json_strcasecmp(unsafe { (*c).string } as *const i8, string) != 0 {
        {
            {
                let __old = i;
                i += 1;
                __old
            };
            c = unsafe { (*c).next }
        };
    }
    if !(c).is_null() {
        unsafe { (*newitem).string = c_json_strdup(string) };
        c_json_replace_item_in_array(unsafe { &mut *object }, i, newitem);
    }
}

/// Duplicate a cJSON item
#[allow(unused_doc_comments)]
pub(crate) extern "C" fn c_json_duplicate(item: *mut CJSON, recurse: i32) -> *mut CJSON {
    let mut newitem: *mut CJSON = core::ptr::null_mut();
    let mut cptr: *mut CJSON = core::ptr::null_mut();
    let mut nptr: *mut CJSON = core::ptr::null_mut();
    let mut newchild: *mut CJSON = core::ptr::null_mut();
    if (item).is_null() as i32 != 0 {
        return core::ptr::null_mut();
    }

    /// Create new item
    (newitem = c_json_new_item());
    if (newitem).is_null() as i32 != 0 {
        return core::ptr::null_mut();
    }

    /// Copy over all vars
    {
        ({
            unsafe { (*newitem).type_ = unsafe { (*item).type_ } & !cJSON_IsReference };
            unsafe { (*newitem).valueint = unsafe { (*item).valueint } }
        }) as f64;
        unsafe { (*newitem).valuedouble = unsafe { (*item).valuedouble } }
    };
    if !(unsafe { (*item).valuestring }).is_null() {
        unsafe {
            (*newitem).valuestring = c_json_strdup(unsafe { (*item).valuestring } as *const i8)
        };
        if (unsafe { (*newitem).valuestring }).is_null() as i32 != 0 {
            c_json_delete(newitem);
            return core::ptr::null_mut();
        }
    }
    if !(unsafe { (*item).string }).is_null() {
        unsafe { (*newitem).string = c_json_strdup(unsafe { (*item).string } as *const i8) };
        if (unsafe { (*newitem).string }).is_null() as i32 != 0 {
            c_json_delete(newitem);
            return core::ptr::null_mut();
        }
    }
    if (recurse == 0) as i32 != 0 {
        return newitem;
    }

    /// Walk the ->next chain for the child.
    (cptr = unsafe { (*item).child });
    while !(cptr).is_null() {
        newchild = c_json_duplicate(cptr, 1);
        if (newchild).is_null() as i32 != 0 {
            c_json_delete(newitem);
            return core::ptr::null_mut();
        }
        if !(nptr).is_null() {
            {
                unsafe { (*nptr).next = newchild };
                unsafe { (*newchild).prev = nptr }
            };
            nptr = newchild;
        } else {
            unsafe { (*newitem).child = newchild };
            nptr = newchild;
        }

        /// Set newitem->child and move to it
        (cptr = unsafe { (*cptr).next });
    }
    return newitem;
}

pub(crate) extern "C" fn c_json_minify(mut json: *mut i8) -> () {
    let mut into: *mut i8 = json;
    while unsafe { *json } != 0 {
        if unsafe { *json } as i32 == ' ' as i32 {
            {
                let __old = json;
                json = unsafe { json.offset(1) };
                __old
            };
        } else if unsafe { *json } as i32 == '\t' as i32 {
            {
                let __old = json;
                json = unsafe { json.offset(1) };
                __old
            };
        } else if unsafe { *json } as i32 == '\r' as i32 {
            {
                let __old = json;
                json = unsafe { json.offset(1) };
                __old
            };
        } else if unsafe { *json } as i32 == '\n' as i32 {
            {
                let __old = json;
                json = unsafe { json.offset(1) };
                __old
            };
        } else if unsafe { *json } as i32 == '/' as i32
            && unsafe { *json.offset(1 as isize) } as i32 == '/' as i32
        {
            while unsafe { *json } != 0 && unsafe { *json } as i32 != '\n' as i32 {
                {
                    let __old = json;
                    json = unsafe { json.offset(1) };
                    __old
                };
            }
        } else if unsafe { *json } as i32 == '/' as i32
            && unsafe { *json.offset(1 as isize) } as i32 == '*' as i32
        {
            while unsafe { *json } != 0
                && !(unsafe { *json } as i32 == '*' as i32
                    && unsafe { *json.offset(1 as isize) } as i32 == '/' as i32)
                    as i32
                    != 0
            {
                {
                    let __old = json;
                    json = unsafe { json.offset(1) };
                    __old
                };
            }
            {
                let __n = 2;
                let __p = &mut json;
                *__p = unsafe { (*__p).offset(__n as isize) };
            };
        } else if unsafe { *json } as i32 == '\"' as i32 {
            unsafe {
                *{
                    let __old = into;
                    into = unsafe { into.offset(1) };
                    __old
                } = unsafe {
                    *{
                        let __old = json;
                        json = unsafe { json.offset(1) };
                        __old
                    }
                }
            };
            while unsafe { *json } != 0 && unsafe { *json } as i32 != '\"' as i32 {
                if unsafe { *json } as i32 == '\\' as i32 {
                    unsafe {
                        *{
                            let __old = into;
                            into = unsafe { into.offset(1) };
                            __old
                        } = unsafe {
                            *{
                                let __old = json;
                                json = unsafe { json.offset(1) };
                                __old
                            }
                        }
                    };
                }
                unsafe {
                    *{
                        let __old = into;
                        into = unsafe { into.offset(1) };
                        __old
                    } = unsafe {
                        *{
                            let __old = json;
                            json = unsafe { json.offset(1) };
                            __old
                        }
                    }
                };
            }
            unsafe {
                *{
                    let __old = into;
                    into = unsafe { into.offset(1) };
                    __old
                } = unsafe {
                    *{
                        let __old = json;
                        json = unsafe { json.offset(1) };
                        __old
                    }
                }
            };
        } else {
            unsafe {
                *{
                    let __old = into;
                    into = unsafe { into.offset(1) };
                    __old
                } = unsafe {
                    *{
                        let __old = json;
                        json = unsafe { json.offset(1) };
                        __old
                    }
                }
            };
        }
    }
    unsafe { *into = 0 as i8 };
}
