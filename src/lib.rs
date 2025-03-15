//! # Quick Macros
//!
//! This crate provides simple macros:
//! - **Derive**
//!     - FieldNames - macro used for generating functions returning a name of a field
//!

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};
mod ctor;
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
#[proc_macro_derive(FieldNames)]
#[proc_macro_error]
pub fn field_names(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    field_names::field_names(input).into()
}

/// Generates a constructor method for a struct, allowing instantiation with all fields.
///
/// This procedural macro derives an implementation of a new method for a struct.
/// The generated method takes each field of the struct as a parameter and returns an instance
/// of the struct with those fields initialized.
///
/// # Example
/// ```
/// use quick_macros::FullCtor;
///
/// #[derive(FullCtor)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let person = Person::new("Alice".to_string(), 30);
/// assert_eq!(person.name, "Alice");
/// assert_eq!(person.age, 30);
/// ```
///
/// # Panics
/// - If the macro is applied to a non-struct type (e.g., an enum or union), it will panic.
#[proc_macro_derive(FullCtor)]
#[proc_macro_error]
pub fn full_ctor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    ctor::full_ctor(input).into()
}
