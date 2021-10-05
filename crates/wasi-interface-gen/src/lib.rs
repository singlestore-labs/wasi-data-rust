use proc_macro::TokenStream;
use quote::quote;
use syn::visit::Visit;
use syn::{parse_macro_input, parse_quote, Item, ItemMod, PathArguments};
use witx2::abi::Direction;
use witx2::Interface;
use witx_bindgen_gen_core::{Files, Generator};
use witx_bindgen_gen_rust_wasm::RustWasm;

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

    match type_name.as_str() {
        "String" => "string",
        "i8" => "s8",
        "i16" => "s16",
        "i32" => "s32",
        "i64" => "s64",
        other => other,
    }
    .to_string()
}

#[derive(Debug, Default)]
struct WitxBuilder {
    witx_source: String,
    extra_rust: String,
}

impl Visit<'_> for WitxBuilder {
    fn visit_item_struct(&mut self, node: &'_ syn::ItemStruct) {
        self.witx_source.push_str(&format!("record {} {{\n", node.ident));

        let fields = match node.fields {
            syn::Fields::Named(ref fields) => &fields.named,
            _ => panic!("struct must have named fields"),
        };

        for field in fields {
            self.witx_source.push_str(&format!(
                "  {}: {},\n",
                field.ident.as_ref().unwrap(),
                rust_type_to_wast(&field.ty),
            ));
        }

        self.witx_source.push_str("}\n");
    }

    fn visit_item_fn(&mut self, node: &'_ syn::ItemFn) {
        let sig = &node.sig;

        let mut witx_extra = String::new();

        self.witx_source.push_str(&format!("{}: function(", sig.ident));

        witx_extra.push_str(&format!("{}_vec: function(", sig.ident));
        self.extra_rust.push_str(&format!("fn {}_vec(", sig.ident));

        sig.inputs.iter().for_each(|input| {
            if let syn::FnArg::Typed(x) = input {
                self.witx_source.push_str(quote! {#x}.to_string().as_str());

                witx_extra.push_str("list<");
                witx_extra.push_str(quote! {Vec<#x>}.to_string().as_str());
                witx_extra.push('>');

                self.extra_rust.push_str(quote! {Vec<#x>}.to_string().as_str());
            }
        });

        self.witx_source.push_str(") -> ");
        witx_extra.push_str(") -> ");
        self.extra_rust.push_str(") -> ");

        if let syn::ReturnType::Type(_, ref ty) = sig.output {
            let type_name = quote! {#ty}.to_string();
            let type_name = type_name.replace("Vec", "list");
            self.witx_source.push_str(type_name.as_str());
            self.witx_source.push('\n');
        }
    }
}

#[proc_macro_attribute]
pub fn wasi_interface(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemMod);

    let mut witx = WitxBuilder::default();
    witx.visit_item_mod(&input);

    let iface = match Interface::parse("abi", &witx.witx_source) {
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

    let use_witx_bindgen_rust = parse_quote! {
        #[allow(unused_imports)]
        use witx_bindgen_rust;
    };

    let mut content = input.content.unwrap();
    content.1.extend(exports);
    content.1.push(use_witx_bindgen_rust);
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
