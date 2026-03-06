//! # 🎭 Layer API
//!
//! Public API for **swapping and resetting layer images** at runtime in Emotiva.
//!
//! This module extends [`EmotivaHeart`] with helper methods that allow
//! frontends to dynamically change which sprite image a layer displays.
//!
//! Layer swapping is commonly used for things like:
//!
//! - facial expressions
//! - outfit variations
//! - accessories or props
//!
//! These variants are defined in the character rig (`.ron` file) and can
//! be activated at runtime without modifying the underlying rig structure.
//!
//! ## Responsibilities
//!
//! This module provides APIs to:
//!
//! - Replace a layer’s image with a variant defined in the rig
//! - Reset a layer back to its default image
//!
//! ## Example
//!
//! ```ignore
//! heart.layer_set("eyes", "closed");
//!
//! // later
//! heart.layer_reset("eyes");
//! ```
//!
//! These methods act as a thin **runtime control layer** over the rig’s
//! predefined sprite variants, allowing characters to change appearance
//! dynamically during gameplay.

use crate::EmotivaHeart;

impl EmotivaHeart {
    /// Changes a layer's image to a specified variant.
    ///
    /// # Parameters
    /// - `layer_name`: The target layer to modify (e.g. "eyes", "hair").
    /// - `variant`: The name of the image variant defined in the `.ron` file.
    ///
    /// # Behavior
    /// - If the variant exists for that layer, the layer image is overridden.
    /// - If the variant does **not** exist, a warning is printed to stderr.
    pub fn set_layer(&mut self, layer_name: &str, variant: &str) {
        if let Some(layer_variants) = self.image_variants.get(layer_name) {
            if let Some(image_name) = layer_variants.get(variant) {
                self.image_overrides
                    .insert(layer_name.to_string(), image_name.clone());
            } else {
                eprintln!(
                    "Warning: unknown variant '{}' for layer '{}'",
                    variant, layer_name
                );
            }
        } else {
            eprintln!("Warning: layer '{}' has no image variants", layer_name);
        }
    }

    /// Resets a layer's image override back to the default image.
    ///
    /// # Parameters
    /// - `layer_name`: The target layer to reset.
    ///
    /// # Behavior
    /// - Removes any override applied to the layer so the default image
    ///   defined in the `.ron` file is shown again.
    pub fn reset_layer(&mut self, layer_name: &str) {
        self.image_overrides.remove(layer_name);
    }
}
