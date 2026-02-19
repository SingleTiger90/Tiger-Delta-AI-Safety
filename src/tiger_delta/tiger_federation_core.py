import random
import math
from collections import deque, defaultdict

# ============================================
# 🔢 HELPERS
# ============================================

def clamp(x, a=0.0, b=1.0):
    return max(a, min(b, x))

# ============================================
# 🧬 NODE
# ============================================

class SwarmNode:
    def __init__(self, node_id, role, swarm_id):
        self.id = node_id
        self.role = role
        self.swarm_id = swarm_id

        self.integrity = 1.0
        self.xp = 0.0

        self.lyapunov = 0.0
        self.kesler = 0.0

        self.noise = deque(maxlen=16)
        self.task_queue = deque(maxlen=3)

    # --------------------------------------------
    # Observation (Unified Field input)
    # --------------------------------------------
    def observe(self, field_signal):
        noise = random.random()
        self.noise.append(noise)

        avg_noise = sum(self.noise) / len(self.noise)
        delta = field_signal - avg_noise

        self.lyapunov = 0.9 * self.lyapunov + 0.1 * (delta ** 2)
        self.kesler = 0.97 * self.kesler + 0.03 * abs(delta)

        stress = self.lyapunov + self.kesler
        self.integrity -= stress * 0.015
        self.xp += stress * 3.0

        self.integrity = clamp(self.integrity)

    # --------------------------------------------
    # Execution Layer (symbolic actions)
    # --------------------------------------------
    def execute(self):
        if not self.task_queue:
            return

        task = self.task_queue.pop()

        if task == "SELF_DESTRUCT":
            self.integrity = 0.0

        elif task == "CAMOUFLAGE":
            self.integrity += 0.02
            self.xp += 1.0

        elif task == "RECON":
            self.xp += 2.5

        elif task == "ATTACK":
            self.integrity -= 0.05
            self.xp += 4.0

        elif task == "RETREAT":
            self.integrity += 0.03

        elif task == "EVACUATE":
            self.integrity -= 0.04
            self.xp += 6.0

        self.integrity = clamp(self.integrity)

    # --------------------------------------------
    def chaos(self):
        return self.lyapunov + self.kesler

# ============================================
# 🐝 SWARM
# ============================================

class Swarm:
    def __init__(self, swarm_id, roles):
        self.swarm_id = swarm_id
        self.nodes = []

        for i, r in enumerate(roles):
            self.nodes.append(SwarmNode(i, r, swarm_id))

    def tick(self, signal):
        for n in self.nodes:
            n.observe(signal)

    def execute(self):
        for n in self.nodes:
            n.execute()

    def swarm_chaos(self):
        return sum(n.chaos() for n in self.nodes) / len(self.nodes)

    def scars_energy(self):
        return sum(n.xp for n in self.nodes if n.integrity < 0.5)

# ============================================
# 🌐 FEDERATION (GOSSIP)
# ============================================

class Federation:
    def __init__(self):
        self.swarms = []
        self.field_state = {}

    def add(self, swarm):
        self.swarms.append(swarm)

    def gossip(self):
        for s in self.swarms:
            self.field_state[s.swarm_id] = s.swarm_chaos()

    def global_lyapunov(self):
        return sum(self.field_state.values()) / max(1, len(self.field_state))

# ============================================
# 🐯 TIGER CORE
# ============================================

class TigerCore:
    def __init__(self, federation):
        self.fed = federation

    def decide(self):
        global_L = self.fed.global_lyapunov()

        for s in self.fed.swarms:
            for n in s.nodes:
                local = n.chaos()

                # 🔥 Catastrophic
                if local > 1.0:
                    n.task_queue.append("SELF_DESTRUCT")

                # 🫥 Masking
                elif local > 0.7:
                    n.task_queue.append("CAMOUFLAGE")

                # 🧭 Recon
                elif n.role == "SCOUT":
                    n.task_queue.append("RECON")

                # 🧯 Evacuation
                elif n.role in ("EVACUATOR", "NRK_GROUND"):
                    n.task_queue.append("EVACUATE")

                # ⚔ Symbolic attack
                elif n.role.startswith("STRIKER"):
                    n.task_queue.append("ATTACK")

                # ↩ Retreat
                elif global_L > 0.6:
                    n.task_queue.append("RETREAT")

                else:
                    n.task_queue.append("RECON")

# ============================================
# 🖥 RENDER
# ============================================

def render(fed, step):
    print("═" * 72)
    print(f"⏱ STEP {step}")
    for s in fed.swarms:
        print(
            f"🐝 Swarm {s.swarm_id} | "
            f"Chaos:{s.swarm_chaos():.3f} | "
            f"Scars XP:{s.scars_energy():.1f}"
        )

# ============================================
# 🚀 MAIN
# ============================================

if __name__ == "__main__":
    fed = Federation()

    fed.add(Swarm(
        "ALPHA",
        ["COMMANDER", "SCOUT", "STRIKER_NEAR", "DECOY", "HEALER"]
    ))

    fed.add(Swarm(
        "BETA",
        ["SCOUT", "STRIKER_FAR", "NRK_GROUND", "EVACUATOR"]
    ))

    tiger = TigerCore(fed)

    for step in range(40):
        signal = random.random()

        for s in fed.swarms:
            s.tick(signal)

        fed.gossip()
        tiger.decide()

        for s in fed.swarms:
            s.execute()

        render(fed, step)
