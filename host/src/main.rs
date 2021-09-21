use anyhow::Result;
use wasmtime::*;

witx_bindgen_wasmtime::import!("witx/component.witx");

pub struct Context {
    wasi: wasmtime_wasi::WasiCtx,
}

pub fn main() -> Result<()> {
    // Create an engine with caching enabled
    let mut config = Config::new();
    config.wasm_module_linking(true);
    config.cache_config_load_default()?;
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
        },
    );

    // The `Instance` gives us access to various exported functions and items,
    // which we access here to pull out our exported function and run it.
    let instance = linker.instantiate(&mut store, &module)?;

    // There's a few ways we can call the `answer` `Func` value. The easiest
    // is to statically assert its signature with `typed` (in this case
    // asserting it takes no arguments and returns one i32) and then call it.
    let add = instance.get_typed_func::<(i32, i32), i32, _>(&mut store, "consume_add")?;

    // And finally we can call our function! Note that the error propagation
    // with `?` is done to handle the case where the wasm function traps.
    let out = add.call(&mut store, (1, 2))?;
    println!("got: {}", out);

    Ok(())
}
