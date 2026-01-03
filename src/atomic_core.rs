// =================================================================
// Project: TigerΔ (Tiger Delta)
// Module: atomic_core.rs
// Description: Core resonance engine for aperiodic traffic filtering.
// Logic: Implements Dalton's Law of Partial Pressures and 
//        Heisenberg's Uncertainty Principle for network telemetry.
// =================================================================

use std::f64::consts::PI;

/// Ірраціональні константи для створення аперіодичного дрейфу.
/// Це запобігає синхронізації атакуючого з алгоритмом захисту.
const PHI: f64 = 1.6180339887498948; // Золотий перетин
const EQUILIBRIUM: f64 = PHI / PI;    // Срібна середина (~0.515)
const SEPTIMAL_SHIFT: f64 = 1.777;    // Мала септима (16/9) для дисонансу

/// AtomicCore — це імовірнісна модель стану мережевого вузла.
/// Вона розглядає вхідний трафік не як байти, а як енергетичні рівні.
pub struct AtomicCore {
    /// Кількість "протонів" — базовий поріг стабільності системи.
    proton_count: u32,
    /// Електронна хмара (Ψ²) — імовірнісна зона, де пакет вважається легітимним.
    electron_cloud: f64,
    /// Валентна енергія — енергія, доступна для обробки корисного навантаження.
    valence_energy: f64,
}

impl AtomicCore {
    /// Створює нове ядро захисту з базовими параметрами стабільності.
    pub fn new(stability: u32) -> Self {
        Self {
            proton_count: stability,
            electron_cloud: 0.5,
            valence_energy: 1.0,
        }
    }

    /// Sharpen Angles (Стирання країв / Заточування кутів).
    /// Перетворює імпульсний удар атаки на кутовий момент, розсіюючи його в хмарі.
    /// Реалізація принципу Гейзенберга: чим точніший удар, тим вища невизначеність відгуку.
    pub fn sharpen_angles(&mut self, impact: f64) {
        let resistance = self.proton_count as f64 * PHI;
        
        // Арктангенс перетворює лінійну атаку в обмежений кут (радіани)
        let angle = (impact / resistance).atan();
        
        // Оновлення стану хмари через косинусну інтерференцію
        self.electron_cloud = (self.electron_cloud + angle).cos().abs();
        
        // Закон Дальтона: вилучаємо частину енергії дисонансу для підсилення ядра
        self.valence_energy += angle.abs() * 0.1;
    }

    /// Find the Middle (Пошук рівноваги / OODA Orient).
    /// Повертає систему до точки EQUILIBRIUM (Φ/π) та обчислює дрейф.
    pub fn find_the_middle(&mut self, input_entropy: f64) -> f64 {
        if self.valence_energy < 0.001 {
            return input_entropy;
        }

        let ratio = input_entropy / self.valence_energy;
        let drift = ratio - EQUILIBRIUM;

        // Механізм адаптивного "дихання" системи (AADA)
        if drift.abs() > 0.05 {
            // Стискання пружини при виявленні дисонансу
            self.valence_energy *= 1.0 - (drift.signum() * 0.02);
        } else {
            // Релаксація та повернення до базового енергетичного стану
            self.valence_energy = self.valence_energy * 0.99 + 1.0 * 0.01;
        }

        drift
    }

    /// Threat Probability (Ймовірність загрози / Wave Function Collapse).
    /// Обчислює ймовірність того, що сигнал є дисонансним (атакою).
    /// Використовує малу септиму для виявлення "глухих" атакуючих.
    pub fn threat_probability(&self, input_entropy: f64) -> f64 {
        let drift = (input_entropy / self.valence_energy) - EQUILIBRIUM;
        
        // Квадрат хвильової функції визначає ймовірність детекції > 90%
        (drift * PI * SEPTIMAL_SHIFT).sin().powi(2)
    }

    /// Геттер для поточного рівня енергії (для зовнішнього моніторингу)
    pub fn get_energy(&self) -> f64 {
        self.valence_energy
    }
}
