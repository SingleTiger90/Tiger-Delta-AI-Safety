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
// КОНСТАНТИ
// ==========================================
const double GOLDEN_CORTISOL = 38.2;

// Складність пастки (кількість нулів у хеші)
// Чим вище число, тим глибший "вогневий мішок"
const int POW_DIFFICULTY = 3; 

std::random_device rd;
std::mt19937 gen(rd());

// ==========================================
// 🔥 МЕМБРАНА (ВОГНЕВИЙ МІШОК)
// ==========================================
class Membrane {
public:
    struct Challenge {
        size_t id;
        int salt;
        bool solved;
    };

    // "Передпокій" - тут ми тримаємо ворогів, поки вони вирішують задачу
    std::map<size_t, Challenge> sack; 

    // Генерація виклику (це коштує нам 0 енергії)
    int create_challenge(size_t packet_id) {
        int salt = gen(); // Випадкове число
        sack[packet_id] = {packet_id, salt, false};
        return salt;
    }

    // Перевірка відповіді (це коштує нам 0.0001 енергії)
    bool verify(size_t packet_id, size_t answer) {
        if (sack.find(packet_id) == sack.end()) return false; // Немає такого в мішку
        
        int salt = sack[packet_id].salt;
        
        // Перевіряємо: hash(salt + answer) має закінчуватися на 000...
        // Тут спрощена імітація хешу для демо
        size_t check = std::hash<int>{}(salt) ^ std::hash<size_t>{}(answer);
        
        // Імітуємо перевірку складності
        bool passed = (check % (int)std::pow(10, POW_DIFFICULTY)) == 0;
        
        if (passed) {
            sack.erase(packet_id); // Випускаємо з мішка до Тигра
            return true;
        }
        return false;
    }
};

// ==========================================
// 🐯 LUMIS (ЯДРО)
// ==========================================
class Lumis {
public:
    double energy = 100.0;
    
    void process_data(std::string data) {
        // Сюди доходять тільки ті, хто вижив у мішку
        energy -= 1.0; // Витрачаємо енергію на корисну роботу
        std::cout << "   🐯 [LUMIS]: Processing data... (Energy: " << (int)energy << "%)\n";
    }
};

// ==========================================
// 🏴‍☠️ ХАКЕР (SIMULATION)
// ==========================================
class Hacker {
public:
    std::string name;
    double cpu_resource = 100.0;

    Hacker(std::string n) : name(n) {}

    // Хакер намагається вирішити задачу
    size_t solve_pow(int salt) {
        std::cout << "   💀 [" << name << "]: Trapped in Sack. Solving puzzle...\n";
        
        // Симуляція брутфорсу (спалювання ресурсу хакера)
        // У реальності це цикл while(true)
        double cost = 20.0; // Це дорого!
        cpu_resource -= cost;
        
        // Імітуємо, що він знайшов відповідь (або ні)
        // Для демо просто повертаємо число, яке "має підійти"
        // У реальності хакер витратив би тут секунди процесорного часу
        std::this_thread::sleep_for(std::chrono::milliseconds(500)); 
        
        return 12345; // Фейкова відповідь, припустимо правильна для демо
    }
};

// ==========================================
// ГОЛОВНИЙ ЦИКЛ
// ==========================================
int main() {
    Membrane membrane;
    Lumis lumis;
    Hacker bad_guy("Botnet-X");

    std::cout << "=== OODA FIRE SACK INITIATED ===\n";
    std::cout << "Trap Difficulty: " << POW_DIFFICULTY << "\n\n";

    size_t packet_id = 101;

    // 1. АТАКА (Observe)
    std::cout << ">>> INCOMING CONNECTION (ID: " << packet_id << ")\n";

    // 2. ЗАМАНЮВАННЯ В МІШОК (Orient)
    // Ми не пускаємо до Lumis. Ми даємо задачу.
    int salt = membrane.create_challenge(packet_id);
    std::cout << "🛡️ [MEMBRANE]: Challenge sent (Salt: " << salt << ").\n";

    // 3. ХАКЕР ПРАЦЮЄ (Decide/Act)
    // Хакер змушений витрачати свій ресурс
    if (bad_guy.cpu_resource > 0) {
        size_t answer = bad_guy.solve_pow(salt);
        std::cout << "   💀 [" << bad_guy.name << "]: Answer sent. CPU Left: " << (int)bad_guy.cpu_resource << "%\n";

        // 4. ПЕРЕВІРКА (Fire!)
        // Ми перевіряємо миттєво.
        // *Примітка: Тут я форсую true для демо, щоб показати прохід*
        // У реальному коді тут реальна математика.
        bool access_granted = true; // membrane.verify(packet_id, answer);

        if (access_granted) {
            std::cout << "✅ [MEMBRANE]: Valid PoW. Forwarding to Core.\n";
            lumis.process_data("Payload");
        } else {
            std::cout << "⛔ [MEMBRANE]: Invalid PoW. DROP.\n";
        }
    } else {
        std::cout << "💀 [" << bad_guy.name << "]: CPU Exhausted. Attack stopped.\n";
    }

    return 0;
}
