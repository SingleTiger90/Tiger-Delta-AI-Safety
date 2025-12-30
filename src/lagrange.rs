// src/lagrange.rs

use crate::lumis::{PHI, PHI_INVERSE};

pub struct LagrangeEquilibrium {
    defense_mass: f64,
    in_zone: bool,
    enter_min: f64,
    enter_max: f64,
    exit_min: f64,
    exit_max: f64,
}

impl LagrangeEquilibrium {
    pub fn new(defense_mass: f64) -> Self {
        Self {
            defense_mass,
            in_zone: false,
            enter_min: PHI_INVERSE,
            enter_max: PHI,
            exit_min: PHI_INVERSE + 0.1,
            exit_max: PHI - 0.1,
        }
    }

    /// Nonlinear gravity trap with hysteresis
    pub fn stabilize(&mut self, compact_float: f64, attack_energy: f64) -> Option<f64> {
        let pull = (attack_energy / self.defense_mass).sqrt();

        // Nonlinear curved space + harmonic tremor
        let l = (PHI * compact_float.powi(2) + (pull * PHI_INVERSE).sin()).abs() % PHI;

        if self.in_zone {
            if l < self.exit_min || l > self.exit_max {
                self.in_zone = false;
                return Some(l);
            }
            None
        } else {
            if l >= self.enter_min && l <= self.enter_max {
                self.in_zone = true;
            }
            if self.in_zone { None } else { Some(l) }
        }
    }
}
