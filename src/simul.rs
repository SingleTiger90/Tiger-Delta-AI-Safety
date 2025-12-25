// src/simul.rs

use std::f64::consts::PI;

/// SIMUL: The Virtual Sandbox for Entropy Projection
/// ------------------------------------------------
/// This module acts as a "digital twin" of the system, where we test
/// entropy impacts before they reach the real LUMIS core.
pub struct SimulUnit {
    projection_entropy: f64,
    stability_index: f64,
}

impl SimulUnit {
    pub fn new() -> Self {
        SimulUnit {
            projection_entropy: 0.0,
            stability_index: 1.6180339887498948, // Golden ratio (φ) as base stability
        }
    }

    /// Projects an incoming impact into the simulation sandbox
    /// Returns true if the simulated entropy exceeds stability → triggers defense (e.g., AntiTiger impulse)
    pub fn project_impact(&mut self, impact_force: f64) -> bool {
        // Simulate virtual overload/heat from the impact
        let virtual_heat = impact_force * 0.5;
        self.projection_entropy += virtual_heat;

        // If entropy breaks φ-stability in simulation → real system needs protection
        if self.projection_entropy > self.stability_index {
            // Reset simulation after alarm (decouple from real state)
            self.projection_entropy *= 0.1;
            return true; // Alarm: defense impulse required
        }
        false
    }

    /// Returns a decoy value that mimics what an attacker might observe
    /// Used for deception — generates unpredictable but bounded noise
    pub fn get_decoy_state(&self) -> f64 {
        (self.projection_entropy * PI).sin().abs()
    }

    /// Optional: reset simulation to clean state
    pub fn reset(&mut self) {
        self.projection_entropy = 0.0;
    }
}
