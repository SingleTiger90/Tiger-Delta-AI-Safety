import json
import os
import time
from typing import Dict, List

class TigerResonance:
    """
    КІСТКОВИЙ МОЗОК (Persistent Memory).
    Відповідає за:
    1. Збереження досвіду (JSON).
    2. Пам'ять про біль (Шрами).
    3. Аналітику трендів (Історія Ентропії).
    """
    def __init__(self, filename="paternum_marrow.json"):
        # Файл пам'яті буде створюватися в корені проекту (рівнем вище src)
        base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
        self.filename = os.path.join(base_dir, filename)
        
        # Налаштування: скільки останніх імпульсів пам'ятати для графіків
        self.HISTORY_LIMIT = 100 
        self.memory = self._load_memory()

    def _load_memory(self) -> Dict:
        """Завантажує пам'ять з диска або створює нову."""
        if os.path.exists(self.filename):
            try:
                with open(self.filename, 'r', encoding='utf-8') as f:
                    return json.load(f)
            except Exception as e:
                print(f"[RESONANCE] Error loading memory: {e}. Creating new.")
                return self._genesis_state()
        return self._genesis_state()

    def _genesis_state(self):
        """Стан 'Народження'."""
        return {
            "created_at": time.time(),
            "scars": [],           # Список небезпечних хешів
            "total_impulses": 0,   # Лічильник досвіду
            "entropy_history": []  # Історія для трендів
        }

    def _save_memory(self):
        """Фізичний запис на диск."""
        try:
            with open(self.filename, 'w', encoding='utf-8') as f:
                json.dump(self.memory, f, indent=4, ensure_ascii=False)
        except Exception as e:
            print(f"[RESONANCE] CRITICAL: Cannot write to memory! {e}")

    def is_scarred(self, input_hash: str) -> bool:
        """Перевіряє, чи є цей хеш у списку шрамів."""
        if "scars" not in self.memory: return False
        
        for scar in self.memory["scars"]:
            if scar["id"] == input_hash:
                return True
        return False

    def process_impulse(self, input_hash: str, entropy: float, is_danger: bool = False):
        """
        Обробляє результат взаємодії.
        Зберігає ентропію в історію і формує шрами при загрозі.
        """
        # Ініціалізація полів для сумісності зі старими файлами
        if "entropy_history" not in self.memory: self.memory["entropy_history"] = []
        if "scars" not in self.memory: self.memory["scars"] = []
        if "total_impulses" not in self.memory: self.memory["total_impulses"] = 0

        # 1. Оновлюємо лічильник
        self.memory["total_impulses"] += 1
        
        # 2. Зберігаємо Ентропію для аналітики
        timestamp = time.time()
        self.memory["entropy_history"].append([timestamp, entropy])

        # 3. Чистка історії (Rolling Window)
        # Якщо записів більше ліміту -> залишаємо тільки останні N
        if len(self.memory["entropy_history"]) > self.HISTORY_LIMIT:
            self.memory["entropy_history"] = self.memory["entropy_history"][-self.HISTORY_LIMIT:]

        # 4. Обробка Загрози (Шрам)
        if is_danger:
            if not self.is_scarred(input_hash):
                new_scar = {
                    "id": input_hash,
                    "timestamp": timestamp,
                    "reason": f"High Entropy: {entropy}"
                }
                self.memory["scars"].append(new_scar)
                print(f"[RESONANCE] ⚠️ SCAR FORMED. New threat remembered.")
        
        self._save_memory()
