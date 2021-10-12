// need to disable this lint for the export! macro below
#![allow(clippy::needless_question_mark)]

use anyhow::Result;
use wasmtime::*;

witx_bindgen_wasmtime::export!({
    src["component"]: "
        record SimpleString {
            s: string,
        }

        record PolarityScores {
            compound: f64,
            positive: f64,
            negative: f64,
            neutral: f64,
        }

        sentiment: function(input: SimpleString) -> PolarityScores
        sentiment_vec: function(input: SimpleString) -> list<PolarityScores>
    "
});

pub struct Context {
    wasi: wasmtime_wasi::WasiCtx,
    exports: component::ComponentData,
}

// fn vector_pack(input: &[f32]) -> Vec<u8> {
//     let mut output = Vec::with_capacity(input.len() * 4);
//     for f in input {
//         output.extend_from_slice(&f32::to_le_bytes(*f));
//     }
//     output
// }

pub fn main() -> Result<()> {
    // Create an engine with caching enabled
    let mut config = Config::new();
    config.wasm_module_linking(true);
    config.cache_config_load_default()?;
    // config.debug_info(true);
    let engine = Engine::new(&config)?;

    // Compile the component wasm module
    let module = Module::from_file(
        &engine,
        "../../target/wasm32-wasi/release/example_wasm.wasm",
    )?;

    // Add the component's WASI/witx exports to the linker
    // For host-provided functions it's recommended to use a `Linker` which does
    // name-based resolution of functions.
    let mut linker = Linker::<Context>::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| &mut cx.wasi)?;

    // Instantiation always happens within a `Store`. This means to
    // actually instantiate with our `Linker` we'll need to create a store.
    // A Store is a collection of WebAssembly instances and host-defined state.
    //
    // Note that we're also initializing the store with our custom data here too.
    //
    // Afterwards we use the `linker` to create the instance.
    let mut store = Store::new(
        &engine,
        Context {
            wasi: wasmtime_wasi::sync::WasiCtxBuilder::new()
                .inherit_stdio()
                .build(),
            exports: component::ComponentData::default(),
        },
    );

    let (exports, _instance) =
        component::Component::instantiate(&mut store, &module, &mut linker, |cx| &mut cx.exports)?;

    let comments = vec![
        "I love singlestore!",
        "ham is not a good sandwich",
        "do you think we should go to the store today?",
        "meetings are the favorite part of my day",
        "bobs your uncle",
        "I really hate the beach on a rainy day...",
        "hello bob! you are looking great today!",
    ];

    for comment in &comments {
        let out = exports.sentiment(&mut store, component::SimpleString { s: comment })?;
        match out.compound {
            x if x > 0.05 => print!("'{}' is POSITIVE", comment),
            x if x < -0.05 => print!("'{}' is NEGATIVE", comment),
            _ => print!("'{}' is NEUTRAL", comment),
        }
        if out.positive > 0.75 || out.negative > 0.75 {
            println!(" (polarized)");
        } else {
            println!();
        }
    }

    for comment in &comments {
        let out = exports.sentiment_vec(&mut store, component::SimpleString { s: comment })?;
        let out = out[0];
        match out.compound {
            x if x > 0.05 => print!("'{}' is POSITIVE", comment),
            x if x < -0.05 => print!("'{}' is NEGATIVE", comment),
            _ => print!("'{}' is NEUTRAL", comment),
        }
        if out.positive > 0.75 || out.negative > 0.75 {
            println!(" (polarized)");
        } else {
            println!();
        }
    }

    // let input = component::SimpleValue { i: 10 };
    // let out = exports.square(&mut store, input)?;
    // println!("got: {:?}", out);

    // let input = component::SplitInput {
    //     s: "hello, how, are, you",
    //     delimiter: ", ",
    // };
    // let out = exports.split(&mut store, input)?;

    // println!("got: {:?}", out);

    // let users = vec![
    //     component::UserParam {
    //         id: 1,
    //         username: "alice",
    //         email: "foo@example.com",
    //         phone: "555-123-4567",
    //     },
    //     component::UserParam {
    //         id: 2,
    //         username: "lucy",
    //         email: "lucy@singlestore.com",
    //         phone: "555-123-4567",
    //     },
    //     component::UserParam {
    //         id: 3,
    //         username: "jones",
    //         email: "jones@example.net",
    //         phone: "555-123-4567",
    //     },
    //     component::UserParam {
    //         id: 4,
    //         username: "bob",
    //         email: "bob@gmail.com",
    //         phone: "555-123-4567",
    //     },
    // ];

    // let mut good_users = vec![];
    // for user in users {
    //     let result = exports.filter_out_bad_users(&mut store, user).unwrap();
    //     if !result.is_empty() {
    //         good_users.extend(result);
    //     }
    // }

    // println!("got: {:?}", good_users);

    // let data = vec![12.0, -3.0, 5.0, 11.0, 22.0, -5.0, -6.0];
    // let data_packed = vector_pack(&data);
    // let input = component::HilbertInput {
    //     vec: &data_packed,
    //     min_value: -6.0,
    //     max_value: 22.0,
    //     scale: 100.0,
    // };
    // let out = exports.hilbert_encode(&mut store, input)?;
    // println!("got: {:?}", out);

    Ok(())
}
