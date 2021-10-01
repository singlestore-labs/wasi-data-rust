// need to disable this lint for the export! macro below
#![allow(clippy::needless_question_mark)]

use anyhow::Result;
use wasmer::*;
use wasmer_wasi::WasiState;

witx_bindgen_wasmer::export!({
    src["component"]: "
        record SimpleValue {
            i: s64,
        }

        square: function(input: SimpleValue) -> list<SimpleValue>

        record SplitInput {
            s: string,
            delimiter: string,
        }

        record SplitOutput {
            c: string,
        }

        split: function(input: SplitInput) -> list<SplitOutput>

        record User {
            id: s64,
            username: string,
            email: string,
            phone: string,
        }

        filter_out_bad_users: function(input: User) -> list<User>

        record HilbertInput {
            vec: list<u8>,
            min_value: f64,
            max_value: f64,
            scale: f64,
        }

        record HilbertOutput {
            idx: string,
        }

        hilbert_encode: function(input: HilbertInput) -> list<HilbertOutput>
    "
});

fn vector_pack(input: &[f32]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len() * 4);
    for f in input {
        output.extend_from_slice(&f32::to_le_bytes(*f));
    }
    output
}

pub fn main() -> Result<()> {
    // Create a store with the default engine and compiler.
    let store = Store::default();

    // Compile the component wasm module
    let module = Module::from_file(&store, "target/wasm32-wasi/debug/example_wasm.wasm")?;

    // Create a WASI environment for the module.
    let mut wasi_env = WasiState::new("hello").finalize()?;
    let mut imports = wasi_env.import_object(&module)?;

    // Instantiate the module and extract a witx exported interface to it.
    let (exports, _instance) = component::Component::instantiate(&store, &module, &mut imports)?;

    let input = component::SimpleValue { i: 10 };
    let out = exports.square(input)?;
    println!("got: {:?}", out);

    let input = component::SplitInput {
        s: "hello, how, are, you",
        delimiter: ", ",
    };
    let out = exports.split(input)?;

    println!("got: {:?}", out);

    let users = vec![
        component::UserParam {
            id: 1,
            username: "alice",
            email: "foo@example.com",
            phone: "555-123-4567",
        },
        component::UserParam {
            id: 2,
            username: "lucy",
            email: "lucy@singlestore.com",
            phone: "555-123-4567",
        },
        component::UserParam {
            id: 3,
            username: "jones",
            email: "jones@example.net",
            phone: "555-123-4567",
        },
        component::UserParam {
            id: 4,
            username: "bob",
            email: "bob@gmail.com",
            phone: "555-123-4567",
        },
    ];

    let mut good_users = vec![];
    for user in users {
        let result = exports.filter_out_bad_users(user).unwrap();
        if !result.is_empty() {
            good_users.extend(result);
        }
    }

    println!("got: {:?}", good_users);

    let data = vec![12.0, -3.0, 5.0, 11.0, 22.0, -5.0, -6.0];
    let data_packed = vector_pack(&data);
    let input = component::HilbertInput {
        vec: &data_packed,
        min_value: -6.0,
        max_value: 22.0,
        scale: 100.0,
    };
    let out = exports.hilbert_encode(input)?;
    println!("got: {:?}", out);

    Ok(())
}
