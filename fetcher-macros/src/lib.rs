extern crate proc_macro;

use syn::{parse_macro_input, Expr};
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn fetch(input: TokenStream) -> TokenStream {
  let request: Expr = parse_macro_input!(input as Expr);

  let expanded = quote! {
    #[cfg(target_arch = "wasm32")]
    let response = fetcher::fetch(#request, fetcher).await;

    #[cfg(not(target_arch = "wasm32"))]
    let response = fetcher::fetch(#request).await;
  };

  TokenStream::from(expanded)
}
