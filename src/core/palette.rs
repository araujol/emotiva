//! Emotiva Color Palette
//! Predefined colors and emotional FX tints used for expressive layer effects
//! Format: [R, G, B, A] where each channel is a multiplier (can exceed 1.0)

// === Basic Colors ===
pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
pub const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
pub const CYAN: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
pub const MAGENTA: [f32; 4] = [1.0, 0.0, 1.0, 1.0];
pub const PINK: [f32; 4] = [1.0, 0.6, 0.8, 1.0];
pub const PURPLE: [f32; 4] = [0.6, 0.3, 0.9, 1.0];
pub const BROWN: [f32; 4] = [0.5, 0.3, 0.1, 1.0];

// === Emotional Colors ===
pub const BLUSH: [f32; 4] = [1.3, 0.6, 0.7, 1.0]; // warm pink tint
pub const ANGER: [f32; 4] = [2.2, 0.2, 0.2, 1.0]; // intense red
pub const SAD: [f32; 4] = [0.5, 0.6, 1.2, 1.0]; // cool blue
pub const EMBARRASS: [f32; 4] = [1.6, 0.4, 0.5, 1.0]; // deeper red-pink
pub const JOY: [f32; 4] = [1.4, 1.2, 0.6, 1.0]; // bright warm yellow
pub const FEAR: [f32; 4] = [1.0, 1.0, 1.6, 1.0]; // pale cold tone
pub const CALM: [f32; 4] = [0.6, 1.0, 0.8, 1.0]; // soft minty green

// === FX Colors ===
pub const SHOCK: [f32; 4] = [1.6, 1.6, 0.2, 1.0]; // yellow flash
pub const DIM: [f32; 4] = [0.6, 0.6, 0.6, 0.8]; // subdued tone
pub const FLASH: [f32; 4] = [2.5, 2.5, 2.5, 1.0]; // intense white flash
pub const GHOST: [f32; 4] = [0.7, 0.8, 1.4, 1.0]; // pale spectral glow
