use std::f64::consts::PI;

/// THE GOLDEN RATIO (Phi)
/// Represents the ideal structural stability in nature (1.618...)
const PHI_IDEAL: f64 = 1.618033988749895;

/// THE MODULATOR (Inverse Phi)
/// Used to create harmonic resonance with the chaos.
const PHI_MODULATOR: f64 = 0.6180339887;

/// LUMIS CORE: The Bio-Mimetic Logic Unit
/// ---------------------------------------
/// This struct represents the "Soul" of the Tiger Delta engine.
/// It does not use static logic (If/Else). Instead, it uses a metabolic
/// state (Entropy) to adapt to the pressure of incoming kinetic data.
pub struct LumisCore {
    /// Internal metabolic stress (0.0 = Calm, 1.0 = Critical)
    entropy_level: f64,
    
    /// Internal clock for phase modulation
    cycle_tick: u64,
}

impl LumisCore {
    /// Genesis: Creates a new Living Core with zero entropy.
    pub fn new() -> Self {
        LumisCore {
            entropy_level: 0.0,
            cycle_tick: 0,
        }
    }

    /// THE Pi-Phi BRIDGE
    /// -----------------
    /// Calculates the "Living Threshold".
    /// Instead of a static firewall rule, we generate a dynamic wave.
    /// We use PI (Infinite Chaos) to modulate PHI (Perfect Order).
    ///
    /// Returns: A floating-point threshold value for the current microsecond.
    pub fn compute_living_phi(&mut self) -> f64 {
        // Increment the internal time tick
        self.cycle_tick = self.cycle_tick.wrapping_add(1);

        // 1. Generate a chaotic oscillator using PI
        let time_phase = self.cycle_tick as f64;
        let chaos_wave = (time_phase * PI * PHI_MODULATOR).sin();

        // 2. Calculate deviation based on stress (Adrenaline)
        // The higher the entropy, the wider the system "breathes" to absorb shock.
        let deviation = chaos_wave * self.entropy_level * 0.1;

        // 3. Return the Living Threshold
        PHI_IDEAL + deviation
    }

    /// METABOLIC ABSORPTION
    /// --------------------
    /// Instead of blocking a packet, we absorb its kinetic energy.
    /// This increases the internal entropy (heat) of the system.
    pub fn absorb_stress(&mut self, impact: f64) {
        // Increase entropy, capped at 1.0 (System Meltdown limit)
        self.entropy_level = (self.entropy_level + impact).min(1.0);
    }

    /// WEEPING PROTOCOL (Homeostasis)
    /// ------------------------------
    /// Every biological system must cool down.
    /// This function dissipates entropy over time.
    /// If stress is too high, it performs a "Phase Shift" (rapid cooling).
    pub fn weep_reset(&mut self) {
        if self.entropy_level > 0.8 {
            // Emergency Cooling (Phase Shift)
            // Simulates the system "shedding" its skin/layer.
            self.entropy_level *= 0.5; 
        } else {
            // Natural Decay (Relaxation)
            // The system slowly returns to a calm state (Golden Ratio stability).
            self.entropy_level *= 0.98;
        }
    }

    /// Diagnostics: Returns the current stress level for visualization.
    pub fn get_entropy(&self) -> f64 {
        self.entropy_level
    }
}
