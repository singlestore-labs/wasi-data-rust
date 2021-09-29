use anyhow::Result;
use wasmtime::*;

witx_bindgen_wasmtime::export!({
    src["component"]: "
        record SimpleValue {
            i: s64,
        }

        double: function(input: SimpleValue) -> list<SimpleValue>

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
    "
});

pub struct Context {
    wasi: wasmtime_wasi::WasiCtx,
    exports: component::ComponentData,
}

pub fn main() -> Result<()> {
    // Create an engine with caching enabled
    let mut config = Config::new();
    config.wasm_module_linking(true);
    config.cache_config_load_default()?;
    config.debug_info(true);
    let engine = Engine::new(&config)?;

    // Compile the component wasm module
    let module = Module::from_file(&engine, "target/wasm32-wasi/debug/component.wasm")?;

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

    let input = component::SimpleValue { i: 10 };
    let out = exports.double(&mut store, input)?;
    println!("got: {:?}", out);

    let input = component::SplitInput {
        s: "hello, how, are, you",
        delimiter: ", ",
    };
    let out = exports.split(&mut store, input)?;

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
        let result = exports.filter_out_bad_users(&mut store, user).unwrap();
        if result.len() > 0 {
            good_users.extend(result);
        }
    }

    println!("got: {:?}", good_users);

    Ok(())
}
