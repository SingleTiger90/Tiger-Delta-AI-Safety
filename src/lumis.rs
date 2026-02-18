// src/lumis.rs

use std::f64::consts::PI;
use log::info; // Використовуємо стандартний фасад логування

pub const PHI: f64 = 1.6180339887498948;
pub const PHI_INVERSE: f64 = 0.6180339887498948;

/// LumisCore — адаптивне ядро захисту, натхненне біологічними циклами.
/// Керує ентропією системи та енергетичним балансом.
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

    /// Основний життєвий цикл: дихання, очищення, відновлення.
    /// tick_cycle викликається на кожну ітерацію обробки пакетів.
    pub fn tick_cycle(&mut self, external_impact: f64, resonance: f64, mass: &mut f64) {
        self.tick = self.tick.wrapping_add(1);

        // Умова спокою: низький зовнішній вплив та низький резонанс ядра
        let quiet = external_impact.abs() < 0.001 && resonance < 0.05;

        if quiet {
            self.rest_ticks += 1;
        } else {
            self.rest_ticks = 0;
            self.in_rest = false;
        }

        // Автоматичний перехід у режим сну після 500 "тихих" тіків
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

        // Механізм EXHALE (Видих): сильний тиск змушує систему скидати ентропію
        if pressure > 0.5 {
            let purge = pressure * 0.2;
            self.entropy -= purge;

            // Плата за очищення: зменшення маси захисту (Father's Apparatus)
            // Правило 111-ї групи: тримаємо масу не нижче 100.0 для стабільності знаменника
            *mass -= purge * 0.05;
            if *mass < 100.0 {
                *mass = 100.0;
            }

            if self.tick % 50 == 0 {
                info!("EXHALE: Purge={:.3}, Mass={:.2}", purge, *mass);
            }
        } else {
            // Механізм INHALE (Вдих): накопичення досвіду/ентропії при слабкому тиску
            self.accumulate_entropy(pressure * 0.1);
        }

        // Внутрішній гармонічний шум (тремор системи)
        let internal_noise = (self.tick as f64 * 0.0001).sin() * 0.01;
        self.entropy += internal_noise;

        self.entropy = self.entropy.clamp(0.0, 1.0);
        self.dissipate_entropy();
    }

    fn rest_mode(&mut self) {
        // Швидке відновлення в стані спокою
        self.entropy *= 0.90;
        if self.entropy < 0.05 {
            self.entropy = 0.0;
        }
    }

    fn accumulate_entropy(&mut self, delta: f64) {
        self.entropy = (self.entropy + delta).clamp(0.0, 1.0);
    }

    fn dissipate_entropy(&mut self) {
        // Нелінійна дисипація: чим вища ентропія, тим активніше скидання
        if self.entropy > 0.8 {
            self.entropy *= 0.5; // Критичний скид
        } else {
            self.entropy *= 0.99; // Природне охолодження
        }
    }

    /// Обчислює динамічний поріг детекції на основі ентропії та золотого перетину.
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
