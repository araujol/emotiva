// ==========================================
// 🎭 Emotiva Callback API
// ------------------------------------------
// Provides core methods for attaching callbacks to animations.
//
// ✅ Responsibilities:
//  - Allow developers to run code when an animation starts or completes
//  - Serve as the single interface for callback registration in Emotiva
//  - Keep the API minimal while still supporting chaining via closures
//
// 📦 Usage:
// ```rust
// let id = emotiva.set_alpha("layer", 0.0, 1.0, 0.5, easing);
// emotiva.on_end(id, |emo| {
//     println!("Animation finished!");
// });
// ```
// ==========================================

use crate::EmotivaHeart;

impl EmotivaHeart {
    pub fn on_start<F>(&mut self, id: u64, cb: F)
    where
        F: FnOnce(&mut EmotivaHeart) + 'static,
    {
        self.register_callback_on_start(id, cb);
    }

    pub fn on_end<F>(&mut self, id: u64, cb: F)
    where
        F: FnOnce(&mut EmotivaHeart) + 'static,
    {
        self.register_callback_on_complete(id, cb);
    }
}
