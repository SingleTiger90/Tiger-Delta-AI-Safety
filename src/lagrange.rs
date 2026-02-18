// src/lagrange.rs

use crate::lumis::{PHI, PHI_INVERSE};

/// LagrangeEquilibrium реалізує нелінійну гравітаційну пастку 
/// для стабілізації енергетичних сплесків.
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
        // Захист: якщо маса 0, ставимо мінімальний поріг (правило 111-ї групи)
        let safe_mass = if defense_mass == 0.0 { 1.0 } else { defense_mass };
        
        Self {
            defense_mass: safe_mass,
            in_zone: false,
            enter_min: PHI_INVERSE, // ~0.618
            enter_max: PHI,         // ~1.618
            exit_min: PHI_INVERSE + 0.1,
            exit_max: PHI - 0.1,
        }
    }

    /// Nonlinear gravity trap with hysteresis.
    /// Повертає Some(l), якщо система виходить з рівноваги (потребує корекції),
    /// або None, якщо система стабільна (в зоні Лагранжа).
    pub fn stabilize(&mut self, compact_float: f64, attack_energy: f64) -> Option<f64> {
        // 1. Розрахунок сили тяжіння (нелінійний pull)
        let pull = (attack_energy / self.defense_mass).sqrt();

        // 2. Викривлення простору: PHI * L^2 + sin(pull * PHI_INV)
        // Це створює гармонічне тремтіння (tremor)
        let l = (PHI * compact_float.powi(2) + (pull * PHI_INVERSE).sin()).abs() % PHI;

        // 3. Логіка гістерезису ( hysteresis loop )
        if self.in_zone {
            // Якщо ми в зоні, перевіряємо вихід за звужені межі (exit_min/max)
            if l < self.exit_min || l > self.exit_max {
                self.in_zone = false;
                // Система дестабілізована - повертаємо нове значення L
                return Some(l);
            }
            // Все ще стабільні
            None
        } else {
            // Якщо ми поза зоною, перевіряємо вхід за широкі межі (enter_min/max)
            if l >= self.enter_min && l <= self.enter_max {
                self.in_zone = true;
            }
            
            // Якщо увійшли - стабільно (None), якщо ні - повертаємо відхилення
            if self.in_zone { 
                None 
            } else { 
                Some(l) 
            }
        }
    }

    /// Метод для оновлення маси захисту (наприклад, після мутації в core.py)
    pub fn update_mass(&mut self, new_mass: f64) {
        if new_mass > 0.0 {
            self.defense_mass = new_mass;
        }
    }
}
