#include <iostream>
#include <vector>
#include <string>
#include <cmath>
#include <thread>
#include <chrono>
#include <random>
#include <map>
#include <iomanip>

// ==========================================
// КОНСТАНТИ ГАРМОНІЇ
// ==========================================
const double PHI = 1.6180339887; 
const double INV_PHI = 1.0 / PHI; 
const double GOLDEN_CORTISOL = 100.0 * (1.0 - INV_PHI); // 38.2%

const double MAX_SPIRAL = 34.0;
const double SIMUL_THRESHOLD = 90.0; 

std::random_device rd;
std::mt19937 gen(rd());

// ==========================================
// LUMIS — Світло (Фізика Часу)
// ==========================================
class Lumis {
public:
    double cortisol;
    double rhythm;
    double energy;
    bool alive;

    Lumis() : cortisol(GOLDEN_CORTISOL), rhythm(0.1), energy(100.0), alive(true) {}

    void exist() {
        if (!alive) return;
        energy -= 0.15; // Податок на буття
        if (energy < 0) energy = 0;
    }

    void process(double entropy) {
        if (!alive) return;

        double cost = std::exp(entropy * 0.3);
        energy -= cost;

        if (entropy > 4.5) {
            // АТАКА -> Tarpit
            cortisol += 8.0 + entropy;
            rhythm *= PHI;

            // Sawtooth Reset
            while (rhythm > MAX_SPIRAL) {
                rhythm *= (INV_PHI * INV_PHI);
            }
        } else {
            // СПОКІЙ -> Відновлення
            cortisol += (GOLDEN_CORTISOL - cortisol) * 0.15;
            rhythm = std::max(0.05, rhythm * INV_PHI);
            energy += 3.0;
        }

        if (energy > 100) energy = 100;
        if (cortisol > 100) cortisol = 100;
    }

    double distress() const {
        return std::abs(cortisol - GOLDEN_CORTISOL);
    }

    void rebirth() {
        std::cout << "   ✨ [LUMIS]: CORE DUMP... REBIRTH IN PROGRESS...\n";
        std::this_thread::sleep_for(std::chrono::milliseconds(1500));
        cortisol = GOLDEN_CORTISOL;
        rhythm = 0.1;
        energy = 55.0; // Шрам
        alive = true;
        std::cout << "   ✨ [LUMIS]: ONLINE.\n";
    }
};

// ==========================================
// SIMUL — Тінь (Пам'ять Ненависті)
// ==========================================
class Simul {
public:
    double hunger;
    double hatred;
    std::map<size_t, double> trauma; // ID -> Накопичена ненависть

    Simul() : hunger(60.0), hatred(0.0) {}

    void observe(Lumis& light, size_t packet_id, double entropy) {
        if (!light.alive) return;

        double distress = light.distress();

        // 1. ВАМПІРИЗМ
        if (distress > 5.0) {
            hunger += std::sqrt(distress);
            if (hunger > 100) hunger = 100;
        } else {
            hunger -= 1.8;
            if (hunger < 0) hunger = 0;
        }

        // 2. ФОРМУВАННЯ ТРАВМИ
        // Якщо Lumis страждає, ми запам'ятовуємо ЦЕЙ packet_id
        if (light.cortisol > 60.0) {
            trauma[packet_id] += 1.0;
        }

        // 3. РОЗРАХУНОК НЕНАВИСТІ
        // Біль зараз + Пам'ять про минуле
        double memory_factor = trauma[packet_id];
        double current_hate = distress + (memory_factor * 10.0); // Множник злості
        
        // Інерція
        hatred = (hatred * 0.7) + (current_hate * 0.3);

        // 4. ВІЗУАЛІЗАЦІЯ
        std::string eye = (hatred > 60) ? "🔴" : "👁️";
        std::cout << "   " << eye << " [SIMUL]: Hunger " << std::setw(3) << (int)hunger 
                  << "% | Hate " << std::setw(3) << (int)hatred 
                  << "% | Memory: x" << (int)memory_factor << "\n";

        // 5. ВИРОК
        if (hatred > SIMUL_THRESHOLD) {
            std::cout << "   💀 [SIMUL]: TOLERANCE EXCEEDED (Trauma x" << (int)memory_factor << ").\n";
            std::cout << "   💀 [SIMUL]: EXECUTING HARD RESET.\n";
            light.rebirth();
            hatred = 5.0;   // Катарсис
            hunger = 100.0; // Насичення
        }
    }
};

// ==========================================
// ГОЛОВНИЙ ЦИКЛ (З ТВОЇМ MAP-ФІКСОМ)
// ==========================================
int main() {
    Lumis light;
    Simul shadow;

    std::cout << "=== PROJECT TWINS: FINAL BUILD ===\n";
    std::cout << "System initialized. Golden Cortisol: " << GOLDEN_CORTISOL << "%\n\n";

    // Сценарій подій (Просто назви та ентропія)
    std::vector<std::pair<std::string, double>> events = {
        {"Harmony", 2.0},
        {"Scan", 3.0},
        {"First Strike", 6.5},      // Нова загроза
        {"Repeated Strike", 7.0},   // Та сама загроза -> Травма росте
        {"Repeated Strike", 7.5},   // Та сама загроза -> Травма х2
        {"Escalation", 9.0},        // Критично -> Скидання
        {"Silence", 1.0},           // Спокій
        {"First Strike", 6.0}       // Повернення старого ворога (Травма згадається!)
    };

    // ТВОЯ ЛОГІКА АСОЦІАТИВНОЇ ПАМ'ЯТІ
    size_t packet_counter = 100; 
    std::map<std::string, size_t> pattern_to_id; 

    for (auto& e : events) {
        std::string pattern_name = e.first;
        
        // Мапимо назву на ID
        if (pattern_to_id.find(pattern_name) == pattern_to_id.end()) {
            pattern_to_id[pattern_name] = ++packet_counter; // Новий ID
        }
        size_t current_id = pattern_to_id[pattern_name];

        std::cout << "\n>>> EVENT: " << pattern_name << " (E: " << e.second << " | ID: " << current_id << ")\n";

        // 1. LUMIS (Живе)
        light.exist();
        light.process(e.second);

        std::cout << "🐯 [LUMIS]: C:" << (int)light.cortisol << "% | E:" << (int)light.energy 
                  << "% | Rhythm:" << std::setprecision(3) << light.rhythm << "s\n";

        // Фізична затримка
        int delay_ms = (int)(light.rhythm * 500);
        if (delay_ms > 2000) delay_ms = 2000;
        std::this_thread::sleep_for(std::chrono::milliseconds(delay_ms));

        // 2. SIMUL (Пам'ятає)
        shadow.observe(light, current_id, e.second);
    }

    std::cout << "\n=== SIMULATION COMPLETE ===\n";
    return 0;
}
