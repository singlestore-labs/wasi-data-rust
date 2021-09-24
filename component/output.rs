#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
mod component {
    pub struct Input {
        pub body: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Input {
        #[inline]
        fn clone(&self) -> Input {
            match *self {
                Input {
                    body: ref __self_0_0,
                } => Input {
                    body: ::core::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    impl std::fmt::Debug for Input {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Input").field("body", &self.body).finish()
        }
    }
    pub struct Output {
        pub tag: String,
        pub translated: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Output {
        #[inline]
        fn clone(&self) -> Output {
            match *self {
                Output {
                    tag: ref __self_0_0,
                    translated: ref __self_0_1,
                } => Output {
                    tag: ::core::clone::Clone::clone(&(*__self_0_0)),
                    translated: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    impl std::fmt::Debug for Output {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Output")
                .field("tag", &self.tag)
                .field("translated", &self.translated)
                .finish()
        }
    }
    #[export_name = "run"]
    unsafe extern "C" fn __witx_bindgen_run() {
        <super::Component as Component>::run();
    }
    #[export_name = "process"]
    unsafe extern "C" fn __witx_bindgen_process(arg0: i32, arg1: i32) -> i32 {
        let len0 = arg1 as usize;
        let result1 = <super::Component as Component>::process(Input {
            body: String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap(),
        });
        let vec5 = result1;
        let len5 = vec5.len() as i32;
        let layout5 = core::alloc::Layout::from_size_align_unchecked(vec5.len() * 16, 4);
        let result5 = std::alloc::alloc(layout5);
        if result5.is_null() {
            std::alloc::handle_alloc_error(layout5);
        }
        for (i, e) in vec5.into_iter().enumerate() {
            let base = result5 as i32 + (i as i32) * 16;
            {
                let Output {
                    tag: tag2,
                    translated: translated2,
                } = e;
                let vec3 = (tag2.into_bytes()).into_boxed_slice();
                let ptr3 = vec3.as_ptr() as i32;
                let len3 = vec3.len() as i32;
                core::mem::forget(vec3);
                *((base + 4) as *mut i32) = len3;
                *((base + 0) as *mut i32) = ptr3;
                let vec4 = (translated2.into_bytes()).into_boxed_slice();
                let ptr4 = vec4.as_ptr() as i32;
                let len4 = vec4.len() as i32;
                core::mem::forget(vec4);
                *((base + 12) as *mut i32) = len4;
                *((base + 8) as *mut i32) = ptr4;
            }
        }
        let ptr6 = RET_AREA.as_mut_ptr() as i32;
        *((ptr6 + 8) as *mut i32) = len5;
        *((ptr6 + 0) as *mut i32) = result5 as i32;
        ptr6
    }
    pub trait Component {
        fn run();
        fn process(r: Input) -> Vec<Output>;
    }
    static mut RET_AREA: [i64; 2] = [0; 2];
}
const _ : & str = "run: function()\n\nrecord Input {\n    body: string\n}\n\nrecord Output {\n    tag: string,\n    translated: string\n}\n\nprocess: function(r: Input) -> list<Output>" ;
mod host {
    /// ## scalar types
    /// decimal types: f32 | f64
    /// integer types: s8 | u8 | s16 | u16 | s32 | u32 | s64 | u64
    /// other types: char
    /// ## structured types
    /// variant NAME { TAG(TYPE), TAG(TYPE), ... }
    /// record NAME { FIELD: TYPE, FIELD: TYPE, ... }
    /// list<TYPE>
    /// ## default type aliases
    /// string
    /// bool
    /// tuple<TYPE, TYPE, ...>
    /// flags NAME { FIELD, FIELD, ... }
    /// enum NAME { FIELD, FIELD, ... }
    /// union NAME { TYPE, TYPE, ... }
    /// option<TYPE>
    /// expected<TYPE, TYPE>
    /// ## other
    /// function(FIELD: TYPE, FIELD: TYPE, ...) [-> TYPE]
    /// resource NAME {
    /// [static] NAME: function(FIELD: TYPE, FIELD: TYPE, ...) [-> TYPE]
    /// NAME: function(FIELD: TYPE, FIELD: TYPE, ...) [-> TYPE]
    /// }
    /// type NAME = TYPE
    #[repr(transparent)]
    pub struct Row(i32);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Row {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Row(ref __self_0_0) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Row");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    impl Row {
        pub unsafe fn from_raw(raw: i32) -> Self {
            Self(raw)
        }
        pub fn into_raw(self) -> i32 {
            let ret = self.0;
            core::mem::forget(self);
            return ret;
        }
        pub fn as_raw(&self) -> i32 {
            self.0
        }
    }
    impl Drop for Row {
        fn drop(&mut self) {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_drop_Row"]
                fn close(fd: i32);
            }
            unsafe {
                close(self.0);
            }
        }
    }
    pub fn next() -> Row {
        unsafe {
            #[link(wasm_import_module = "host")]
            extern "C" {
                #[link_name = "next"]
                fn witx_import() -> i32;
            }
            let ret = witx_import();
            Row(ret)
        }
    }
    pub fn emit(r: &Row) {
        unsafe {
            #[link(wasm_import_module = "host")]
            extern "C" {
                #[link_name = "emit"]
                fn witx_import(_: i32);
            }
            witx_import(r.0);
        }
    }
}
const _ : & str = "// ## scalar types\n// decimal types: f32 | f64\n// integer types: s8 | u8 | s16 | u16 | s32 | u32 | s64 | u64\n// other types: char\n\n// ## structured types\n// variant NAME { TAG(TYPE), TAG(TYPE), ... }\n// record NAME { FIELD: TYPE, FIELD: TYPE, ... }\n// list<TYPE>\n\n// ## default type aliases\n// string\n// bool\n// tuple<TYPE, TYPE, ...>\n// flags NAME { FIELD, FIELD, ... }\n// enum NAME { FIELD, FIELD, ... }\n// union NAME { TYPE, TYPE, ... }\n// option<TYPE>\n// expected<TYPE, TYPE>\n\n// ## other\n// function(FIELD: TYPE, FIELD: TYPE, ...) [-> TYPE]\n// resource NAME {\n//   [static] NAME: function(FIELD: TYPE, FIELD: TYPE, ...) [-> TYPE]\n//   NAME: function(FIELD: TYPE, FIELD: TYPE, ...) [-> TYPE]\n// }\n// type NAME = TYPE\n\nresource Row\n\nnext: function() -> Row\nemit: function(r: Row)" ;
struct Component;
impl component::Component for Component {
    fn run() {
        let r = host::next();
        {
            ::std::io::_print(
                match match (&r,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
                } {
                    ref args => unsafe { ::core::fmt::Arguments::new_v1(&["", "\n"], args) },
                },
            );
        };
        host::emit(&r);
    }
    fn process(r: component::Input) -> Vec<component::Output> {
        let mut out = Vec::new();
        out.push(component::Output {
            tag: "english".to_string(),
            translated: r.body,
        });
        out
    }
}
