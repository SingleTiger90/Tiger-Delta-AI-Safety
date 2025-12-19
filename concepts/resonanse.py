import json
import os
import time
from typing import Dict

class TigerResonance:
    """
    КІСТКОВИЙ МОЗОК (Persistent Memory).
    Зберігає шрами, історію ентропії, звички для AAD.
    """
    def __init__(self, filename="paternum_marrow.json"):
        # Універсальний шлях: шукаємо корінь проекту
        current_dir = os.path.dirname(os.path.abspath(__file__))
        root_dir = current_dir
        
        # Шукаємо файл run.py (або main.py), щоб зрозуміти, де корінь
        # Це дозволяє переміщати src куди завгодно без помилок
        while not os.path.exists(os.path.join(root_dir, "run.py")) and len(root_dir) > 3:
            root_dir = os.path.dirname(root_dir)
        
        self.filename = os.path.join(root_dir, filename)
        # print(f"[RESONANCE] Memory file: {self.filename}")  # Розкоментуй для дебагу, якщо треба
        
        self.HISTORY_LIMIT = 100
        self.memory = self._load_memory()

    def _load_memory(self) -> Dict:
        if os.path.exists(self.filename):
            try:
                with open(self.filename, 'r', encoding='utf-8') as f:
                    data = json.load(f)
                    # print(f"[RESONANCE] Memory loaded. Impulses: {data.get('total_impulses', 0)}")
                    return data
            except Exception as e:
                print(f"[RESONANCE] Corrupted memory! Starting fresh: {e}")
        
        # print("[RESONANCE] New life begins.")
        return self._genesis_state()

    def _genesis_state(self) -> Dict:
        return {
            "created_at": time.time(),
            "scars": [],
            "habits": [],           # Для AAD — звички та кордони
            "total_impulses": 0,
            "entropy_history": []
        }

    def _save_memory(self):
        try:
            with open(self.filename, 'w', encoding='utf-8') as f:
                json.dump(self.memory, f, indent=4, ensure_ascii=False)
        except Exception as e:
            print(f"[RESONANCE CRITICAL] Cannot save memory: {e}")

    def is_scarred(self, input_hash: str) -> bool:
        for scar in self.memory.get("scars", []):
            if scar["id"] == input_hash:
                return True
        return False

    def process_impulse(self, input_hash: str, entropy: float, is_danger: bool = False):
        """
        Обробляє лише фізичні показники (ентропія, шрами).
        Habits (звички) керуються окремо через AAD.
        """
        # Ініціалізація полів (setdefault - це топ, безпечно і швидко)
        self.memory.setdefault("scars", [])
        self.memory.setdefault("habits", [])
        self.memory.setdefault("total_impulses", 0)
        self.memory.setdefault("entropy_history", [])

        self.memory["total_impulses"] += 1
        
        timestamp = time.time()
        self.memory["entropy_history"].append([timestamp, entropy])
        
        # Rolling Window (щоб файл не розпух)
        if len(self.memory["entropy_history"]) > self.HISTORY_LIMIT:
            self.memory["entropy_history"] = self.memory["entropy_history"][-self.HISTORY_LIMIT:]

        # Якщо небезпека і це не повтор старого шраму — записуємо
        if is_danger and not self.is_scarred(input_hash):
            new_scar = {
                "id": input_hash,
                "timestamp": timestamp,
                "reason": f"High Entropy or AAD Trigger (Entropy: {entropy:.0f})"
            }
            self.memory["scars"].append(new_scar)
            print(f"[RESONANCE] ⚠️ SCAR FORMED.")

        self._save_memory()
