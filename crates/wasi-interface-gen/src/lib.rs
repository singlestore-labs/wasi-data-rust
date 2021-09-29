use proc_macro::TokenStream;
use quote::quote;
use syn::visit::Visit;
use syn::{parse_macro_input, parse_quote, Item, ItemMod};
use witx2::abi::Direction;
use witx2::Interface;
use witx_bindgen_gen_core::{Files, Generator};
use witx_bindgen_gen_rust_wasm::RustWasm;

#[derive(Debug, Default)]
struct WitxBuilder {
    source: String,
}

fn rust_type_to_wast(ty: &syn::Type) -> String {
    let type_name = match ty {
        syn::Type::Path(x) => quote! {#x}.to_string(),
        _ => panic!("unsupported: {:?}", ty),
    };

    match type_name.as_str() {
        "String" => "string",
        "char" => "char",
        "bool" => "bool",
        "i8" => "s8",
        "i16" => "s16",
        "i32" => "s32",
        "i64" => "s64",
        "u8" => "u8",
        "u16" => "u16",
        "u32" => "u32",
        "u64" => "u64",
        "f32" => "f32",
        "f64" => "f64",
        _ => panic!("unsupported: {:?}", type_name),
    }
    .to_string()
}

impl Visit<'_> for WitxBuilder {
    fn visit_item_struct(&mut self, node: &'_ syn::ItemStruct) {
        self.source.push_str(&format!("record {} {{\n", node.ident));

        let fields = match node.fields {
            syn::Fields::Named(ref fields) => &fields.named,
            _ => panic!("unsupported"),
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

    fn visit_item_fn(&mut self, node: &'_ syn::ItemFn) {
        let sig = &node.sig;

        self.source.push_str(&format!("{}: function(", sig.ident));

        sig.inputs.iter().for_each(|input| {
            if let syn::FnArg::Typed(x) = input {
                self.source.push_str(quote! {#x}.to_string().as_str());
            }
        });

        self.source.push_str(") -> ");

        if let syn::ReturnType::Type(_, ref ty) = sig.output {
            let type_name = quote! {#ty}.to_string();
            let type_name = type_name.replace("Vec", "list");
            self.source.push_str(type_name.as_str());
        }
    }
}

#[proc_macro_attribute]
pub fn wasi_interface(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemMod);

    let mut witx = WitxBuilder::default();
    witx.visit_item_mod(&input);

    let iface = match Interface::parse("abi", &witx.source) {
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

    quote! {
        #input
    }
    .into()
}
