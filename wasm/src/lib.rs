extern crate proc_macro;

use syn::{parse_macro_input, parse_quote, Attribute, DeriveInput, FnArg, ItemFn};
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;

/// This macro adds the `#[wasm_bindgen]` attribute.
///
/// If applied to a function, it will also rename the
/// name to camel case using `js_name`.
///
/// # Usages
///
/// ```rust
/// // The method will be called `retrieveCas` in the generated bindings.
/// #[cfg_attr(target_arch = "wasm32", wasm::export)]
/// pub async fn retrieve_cas() -> Result<String, Error> {
///  // ...
/// }
///
/// #[cfg_attr(target_arch = "wasm32", wasm::export)]
/// pub struct Session {
///   // ...
/// }
///
/// #[cfg_attr(target_arch = "wasm32", wasm::export)]
/// impl Session {
///   // ...
/// }
///
#[proc_macro_attribute]
pub fn export(_args: TokenStream, input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as syn::Item);

  match input {
    syn::Item::Fn(mut input) => {
      let vis = &input.vis;
      let sig = &input.sig;

      let block = &input.block;
      let attrs = &mut input.attrs;

      let name = sig.ident.to_string();
      let camel_case_name = name.to_case(Case::Camel);

      let wasm_bindgen_attr: Attribute = parse_quote!(#[wasm_bindgen::prelude::wasm_bindgen(js_name = #camel_case_name)]);
      attrs.push(wasm_bindgen_attr);

      let output = quote! {
        #(#attrs)*
        #vis #sig {
          #block
        }
      };

      TokenStream::from(output)
    },
    syn::Item::Struct(mut input) => {
      let vis = &input.vis;
      let ident = &input.ident;
      let fields = &input.fields;
      let attrs = &mut input.attrs;

      let wasm_bindgen_attr: Attribute = parse_quote!(#[wasm_bindgen::prelude::wasm_bindgen]);
      attrs.push(wasm_bindgen_attr);

      let output = quote! {
        #(#attrs)*
        #vis struct #ident #fields
      };

      TokenStream::from(output)
    },
    syn::Item::Impl(mut input) => {
      let attrs = &mut input.attrs;
      let self_ty = &input.self_ty;
      let items = &input.items;

      let wasm_bindgen_attr: Attribute = parse_quote!(#[wasm_bindgen::prelude::wasm_bindgen]);
      attrs.push(wasm_bindgen_attr);

      let output = quote! {
        #(#attrs)*
        impl #self_ty {
          #(#items)*
        }
      };

      TokenStream::from(output)
    },
    _ => panic!("Only functions, structs and impls are supported"),
  }
}

/// This macro adds a `fetcher: js_sys::Function` parameter to the function signature.
///
/// ## Usages
///
/// ```rust
/// #[cfg_attr(target_arch = "wasm32", wasm::append_fetcher)]
/// pub async fn update() -> Result<String, Error> {
///   // A `fetcher` variable is available
///   // if the target architecture is `wasm32`
/// }
/// ```
#[proc_macro_attribute]
pub fn append_fetcher(_args: TokenStream, input: TokenStream) -> TokenStream {
  let mut input = parse_macro_input!(input as ItemFn);
  let vis = &input.vis;
  let sig = &mut input.sig;
  let block = &input.block;
  let attrs = &input.attrs;

  let fetcher_param: FnArg = parse_quote!(fetcher: js_sys::Function);
  sig.inputs.push(fetcher_param);

  let output = quote! {
    #(#attrs)*
    #vis #sig {
      #block
    }
  };

  TokenStream::from(output)
}

#[proc_macro]
pub fn setup_allocator(_input: TokenStream) -> TokenStream {
  let expanded = quote! {
    extern crate wee_alloc;

    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
  };

  TokenStream::from(expanded)
}

#[proc_macro_derive(Error)]
pub fn derive_wasm_error(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let name = &input.ident;

  let expanded = quote! {
    impl From<#name> for wasm_bindgen::JsValue {
      fn from(error: #name) -> Self {
        js_sys::Error::new(&error.to_string()).into()
      }
    }
  };

  TokenStream::from(expanded)
}
