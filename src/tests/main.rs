use super::*;
use crate::src::c_json::{
    c_json_add_item_to_array, c_json_add_item_to_object, c_json_create_array, c_json_create_false,
    c_json_create_int_array, c_json_create_number, c_json_create_object, c_json_create_string,
    c_json_create_string_array, c_json_delete, c_json_get_error_ptr, c_json_parse, c_json_print,
};
use crate::src::c_json_h::CJSON;
use crate::{__assert_rtn, fclose, fopen, fread, free, fseek, ftell, malloc, printf, FILE};

pub(crate) extern "C" fn doit(text: *const i8) -> () {
    let mut out: *mut i8 = core::ptr::null_mut();
    let json: *mut CJSON = c_json_parse(text as *const i8);
    if !(json as *mut () != 0 as *mut ()) as i32 as i64 != 0 {
        unsafe {
            __assert_rtn(
                c"doit".as_ptr() as *const i8,
                c"main.c".as_ptr() as *mut i8 as *const i8,
                34,
                c"json != NULL".as_ptr() as *mut i8 as *const i8,
            )
        }
    } else {
        {
            let _ = 0;
        }
    };
    if (json).is_null() as i32 != 0 {
        unsafe {
            printf(
                c"Error before: [%s]\n".as_ptr() as *mut i8 as *const i8,
                c_json_get_error_ptr(),
            )
        };
    } else {
        out = c_json_print(json);
        c_json_delete(json);
        unsafe { printf(c"%s\n".as_ptr() as *mut i8 as *const i8, out) };
        unsafe { free(out as *mut ()) };
    }
}

pub(crate) extern "C" fn dofile(filename: *const i8) -> () {
    let mut f: *mut FILE = core::ptr::null_mut();
    let mut len: i64 = 0 as i64;
    let mut data: *mut i8 = core::ptr::null_mut();
    f = unsafe {
        fopen(
            filename as *const i8,
            c"rb".as_ptr() as *mut i8 as *const i8,
        )
    };
    unsafe { fseek(f, 0 as i64, 2) };
    len = unsafe { ftell(f) };
    unsafe { fseek(f, 0 as i64, 0) };
    data = unsafe { malloc((len + 1 as i64) as u64) } as *mut i8;
    unsafe { fread(data as *mut (), 1 as u64, len as u64, f) };
    unsafe { fclose(f) };
    doit(data as *const i8);
    unsafe { free(data as *mut ()) };
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Record {
    pub(crate) precision: *const i8,
    pub(crate) lat: f64,
    pub(crate) lon: f64,
    pub(crate) address: *const i8,
    pub(crate) city: *const i8,
    pub(crate) state: *const i8,
    pub(crate) zip: *const i8,
    pub(crate) country: *const i8,
}

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn create_objects() -> () {
    let mut root: *mut CJSON = core::ptr::null_mut();
    let mut fmt: *mut CJSON = core::ptr::null_mut();
    let mut img: *mut CJSON = core::ptr::null_mut();
    let mut thm: *mut CJSON = core::ptr::null_mut();
    let mut fld: *mut CJSON = core::ptr::null_mut();
    let mut out: *mut i8 = core::ptr::null_mut();
    let mut i: i32 = 0;
    let mut strings: [*const i8; 7] = [
        c"Sunday".as_ptr() as *const i8,
        c"Monday".as_ptr() as *const i8,
        c"Tuesday".as_ptr() as *const i8,
        c"Wednesday".as_ptr() as *const i8,
        c"Thursday".as_ptr() as *const i8,
        c"Friday".as_ptr() as *const i8,
        c"Saturday".as_ptr() as *const i8,
    ];
    let mut numbers: [[i32; 3]; 3] = [[0, -1, 0], [1, 0, 0], [0, 0, 1]];
    let mut ids: [i32; 4] = [116, 943, 234, 38793];
    /// Delete a cJSON structure.
    let fields: [Record; 2] = [
        Record {
            precision: c"zip".as_ptr() as *const i8,
            lat: 37.7668,
            lon: -122.3959,
            address: c"".as_ptr() as *const i8,
            city: c"SAN FRANCISCO".as_ptr() as *const i8,
            state: c"CA".as_ptr() as *const i8,
            zip: c"94107".as_ptr() as *const i8,
            country: c"US".as_ptr() as *const i8,
        },
        Record {
            precision: c"zip".as_ptr() as *const i8,
            lat: 37.371991,
            lon: -122.026,
            address: c"".as_ptr() as *const i8,
            city: c"SUNNYVALE".as_ptr() as *const i8,
            state: c"CA".as_ptr() as *const i8,
            zip: c"94085".as_ptr() as *const i8,
            country: c"US".as_ptr() as *const i8,
        },
    ];
    root = c_json_create_object();
    c_json_add_item_to_object(
        root,
        c"name".as_ptr() as *mut i8 as *const i8,
        c_json_create_string(c"Jack (\"Bee\") Nimble".as_ptr() as *mut i8 as *const i8),
    );
    c_json_add_item_to_object(root, c"format".as_ptr() as *mut i8 as *const i8, {
        fmt = c_json_create_object();
        fmt
    });
    c_json_add_item_to_object(
        fmt,
        c"type".as_ptr() as *mut i8 as *const i8,
        c_json_create_string(c"rect".as_ptr() as *mut i8 as *const i8),
    );
    c_json_add_item_to_object(
        fmt,
        c"width".as_ptr() as *mut i8 as *const i8,
        c_json_create_number(1920 as f64),
    );
    c_json_add_item_to_object(
        fmt,
        c"height".as_ptr() as *mut i8 as *const i8,
        c_json_create_number(1080 as f64),
    );
    c_json_add_item_to_object(
        fmt,
        c"interlace".as_ptr() as *mut i8 as *const i8,
        c_json_create_false(),
    );
    c_json_add_item_to_object(
        fmt,
        c"frame rate".as_ptr() as *mut i8 as *const i8,
        c_json_create_number(24 as f64),
    );

    /// Has sign?
    /// is zero
    (out = c_json_print(root));
    c_json_delete(root);
    unsafe { printf(c"%s\n".as_ptr() as *mut i8 as *const i8, out) };
    unsafe { free(out as *mut ()) };

    /// Number?
    (root = c_json_create_string_array(&raw mut strings[0 as usize] as *mut *const i8, 7));
    out = c_json_print(root);
    c_json_delete(root);
    unsafe { printf(c"%s\n".as_ptr() as *mut i8 as *const i8, out) };
    unsafe { free(out as *mut ()) };
    root = c_json_create_array();
    {
        i = 0;
        '__b51: loop {
            if !(i < 3) {
                break '__b51;
            }
            '__c51: loop {
                c_json_add_item_to_array(
                    unsafe { &mut *root },
                    c_json_create_int_array(
                        &raw mut numbers[i as usize][0 as usize] as *mut i32 as *const i32,
                        3,
                    ),
                );
                break '__c51;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }

    /// Number?
    (out = c_json_print(root));
    c_json_delete(root);
    unsafe { printf(c"%s\n".as_ptr() as *mut i8 as *const i8, out) };
    unsafe { free(out as *mut ()) };
    root = c_json_create_object();
    c_json_add_item_to_object(root, c"Image".as_ptr() as *mut i8 as *const i8, {
        img = c_json_create_object();
        img
    });
    c_json_add_item_to_object(
        img,
        c"Width".as_ptr() as *mut i8 as *const i8,
        c_json_create_number(800 as f64),
    );
    c_json_add_item_to_object(
        img,
        c"Height".as_ptr() as *mut i8 as *const i8,
        c_json_create_number(600 as f64),
    );
    c_json_add_item_to_object(
        img,
        c"Title".as_ptr() as *mut i8 as *const i8,
        c_json_create_string(c"View from 15th Floor".as_ptr() as *mut i8 as *const i8),
    );
    c_json_add_item_to_object(img, c"Thumbnail".as_ptr() as *mut i8 as *const i8, {
        thm = c_json_create_object();
        thm
    });
    c_json_add_item_to_object(
        thm,
        c"Url".as_ptr() as *mut i8 as *const i8,
        c_json_create_string(
            c"http:/*www.example.com/image/481989943".as_ptr() as *mut i8 as *const i8,
        ),
    );
    c_json_add_item_to_object(
        thm,
        c"Height".as_ptr() as *mut i8 as *const i8,
        c_json_create_number(125 as f64),
    );
    c_json_add_item_to_object(
        thm,
        c"Width".as_ptr() as *mut i8 as *const i8,
        c_json_create_string(c"100".as_ptr() as *mut i8 as *const i8),
    );
    c_json_add_item_to_object(
        img,
        c"IDs".as_ptr() as *mut i8 as *const i8,
        c_json_create_int_array(&raw mut ids[0 as usize] as *mut i32 as *const i32, 4),
    );
    out = c_json_print(root);
    c_json_delete(root);
    unsafe { printf(c"%s\n".as_ptr() as *mut i8 as *const i8, out) };
    unsafe { free(out as *mut ()) };
    root = c_json_create_array();
    {
        i = 0;
        '__b52: loop {
            if !(i < 2) {
                break '__b52;
            }
            '__c52: loop {
                c_json_add_item_to_array(unsafe { &mut *root }, {
                    fld = c_json_create_object();
                    fld
                });
                c_json_add_item_to_object(
                    fld,
                    c"precision".as_ptr() as *mut i8 as *const i8,
                    c_json_create_string(fields[i as usize].precision),
                );
                c_json_add_item_to_object(
                    fld,
                    c"Latitude".as_ptr() as *mut i8 as *const i8,
                    c_json_create_number(fields[i as usize].lat),
                );
                c_json_add_item_to_object(
                    fld,
                    c"Longitude".as_ptr() as *mut i8 as *const i8,
                    c_json_create_number(fields[i as usize].lon),
                );
                c_json_add_item_to_object(
                    fld,
                    c"Address".as_ptr() as *mut i8 as *const i8,
                    c_json_create_string(fields[i as usize].address),
                );
                c_json_add_item_to_object(
                    fld,
                    c"City".as_ptr() as *mut i8 as *const i8,
                    c_json_create_string(fields[i as usize].city),
                );
                c_json_add_item_to_object(
                    fld,
                    c"State".as_ptr() as *mut i8 as *const i8,
                    c_json_create_string(fields[i as usize].state),
                );
                c_json_add_item_to_object(
                    fld,
                    c"Zip".as_ptr() as *mut i8 as *const i8,
                    c_json_create_string(fields[i as usize].zip),
                );
                c_json_add_item_to_object(
                    fld,
                    c"Country".as_ptr() as *mut i8 as *const i8,
                    c_json_create_string(fields[i as usize].country),
                );
                break '__c52;
            }
            {
                let __p = &mut i;
                let __t = *__p;
                *__p += 1;
                __t
            };
        }
    }

    /// 2^64+1 can be represented in 21 chars.
    (out = c_json_print(root));
    c_json_delete(root);
    unsafe { printf(c"%s\n".as_ptr() as *mut i8 as *const i8, out) };
    unsafe { free(out as *mut ()) };
}

#[allow(unused_doc_comments)]
pub(crate) extern "C" fn __main_inner(argc: i32, argv: *const *const i8) -> Result<(), i32> {
    let mut text1: [i8; 153] = [
        123 as i8, 10 as i8, 34 as i8, 110 as i8, 97 as i8, 109 as i8, 101 as i8, 34 as i8,
        58 as i8, 32 as i8, 34 as i8, 74 as i8, 97 as i8, 99 as i8, 107 as i8, 32 as i8, 40 as i8,
        92 as i8, 34 as i8, 66 as i8, 101 as i8, 101 as i8, 92 as i8, 34 as i8, 41 as i8, 32 as i8,
        78 as i8, 105 as i8, 109 as i8, 98 as i8, 108 as i8, 101 as i8, 34 as i8, 44 as i8,
        32 as i8, 10 as i8, 34 as i8, 102 as i8, 111 as i8, 114 as i8, 109 as i8, 97 as i8,
        116 as i8, 34 as i8, 58 as i8, 32 as i8, 123 as i8, 34 as i8, 116 as i8, 121 as i8,
        112 as i8, 101 as i8, 34 as i8, 58 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
        32 as i8, 32 as i8, 34 as i8, 114 as i8, 101 as i8, 99 as i8, 116 as i8, 34 as i8,
        44 as i8, 32 as i8, 10 as i8, 34 as i8, 119 as i8, 105 as i8, 100 as i8, 116 as i8,
        104 as i8, 34 as i8, 58 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
        49 as i8, 57 as i8, 50 as i8, 48 as i8, 44 as i8, 32 as i8, 10 as i8, 34 as i8, 104 as i8,
        101 as i8, 105 as i8, 103 as i8, 104 as i8, 116 as i8, 34 as i8, 58 as i8, 32 as i8,
        32 as i8, 32 as i8, 32 as i8, 32 as i8, 49 as i8, 48 as i8, 56 as i8, 48 as i8, 44 as i8,
        32 as i8, 10 as i8, 34 as i8, 105 as i8, 110 as i8, 116 as i8, 101 as i8, 114 as i8,
        108 as i8, 97 as i8, 99 as i8, 101 as i8, 34 as i8, 58 as i8, 32 as i8, 32 as i8,
        102 as i8, 97 as i8, 108 as i8, 115 as i8, 101 as i8, 44 as i8, 34 as i8, 102 as i8,
        114 as i8, 97 as i8, 109 as i8, 101 as i8, 32 as i8, 114 as i8, 97 as i8, 116 as i8,
        101 as i8, 34 as i8, 58 as i8, 32 as i8, 50 as i8, 52 as i8, 10 as i8, 125 as i8, 10 as i8,
        125 as i8, 0 as i8,
    ];
    let mut text2: [i8; 79] = [
        91 as i8, 34 as i8, 83 as i8, 117 as i8, 110 as i8, 100 as i8, 97 as i8, 121 as i8,
        34 as i8, 44 as i8, 32 as i8, 34 as i8, 77 as i8, 111 as i8, 110 as i8, 100 as i8,
        97 as i8, 121 as i8, 34 as i8, 44 as i8, 32 as i8, 34 as i8, 84 as i8, 117 as i8,
        101 as i8, 115 as i8, 100 as i8, 97 as i8, 121 as i8, 34 as i8, 44 as i8, 32 as i8,
        34 as i8, 87 as i8, 101 as i8, 100 as i8, 110 as i8, 101 as i8, 115 as i8, 100 as i8,
        97 as i8, 121 as i8, 34 as i8, 44 as i8, 32 as i8, 34 as i8, 84 as i8, 104 as i8,
        117 as i8, 114 as i8, 115 as i8, 100 as i8, 97 as i8, 121 as i8, 34 as i8, 44 as i8,
        32 as i8, 34 as i8, 70 as i8, 114 as i8, 105 as i8, 100 as i8, 97 as i8, 121 as i8,
        34 as i8, 44 as i8, 32 as i8, 34 as i8, 83 as i8, 97 as i8, 116 as i8, 117 as i8,
        114 as i8, 100 as i8, 97 as i8, 121 as i8, 34 as i8, 93 as i8, 0 as i8,
    ];
    let mut text3: [i8; 51] = [
        91 as i8, 10 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 91 as i8, 48 as i8, 44 as i8,
        32 as i8, 45 as i8, 49 as i8, 44 as i8, 32 as i8, 48 as i8, 93 as i8, 44 as i8, 10 as i8,
        32 as i8, 32 as i8, 32 as i8, 32 as i8, 91 as i8, 49 as i8, 44 as i8, 32 as i8, 48 as i8,
        44 as i8, 32 as i8, 48 as i8, 93 as i8, 44 as i8, 10 as i8, 32 as i8, 32 as i8, 32 as i8,
        32 as i8, 91 as i8, 48 as i8, 44 as i8, 32 as i8, 48 as i8, 44 as i8, 32 as i8, 49 as i8,
        93 as i8, 10 as i8, 9 as i8, 93 as i8, 10 as i8, 0 as i8,
    ];
    let mut text4: [i8; 247] = [
        123 as i8, 10 as i8, 9 as i8, 9 as i8, 34 as i8, 73 as i8, 109 as i8, 97 as i8, 103 as i8,
        101 as i8, 34 as i8, 58 as i8, 32 as i8, 123 as i8, 10 as i8, 9 as i8, 9 as i8, 9 as i8,
        34 as i8, 87 as i8, 105 as i8, 100 as i8, 116 as i8, 104 as i8, 34 as i8, 58 as i8,
        32 as i8, 32 as i8, 56 as i8, 48 as i8, 48 as i8, 44 as i8, 10 as i8, 9 as i8, 9 as i8,
        9 as i8, 34 as i8, 72 as i8, 101 as i8, 105 as i8, 103 as i8, 104 as i8, 116 as i8,
        34 as i8, 58 as i8, 32 as i8, 54 as i8, 48 as i8, 48 as i8, 44 as i8, 10 as i8, 9 as i8,
        9 as i8, 9 as i8, 34 as i8, 84 as i8, 105 as i8, 116 as i8, 108 as i8, 101 as i8, 34 as i8,
        58 as i8, 32 as i8, 32 as i8, 34 as i8, 86 as i8, 105 as i8, 101 as i8, 119 as i8,
        32 as i8, 102 as i8, 114 as i8, 111 as i8, 109 as i8, 32 as i8, 49 as i8, 53 as i8,
        116 as i8, 104 as i8, 32 as i8, 70 as i8, 108 as i8, 111 as i8, 111 as i8, 114 as i8,
        34 as i8, 44 as i8, 10 as i8, 9 as i8, 9 as i8, 9 as i8, 34 as i8, 84 as i8, 104 as i8,
        117 as i8, 109 as i8, 98 as i8, 110 as i8, 97 as i8, 105 as i8, 108 as i8, 34 as i8,
        58 as i8, 32 as i8, 123 as i8, 10 as i8, 9 as i8, 9 as i8, 9 as i8, 9 as i8, 34 as i8,
        85 as i8, 114 as i8, 108 as i8, 34 as i8, 58 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
        34 as i8, 104 as i8, 116 as i8, 116 as i8, 112 as i8, 58 as i8, 47 as i8, 42 as i8,
        119 as i8, 119 as i8, 119 as i8, 46 as i8, 101 as i8, 120 as i8, 97 as i8, 109 as i8,
        112 as i8, 108 as i8, 101 as i8, 46 as i8, 99 as i8, 111 as i8, 109 as i8, 47 as i8,
        105 as i8, 109 as i8, 97 as i8, 103 as i8, 101 as i8, 47 as i8, 52 as i8, 56 as i8,
        49 as i8, 57 as i8, 56 as i8, 57 as i8, 57 as i8, 52 as i8, 51 as i8, 34 as i8, 44 as i8,
        10 as i8, 9 as i8, 9 as i8, 9 as i8, 9 as i8, 34 as i8, 72 as i8, 101 as i8, 105 as i8,
        103 as i8, 104 as i8, 116 as i8, 34 as i8, 58 as i8, 32 as i8, 49 as i8, 50 as i8,
        53 as i8, 44 as i8, 10 as i8, 9 as i8, 9 as i8, 9 as i8, 9 as i8, 34 as i8, 87 as i8,
        105 as i8, 100 as i8, 116 as i8, 104 as i8, 34 as i8, 58 as i8, 32 as i8, 32 as i8,
        34 as i8, 49 as i8, 48 as i8, 48 as i8, 34 as i8, 10 as i8, 9 as i8, 9 as i8, 9 as i8,
        125 as i8, 44 as i8, 10 as i8, 9 as i8, 9 as i8, 9 as i8, 34 as i8, 73 as i8, 68 as i8,
        115 as i8, 34 as i8, 58 as i8, 32 as i8, 91 as i8, 49 as i8, 49 as i8, 54 as i8, 44 as i8,
        32 as i8, 57 as i8, 52 as i8, 51 as i8, 44 as i8, 32 as i8, 50 as i8, 51 as i8, 52 as i8,
        44 as i8, 32 as i8, 51 as i8, 56 as i8, 55 as i8, 57 as i8, 51 as i8, 93 as i8, 10 as i8,
        9 as i8, 9 as i8, 125 as i8, 10 as i8, 9 as i8, 125 as i8, 0 as i8,
    ];
    let mut text5: [i8; 399] = [
        91 as i8, 10 as i8, 9 as i8, 32 as i8, 123 as i8, 10 as i8, 9 as i8, 32 as i8, 34 as i8,
        112 as i8, 114 as i8, 101 as i8, 99 as i8, 105 as i8, 115 as i8, 105 as i8, 111 as i8,
        110 as i8, 34 as i8, 58 as i8, 32 as i8, 34 as i8, 122 as i8, 105 as i8, 112 as i8,
        34 as i8, 44 as i8, 10 as i8, 9 as i8, 32 as i8, 34 as i8, 76 as i8, 97 as i8, 116 as i8,
        105 as i8, 116 as i8, 117 as i8, 100 as i8, 101 as i8, 34 as i8, 58 as i8, 32 as i8,
        32 as i8, 51 as i8, 55 as i8, 46 as i8, 55 as i8, 54 as i8, 54 as i8, 56 as i8, 44 as i8,
        10 as i8, 9 as i8, 32 as i8, 34 as i8, 76 as i8, 111 as i8, 110 as i8, 103 as i8,
        105 as i8, 116 as i8, 117 as i8, 100 as i8, 101 as i8, 34 as i8, 58 as i8, 32 as i8,
        45 as i8, 49 as i8, 50 as i8, 50 as i8, 46 as i8, 51 as i8, 57 as i8, 53 as i8, 57 as i8,
        44 as i8, 10 as i8, 9 as i8, 32 as i8, 34 as i8, 65 as i8, 100 as i8, 100 as i8, 114 as i8,
        101 as i8, 115 as i8, 115 as i8, 34 as i8, 58 as i8, 32 as i8, 32 as i8, 32 as i8,
        34 as i8, 34 as i8, 44 as i8, 10 as i8, 9 as i8, 32 as i8, 34 as i8, 67 as i8, 105 as i8,
        116 as i8, 121 as i8, 34 as i8, 58 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
        32 as i8, 34 as i8, 83 as i8, 65 as i8, 78 as i8, 32 as i8, 70 as i8, 82 as i8, 65 as i8,
        78 as i8, 67 as i8, 73 as i8, 83 as i8, 67 as i8, 79 as i8, 34 as i8, 44 as i8, 10 as i8,
        9 as i8, 32 as i8, 34 as i8, 83 as i8, 116 as i8, 97 as i8, 116 as i8, 101 as i8, 34 as i8,
        58 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 34 as i8, 67 as i8, 65 as i8,
        34 as i8, 44 as i8, 10 as i8, 9 as i8, 32 as i8, 34 as i8, 90 as i8, 105 as i8, 112 as i8,
        34 as i8, 58 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
        34 as i8, 57 as i8, 52 as i8, 49 as i8, 48 as i8, 55 as i8, 34 as i8, 44 as i8, 10 as i8,
        9 as i8, 32 as i8, 34 as i8, 67 as i8, 111 as i8, 117 as i8, 110 as i8, 116 as i8,
        114 as i8, 121 as i8, 34 as i8, 58 as i8, 32 as i8, 32 as i8, 32 as i8, 34 as i8, 85 as i8,
        83 as i8, 34 as i8, 10 as i8, 9 as i8, 32 as i8, 125 as i8, 44 as i8, 10 as i8, 9 as i8,
        32 as i8, 123 as i8, 10 as i8, 9 as i8, 32 as i8, 34 as i8, 112 as i8, 114 as i8,
        101 as i8, 99 as i8, 105 as i8, 115 as i8, 105 as i8, 111 as i8, 110 as i8, 34 as i8,
        58 as i8, 32 as i8, 34 as i8, 122 as i8, 105 as i8, 112 as i8, 34 as i8, 44 as i8,
        10 as i8, 9 as i8, 32 as i8, 34 as i8, 76 as i8, 97 as i8, 116 as i8, 105 as i8, 116 as i8,
        117 as i8, 100 as i8, 101 as i8, 34 as i8, 58 as i8, 32 as i8, 32 as i8, 51 as i8,
        55 as i8, 46 as i8, 51 as i8, 55 as i8, 49 as i8, 57 as i8, 57 as i8, 49 as i8, 44 as i8,
        10 as i8, 9 as i8, 32 as i8, 34 as i8, 76 as i8, 111 as i8, 110 as i8, 103 as i8,
        105 as i8, 116 as i8, 117 as i8, 100 as i8, 101 as i8, 34 as i8, 58 as i8, 32 as i8,
        45 as i8, 49 as i8, 50 as i8, 50 as i8, 46 as i8, 48 as i8, 50 as i8, 54 as i8, 48 as i8,
        50 as i8, 48 as i8, 44 as i8, 10 as i8, 9 as i8, 32 as i8, 34 as i8, 65 as i8, 100 as i8,
        100 as i8, 114 as i8, 101 as i8, 115 as i8, 115 as i8, 34 as i8, 58 as i8, 32 as i8,
        32 as i8, 32 as i8, 34 as i8, 34 as i8, 44 as i8, 10 as i8, 9 as i8, 32 as i8, 34 as i8,
        67 as i8, 105 as i8, 116 as i8, 121 as i8, 34 as i8, 58 as i8, 32 as i8, 32 as i8,
        32 as i8, 32 as i8, 32 as i8, 32 as i8, 34 as i8, 83 as i8, 85 as i8, 78 as i8, 78 as i8,
        89 as i8, 86 as i8, 65 as i8, 76 as i8, 69 as i8, 34 as i8, 44 as i8, 10 as i8, 9 as i8,
        32 as i8, 34 as i8, 83 as i8, 116 as i8, 97 as i8, 116 as i8, 101 as i8, 34 as i8,
        58 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 34 as i8, 67 as i8, 65 as i8,
        34 as i8, 44 as i8, 10 as i8, 9 as i8, 32 as i8, 34 as i8, 90 as i8, 105 as i8, 112 as i8,
        34 as i8, 58 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
        34 as i8, 57 as i8, 52 as i8, 48 as i8, 56 as i8, 53 as i8, 34 as i8, 44 as i8, 10 as i8,
        9 as i8, 32 as i8, 34 as i8, 67 as i8, 111 as i8, 117 as i8, 110 as i8, 116 as i8,
        114 as i8, 121 as i8, 34 as i8, 58 as i8, 32 as i8, 32 as i8, 32 as i8, 34 as i8, 85 as i8,
        83 as i8, 34 as i8, 10 as i8, 9 as i8, 32 as i8, 125 as i8, 10 as i8, 9 as i8, 32 as i8,
        93 as i8, 0 as i8,
    ];
    doit(&raw mut text1[0 as usize] as *mut i8 as *const i8);
    doit(&raw mut text2[0 as usize] as *mut i8 as *const i8);
    doit(&raw mut text3[0 as usize] as *mut i8 as *const i8);
    doit(&raw mut text4[0 as usize] as *mut i8 as *const i8);
    doit(&raw mut text5[0 as usize] as *mut i8 as *const i8);

    /// not a string!
    /// Skip escaped quotes.
    /// This is how long we need for the string, roughly.
    create_objects();
    return Ok(());
}
