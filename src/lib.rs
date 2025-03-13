//! # Quick Macros
//!
//! This crate provides simple macros:
//! - **Derive**
//!     - FieldNames - macro used for generating functions returning a name of a field
//!

#[cfg(feature = "derive")]
use proc_macro::TokenStream;

#[cfg(feature = "derive")]
use syn::{parse_macro_input, DeriveInput};

#[cfg(feature = "derive")]
mod field_names;

/// Generates methods for retrieving field names of a struct as string literals.
///
/// This function is used as part of a procedural macro. It takes a struct definition,
/// extracts its named fields, and generates a method for each field. Each generated method
/// follows the naming pattern `nameof_<field_name>()`, which returns the field name as a static string.
///
/// # Example
/// ```
/// use quick_macros::FieldNames;
///
/// #[derive(FieldNames)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// assert_eq!(Person::nameof_name(), "name");
/// assert_eq!(Person::nameof_age(), "age");
/// ```
///
/// # Panics
/// - If the macro is applied to a non-struct type (e.g., an enum or union), it will panic.
#[cfg(feature = "derive")]
#[proc_macro_derive(FieldNames)]
pub fn field_names(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    field_names::field_names(input).into()
}
