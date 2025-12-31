import numpy as np
import matplotlib.pyplot as plt

PHI = (1 + np.sqrt(5)) / 2  # Золотий перетин ≈ 1.618

def generate_pulse(cycle):
    # Ірраціональний дрейф на основі Φ і π — аперіодичний резонанс
    seed = np.pi * (PHI ** (cycle % 12))
    return seed - np.floor(seed)

def run_simulation(steps=500):
    cycles = np.arange(steps)
    
    # Векторизована версія для швидкості
    vectorized_pulse = np.vectorize(generate_pulse)
    pulses = vectorized_pulse(cycles)
    
    threshold = 1 / PHI  # Поріг "Exhale" ≈ 0.618

    plt.figure(figsize=(12, 6), facecolor='black')
    ax = plt.gca()
    ax.set_facecolor('black')

    # Пульс резонансу — неоновий синій
    plt.plot(cycles, pulses, color='#00f2ff', linewidth=2, label='Resonance Pulse (Math Core)')

    # Поріг Exhale — червоний
    plt.axhline(y=threshold, color='#ff0066', linestyle='--', linewidth=2, label='Exhale Threshold (Defense)')

    # Зона нейтралізації тиску — напівпрозорий червоний
    plt.fill_between(cycles, pulses, threshold, where=(pulses > threshold),
                     color='#ff3366', alpha=0.4, label='Pressure Neutralized (Exhale Active)')

    # Сітка і оформлення
    plt.grid(True, color='gray', alpha=0.3, linestyle=':')
    plt.title("TigerΔ Phase 2: Aperiodic Resonance Visualization", color='white', fontsize=16, pad=20)
    plt.xlabel("Cycle Tick", color='white')
    plt.ylabel("Normalized Resonance", color='white')
    plt.legend(loc='upper right', frameon=False, fontsize=12, labelcolor='white')

    # Колір міток
    ax.tick_params(colors='white')
    ax.spines['bottom'].set_color('white')
    ax.spines['left'].set_color('white')

    plt.tight_layout()
    plt.show()

if __name__ == "__main__":
    run_simulation(steps=800)  # Більше кроків — красивіше видно аперіодичність
