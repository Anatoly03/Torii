//! Reused macros in the Torii source code.

mod macro_ts;

use proc_macro::TokenStream;
use syn::{Item, parse_macro_input};

/// A procedural macro that generates typescript bindings for the annotated Rust item.
///
/// This will generate a TypeScript declaration file (.d.ts) for the item, allowing it
/// to be used in TypeScript code.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use torii_desktop_macro::ts_bind;
///
/// /// This struct will have TypeScript bindings generated for it.
/// /// You can run `cargo test` or run this test manually to generate
/// /// the TypeScript declaration file.
/// #[ts_bind()]
/// struct MyStruct {
///     /// This field is a string and will be represented as a string in TypeScript.
///     field1: String,
///
///     /// This field is an optional integer and will be represented as a number
///     /// or undefined in TypeScript.
///     field2: Option<i32>,
///
///     /// This field is a vector of floating-point numbers and will be represented
///     /// as an array of numbers in TypeScript.
///     field3: Vec<f64>,
///
///     /// This field is a HashMap with string keys and boolean values, and will be
///     /// represented as an object with string keys and boolean values in TypeScript.
///     field4: HashMap<String, Option<bool>>,
/// }
/// ```
#[proc_macro_attribute]
pub fn ts_bind(_: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    match item {
        Item::Struct(strucc) => macro_ts::bind_struct(strucc).into(),
        other => {
            return syn::Error::new_spanned(
                other,
                "The #[ts_bind] macro can only be applied to structs.",
            )
            .to_compile_error()
            .into();
        }
    }
}
