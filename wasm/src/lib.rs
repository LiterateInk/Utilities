extern crate proc_macro;

use syn::{parse_macro_input, ItemFn, Attribute, FnArg, parse_quote};
use proc_macro::TokenStream;
use quote::quote;

/// This macro adds the `#[wasm_bindgen]` attribute to the function
/// and adds a `fetcher: js_sys::Function` parameter to the function signature.
#[proc_macro_attribute]
pub fn api_method(_args: TokenStream, input: TokenStream) -> TokenStream {
  let mut input = parse_macro_input!(input as ItemFn);
  let vis = &input.vis;
  let sig = &mut input.sig;
  let block = &input.block;
  let attrs = &mut input.attrs;

  let wasm_bindgen_attr: Attribute = parse_quote!(#[wasm_bindgen::prelude::wasm_bindgen]);
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
