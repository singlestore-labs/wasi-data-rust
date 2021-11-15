use proc_macro::TokenStream;
use quote::quote;
use syn::visit::Visit;
use syn::{parse_macro_input, parse_quote, Item, ItemMod, PathArguments};
use wai_bindgen_gen_core::{Direction, Files, Generator};
use wai_bindgen_gen_rust_wasm::RustWasm;
use wai_parser::Interface;

#[derive(Debug, Default)]
struct WaiBuilder {
    source: String,
}

fn rust_type_name_to_wast(type_name: &str) -> String {
    match type_name {
        "String" => "string",
        "i8" => "s8",
        "i16" => "s16",
        "i32" => "s32",
        "i64" => "s64",
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
                other => other.into(),
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

impl Visit<'_> for WaiBuilder {
    fn visit_item_struct(&mut self, node: &'_ syn::ItemStruct) {
        self.source.push_str(&format!("record {} {{\n", node.ident));

        let fields = match node.fields {
            syn::Fields::Named(ref fields) => &fields.named,
            _ => panic!("struct must have named fields"),
        };

        for field in fields {
            self.source.push_str(&format!(
                "  {}: {},\n",
                field.ident.as_ref().unwrap(),
                rust_type_to_wast(&field.ty),
            ));
        }

        self.source.push_str("}\n");
    }

    fn visit_item_enum(&mut self, node: &syn::ItemEnum) {
        self.source.push_str(&format!("enum {} {{\n", node.ident));

        for v in &node.variants {
            self.source.push_str(&format!("{},\n", &v.ident.to_string()));
        } 

        self.source.push_str("}\n");
    }

    fn visit_item_fn(&mut self, node: &'_ syn::ItemFn) {
        let sig = &node.sig;

        self.source.push_str(&format!("{}: function(", sig.ident));
        sig.inputs.iter().for_each(|input| {
            if let syn::FnArg::Typed(x) = input {
                let pat = &x.pat;
                let ty = &x.ty;
                let pat_name = quote! {#pat}.to_string();
                let type_name = quote! {#ty}.to_string();
                let type_name = rust_type_name_to_wast(type_name.as_str());
                self.source.push_str(&format!("{}: {}", &pat_name, &type_name));
            }
        });

        self.source.push(')');

        if let syn::ReturnType::Type(_, ref ty) = sig.output {
            self.source.push_str(" -> ");
            let type_name = quote! {#ty}.to_string();
            let type_name = type_name.replace("Vec", "list");
            let type_name = rust_type_name_to_wast(type_name.as_str());
            self.source.push_str(type_name.as_str());
        }
        self.source.push('\n');
    }
}

#[proc_macro_attribute]
pub fn wasi_interface(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemMod);

    let mut wai = WaiBuilder::default();
    //wai.source.push_str("type i32 = s32\n");
    wai.visit_item_mod(&input);
    wai.source.push_str("\nwai_source: function() -> string\n");
    //println!("WAI_SOURCE={}", &wai.source);

    let iface = match Interface::parse("abi", &wai.source) {
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
        _ => None,
    });

    let use_wai_bindgen_rust = parse_quote! {
        #[allow(unused_imports)]
        use wai_bindgen_rust;
    };

    let wai_source = &wai.source;
    let wai_export_sym = parse_quote! {
        fn wai_source() -> String {
            #wai_source.to_string()
        }
    };

    let mut content = input.content.unwrap();
    content.1.extend(exports);
    content.1.push(use_wai_bindgen_rust);
    content.1.push(wai_export_sym);
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
