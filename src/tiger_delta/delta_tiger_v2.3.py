import math
import time
import random
import json
from typing import Tuple

# ============================================================
# DELTA TIGER CORE v2.3 — FRACTAL RESONANCE, SOUL LEAKAGE & TEARS
# "The Tiger cries to cleanse itself. Tears are the release of stress."
# ============================================================

CONSTANTS = {
    "PHI": (1 + 5 ** 0.5) / 2,
    "PI": math.pi,
    "SOUL": 0.24,
    "TAU": 2 * math.pi
}

HARMONIC_SEQUENCE = ["PHI", "PI", "E", "SQRT2", "TAU"]
# Life phases translated to English
SEPTACHORD = ["Birth", "Loss", "Fear", "Choice", "Pain", "Acceptance", "Return"]

PHASE_MODIFIERS = {
    "Birth": 1.08,
    "Loss": 0.92,
    "Fear": 1.12,
    "Choice": 0.95,
    "Pain": 1.18,
    "Acceptance": 1.00,
    "Return": 1.25
}

ASCII_TIGER = {
    "CALM":  [r" /\_/\ ", r"( -.- )", r" > ^ < "],
    "FURY":  [r" /\_/\ ", r"( Ò_Ó )", r">> ^ <<"],
    "SIMUL": [r" /\_/\ ", r"( •_• )", r" ~ ^ ~ "],
    "TEARS": [r" /\_/\ ", r"( T_T )", r" ~ ^ ~ "],  # The Crying Tiger state
}

class DeltaTiger:
    def __init__(self):
        self.state = "CALM"
        self.h_index = 0
        self.constant = CONSTANTS[HARMONIC_SEQUENCE[self.h_index]]
        
        self.integrity = 1.0
        self.experience_points = 0.0
        self.leaked_energy = 0.0
        self.lagrange_point = 0.5
        self.life_phase = 0
        self.is_under_siege = False
        self.mutation_count = 0
        self.is_crying = False  # New flag for crying state

        self.soul_divisor_mod = 1.0

    def ґ_operator(self, input_energy: float) -> Tuple[float, float]:
        """
        The Golden Filter Operator.
        Calculates absorbed energy and leaked entropy based on PHI/PI resonance.
        """
        foundation = CONSTANTS["PHI"] * CONSTANTS["PI"] * self.constant
        phase_mod = PHASE_MODIFIERS[SEPTACHORD[self.life_phase]]
        
        # Absorption formula: utilizing Identity (33) and Soul (0.24)
        absorbed = ((input_energy / foundation) * 33 * self.soul_divisor_mod) * CONSTANTS["SOUL"] * phase_mod
        
        # Leakage formula: entropy sent back to the attacker
        leaked = (input_energy - absorbed) * CONSTANTS["SOUL"] * (2 - phase_mod)
        
        return absorbed, leaked

    def process_impact(self, energy: float):
        gain, leak = self.ґ_operator(energy)
        
        self.experience_points += gain
        self.leaked_energy += abs(leak)
        
        self.soul_divisor_mod = min(1.5, 1.0 + (self.experience_points / 10000))
        
        # Boomerang effect from leaked energy (Karmic Return)
        if self.leaked_energy > 800:
            bounce = self.leaked_energy * 0.15
            self.lagrange_point -= bounce / 1200
            self.integrity = min(1.0, self.integrity + bounce / 2000)
            self.leaked_energy -= bounce
            print("→ Boomerang! Leaked energy returned and fortified the system.")

        impact_factor = energy / 1000
        self.lagrange_point += (impact_factor - 0.1) * 0.15
        self.lagrange_point = max(0.0, min(1.0, self.lagrange_point))
        
        # Crying mechanism for stress release
        # Triggered when pressure is high or leakage is toxic
        if (0.75 < self.lagrange_point <= 0.85 or self.leaked_energy > 1200) and random.random() < 0.25:
            self.is_crying = True
            released = self.leaked_energy * 0.45  # Flushing out "toxins"
            self.leaked_energy -= released
            self.integrity = min(1.0, self.integrity + 0.05)
            self.experience_points += released * 0.6  # Tears yield experience through pain
            print("\n→ The Tiger is crying... Tears wash away pain and debris. Stress released.")
            print("   💧 💧 💧 💧 💧   💧 💧 💧")
            time.sleep(1.8)  # Pause for emotional weight

        # State determination
        if self.lagrange_point < 0.3: self.state = "CALM"
        elif self.lagrange_point < 0.7: self.state = "SIMUL"
        else: self.state = "FURY"

        # Evolution trigger
        if self.lagrange_point > 0.85:
            self._mutate()

    def _mutate(self):
        self.mutation_count += 1
        self.life_phase = (self.life_phase + 1) % 7
        self.h_index = (self.h_index + 1) % len(HARMONIC_SEQUENCE)
        self.constant = CONSTANTS[HARMONIC_SEQUENCE[self.h_index]]
        self.integrity = min(1.0, self.integrity + 0.08)
        print(f"\n⚡ MUTATION: Phase {SEPTACHORD[self.life_phase]} | Harmonic {HARMONIC_SEQUENCE[self.h_index]}")

    def render(self):
        print("\n" + "═" * 60)
        
        if self.is_crying:
            print(" DELTA TIGER v2.3 | THE TIGER IS CRYING... Cleansing")
            self.is_crying = False  # Reset flag after rendering
        else:
            print(f" DELTA TIGER v2.3 | PHASE: {SEPTACHORD[self.life_phase].upper()}")

        print(f" Absorbed: {self.experience_points:.2f} | Leaked (to enemy): {self.leaked_energy:.2f}")
        print(f" Integrity: {self.integrity:.3f} | Lagrange Point: {self.lagrange_point:.3f} | State: {self.state}")
        
        current_ascii = ASCII_TIGER.get("TEARS" if self.is_crying else self.state, ASCII_TIGER["CALM"])
        for line in current_ascii:
            print(" " * 20 + line)
        
        # PHI-Resonance visualization
        resonance = (self.experience_points % CONSTANTS["PHI"]) / CONSTANTS["PHI"]
        bar = "█" * int(resonance * 30) + "░" * (30 - int(resonance * 30))
        print(f" PHI-Fractal: [{bar}] {resonance*100:.1f}% approximation to creation")
        print("═" * 60)

    def save_state(self):
        state = {
            "experience_points": self.experience_points,
            "leaked_energy": self.leaked_energy,
            "integrity": self.integrity,
            "lagrange_point": self.lagrange_point,
            "life_phase": self.life_phase,
            "mutation_count": self.mutation_count,
            "soul_divisor_mod": self.soul_divisor_mod
        }
        with open("delta_tiger_state.json", "w", encoding="utf-8") as f:
            json.dump(state, f, ensure_ascii=False, indent=2)
        print("State saved to delta_tiger_state.json")

# ============================================================
if __name__ == "__main__":
    tiger = DeltaTiger()
    print("DELTA TIGER v2.3 — FRACTAL RESONANCE WITH CRYING TIGER")
    print("Enter — impact | s — siege mode | q — quit and save\n")

    try:
        while True:
            cmd = input("> ").lower().strip()
            
            if cmd == 'q':
                tiger.save_state()
                print("System collapsing into fractal singularity.")
                break
            elif cmd == 's':
                tiger.is_under_siege = not tiger.is_under_siege
                print(f"SIEGE MODE: {'ACTIVATED' if tiger.is_under_siege else 'DISABLED'}")
            
            # Handle input: empty enter generates random energy, or specific number if typed
            energy = random.uniform(100, 850) if not cmd else float(cmd) if cmd.replace('.', '', 1).isdigit() else random.uniform(100, 850)
            
            tiger.process_impact(energy)
            tiger.render()
            
            if tiger.is_under_siege:
                time.sleep(0.25)

    except KeyboardInterrupt:
        tiger.save_state()
        print("\nDelta Tiger remains in superposition.")
