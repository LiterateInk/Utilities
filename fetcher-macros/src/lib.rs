extern crate proc_macro;

use syn::{parse_macro_input, Expr, punctuated::Punctuated, Token};
use proc_macro::TokenStream;
use quote::quote;

///
/// ## Examples
///
/// ```rust
/// // will use the variable `fetcher` on wasm32 target
/// #[cfg_attr(target_arch = "wasm32", wasm::append_fetcher)]
/// async fn something () {
///   let response = fetch!(request);
/// }
///
/// async fn something (session: &Session) {
///   // will use the `session.fetcher()` method on wasm32 target
///   // return type of `session.fetcher()` is `&js_sys::Function`
///   let response = fetch!(request, session.fetcher());
/// }
/// ```
#[proc_macro]
pub fn fetch(input: TokenStream) -> TokenStream {
  let args = parse_macro_input!(input with Punctuated::<Expr, Token![,]>::parse_terminated);

  let expanded = match args.len() {
    1 => { // here we expect #[wasm::append_fetcher] to be used so `fetcher` is defined.
      let req = &args[0];
      quote! {
        {
          #[cfg(target_arch = "wasm32")]
          let response = fetcher::fetch(#req, &fetcher).await;
          #[cfg(not(target_arch = "wasm32"))]
          let response = fetcher::fetch(#req).await;

          response
        }
      }
    },
    2 => { // here we expect the fetcher to come from another source (manual)
      let req = &args[0];
      let fetcher_expr = &args[1];
      quote! {
        {
          #[cfg(target_arch = "wasm32")]
          let response = fetcher::fetch(#req, #fetcher_expr).await;
          #[cfg(not(target_arch = "wasm32"))]
          let response = fetcher::fetch(#req).await;

          response
        }
      }
    },
    _ => {
      quote! {
        compile_error!("fetch! macro supports at most two arguments: fetch!(request) or fetch!(request, fetcher_expr)");
      }
    }
  };

  expanded.into()
}
