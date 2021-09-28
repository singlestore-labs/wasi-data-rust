extern crate wasi_iface_gen;
use wasi_iface_gen::*;

#[wasi_interface]
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
        vec![Output {
            a: input.i * input.i,
            b: (input.i as f64) * 123.234,
            c: format!("hello {}", input.s),
        }]
    }
}

#[test]
fn sanity() {}
