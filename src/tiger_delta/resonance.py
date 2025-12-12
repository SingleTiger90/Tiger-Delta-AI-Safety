import json
import time
import os

class TigerResonance:
    """
    Модуль резонансної пам'яті.
    Зберігає не текст, а фізичні вектори ентропії та стабільності.
    """
    def __init__(self, filepath="tiger_state.json"):
        self.filepath = filepath
        self.state = self._load_state()

    def _load_state(self):
        if os.path.exists(self.filepath):
            try:
                with open(self.filepath, 'r', encoding='utf-8') as f:
                    return json.load(f)
            except json.JSONDecodeError:
                return self._new_born()
        return self._new_born()

    def _new_born(self):
        return {
            "cycles": 0, 
            "entropy_vectors": [], 
            "stable_patterns": []
        }

    def process_impulse(self, input_hash, entropy):
        # Поріг стабільності для шкали 0-1,000,000
        # Якщо ентропія < 1000, це вважається "тишею" (сигналом)
        is_stable = entropy < 1000.0 
        
        self.state["entropy_vectors"].append({
            "t": time.time(),
            "impulse": input_hash,
            "entropy": entropy,
            "is_stable": is_stable
        })
        self._crystallize()
        return {"entropy": entropy, "stable": is_stable}

    def _crystallize(self):
        """Ущільнення даних: відкидаємо шум, зберігаємо патерни."""
        if len(self.state["entropy_vectors"]) > 50:
            stable = [v for v in self.state["entropy_vectors"] if v["is_stable"]]
            if stable:
                self.state["stable_patterns"].append({
                    "cycle": self.state["cycles"], 
                    "count": len(stable)
                })
            self.state["entropy_vectors"] = []
            self.state["cycles"] += 1
            self._save()

    def _save(self):
        with open(self.filepath, 'w', encoding='utf-8') as f:
            json.dump(self.state, f, indent=2)
