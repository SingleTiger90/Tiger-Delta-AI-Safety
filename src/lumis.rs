// src/lumis.rs

use std::f64::consts::PI;

/// Golden Ratio (φ) — mathematical constant representing ideal proportion and stability
pub const PHI: f64 = 1.6180339887498948;

/// Inverse Golden Ratio (φ⁻¹) — used for controlled harmonic oscillation
pub const PHI_INVERSE: f64 = 0.6180339887498948;

/// LumisCore: Bio-Inspired Adaptive Defense Engine
/// ---------------------------------------------
/// Replaces traditional static rules with a dynamic, entropy-driven decision system.
/// The core maintains an internal stress state and generates time-varying thresholds
/// using harmonic resonance between π (chaos) and φ (order).
pub struct LumisCore {
    /// Current system entropy/stress level (0.0 = fully stable, 1.0 = saturated)
    entropy: f64,

    /// Internal monotonic tick counter for temporal phase modulation
    tick: u64,
}

impl LumisCore {
    /// Initializes the core in a pristine, zero-entropy state
    pub fn new() -> Self {
        Self {
            entropy: 0.0,
            tick: 0,
        }
    }

    /// Computes the current dynamic decision threshold
    /// The threshold naturally oscillates over time and adapts based on entropy,
    /// allowing the system to "breathe" under load like a biological organism.
    pub fn dynamic_threshold(&mut self) -> f64 {
        self.tick = self.tick.wrapping_add(1);

        // Generate harmonic oscillation using time, π, and inverse golden ratio
        let phase = self.tick as f64;
        let oscillation = (phase * PI * PHI_INVERSE).sin();

        // Entropy modulates the amplitude of oscillation (adaptive tolerance)
        let adaptation = oscillation * self.entropy.clamp(0.0, 1.0) * 0.1;

        // Final threshold centered around the golden ratio
        PHI + adaptation
    }

    /// Accumulates threat impact into internal entropy
    /// Higher impact increases system stress, widening future thresholds.
    pub fn accumulate_entropy(&mut self, impact: f64) {
        if impact > 0.0 {
            self.entropy = (self.entropy + impact).min(1.0);
        }
    }

    /// Performs homeostatic entropy dissipation
    /// - Normal conditions: slow natural decay
    /// - High stress (>80%): rapid phase-shift cooling
    pub fn dissipate_entropy(&mut self) {
        if self.entropy > 0.8 {
            // Emergency stabilization — fast entropy reduction
            self.entropy *= 0.5;
        } else {
            // Gradual relaxation toward equilibrium
            self.entropy *= 0.98;
        }
    }

    /// Returns current entropy level (useful for metrics, logging, UI)
    pub fn entropy_level(&self) -> f64 {
        self.entropy
    }

    /// Resets the core to initial stable state (e.g., after full recovery or reboot)
    pub fn reset(&mut self) {
        self.entropy = 0.0;
        self.tick = 0;
    }
}

// Unit tests — validate core adaptive behavior
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let core = LumisCore::new();
        assert_eq!(core.entropy_level(), 0.0);
    }

    #[test]
    fn test_entropy_accumulation_and_cap() {
        let mut core = LumisCore::new();
        core.accumulate_entropy(0.7);
        assert_eq!(core.entropy_level(), 0.7);
        core.accumulate_entropy(0.8);
        assert_eq!(core.entropy_level(), 1.0); // capped at 1.0
    }

    #[test]
    fn test_threshold_is_dynamic() {
        let mut core = LumisCore::new();
        let t1 = core.dynamic_threshold();
        let t2 = core.dynamic_threshold();
        assert_ne!(t1, t2, "Threshold must change over time");
    }

    #[test]
    fn test_homeostasis_recovery() {
        let mut core = LumisCore::new();
        core.accumulate_entropy(0.95);

        // Simulate 100 cycles of dissipation
        for _ in 0..100 {
            core.dissipate_entropy();
        }

        assert!(core.entropy_level() < 0.3, "System should recover from high stress");
    }
}
