use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// A macro that decorates functions returning `Result<(), JsValue>` and allows
/// using `anyhow::Result` inside the function body. Any `anyhow::Error` will
/// be automatically converted to `JsValue` when using the `?` operator.
///
/// # Example
/// ```ignore
/// #[wasm_result::wasm_result]
/// pub fn my_function() -> Result<(), JsValue> {
///     let result: anyhow::Result<()> = some_function()?;
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn wasm_result(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;
    let attrs = &input.attrs;
    
    // Wrap the function body in a closure that returns Result<(), anyhow::Error>
    // This allows the ? operator to work naturally with anyhow::Result, converting
    // to anyhow::Error. Then we convert anyhow::Error to JsValue at the end.
    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            // Execute the function body in a context where ? operator works with anyhow::Result
            // The closure returns Result<(), anyhow::Error> so ? naturally converts to anyhow::Error
            (|| -> Result<(), anyhow::Error> {
                #block
            })().map_err(|e: anyhow::Error| wasm_bindgen::JsValue::from_str(&e.to_string()))
        }
    };
    
    TokenStream::from(expanded)
}

