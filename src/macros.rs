// src/macros.rs

/// 🧰 Emotiva Macro Helpers
///
/// This module provides macros to simplify trait method forwarding
/// for wrapper types like `EmotivaQuad`, delegating calls to an inner
/// `EmotivaHeart` instance.
///
/// ### Macros:
///
/// #### `impl_fns_ref!`
/// Implements multiple `&self` trait methods by forwarding to a named inner field.
///
/// #### `impl_fns_mut!`
/// Implements multiple `&mut self` trait methods by forwarding to a named inner field.
///
/// These macros help reduce boilerplate in trait implementations and preserve a clean structure.
///
/// ## Example:
/// ```rust
/// struct EmotivaQuad { heart: EmotivaHeart }
///
/// impl EmotivaAPI for EmotivaQuad {
///     impl_fns_ref! {
///         heart => {
///             fn get_alpha(&self, layer: &str) -> f32;
///         }
///     }
///
///     impl_fns_mut! {
///         heart => {
///             fn set_layer(&mut self, layer: &str, variant: &str);
///         }
///     }
/// }
/// ```

/// 🎯 Implement multiple `&self` trait methods by forwarding to `self`
#[macro_export]
macro_rules! impl_fns_ref {
    (
        $field:ident => {
            $( fn $name:ident(&self $(, $arg:ident: $arg_ty:ty )* ) $(-> $ret:ty)? ; )*
        }
    ) => {
        $(
            fn $name(&self, $($arg: $arg_ty),*) $(-> $ret)? {
                self.$field.$name($($arg),*)
            }
        )*
    };
}

/// 🎯 Implement multiple `&mut self` trait methods by forwarding to an inner field
#[macro_export]
macro_rules! impl_fns_mut {
    (
        $field:ident => {
            $( fn $name:ident(&mut self $(, $arg:ident: $arg_ty:ty )* ) $(-> $ret:ty)? ; )*
        }
    ) => {
        $(
            fn $name(&mut self, $($arg: $arg_ty),*) $(-> $ret)? {
                self.$field.$name($($arg),*)
            }
        )*
    };
}
