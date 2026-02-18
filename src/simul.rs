// src/simul.rs

use std::f64::consts::PI;
use crate::lumis::PHI;

/// SIMUL: Віртуальна пісочниця для проекції ентропії.
/// ------------------------------------------------
/// Дозволяє тестувати вплив трафіку в "безпечному просторі"
/// перед тим, як приймати рішення в основному ядрі.
pub struct SimulUnit {
    projection_entropy: f64,
    stability_index: f64,
    learning_rate: f64,
}

impl SimulUnit {
    pub fn new() -> Self {
        SimulUnit {
            projection_entropy: 0.0,
            stability_index: PHI, // Золотий перетин як база стабільності
            learning_rate: 0.001,
        }
    }

    /// Проектує вхідний удар у пісочницю.
    /// Повертає true, якщо симульована ентропія ламає φ-стабільність.
    pub fn project_impact(&mut self, impact_force: f64) -> bool {
        // 1. Моделюємо віртуальний перегрів (virtual heat)
        // Коефіцієнт 0.5 дозволяє симуляції бути "чутливішою" за реальність
        let virtual_heat = impact_force * 0.5;
        self.projection_entropy += virtual_heat;

        // 2. Аналіз стійкості
        if self.projection_entropy > self.stability_index {
            // Alarm: система в симуляції не витримала.
            // Декуплимо стан: частковий ресет, щоб не втратити контекст повністю.
            self.projection_entropy *= 0.1;
            
            // Підвищуємо поріг стабільності (симуляція "навчається" на помилці)
            self.stability_index += self.learning_rate;
            
            return true; // Тривога: потрібен захисний імпульс AntiTiger
        }

        // Природне охолодження симуляції (entropy decay)
        self.projection_entropy *= 0.95;
        false
    }

    /// Повертає "фейковий" стан для децепції (обману атакуючого).
    /// Генерує непередбачуваний, але обмежений шум на базі синусоїди.
    pub fn get_decoy_state(&self) -> f64 {
        // Використовуємо PI для створення гармонійного обману
        (self.projection_entropy * PI).sin().abs()
    }

    /// Оновлює параметри симуляції на основі реального досвіду (XP) з Python-шару.
    pub fn sync_with_reality(&mut self, real_stability: f64) {
        // Поступово підтягуємо віртуальну стабільність до реальної
        self.stability_index = self.stability_index * 0.9 + real_stability * 0.1;
    }

    pub fn reset(&mut self) {
        self.projection_entropy = 0.0;
        self.stability_index = PHI;
    }
}
