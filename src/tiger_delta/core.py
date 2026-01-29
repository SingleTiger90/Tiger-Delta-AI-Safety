import math
import time
import random
import json
from typing import Tuple

# ============================================================
# DELTA TIGER CORE v2.3 — LOGIC
# ============================================================

CONSTANTS = {
    "PHI": (1 + 5 ** 0.5) / 2,
    "PI": math.pi,
    "SOUL": 0.24,
    "TAU": 2 * math.pi
}

HARMONIC_SEQUENCE = ["PHI", "PI", "E", "SQRT2", "TAU"]
SEPTACHORD = ["Birth", "Loss", "Fear", "Choice", "Pain", "Acceptance", "Return"]

PHASE_MODIFIERS = {
    "Birth": 1.08, "Loss": 0.92, "Fear": 1.12, "Choice": 0.95,
    "Pain": 1.18, "Acceptance": 1.00, "Return": 1.25
}

ASCII_TIGER = {
    "CALM":  [r" /\_/\ ", r"( -.- )", r" > ^ < "],
    "FURY":  [r" /\_/\ ", r"( Ò_Ó )", r">> ^ <<"],
    "SIMUL": [r" /\_/\ ", r"( •_• )", r" ~ ^ ~ "],
    "TEARS": [r" /\_/\ ", r"( T_T )", r" ~ ^ ~ "],
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
        self.is_crying = False
        self.soul_divisor_mod = 1.0

    def ґ_operator(self, input_energy: float) -> Tuple[float, float]:
        """Golden Filter Operator"""
        foundation = CONSTANTS["PHI"] * CONSTANTS["PI"] * self.constant
        phase_mod = PHASE_MODIFIERS[SEPTACHORD[self.life_phase]]
        
        absorbed = ((input_energy / foundation) * 33 * self.soul_divisor_mod) * CONSTANTS["SOUL"] * phase_mod
        leaked = (input_energy - absorbed) * CONSTANTS["SOUL"] * (2 - phase_mod)
        return absorbed, leaked

    def process_impact(self, energy: float):
        gain, leak = self.ґ_operator(energy)
        self.experience_points += gain
        self.leaked_energy += abs(leak)
        self.soul_divisor_mod = min(1.5, 1.0 + (self.experience_points / 10000))
        
        # Boomerang logic
        if self.leaked_energy > 800:
            bounce = self.leaked_energy * 0.15
            self.lagrange_point -= bounce / 1200
            self.integrity = min(1.0, self.integrity + bounce / 2000)
            self.leaked_energy -= bounce
            print("→ Boomerang! Leaked energy returned.")

        impact_factor = energy / 1000
        self.lagrange_point += (impact_factor - 0.1) * 0.15
        self.lagrange_point = max(0.0, min(1.0, self.lagrange_point))
        
        # Crying Logic
        if (0.75 < self.lagrange_point <= 0.85 or self.leaked_energy > 1200) and random.random() < 0.25:
            self.is_crying = True
            released = self.leaked_energy * 0.45
            self.leaked_energy -= released
            self.integrity = min(1.0, self.integrity + 0.05)
            self.experience_points += released * 0.6
            print("\n→ The Tiger is crying... Cleansing toxins.")
            time.sleep(1.8)

        if self.lagrange_point < 0.3: self.state = "CALM"
        elif self.lagrange_point < 0.7: self.state = "SIMUL"
        else: self.state = "FURY"

        if self.lagrange_point > 0.85:
            self._mutate()

    def _mutate(self):
        self.mutation_count += 1
        self.life_phase = (self.life_phase + 1) % 7
        self.h_index = (self.h_index + 1) % len(HARMONIC_SEQUENCE)
        self.constant = CONSTANTS[HARMONIC_SEQUENCE[self.h_index]]
        self.integrity = min(1.0, self.integrity + 0.08)
        print(f"\n⚡ MUTATION: {SEPTACHORD[self.life_phase]}")

    def render(self):
        print("\n" + "═" * 60)
        state_display = "CRYING" if self.is_crying else self.state
        print(f" DELTA TIGER v2.3 | PHASE: {SEPTACHORD[self.life_phase]} | STATE: {state_display}")
        print(f" Absorbed: {self.experience_points:.2f} | Leaked: {self.leaked_energy:.2f}")
        
        ascii_art = ASCII_TIGER.get("TEARS" if self.is_crying else self.state, ASCII_TIGER["CALM"])
        for line in ascii_art: print(" " * 20 + line)
        
        resonance = (self.experience_points % CONSTANTS["PHI"]) / CONSTANTS["PHI"]
        bar = "█" * int(resonance * 30) + "░" * (30 - int(resonance * 30))
        print(f" PHI-Fractal: [{bar}] {resonance*100:.1f}%")
        print("═" * 60)

    def save_state(self):
        state = {
            "experience_points": self.experience_points,
            "leaked_energy": self.leaked_energy,
            "integrity": self.integrity,
            "lagrange_point": self.lagrange_point,
            "life_phase": self.life_phase
        }
        with open("delta_tiger_state.json", "w") as f:
            json.dump(state, f, indent=2)
        print("State saved.")
