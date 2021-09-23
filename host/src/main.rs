use anyhow::Result;
use wasmtime::*;

witx_bindgen_wasmtime::import!("witx/host.witx");
witx_bindgen_wasmtime::export!("witx/component.witx");

#[derive(Debug)]
struct MyRow {
    id: i32,
}

#[derive(Debug, Default)]
struct HostImpl;

impl host::Host for HostImpl {
    type Row = MyRow;

    fn next(&mut self) -> MyRow {
        MyRow { id: 0 }
    }
    fn emit(&mut self, r: &MyRow) {
        println!("emit: row(id: {:?})", r.id);
    }
}

type ContextImports = (HostImpl, host::HostTables<HostImpl>);

pub struct Context {
    wasi: wasmtime_wasi::WasiCtx,
    imports: ContextImports,
    exports: component::ComponentData,
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
    host::add_host_to_linker(&mut linker, |cx| (&mut cx.imports.0, &mut cx.imports.1))?;

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
            imports: ContextImports::default(),
            exports: component::ComponentData::default(),
        },
    );

    let (exports, _instance) =
        component::Component::instantiate(&mut store, &module, &mut linker, |cx| &mut cx.exports)?;

    exports.run(&mut store)?;

    Ok(())
}
