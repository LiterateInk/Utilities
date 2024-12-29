extern crate proc_macro;

use syn::{parse_macro_input, parse_quote, Attribute, DeriveInput, FnArg, Ident, ItemFn};
use syn::parse::{Parse, ParseStream, Result};
use proc_macro::TokenStream;
use quote::quote;

struct ApiMethodArgs {
  js_name: Option<String>,
}

impl Parse for ApiMethodArgs {
  fn parse(input: ParseStream) -> Result<Self> {
    if input.is_empty() {
      return Ok(ApiMethodArgs { js_name: None });
    }

    let ident = input.parse::<Ident>()?;
    Ok(ApiMethodArgs { js_name: Some(ident.to_string()) })
  }
}

/// This macro adds the `#[wasm_bindgen]` attribute to the function
/// and adds a `fetcher: js_sys::Function` parameter to the function signature.
///
/// ## Usages
///
/// ```rust
/// // the method will be called `retrieveCAS` in the generated bindings
/// #[cfg_attr(target_arch = "wasm32", wasm::api_method(retrieveCAS))]
/// pub async fn retrieve_cas() -> Result<String, Error> {
///   // a `fetcher` variable is available
///   // if the target architecture is `wasm32`
/// }
///
/// // the method will be still called `update` in the generated bindings
/// #[cfg_attr(target_arch = "wasm32", wasm::api_method)]
/// pub async fn update() -> Result<String, Error> {
///   // a `fetcher` variable is available
///   // if the target architecture is `wasm32`
/// }
/// ```
#[proc_macro_attribute]
pub fn api_method(args: TokenStream, input: TokenStream) -> TokenStream {
  let args = parse_macro_input!(args with ApiMethodArgs::parse);
  let mut input = parse_macro_input!(input as ItemFn);
  let vis = &input.vis;
  let sig = &mut input.sig;
  let block = &input.block;
  let attrs = &mut input.attrs;

  let wasm_bindgen_attr: Attribute = if let Some(name) = args.js_name {
    parse_quote!(#[wasm_bindgen::prelude::wasm_bindgen(js_name = #name)])
  }
  else {
    parse_quote!(#[wasm_bindgen::prelude::wasm_bindgen])
  };

  attrs.push(wasm_bindgen_attr);

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
