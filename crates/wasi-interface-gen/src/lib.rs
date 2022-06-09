use heck::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::visit::Visit;
use syn::{parse_macro_input, parse_quote, Item, ItemMod, PathArguments};
use wit_bindgen_gen_core::{Direction, Files, Generator};
use wit_bindgen_gen_rust_wasm::RustWasm;
use wit_parser::Interface;

#[derive(Debug, Default)]
struct WitBuilder {
    source: String,
}

fn rust_type_name_to_wast(type_name: &str) -> String {
    match type_name {
        "String" => "string",
        "i8" => "s8",
        "i16" => "s16",
        "i32" => "s32",
        "i64" => "s64",
        "f32" => "float32",
        "f64" => "float64",
        other => other,
    }
    .to_string()
}

fn rust_type_to_wast(ty: &syn::Type) -> String {
    let type_name = match ty {
        syn::Type::Path(x) => {
            let last_segment = x.path.segments.last().unwrap();
            let type_param = match &last_segment.arguments {
                PathArguments::AngleBracketed(ref params) => params.args.first(),
                _ => None,
            }
            .and_then(|generic_arg| match generic_arg {
                syn::GenericArgument::Type(ty) => Some(ty),
                _ => None,
            });

            //println!("    LASTSEG-->{:?}, {:?}", &last_segment.ident.to_string().as_str(), &type_param);

            match last_segment.ident.to_string().as_str() {
                "Vec" => format!(
                    "list<{}>",
                    type_param
                        .map(rust_type_to_wast)
                        .unwrap_or_else(|| "any".to_string())
                ),
                "Option" => format!(
                    "option<{}>",
                    type_param
                        .map(rust_type_to_wast)
                        .unwrap_or_else(|| "any".to_string())
                ),
                other => other.to_kebab_case(),
            }
        }
        syn::Type::Reference(x) => rust_type_to_wast(&x.elem),
        syn::Type::Slice(x) => {
            let inner = rust_type_to_wast(&x.elem);
            format!("list<{}>", inner)
        }
        _ => panic!("unsupported syn::Type: {:?}", ty),
    };

    rust_type_name_to_wast(type_name.as_str())
}

impl Visit<'_> for WitBuilder {
    fn visit_item_struct(&mut self, node: &'_ syn::ItemStruct) {
        let ident_kebab = node.ident.to_string().to_kebab_case();
        self.source
            .push_str(&format!("record {} {{\n", ident_kebab));

        let fields = match node.fields {
            syn::Fields::Named(ref fields) => &fields.named,
            _ => panic!("struct must have named fields"),
        };

        for field in fields {
            let field_kebab = field.ident.as_ref().unwrap().to_string().to_kebab_case();
            self.source.push_str(&format!(
                "  {}: {},\n",
                field_kebab,
                rust_type_to_wast(&field.ty),
            ));
        }

        self.source.push_str("}\n");
    }

    fn visit_item_enum(&mut self, node: &syn::ItemEnum) {
        self.source.push_str(&format!("enum {} {{\n", node.ident));

        for v in &node.variants {
            self.source
                .push_str(&format!("{},\n", &v.ident.to_string()));
        }

        self.source.push_str("}\n");
    }

    fn visit_item_fn(&mut self, node: &'_ syn::ItemFn) {
        let sig = &node.sig;

        let name_kebab = sig.ident.to_string().to_kebab_case();
        self.source.push_str(&format!("{}: func(", name_kebab));
        sig.inputs.iter().for_each(|input| {
            if let syn::FnArg::Typed(x) = input {
                let pat = &x.pat;
                let ty = &x.ty;
                let pat_name = quote! {#pat}.to_string().to_kebab_case();
                let type_name = rust_type_to_wast(ty);
                self.source
                    .push_str(&format!("{}: {}, ", &pat_name, &type_name));
            }
        });

        self.source.push(')');

        if let syn::ReturnType::Type(_, ref ty) = sig.output {
            self.source.push_str(" -> ");
            let type_name = rust_type_to_wast(ty);
            let type_name = type_name.replace("Vec", "list");
            self.source.push_str(type_name.as_str());
            self.source.push('\n');
        }
        self.source.push('\n');
    }
}

#[proc_macro_attribute]
pub fn wasi_interface(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemMod);

    let mut wit = WitBuilder::default();
    wit.visit_item_mod(&input);
    if cfg!(feature = "embed-wit") {
        wit.source
            .push_str("\nwit-source-get: func() -> string\n");
        wit.source.push_str("\nwit-source-print: func()\n");
        //println!("WIT_SOURCE={}", &wit.source);
    }

    let iface = match Interface::parse("abi", &wit.source) {
        Ok(i) => i,
        Err(e) => panic!("{}", e),
    };

    let mut gen = RustWasm::new();
    let mut files = Files::default();
    gen.generate_one(&iface, Direction::Export, &mut files);

    let (_, contents) = files.iter().next().unwrap();
    let contents = std::str::from_utf8(contents).unwrap().to_string();
    let contents = contents.replace("<super::Abi as Abi>::", "");

    let abi = contents.parse().unwrap();
    let abi_mod = parse_macro_input!(abi as ItemMod);
    let abi_mod_contents = abi_mod.content.unwrap();

    let exports = abi_mod_contents.1.iter().filter_map(|item| match item {
        Item::Fn(_) => Some(item.clone()),
        Item::Static(_) => Some(item.clone()),
        Item::Struct(s) => {
            let struct_name = s.ident.to_string();
            if struct_name == "RetArea" {
                Some(item.clone())
            } else {
                None
            }
        }
        _ => None,
    });

    let use_wit_bindgen_rust = parse_quote! {
        #[allow(unused_imports)]
        use wit_bindgen_rust;
    };

    let wit_source = &wit.source.as_str();
    let wit_source_const: Item = parse_quote! {
        const _WIT_SOURCE_: &str = #wit_source;
    };
    let wit_source_getter = parse_quote! {
        fn wit_source_get() -> String {
            _WIT_SOURCE_.to_string()
        }
    };
    let wit_source_printer = parse_quote! {
        fn wit_source_print() {
            println!("{}", _WIT_SOURCE_.to_string());
        }
    };

    let mut content = input.content.unwrap();
    content.1.extend(exports);
    content.1.push(use_wit_bindgen_rust);

    if cfg!(feature = "embed-wit") {
        content.1.push(wit_source_const);
        content.1.push(wit_source_getter);
        content.1.push(wit_source_printer);
    }
    input.content = Some(content);

    // Need to allow dead_code since the generated code doesn't always directly
    // read the fields of user structs. For simple structs composed entirely of
    // scalar types, they are mapped to the host ABI in one shot.
    quote! {
        #[allow(dead_code)]
        #input
    }
    .into()
}
