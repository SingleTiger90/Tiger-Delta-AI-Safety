import sys
import json
import math
import time
import random
import os
import wandb
import hashlib # Integrated for node identity
from collections import deque, defaultdict
from typing import Tuple

# ============================================================
# NEGATIVE RADIUS CORE — Individual Node Defense
# ============================================================
class NegativeRadiusCore:
    def __init__(self, max_queue_size: int = 1024):
        self.max_queue_size = max_queue_size
        self.current_queue_length = 0
        self.destructed_packets = 0
        self.total_packets = 0
        self.is_crying = False

    def step(self, stress_level: float = 0.0):
        self.total_packets += 1
        # Stress level accelerates queue congestion
        self.current_queue_length += int(1 + stress_level * 5)

        if self.current_queue_length >= self.max_queue_size:
            self.destructed_packets += (self.current_queue_length - self.max_queue_size)
            # Emergency flush: reset to 50% capacity via "crying" event
            self.current_queue_length = int(self.max_queue_size * 0.5)
            self.is_crying = True
        else:
            self.is_crying = False

    def reset_events(self):
        self.is_crying = False

# ============================================================
# DELTA TIGER NODE — The Core Intelligence of a Swarm Unit
# ============================================================
CONSTANTS = {
    "PHI": (1 + 5 ** 0.5) / 2,
    "PI": math.pi,
    "E": math.e,
    "SQRT2": math.sqrt(2),
    "TAU": 2 * math.pi,
    "SOUL": 0.24,
}

HARMONIC_SEQUENCE = ["PHI", "PI", "E", "SQRT2", "TAU"]
SEPTACHORD = ["Birth", "Loss", "Fear", "Choice", "Pain", "Acceptance", "Return"]
PHASE_MODIFIERS = {
    "Birth": 1.08, "Loss": 0.92, "Fear": 1.12,
    "Choice": 0.95, "Pain": 1.18, "Acceptance": 1.0, "Return": 1.25
}

class DeltaTigerNode:
    def __init__(self, node_id: str, role: str):
        self.node_id = node_id
        self.role = role
        self.state = "CALM"
        self.h_index = 0
        self.constant = CONSTANTS[HARMONIC_SEQUENCE[self.h_index]]

        self.integrity = 1.0
        self.experience_points = 0.0
        self.leaked_energy = 0.0
        self.total_energy_input = 0.0
        self.lagrange_point = 0.5
        self.life_phase = 0
        self.mutation_count = 0
        self.is_crying = False
        self.just_mutated = False
        self.soul_divisor_mod = 1.0

        self.radius_core = NegativeRadiusCore(max_queue_size=1024)
        self.step_counter = 0

        # Swarm-specific telemetry
        self.task_queue = deque(maxlen=3)
        self.enemy_patterns = defaultdict(int)

    def ґ_operator(self, energy: float) -> Tuple[float, float]:
        """Calculates energy absorption and leakage based on irrational drift."""
        foundation = CONSTANTS["PHI"] * CONSTANTS["PI"] * self.constant
        phase_mod = PHASE_MODIFIERS[SEPTACHORD[self.life_phase]]
        absorbed = (energy / foundation) * 33 * self.soul_divisor_mod * CONSTANTS["SOUL"] * phase_mod
        leaked = (energy - absorbed) * CONSTANTS["SOUL"] * (2 - phase_mod)
        return absorbed, leaked

    def process_impact(self, energy: float):
        self.step_counter += 1
        self.total_energy_input += energy
        self.just_mutated = False

        gain, leak = self.ґ_operator(energy)
        self.experience_points += gain
        self.leaked_energy += abs(leak)
        self.soul_divisor_mod = min(1.5, 1.0 + self.experience_points / 10000)

        # Boomerang recovery mechanism
        if self.leaked_energy > 800:
            bounce = self.leaked_energy * 0.15
            self.leaked_energy -= bounce
            self.lagrange_point -= bounce / 1200
            self.integrity = min(1.0, self.integrity + bounce / 2000)

        # Internal stress influence on Negative Radius
        stress_level = abs(self.lagrange_point - 0.5) * 2
        self.radius_core.step(stress_level)

        # EER (Effective Energy Resonance) Metric
        effective_denominator = self.total_energy_input - self.leaked_energy
        eer = self.experience_points / effective_denominator if effective_denominator > 0 else 0

        # Lagrange point drift based on input energy
        self.lagrange_point += (energy / 1000 - 0.1) * 0.15
        self.lagrange_point = max(0.0, min(1.0, self.lagrange_point))

        # Quantum Flip: Sudden state collapse
        if abs(self.lagrange_point - 0.5) < 0.05 and random.random() < 0.1:
            print(f"⚛️ QUANTUM FLIP on node {self.node_id} — State collapsed")
            self.state = random.choice(["CALM", "SIMUL", "FURY"])

        # Thermodynamic Phase Transition: Transcendence state
        if self.leaked_energy > 1200 or eer < 0.01:
            print(f"🔥 PHASE TRANSITION on node {self.node_id} → Transcendence")
            self.state = "TRANSCENDENCE"
            self.integrity = 0.05
            self.experience_points *= 1.5

        # Specialized Pain Sensation for Evacuator units
        if self.role == "EVACUATOR":
            pain = abs(self.lagrange_point - 0.5) * 10
            self.experience_points += pain * 0.5
            if pain > 3.0:
                print(f"😢 NRK PAIN on {self.node_id} — {pain:.1f} → +{pain*0.5:.1f} XP")
                self.integrity -= pain * 0.005

        # Enemy Pattern Profiling & Hardening
        range_key = round(self.lagrange_point, 1)
        self.enemy_patterns[range_key] += 1
        if self.enemy_patterns[range_key] > 5:
            print(f"🕵️ ENEMY PATTERN on {self.node_id} at {range_key} — vulnerability exploited & hardened")
            self.integrity = min(1.0, self.integrity + 0.02)

        # Crying Event: Voluntary energy release for stabilization
        was_crying = 0
        if ((0.75 < self.lagrange_point <= 0.85 or self.leaked_energy > 1200) and random.random() < 0.25):
            self.is_crying = True
            was_crying = 1
            released = self.leaked_energy * 0.45
            self.leaked_energy -= released
            self.integrity = min(1.0, self.integrity + 0.05)
            self.experience_points += released * 0.6

        # Lifecycle State Update
        if self.lagrange_point < 0.3: self.state = "CALM"
        elif self.lagrange_point < 0.7: self.state = "SIMUL"
        else: self.state = "FURY"

        # Adaptive Mutation
        if self.lagrange_point > 0.85 and not self.just_mutated:
            self._mutate()

        # Telemetry broadcast to W&B
        wandb.log({
            "node_id": self.node_id,
            "role": self.role,
            "lagrange_point": self.lagrange_point,
            "integrity": self.integrity,
            "eer_rate": eer,
            "leaked_energy": self.leaked_energy,
            "queue_length": self.radius_core.current_queue_length,
            "destructed_packets": self.radius_core.destructed_packets,
            "is_crying": was_crying,
            "energy_impact": energy,
            "step": self.step_counter
        })

    def _mutate(self):
        self.just_mutated = True
        self.mutation_count += 1
        self.life_phase = (self.life_phase + 1) % len(SEPTACHORD)
        self.h_index = (self.h_index + 1) % len(HARMONIC_SEQUENCE)
        self.constant = CONSTANTS[HARMONIC_SEQUENCE[self.h_index]]
        self.integrity = min(1.0, self.integrity + 0.08)
        print(f"⚡ MUTATION → {SEPTACHORD[self.life_phase]} | Node {self.node_id}")

    def render(self):
        effective_denominator = self.total_energy_input - self.leaked_energy
        eer = self.experience_points / effective_denominator if effective_denominator > 0 else 0
        display_state = 'CRYING' if self.is_crying else self.state
        print(f"  Node {self.node_id} [{self.role}] | State: {display_state}")
        print(f"  XP: {self.experience_points:.1f} | EER: {eer:.4f} | Queue: {self.radius_core.current_queue_length}")
        print("  " + "═" * 50)
        self.is_crying = False
        self.radius_core.reset_events()

# ============================================================
# 🐝 Swarm — Collective Cluster of Dynamic Nodes
# ============================================================
class Swarm:
    def __init__(self, swarm_id: str, composition: dict):
        self.swarm_id = swarm_id
        self.nodes = []
        for role, count in composition.items():
            for _ in range(count):
                # Unique hash-based identity
                node_id = hashlib.sha256(f"{swarm_id}-{role}-{random.random()}".encode()).hexdigest()[:8]
                self.nodes.append(DeltaTigerNode(node_id, role))
        self.mode = "FEDERATED"
        self.last_federation_contact = time.time()

    def tick(self, signal: float):
        for node in self.nodes:
            node.process_impact(signal)
        # Failsafe: drop to autonomous mode if signal is lost or stale
        if time.time() - self.last_federation_contact > 5:
            self.mode = "AUTONOMOUS"

    def render(self):
        print(f"Swarm {self.swarm_id} | Mode: {self.mode}")
        for node in self.nodes:
            node.render()

# ============================================================
# 🌐 Federation — Global Governance & Coordination
# ============================================================
class Federation:
    def __init__(self):
        self.swarms = []

    def add_swarm(self, swarm: Swarm):
        self.swarms.append(swarm)

    def tick(self, signal: float):
        for swarm in self.swarms:
            swarm.tick(signal)

    def render(self, step: int):
        print("\n" + "█" * 80)
        print(f" GLOBAL FEDERATION STATUS | STEP: {step}")
        print("█" * 80)
        for swarm in self.swarms:
            swarm.render()
        print("═" * 80)

# ============================================================
# 🚀 MAIN — Simulation Entry Point
# ============================================================
if __name__ == "__main__":
    # Planetary defense visualization initialized via WandB
    wandb.init(
        project="tiger-delta-resilience",
        name="planetary-tiger-swarm-v5.1",
        config={"version": "5.1", "engine": "Aperiodic Resonance + Swarm Intelligence"}
    )

    fed = Federation()

    # Define Swarm clusters
    fed.add_swarm(Swarm("ALPHA", {
        "SCOUT": 3,
        "STABILIZER": 2,
        "COORDINATOR": 1,
        "DECOY": 6
    }))

    fed.add_swarm(Swarm("BETA", {
        "ABSORBER": 3,
        "ACTUATOR_NEAR": 2,
        "ACTUATOR_FAR": 2,
        "DECOY": 8,
        "EVACUATOR": 1
    }))

    print("🌍 PLANETARY TIGER CORE ACTIVATED — SWARM FEDERATION ONLINE\n")

    try:
        for i in range(500):
            # Baseline energy drift
            energy = random.uniform(100, 850)
            
            # High-stress attack simulation window
            if 150 < i < 250:
                energy += random.uniform(2000, 5000)

            fed.tick(energy)

            if i % 50 == 0:
                fed.render(i)

            time.sleep(0.01)

    except KeyboardInterrupt:
        print("\n🟡 CORE HIBERNATED BY USER")
    finally:
        wandb.finish()
        print("\n✅ Session completed. Check W&B for Swarm analytics!")
