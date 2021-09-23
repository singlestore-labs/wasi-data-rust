#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use anyhow::Result;
use wasmtime::*;
pub mod host {
    #[allow(unused_imports)]
    use witx_bindgen_wasmtime::{wasmtime, anyhow};
    pub trait Host: Sized {
        type Row: std::fmt::Debug;
        fn next(&mut self) -> Self::Row;
        fn emit(&mut self, r: &Self::Row);
        fn drop_row(&mut self, state: Self::Row) {
            drop(state);
        }
    }
    pub struct HostTables<T: Host> {
        pub(crate) row_table: witx_bindgen_wasmtime::Table<T::Row>,
    }
    impl<T: Host> Default for HostTables<T> {
        fn default() -> Self {
            Self {
                row_table: Default::default(),
            }
        }
    }
    pub fn add_host_to_linker<T, U>(
        linker: &mut wasmtime::Linker<T>,
        get: impl Fn(&mut T) -> (&mut U, &mut HostTables<U>) + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<()>
    where
        U: Host,
    {
        linker.func_wrap(
            "host",
            "next",
            move |mut caller: wasmtime::Caller<'_, T>| {
                let host = get(caller.data_mut());
                let (host, _tables) = host;
                let result0 = host.next();
                Ok(_tables.row_table.insert(result0) as i32)
            },
        )?;
        linker.func_wrap(
            "host",
            "emit",
            move |mut caller: wasmtime::Caller<'_, T>, arg0: i32| {
                let host = get(caller.data_mut());
                let (host, _tables) = host;
                let param0 = _tables
                    .row_table
                    .get((arg0) as u32)
                    .ok_or_else(|| wasmtime::Trap::new("invalid handle index"))?;
                host.emit(param0);
                Ok(())
            },
        )?;
        linker.func_wrap(
            "canonical_abi",
            "resource_drop_row",
            move |mut caller: wasmtime::Caller<'_, T>, handle: u32| {
                let (host, tables) = get(caller.data_mut());
                let handle = tables.row_table.remove(handle).map_err(|e| {
                    wasmtime::Trap::new({
                        let res = ::alloc::fmt::format(
                            match match (&e,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            } {
                                ref args => unsafe {
                                    ::core::fmt::Arguments::new_v1(
                                        &["failed to remove handle: "],
                                        args,
                                    )
                                },
                            },
                        );
                        res
                    })
                })?;
                host.drop_row(handle);
                Ok(())
            },
        )?;
        Ok(())
    }
}
const _ : & str = "// ## scalar types\n// decimal types: f32 | f64\n// integer types: s8 | u8 | s16 | u16 | s32 | u32 | s64 | u64\n// other types: char\n\n// ## structured types\n// variant NAME { TAG(TYPE), TAG(TYPE), ... }\n// record NAME { FIELD: TYPE, FIELD: TYPE, ... }\n// list<TYPE>\n\n// ## default type aliases\n// string\n// bool\n// tuple<TYPE, TYPE, ...>\n// flags NAME { FIELD, FIELD, ... }\n// enum NAME { FIELD, FIELD, ... }\n// union NAME { TYPE, TYPE, ... }\n// option<TYPE>\n// expected<TYPE, TYPE>\n\n// ## other\n// function(FIELD: TYPE, FIELD: TYPE, ...) [-> TYPE]\n// resource NAME {\n//   [static] NAME: function(FIELD: TYPE, FIELD: TYPE, ...) [-> TYPE]\n//   NAME: function(FIELD: TYPE, FIELD: TYPE, ...) [-> TYPE]\n// }\n// type NAME = TYPE\n\nresource Row\n\nnext: function() -> Row\nemit: function(r: Row)" ;
pub mod component {
    #[allow(unused_imports)]
    use witx_bindgen_wasmtime::{wasmtime, anyhow};
    /// Auxiliary data associated with the wasm exports.
    ///
    /// This is required to be stored within the data of a
    /// `Store<T>` itself so lifting/lowering state can be managed
    /// when translating between the host and wasm.
    pub struct ComponentData {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for ComponentData {
        #[inline]
        fn default() -> ComponentData {
            ComponentData {}
        }
    }
    pub struct Component<T> {
        get_state: Box<dyn Fn(&mut T) -> &mut ComponentData + Send + Sync>,
        run: wasmtime::TypedFunc<(), ()>,
    }
    impl<T> Component<T> {
        #[allow(unused_variables)]
        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `linker` provided.
        ///
        /// The `get_state` closure is required to access the
        /// auxiliary data necessary for these wasm exports from
        /// the general store's state.
        pub fn add_to_linker(
            linker: &mut wasmtime::Linker<T>,
            get_state: impl Fn(&mut T) -> &mut ComponentData + Send + Sync + Copy + 'static,
        ) -> anyhow::Result<()> {
            Ok(())
        }
        /// Instantaites the provided `module` using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        ///
        /// The `linker` provided will have intrinsics added to it
        /// automatically, so it's not necessary to call
        /// `add_to_linker` beforehand. This function will
        /// instantiate the `module` otherwise using `linker`, and
        /// both an instance of this structure and the underlying
        /// `wasmtime::Instance` will be returned.
        ///
        /// The `get_state` parameter is used to access the
        /// auxiliary state necessary for these wasm exports from
        /// the general store state `T`.
        pub fn instantiate(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            module: &wasmtime::Module,
            linker: &mut wasmtime::Linker<T>,
            get_state: impl Fn(&mut T) -> &mut ComponentData + Send + Sync + Copy + 'static,
        ) -> anyhow::Result<(Self, wasmtime::Instance)> {
            Self::add_to_linker(linker, get_state)?;
            let instance = linker.instantiate(&mut store, module)?;
            Ok((Self::new(store, &instance, get_state)?, instance))
        }
        /// Low-level creation wrapper for wrapping up the exports
        /// of the `instance` provided in this structure of wasm
        /// exports.
        ///
        /// This function will extract exports from the `instance`
        /// defined within `store` and wrap them all up in the
        /// returned structure which can be used to interact with
        /// the wasm module.
        pub fn new(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            instance: &wasmtime::Instance,
            get_state: impl Fn(&mut T) -> &mut ComponentData + Send + Sync + Copy + 'static,
        ) -> anyhow::Result<Self> {
            let mut store = store.as_context_mut();
            let run = instance.get_typed_func::<(), (), _>(&mut store, "run")?;
            Ok(Component {
                run,
                get_state: Box::new(get_state),
            })
        }
        pub fn run(
            &self,
            mut caller: impl wasmtime::AsContextMut<Data = T>,
        ) -> Result<(), wasmtime::Trap> {
            self.run.call(&mut caller, ())?;
            Ok(())
        }
    }
}
const _: &str = "run: function()";
struct HostImpl;
impl host::Host for HostImpl {
    type Row = u32;
    fn next(&mut self) -> Self::Row {
        10
    }
    fn emit(&mut self, r: &Self::Row) {
        {
            ::std::io::_print(
                match match (&r,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                } {
                    ref args => unsafe { ::core::fmt::Arguments::new_v1(&["emit: ", "\n"], args) },
                },
            );
        };
    }
}
pub struct Context {
    wasi: wasmtime_wasi::WasiCtx,
    imports: (HostImpl, host::HostTables<HostImpl>),
    exports: component::ComponentData,
}
pub fn main() -> Result<()> {
    let mut config = Config::new();
    config.wasm_module_linking(true);
    config.cache_config_load_default()?;
    let engine = Engine::new(&config)?;
    let module = Module::from_file(&engine, "target/wasm32-wasi/debug/component.wasm")?;
    let mut linker = Linker::<Context>::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| &mut cx.wasi)?;
    host::add_host_to_linker(&mut linker, |cx| (&mut cx.imports.0, &mut cx.imports.1))?;
    let mut store = Store::new(
        &engine,
        Context {
            wasi: wasmtime_wasi::sync::WasiCtxBuilder::new()
                .inherit_stdio()
                .build(),
            imports: (HostImpl, host::HostTables::default()),
            exports: component::ComponentData::default(),
        },
    );
    let (exports, _instance) =
        component::Component::instantiate(&mut store, &module, &mut linker, |cx| &mut cx.exports)?;
    exports.run(&mut store)?;
    Ok(())
}
