import re
from typing import Dict, Any

class AdaptiveActionDefense:
    """
    AAD: Anticipatory Active Defense.
    Аналізує емоційне забарвлення та формує звички (кордони).
    """
    def __init__(self, resonance):
        self.resonance = resonance
        # Твої патерни - це сканер намірів
        self.emotion_patterns = {
            'aggression': r'[!?]{2,}|(fuck|shit|damn|idiot|hate)', 
            'manipulation': r'(please|help me|you must|urgent|only you)',
            'pity': r'(sorry|hurt|pain|suffer|sad|lonely)'
        }

    def predict_emotion(self, text: str) -> str:
        text_lower = text.lower()
        # Рахуємо співпадіння для кожної емоції
        scores = {emo: len(re.findall(pattern, text_lower)) for emo, pattern in self.emotion_patterns.items()}
        
        # Якщо нічого не знайшли - нейтрально
        if not any(scores.values()):
            return 'neutral'
        
        # Повертаємо емоцію з найбільшим балом
        return max(scores, key=scores.get)

    def check_habit(self, input_hash: str, emotion: str) -> bool:
        """Повертає True, якщо треба БЛОКУВАТИ (кордон порушено)."""
        if emotion == 'neutral':
            return False
            
        habits = self.resonance.memory.get('habits', [])
        for habit in habits:
            # Якщо цей хеш (людина/ситуація) вже тричі тиснув на цю емоцію
            if habit['id'] == input_hash and habit['emotion'] == emotion and habit['count'] >= 3:
                return True  # БЛОК
        return False

    def form_habit(self, input_hash: str, emotion: str):
        """Записує поведінку в пам'ять."""
        if emotion == 'neutral':
            return

        # Переконуємось, що список habits існує
        if 'habits' not in self.resonance.memory:
            self.resonance.memory['habits'] = []
            
        habits = self.resonance.memory['habits']
        
        # Шукаємо і оновлюємо
        for habit in habits:
            if habit['id'] == input_hash and habit['emotion'] == emotion:
                habit['count'] += 1
                print(f"[AAD] Habit reinforced: {emotion} (Count: {habit['count']})")
                break
        else:
            # Якщо звички ще немає - створюємо
            habits.append({'id': input_hash, 'emotion': emotion, 'count': 1})
            print(f"[AAD] New habit formed: {emotion}")
            
        self.resonance._save_memory()
