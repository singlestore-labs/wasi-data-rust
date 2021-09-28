#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use anyhow::Result;
use wasmtime::*;
pub mod component {
    #[allow(unused_imports)]
    use witx_bindgen_wasmtime::{wasmtime, anyhow};
    pub struct SplitInput<'a> {
        pub s: &'a str,
        pub delimiter: &'a str,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::core::clone::Clone for SplitInput<'a> {
        #[inline]
        fn clone(&self) -> SplitInput<'a> {
            match *self {
                SplitInput {
                    s: ref __self_0_0,
                    delimiter: ref __self_0_1,
                } => SplitInput {
                    s: ::core::clone::Clone::clone(&(*__self_0_0)),
                    delimiter: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    impl<'a> std::fmt::Debug for SplitInput<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("SplitInput")
                .field("s", &self.s)
                .field("delimiter", &self.delimiter)
                .finish()
        }
    }
    pub struct SplitOutput {
        pub c: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for SplitOutput {
        #[inline]
        fn clone(&self) -> SplitOutput {
            match *self {
                SplitOutput { c: ref __self_0_0 } => SplitOutput {
                    c: ::core::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    impl std::fmt::Debug for SplitOutput {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("SplitOutput").field("c", &self.c).finish()
        }
    }
    pub struct UserParam<'a> {
        pub id: i64,
        pub username: &'a str,
        pub email: &'a str,
        pub phone: &'a str,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<'a> ::core::clone::Clone for UserParam<'a> {
        #[inline]
        fn clone(&self) -> UserParam<'a> {
            match *self {
                UserParam {
                    id: ref __self_0_0,
                    username: ref __self_0_1,
                    email: ref __self_0_2,
                    phone: ref __self_0_3,
                } => UserParam {
                    id: ::core::clone::Clone::clone(&(*__self_0_0)),
                    username: ::core::clone::Clone::clone(&(*__self_0_1)),
                    email: ::core::clone::Clone::clone(&(*__self_0_2)),
                    phone: ::core::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    impl<'a> std::fmt::Debug for UserParam<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("UserParam")
                .field("id", &self.id)
                .field("username", &self.username)
                .field("email", &self.email)
                .field("phone", &self.phone)
                .finish()
        }
    }
    pub struct UserResult {
        pub id: i64,
        pub username: String,
        pub email: String,
        pub phone: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for UserResult {
        #[inline]
        fn clone(&self) -> UserResult {
            match *self {
                UserResult {
                    id: ref __self_0_0,
                    username: ref __self_0_1,
                    email: ref __self_0_2,
                    phone: ref __self_0_3,
                } => UserResult {
                    id: ::core::clone::Clone::clone(&(*__self_0_0)),
                    username: ::core::clone::Clone::clone(&(*__self_0_1)),
                    email: ::core::clone::Clone::clone(&(*__self_0_2)),
                    phone: ::core::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    impl std::fmt::Debug for UserResult {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("UserResult")
                .field("id", &self.id)
                .field("username", &self.username)
                .field("email", &self.email)
                .field("phone", &self.phone)
                .finish()
        }
    }
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
        canonical_abi_free: wasmtime::TypedFunc<(i32, i32, i32), ()>,
        canonical_abi_realloc: wasmtime::TypedFunc<(i32, i32, i32, i32), i32>,
        filter_out_bad_users: wasmtime::TypedFunc<(i64, i32, i32, i32, i32, i32, i32), (i32,)>,
        memory: wasmtime::Memory,
        split: wasmtime::TypedFunc<(i32, i32, i32, i32), (i32,)>,
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
            let canonical_abi_free = instance
                .get_typed_func::<(i32, i32, i32), (), _>(&mut store, "canonical_abi_free")?;
            let canonical_abi_realloc = instance.get_typed_func::<(i32, i32, i32, i32), i32, _>(
                &mut store,
                "canonical_abi_realloc",
            )?;
            let filter_out_bad_users = instance
                .get_typed_func::<(i64, i32, i32, i32, i32, i32, i32), (i32,), _>(
                    &mut store,
                    "filter_out_bad_users",
                )?;
            let memory = instance
                .get_memory(&mut store, "memory")
                .ok_or_else(|| ::anyhow::private::new_adhoc("`memory` export not a memory"))?;
            let split =
                instance.get_typed_func::<(i32, i32, i32, i32), (i32,), _>(&mut store, "split")?;
            Ok(Component {
                canonical_abi_free,
                canonical_abi_realloc,
                filter_out_bad_users,
                memory,
                split,
                get_state: Box::new(get_state),
            })
        }
        pub fn split(
            &self,
            mut caller: impl wasmtime::AsContextMut<Data = T>,
            input: SplitInput<'_>,
        ) -> Result<Vec<SplitOutput>, wasmtime::Trap> {
            let func_canonical_abi_free = &self.canonical_abi_free;
            let func_canonical_abi_realloc = &self.canonical_abi_realloc;
            let memory = &self.memory;
            let SplitInput {
                s: s0,
                delimiter: delimiter0,
            } = input;
            let vec1 = s0;
            let ptr1 =
                func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec1.len() as i32) * 1))?;
            memory
                .data_mut(&mut caller)
                .store_many(ptr1, vec1.as_ref())?;
            let vec2 = delimiter0;
            let ptr2 =
                func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec2.len() as i32) * 1))?;
            memory
                .data_mut(&mut caller)
                .store_many(ptr2, vec2.as_ref())?;
            let (result3_0,) = self.split.call(
                &mut caller,
                (ptr1, vec1.len() as i32, ptr2, vec2.len() as i32),
            )?;
            let load4 = memory.data_mut(&mut caller).load::<i32>(result3_0 + 0)?;
            let load5 = memory.data_mut(&mut caller).load::<i32>(result3_0 + 8)?;
            let len9 = load5;
            let base9 = load4;
            let mut result9 = Vec::with_capacity(len9 as usize);
            for i in 0..len9 {
                let base = base9 + i * 8;
                result9.push({
                    let load6 = memory.data_mut(&mut caller).load::<i32>(base + 0)?;
                    let load7 = memory.data_mut(&mut caller).load::<i32>(base + 4)?;
                    let ptr8 = load6;
                    let len8 = load7;
                    SplitOutput {
                        c: String::from_utf8(copy_slice(
                            &mut caller,
                            memory,
                            func_canonical_abi_free,
                            ptr8,
                            len8,
                            1,
                        )?)
                        .map_err(|_| wasmtime::Trap::new("invalid utf-8"))?,
                    }
                });
            }
            func_canonical_abi_free.call(&mut caller, (base9, len9 * 8, 4))?;
            Ok(result9)
        }
        pub fn filter_out_bad_users(
            &self,
            mut caller: impl wasmtime::AsContextMut<Data = T>,
            input: UserParam<'_>,
        ) -> Result<Vec<UserResult>, wasmtime::Trap> {
            let func_canonical_abi_realloc = &self.canonical_abi_realloc;
            let func_canonical_abi_free = &self.canonical_abi_free;
            let memory = &self.memory;
            let UserParam {
                id: id0,
                username: username0,
                email: email0,
                phone: phone0,
            } = input;
            let vec1 = username0;
            let ptr1 =
                func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec1.len() as i32) * 1))?;
            memory
                .data_mut(&mut caller)
                .store_many(ptr1, vec1.as_ref())?;
            let vec2 = email0;
            let ptr2 =
                func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec2.len() as i32) * 1))?;
            memory
                .data_mut(&mut caller)
                .store_many(ptr2, vec2.as_ref())?;
            let vec3 = phone0;
            let ptr3 =
                func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec3.len() as i32) * 1))?;
            memory
                .data_mut(&mut caller)
                .store_many(ptr3, vec3.as_ref())?;
            let (result4_0,) = self.filter_out_bad_users.call(
                &mut caller,
                (
                    witx_bindgen_wasmtime::rt::as_i64(id0),
                    ptr1,
                    vec1.len() as i32,
                    ptr2,
                    vec2.len() as i32,
                    ptr3,
                    vec3.len() as i32,
                ),
            )?;
            let load5 = memory.data_mut(&mut caller).load::<i32>(result4_0 + 0)?;
            let load6 = memory.data_mut(&mut caller).load::<i32>(result4_0 + 8)?;
            let len17 = load6;
            let base17 = load5;
            let mut result17 = Vec::with_capacity(len17 as usize);
            for i in 0..len17 {
                let base = base17 + i * 32;
                result17.push({
                    let load7 = memory.data_mut(&mut caller).load::<i64>(base + 0)?;
                    let load8 = memory.data_mut(&mut caller).load::<i32>(base + 8)?;
                    let load9 = memory.data_mut(&mut caller).load::<i32>(base + 12)?;
                    let ptr10 = load8;
                    let len10 = load9;
                    let load11 = memory.data_mut(&mut caller).load::<i32>(base + 16)?;
                    let load12 = memory.data_mut(&mut caller).load::<i32>(base + 20)?;
                    let ptr13 = load11;
                    let len13 = load12;
                    let load14 = memory.data_mut(&mut caller).load::<i32>(base + 24)?;
                    let load15 = memory.data_mut(&mut caller).load::<i32>(base + 28)?;
                    let ptr16 = load14;
                    let len16 = load15;
                    UserResult {
                        id: load7,
                        username: String::from_utf8(copy_slice(
                            &mut caller,
                            memory,
                            func_canonical_abi_free,
                            ptr10,
                            len10,
                            1,
                        )?)
                        .map_err(|_| wasmtime::Trap::new("invalid utf-8"))?,
                        email: String::from_utf8(copy_slice(
                            &mut caller,
                            memory,
                            func_canonical_abi_free,
                            ptr13,
                            len13,
                            1,
                        )?)
                        .map_err(|_| wasmtime::Trap::new("invalid utf-8"))?,
                        phone: String::from_utf8(copy_slice(
                            &mut caller,
                            memory,
                            func_canonical_abi_free,
                            ptr16,
                            len16,
                            1,
                        )?)
                        .map_err(|_| wasmtime::Trap::new("invalid utf-8"))?,
                    }
                });
            }
            func_canonical_abi_free.call(&mut caller, (base17, len17 * 32, 8))?;
            Ok(result17)
        }
    }
    use witx_bindgen_wasmtime::rt::RawMem;
    use witx_bindgen_wasmtime::rt::copy_slice;
}
pub struct Context {
    wasi: wasmtime_wasi::WasiCtx,
    exports: component::ComponentData,
}
pub fn main() -> Result<()> {
    let mut config = Config::new();
    config.wasm_module_linking(true);
    config.cache_config_load_default()?;
    config.debug_info(true);
    let engine = Engine::new(&config)?;
    let module = Module::from_file(&engine, "target/wasm32-wasi/debug/component.wasm")?;
    let mut linker = Linker::<Context>::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| &mut cx.wasi)?;
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
    let input = component::SplitInput {
        s: "hello, how, are, you",
        delimiter: ", ",
    };
    let out = exports.split(&mut store, input)?;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["got: ", "\n"],
            &match (&out,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
    let users = <[_]>::into_vec(box [
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
    ]);
    let mut good_users = ::alloc::vec::Vec::new();
    for user in users {
        let result = exports.filter_out_bad_users(&mut store, user).unwrap();
        if result.len() > 0 {
            good_users.extend(result);
        }
    }
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["got: ", "\n"],
            &match (&good_users,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
    Ok(())
}
