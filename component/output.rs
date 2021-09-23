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
