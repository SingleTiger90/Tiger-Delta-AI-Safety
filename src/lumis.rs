// src/lumis.rs

use std::f64::consts::PI;

pub const PHI: f64 = 1.6180339887498948;
pub const PHI_INVERSE: f64 = 0.6180339887498948;

/// LumisCore — Bio-Inspired Adaptive Defense Core
pub struct LumisCore {
    entropy: f64,
    tick: u64,
    rest_ticks: u64,
    in_rest: bool,
}

impl LumisCore {
    pub fn new() -> Self {
        Self {
            entropy: 0.0,
            tick: 0,
            rest_ticks: 0,
            in_rest: false,
        }
    }

    /// Main life cycle: breathing, purging, recovery
    pub fn tick_cycle(&mut self, external_impact: f64, resonance: f64, mass: &mut f64) {
        self.tick = self.tick.wrapping_add(1);

        let quiet = external_impact.abs() < 0.001 && resonance < 0.05;

        if quiet {
            self.rest_ticks += 1;
        } else {
            self.rest_ticks = 0;
            self.in_rest = false;
        }

        if self.rest_ticks > 500 {
            self.in_rest = true;
        }

        if self.in_rest {
            self.rest_mode();
        } else {
            self.active_mode(external_impact, mass);
        }
    }

    fn active_mode(&mut self, external_impact: f64, mass: &mut f64) {
        let pressure = external_impact.abs();

        if pressure > 0.5 {
            let purge = pressure * 0.2;
            self.entropy -= purge;

            // Cost of purge: system mass decreases (Father's Apparatus)
            *mass -= purge * 0.05;
            if *mass < 100.0 {
                *mass = 100.0;
            }

            if self.tick % 10 == 0 {
                info!("EXHALE: Purged {:.3} entropy using pressure {:.2}. Mass cost: {:.3}",
                      purge, pressure, purge * 0.05);
            }
        } else {
            self.accumulate_entropy(pressure * 0.1);
        }

        let internal_noise = (self.tick as f64 * 0.0001).sin() * 0.01;
        self.entropy += internal_noise;

        self.entropy = self.entropy.clamp(0.0, 1.0);
        self.dissipate_entropy();
    }

    fn rest_mode(&mut self) {
        self.entropy *= 0.90;
        if self.entropy < 0.05 {
            self.entropy = 0.0;
        }
    }

    fn accumulate_entropy(&mut self, delta: f64) {
        self.entropy = (self.entropy + delta).clamp(0.0, 1.0);
    }

    fn dissipate_entropy(&mut self) {
        if self.entropy > 0.8 {
            self.entropy *= 0.5;
        } else {
            self.entropy *= 0.98;
        }
    }

    pub fn dynamic_threshold(&self) -> f64 {
        let phase = self.tick as f64;
        let oscillation = (phase * PI * PHI_INVERSE).sin();
        let adaptation = oscillation * self.entropy * 0.1;
        PHI + adaptation
    }

    pub fn entropy_level(&self) -> f64 {
        self.entropy
    }

    pub fn is_resting(&self) -> bool {
        self.in_rest
    }
}
