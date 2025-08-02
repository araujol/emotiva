// src/macros.rs

/// 🏗️ Macro to forward selected methods from an inner field
/// (e.g. `heart: EmotivaHeart`) to an outer type
/// (e.g. `EmotivaQuad`).
///
/// ## Why?
/// Frontends like `EmotivaQuad` or `EmotivaBevy` don’t implement
/// the animation logic themselves — they forward API calls
/// to the core `EmotivaHeart` type.
///
/// ## How it works
/// Expands into a standard `impl` block for `$outer`,
/// generating wrapper methods that call the same methods on
/// `$field` (of type `$inner`).
///
/// ## Supports:
/// ✅ `&self` methods  
/// ✅ `&mut self` methods  
/// ✅ Methods with and without return types
///
/// ## Example
/// ```rust
/// struct EmotivaHeart;
/// impl EmotivaHeart {
///     fn tween(&self, x: i32) { println!("Tween {}", x); }
///     fn reset(&mut self) { println!("Reset"); }
/// }
///
/// struct EmotivaQuad { heart: EmotivaHeart }
///
/// forward_methods!(EmotivaQuad, heart: EmotivaHeart => {
///     fn tween(&self, x: i32);
/// });
///
///
/// forward_methods_mut!(EmotivaQuad, heart: EmotivaHeart => {
///     fn reset(&mut self);
/// });
///
/// fn main() {
///     let mut quad = EmotivaQuad { heart: EmotivaHeart };
///     quad.tween(5);   // ✅ calls EmotivaHeart::tween(5)
///     quad.reset();    // ✅ calls EmotivaHeart::reset()
/// }
/// ```

#[macro_export]
macro_rules! forward_methods {
    (
        $outer:ty, $field:ident: $inner:ty => {
            $( pub fn $name:ident(&self $(, $arg:ident: $arg_ty:ty )* ) $(-> $ret:ty)? ; )*
        }
    ) => {
        impl $outer {
            $(
                pub fn $name(&self, $($arg: $arg_ty),*) $(-> $ret)? {
                    self.$field.$name($($arg),*)
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! forward_methods_mut {
    (
        $outer:ty, $field:ident: $inner:ty => {
            $( pub fn $name:ident(&mut self $(, $arg:ident: $arg_ty:ty )* ) $(-> $ret:ty)? ; )*
        }
    ) => {
        impl $outer {
            $(
                pub fn $name(&mut self, $($arg: $arg_ty),*) $(-> $ret)? {
                    self.$field.$name($($arg),*)
                }
            )*
        }
    };
}
