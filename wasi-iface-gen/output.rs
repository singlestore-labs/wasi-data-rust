#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
extern crate wasi_iface_gen;
use wasi_iface_gen::*;
mod foo {
    pub struct Input {
        pub s: String,
        pub i: i64,
    }
    pub struct Output {
        pub a: i64,
        pub b: f64,
        pub c: String,
    }
    pub fn mapper(input: Input) -> Vec<Output> {
        Output {
            a: input.i * input.i,
            b: (input.i as f64) * 123.234,
            c: {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["hello "],
                    &match (&input.s,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
                res
            },
        }
    }
    #[export_name = "mapper"]
    unsafe extern "C" fn __witx_bindgen_mapper(arg0: i32, arg1: i32, arg2: i64) -> i32 {
        let len0 = arg1 as usize;
        let result1 = mapper(Input {
            s: String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap(),
            i: arg2,
        });
        let vec4 = result1;
        let len4 = vec4.len() as i32;
        let layout4 = core::alloc::Layout::from_size_align_unchecked(vec4.len() * 24, 8);
        let result4 = std::alloc::alloc(layout4);
        if result4.is_null() {
            std::alloc::handle_alloc_error(layout4);
        }
        for (i, e) in vec4.into_iter().enumerate() {
            let base = result4 as i32 + (i as i32) * 24;
            {
                let Output {
                    a: a2,
                    b: b2,
                    c: c2,
                } = e;
                *((base + 0) as *mut i64) = witx_bindgen_rust::rt::as_i64(a2);
                *((base + 8) as *mut f64) = witx_bindgen_rust::rt::as_f64(b2);
                let vec3 = (c2.into_bytes()).into_boxed_slice();
                let ptr3 = vec3.as_ptr() as i32;
                let len3 = vec3.len() as i32;
                core::mem::forget(vec3);
                *((base + 20) as *mut i32) = len3;
                *((base + 16) as *mut i32) = ptr3;
            }
        }
        let ptr5 = RET_AREA.as_mut_ptr() as i32;
        *((ptr5 + 8) as *mut i32) = len4;
        *((ptr5 + 0) as *mut i32) = result4 as i32;
        ptr5
    }
    static mut RET_AREA: [i64; 2] = [0; 2];
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const sanity: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("sanity"),
        ignore: false,
        allow_fail: false,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(sanity())),
};
fn sanity() {}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&sanity])
}
