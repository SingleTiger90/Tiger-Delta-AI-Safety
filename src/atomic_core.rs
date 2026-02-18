// =================================================================
// Project: TigerΔ (Tiger Delta)
// Module: atomic_core.rs
// Description: Core resonance engine with Quantum Hysteresis.
// =================================================================

use std::f64::consts::PI;

/// Ірраціональні константи для аперіодичного дрейфу.
const PHI: f64 = 1.6180339887498948; 
const EQUILIBRIUM: f64 = PHI / PI;    
const SEPTIMAL_SHIFT: f64 = 1.777;    



pub struct AtomicCore {
    /// Proton count — поріг стабільності (маса ядра).
    proton_count: u32,
    /// Electron cloud (Ψ²) — зона імовірнісного знаходження легітимного сигналу.
    electron_cloud: f64,
    /// Valence energy — енергія, доступна для обробки.
    valence_energy: f64,
    /// Поріг деструкції — інтеграція з Negative Radius.
    is_critical: bool,
}

impl AtomicCore {
    pub fn new(stability: u32) -> Self {
        Self {
            proton_count: stability,
            electron_cloud: 0.5,
            valence_energy: 1.0,
            is_critical: false,
        }
    }

    /// Sharpen Angles (Заточування кутів).
    /// Перетворює лінійний удар атаки на кутовий момент.
    pub fn sharpen_angles(&mut self, impact: f64) {
        // Захист від від'ємної енергії
        let abs_impact = impact.abs();
        let resistance = (self.proton_count as f64 * PHI).max(1.0);
        
        // Арктангенс стискає будь-яку енергію в діапазон (-π/2, π/2)
        let angle = (abs_impact / resistance).atan();
        
        // Косинусна інтерференція: чим сильніший кут, тим більше коливання хмари
        self.electron_cloud = (self.electron_cloud + angle).cos().abs();
        
        // Поглинання частини енергії (Закон Дальтона)
        self.valence_energy += angle * 0.1;
    }

    /// Find the Middle (Пошук рівноваги / AADA Logic).
    /// Повертає відхилення (drift) від золотого перетину.
    pub fn find_the_middle(&mut self, input_entropy: f64) -> f64 {
        // Правило 111-ї групи: контроль знаменника
        if self.valence_energy <= 1e-9 {
            self.valence_energy = 0.001; 
        }

        let ratio = input_entropy / self.valence_energy;
        let drift = ratio - EQUILIBRIUM;

        // Адаптивне дихання системи
        if drift.abs() > 0.05 {
            // Дисонанс: стискаємо валентну зону (підвищуємо чутливість)
            self.valence_energy *= 1.0 - (drift.signum() * 0.02);
            self.is_critical = true;
        } else {
            // Гармонія: релаксація до одиничного стану
            self.valence_energy = self.valence_energy * 0.95 + 0.05;
            self.is_critical = false;
        }

        drift
    }

    /// Threat Probability (Колапс хвильової функції).
    /// Обчислює ймовірність атаки через Septimal Shift.
    pub fn threat_probability(&self, input_entropy: f64) -> f64 {
        if self.valence_energy <= 0.0 { return 1.0; }
        
        let drift = (input_entropy / self.valence_energy) - EQUILIBRIUM;
        
        // Квадрат синуса для отримання імовірності [0, 1]
        (drift * PI * SEPTIMAL_SHIFT).sin().powi(2)
    }

    /// Встановлює критичний стан (викликається при Negative Radius Overflow)
    pub fn set_critical(&mut self, state: bool) {
        self.is_critical = state;
        if state {
            // При критичному стані ядро "важчає"
            self.valence_energy *= 0.8;
        }
    }

    pub fn get_metrics(&self) -> (f64, f64, bool) {
        (self.valence_energy, self.electron_cloud, self.is_critical)
    }
}
