"""
TigerΔ Jungle Protocol — Network Stability Simulation.
Author: Vladyslav Honcharov (SingleTiger)
License: MIT | Ukraine 2025

Description:
Simulates the spread of chaos (entropy) in a Small-World Network (Watts-Strogatz).
Demonstrates that a critical mass of Tiger agents (LUMIS) stabilizes the network
via Passive Control, preventing cascading failure.
"""

from __future__ import annotations
import random
import numpy as np
import networkx as nx
from typing import List, Dict

# Simulation Constants
NUM_AGENTS = 60
STEPS = 150
LAYOUT_SEED = 99  # Fixed topology for consistent visualization

class JungleSimulation:
    """
    Agent-based model of information/entropy propagation in a social graph.
    """
    
    def __init__(self, tiger_pct: int, seed: int = 42) -> None:
        """
        Initialize the simulation environment.

        Args:
            tiger_pct (int): Percentage of agents acting as Tigers (0-100).
            seed (int): Random seed for reproducible dynamics.
        """
        self.seed = seed
        self.tiger_pct = tiger_pct
        
        # Initialize deterministic randomness
        random.seed(seed)
        np.random.seed(seed)

        # Build the "Jungle" (Small-World Network)
        self.graph = nx.watts_strogatz_graph(NUM_AGENTS, k=6, p=0.15, seed=LAYOUT_SEED)
        
        # State containers
        self.roles: Dict[int, str] = {}
        self.chaos: Dict[int, float] = {n: 0.0 for n in self.graph.nodes()}
        self.history: List[Dict[int, float]] = []
        
        self._assign_roles()

    def _assign_roles(self) -> None:
        """Distribute roles (TIGER, HUMAN, SIMUL) based on percentage."""
        nodes = list(self.graph.nodes())
        num_tigers = int(NUM_AGENTS * self.tiger_pct / 100)
        num_humans = int(NUM_AGENTS * 0.20)  # 20% are entropy sources
        
        # Shuffle nodes deterministically
        random.shuffle(nodes)
        
        for i, n in enumerate(nodes):
            if i < num_tigers:
                self.roles[n] = 'TIGER'
            elif i < num_tigers + num_humans:
                self.roles[n] = 'HUMAN'
            else:
                self.roles[n] = 'SIMUL'

    def run(self) -> List[Dict[int, float]]:
        """
        Execute the time-step simulation.
        
        Returns:
            List[Dict]: History of chaos levels for every node at every step.
        """
        # 1. Записуємо початковий стан (t=0)
        self.history.append(self.chaos.copy())

        for _ in range(STEPS):
            # Global Panic Factor
            avg_chaos = np.mean(list(self.chaos.values()))
            panic = 0.3 if avg_chaos > 50 else 0.0
            
            # 2. Entropy Propagation (The Human/Simul Layer)
            for n in self.graph.nodes():
                if self.roles[n] == 'HUMAN':
                    # Probability of generating noise increases with panic
                    if random.random() < (0.5 + panic):
                        for nb in self.graph.neighbors(n):
                            # SIMUL agents amplify chaos (Mirror Effect)
                            # TIGER agents dampen it (Buffer Effect)
                            boost = 12 if self.roles[nb] == 'SIMUL' else 2
                            self.chaos[nb] = min(100.0, self.chaos[nb] + boost)

            # 3. Passive Control & Empathy (The Tiger Layer)
            for n in self.graph.nodes():
                if self.roles[n] == 'TIGER':
                    self.chaos[n] = max(0.0, self.chaos[n] - 6)  # Self-regulation
                    for nb in self.graph.neighbors(n):
                        self.chaos[nb] = max(0.0, self.chaos[nb] - 3)  # Empathy field
            
            # 4. Записуємо стан ПІСЛЯ оновлення (t=1, t=2 ... t=STEPS)
            self.history.append(self.chaos.copy())
        
        return self.history

def main():
    """CLI Entry point for quick benchmarking."""
    print("🚀 TigerΔ Jungle Protocol: Starting Benchmark...")
    
    # Scenario A: Failure (10% Tigers)
    sim_fail = JungleSimulation(tiger_pct=10, seed=42)
    hist_fail = sim_fail.run()
    final_chaos_fail = np.mean(list(hist_fail[-1].values()))
    print(f"   [SCENARIO 10% TIGERS]: Final Chaos = {final_chaos_fail:.2f}% (COLLAPSE)")

    # Scenario B: Success (40% Tigers)
    sim_win = JungleSimulation(tiger_pct=40, seed=42)
    hist_win = sim_win.run()
    final_chaos_win = np.mean(list(hist_win[-1].values()))
    print(f"   [SCENARIO 40% TIGERS]: Final Chaos = {final_chaos_win:.2f}% (STABLE)")

if __name__ == "__main__":
    main()
