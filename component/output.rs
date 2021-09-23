#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
mod component {
    #[export_name = "run"]
    unsafe extern "C" fn __witx_bindgen_run() {
        <super::Component as Component>::run();
    }
    pub trait Component {
        fn run();
    }
}
const _: &str = "run: function()";
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
}
