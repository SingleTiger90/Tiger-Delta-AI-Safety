// src/string_state.rs

use getrandom::getrandom;
use std::time::{Duration, Instant};

// Fixed-point format: Q32.32 (32-bit integer + 32-bit fractional part)
const FIXED_SCALE: i64 = 1i64 << 32;
const PI_FIXED: i64 = 3373259426i64;         // ≈ π × 2³²
const PHI_FIXED: i64 = 6941587532i64;        // ≈ φ × 2³² (golden ratio)
const ONE_FIXED: i64 = FIXED_SCALE;         // Represents 1.0

// Nonce refresh interval (every 5 seconds — adjustable)
const NONCE_EPOCH: Duration = Duration::from_secs(5);

/// StringState: Core mathematical engine for data compactification
/// ---------------------------------------------------------------
/// Performs fixed-point folding of network packet attributes into a compact scalar,
/// using irrational constants (π, φ) and dynamic nonce for security and diffusion.
/// Designed for O(1) complexity and future eBPF/XDP compatibility.
pub struct StringState {
    nonce: u64,
    last_update: Instant,
}

impl StringState {
    /// Creates a new state with fresh nonce
    pub fn new() -> Self {
        let mut state = Self {
            nonce: 0,
            last_update: Instant::now().checked_sub(Duration::from_secs(100)).unwrap(),
        };
        state.refresh_nonce();
        state
    }

    /// Ensures nonce is fresh (called before each compactification)
    pub fn ensure_fresh_nonce(&mut self) {
        if self.last_update.elapsed() >= NONCE_EPOCH {
            self.refresh_nonce();
        }
    }

    /// Refreshes nonce using hardware RNG (fallback to increment)
    fn refresh_nonce(&mut self) {
        let mut buf = [0u8; 8];
        if getrandom(&mut buf).is_ok() {
            self.nonce = u64::from_le_bytes(buf);
        } else {
            self.nonce = self.nonce.wrapping_add(1);
        }
        self.last_update = Instant::now();
    }

    /// Taylor series approximation of sin(x) in fixed-point
    /// Accurate enough for chaotic diffusion, very fast
    fn sin_fixed(mut x: i64) -> i64 {
        // Reduce to [-π, π]
        x %= 2 * PI_FIXED;
        if x > PI_FIXED {
            x -= 2 * PI_FIXED;
        }
        if x < -PI_FIXED {
            x += 2 * PI_FIXED;
        }

        let x2 = (x * x) >> 32;
        let x3 = (x2 * x) >> 32;
        let x5 = (x3 * x2) >> 32;
        let x7 = (x5 * x2) >> 32;

        let term1 = x;
        let term3 = (x3 * 716861901) >> 32;      // 1/6  approximated
        let term5 = (x5 * 35791394) >> 32;       // 1/120 approximated
        let term7 = (x7 * 1429388) >> 32;        // 1/5040 approximated

        term1 - term3 + term5 - term7
    }

    /// Main compactification function
    /// Input: 10 normalized attributes in fixed-point format [0, 1.0)
    /// Output: compact scalar in [0, 1.0) as i64
    pub fn compactify(&mut self, attributes: &[i64; 10]) -> i64 {
        self.ensure_fresh_nonce();

        let mut sum: i64 = 0;

        for (i, &a_i) in attributes.iter().enumerate() {
            let a_nonce = a_i.wrapping_add(self.nonce as i64);
            let scaled = (a_nonce * PI_FIXED) >> 32;
            let sin_val = Self::sin_fixed(scaled);
            let contrib = (sin_val * PHI_FIXED) >> 32;

            // Additional diffusion using index
            let idx_offset = ((i as i64) * 123456789i64) << 16;
            sum = sum.wrapping_add(contrib.wrapping_add(idx_offset));
        }

        // Return fractional part (mod 1)
        sum.rem_euclid(ONE_FIXED)
    }

    /// Helper: convert fixed-point back to f64 for debugging/logging
    pub fn to_float(value: i64) -> f64 {
        value as f64 / FIXED_SCALE as f64
    }
}

// Public constants for use in main.rs or other modules
pub const FIXED_SCALE_F64: f64 = FIXED_SCALE as f64;
