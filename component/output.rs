#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use wasi_iface_gen::wasi_interface;
#[allow(unused_imports)]
use witx_bindgen_rust;
mod component {
    struct SimpleValue {
        i: i64,
    }
    fn double(input: SimpleValue) -> Vec<SimpleValue> {
        <[_]>::into_vec(box [SimpleValue { i: input.i * 2 }])
    }
    struct SplitInput {
        s: String,
        delimiter: String,
    }
    struct SplitOutput {
        c: String,
    }
    fn split(input: SplitInput) -> Vec<SplitOutput> {
        input
            .s
            .split(&input.delimiter)
            .map(|s| SplitOutput { c: s.to_string() })
            .collect()
    }
    struct User {
        id: i64,
        username: String,
        email: String,
        phone: String,
    }
    static BAD_DOMAINS: &[&str] = &["example.com", "example.net", "example.org"];
    fn filter_out_bad_users(input: User) -> Vec<User> {
        if BAD_DOMAINS
            .iter()
            .any(|domain| input.email.ends_with(domain))
        {
            ::alloc::vec::Vec::new()
        } else {
            <[_]>::into_vec(box [input])
        }
    }
    #[export_name = "double"]
    unsafe extern "C" fn __witx_bindgen_double(arg0: i64) -> i32 {
        let result0 = double(SimpleValue { i: arg0 });
        let vec1 = (result0).into_boxed_slice();
        let ptr1 = vec1.as_ptr() as i32;
        let len1 = vec1.len() as i32;
        core::mem::forget(vec1);
        let ptr2 = RET_AREA.as_mut_ptr() as i32;
        *((ptr2 + 8) as *mut i32) = len1;
        *((ptr2 + 0) as *mut i32) = ptr1;
        ptr2
    }
    #[export_name = "split"]
    unsafe extern "C" fn __witx_bindgen_split(arg0: i32, arg1: i32, arg2: i32, arg3: i32) -> i32 {
        let len0 = arg1 as usize;
        let len1 = arg3 as usize;
        let result2 = split(SplitInput {
            s: String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap(),
            delimiter: String::from_utf8(Vec::from_raw_parts(arg2 as *mut _, len1, len1)).unwrap(),
        });
        let vec5 = result2;
        let len5 = vec5.len() as i32;
        let layout5 = core::alloc::Layout::from_size_align_unchecked(vec5.len() * 8, 4);
        let result5 = std::alloc::alloc(layout5);
        if result5.is_null() {
            std::alloc::handle_alloc_error(layout5);
        }
        for (i, e) in vec5.into_iter().enumerate() {
            let base = result5 as i32 + (i as i32) * 8;
            {
                let SplitOutput { c: c3 } = e;
                let vec4 = (c3.into_bytes()).into_boxed_slice();
                let ptr4 = vec4.as_ptr() as i32;
                let len4 = vec4.len() as i32;
                core::mem::forget(vec4);
                *((base + 4) as *mut i32) = len4;
                *((base + 0) as *mut i32) = ptr4;
            }
        }
        let ptr6 = RET_AREA.as_mut_ptr() as i32;
        *((ptr6 + 8) as *mut i32) = len5;
        *((ptr6 + 0) as *mut i32) = result5 as i32;
        ptr6
    }
    #[export_name = "filter_out_bad_users"]
    unsafe extern "C" fn __witx_bindgen_filter_out_bad_users(
        arg0: i64,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
        arg6: i32,
    ) -> i32 {
        let len0 = arg2 as usize;
        let len1 = arg4 as usize;
        let len2 = arg6 as usize;
        let result3 = filter_out_bad_users(User {
            id: arg0,
            username: String::from_utf8(Vec::from_raw_parts(arg1 as *mut _, len0, len0)).unwrap(),
            email: String::from_utf8(Vec::from_raw_parts(arg3 as *mut _, len1, len1)).unwrap(),
            phone: String::from_utf8(Vec::from_raw_parts(arg5 as *mut _, len2, len2)).unwrap(),
        });
        let vec8 = result3;
        let len8 = vec8.len() as i32;
        let layout8 = core::alloc::Layout::from_size_align_unchecked(vec8.len() * 32, 8);
        let result8 = std::alloc::alloc(layout8);
        if result8.is_null() {
            std::alloc::handle_alloc_error(layout8);
        }
        for (i, e) in vec8.into_iter().enumerate() {
            let base = result8 as i32 + (i as i32) * 32;
            {
                let User {
                    id: id4,
                    username: username4,
                    email: email4,
                    phone: phone4,
                } = e;
                *((base + 0) as *mut i64) = witx_bindgen_rust::rt::as_i64(id4);
                let vec5 = (username4.into_bytes()).into_boxed_slice();
                let ptr5 = vec5.as_ptr() as i32;
                let len5 = vec5.len() as i32;
                core::mem::forget(vec5);
                *((base + 12) as *mut i32) = len5;
                *((base + 8) as *mut i32) = ptr5;
                let vec6 = (email4.into_bytes()).into_boxed_slice();
                let ptr6 = vec6.as_ptr() as i32;
                let len6 = vec6.len() as i32;
                core::mem::forget(vec6);
                *((base + 20) as *mut i32) = len6;
                *((base + 16) as *mut i32) = ptr6;
                let vec7 = (phone4.into_bytes()).into_boxed_slice();
                let ptr7 = vec7.as_ptr() as i32;
                let len7 = vec7.len() as i32;
                core::mem::forget(vec7);
                *((base + 28) as *mut i32) = len7;
                *((base + 24) as *mut i32) = ptr7;
            }
        }
        let ptr9 = RET_AREA.as_mut_ptr() as i32;
        *((ptr9 + 8) as *mut i32) = len8;
        *((ptr9 + 0) as *mut i32) = result8 as i32;
        ptr9
    }
    static mut RET_AREA: [i64; 2] = [0; 2];
}
