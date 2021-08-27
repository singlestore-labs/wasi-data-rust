use anyhow::Result;
use wasmtime::*;

witx_bindgen_wasmtime::import!("spec/calculator.witx");

const WAT: &str = r#"
    (module
        (import "calculator" "add" (
            func $calc_add
            (param i32 i32)
            (result i32)
        ))

        (func $consume_add (param $lhs i32) (param $rhs i32) (result i32)
            local.get $lhs
            local.get $rhs
            call $calc_add
        )
        (export "consume_add" (func $consume_add))
    )
"#;

pub struct State {
    wasi: wasmtime_wasi::WasiCtx,
    calc: Calculator,
}

pub fn main() -> Result<()> {
    let mut config = Config::new();
    config.cache_config_load_default()?;
    let engine = Engine::new(&config)?;

    let module = Module::new(&engine, &WAT)?;
    let mut linker = Linker::<State>::new(&engine);

    wasmtime_wasi::add_to_linker(&mut linker, |cx| &mut cx.wasi)?;
    calculator::add_calculator_to_linker(&mut linker, |cx| &mut cx.calc)?;

    let mut store = Store::new(
        &engine,
        State {
            wasi: wasmtime_wasi::sync::WasiCtxBuilder::new()
                .inherit_stdio()
                .build(),
            calc: Calculator {},
        },
    );

    let instance = linker.instantiate(&mut store, &module)?;

    let func = instance.get_typed_func::<(i32, i32), i32, _>(&mut store, "consume_add")?;
    let out = func.call(&mut store, (1, 2))?;

    println!("got: {}", out);

    Ok(())
}

struct Calculator {}

impl calculator::Calculator for Calculator {
    fn add(&mut self, lh: i32, rh: i32) -> i32 {
        lh + rh
    }
}
