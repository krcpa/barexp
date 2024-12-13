use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::Item;
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;

use proc_macro2::TokenStream as TokenStream2;

fn module_path_literal() -> TokenStream2 {
    quote! {
        module_path!()
    }
}

// Global kayıt için static map
static EXPORTS: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

fn register_export(name: &str, path: &str) -> Result<(), String> {
    let mut exports = EXPORTS.lock().unwrap();
    if let Some(existing) = exports.get(name) {
        if existing != path {
            return Err(format!(
                "Name collision detected: '{}' is already exported in '{}'",
                name, existing
            ));
        }
    }
    exports.insert(name.to_string(), path.to_string());
    Ok(())
}

#[proc_macro_attribute]
pub fn export(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);
    process_export(item, false)
}

#[proc_macro_attribute]
pub fn export_fullpath(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);
    process_export(item, true)
}

fn process_export(item: Item, use_fullpath: bool) -> TokenStream {
    let (name, module_path) = match &item {
        Item::Struct(s) => (&s.ident, module_path_literal()),
        Item::Enum(e) => (&e.ident, module_path_literal()),
        Item::Fn(f) => (&f.sig.ident, module_path_literal()),
        _ => panic!("Export only supports structs, enums, and functions"),
    };

    let name_str = name.to_string();
    let export_path = if use_fullpath {
        quote! { concat!(#module_path, "::", stringify!(#name)) }
    } else {
        quote! { stringify!(#name) }
    };

    // Compile-time collision check
    if let Err(err) = register_export(&name_str, &format!("{}::{}", module_path, name_str)) {
        return syn::Error::new_spanned(&item, err)
            .to_compile_error()
            .into();
    }

    quote! {
        #item

        inventory::submit! {
            crate::ExportItem {
                name: #export_path,
                module_path: #module_path,
                full_path: concat!(#module_path, "::", stringify!(#name)),
                is_fullpath: #use_fullpath
            }
        }
    }.into()
}
