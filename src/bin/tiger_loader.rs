use aya::programs::{Xdp, XdpFlags};
use aya::maps::{Array, PerCpuArray}; // Додано PerCpuArray
use aya::{include_bytes_aligned, Bpf};
use std::{env, thread, time::{Duration, Instant}};
use rand::{Rng, thread_rng};

fn main() -> Result<(), anyhow::Error> {
    let iface = env::args().nth(1).expect("Usage: tiger_loader <INTERFACE>");

    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../src/kernel/tiger_delta_xdp.o"
    ))?;

    let program: &mut Xdp = bpf.program_mut("tiger_delta_xdp")?.try_into()?;
    program.load()?;
    program.attach(&iface, XdpFlags::SKB_MODE)?;

    let mut config_map: Array<_, u64> = Array::try_from(bpf.map("config_map")?)?;
    let mut policy_map: Array<_, u32> = Array::try_from(bpf.map("policy_map")?)?;
    
    // Отримуємо Per-CPU мапу
    let resonance_map: PerCpuArray<_, u64> = PerCpuArray::try_from(bpf.map("resonance_state")?)?;

    let mut rng = thread_rng();
    let mut last_rotation = Instant::now();

    println!("🐅 TigerΔ v1.0 Ulenspiegel: Per-CPU High-Performance Mode");

    loop {
        // --- Salt Rotation (кожні 30 сек) ---
        if last_rotation.elapsed() >= Duration::from_secs(30) {
            config_map.set(0, rng.gen::<u64>(), 0)?;
            config_map.set(1, rng.gen::<u64>(), 0)?;
            last_rotation = Instant::now();
            println!("🔄 Dynamic Manifold Shifted");
        }

        // --- Агрегація ентропії з усіх ядер ---
        let mut global_entropy = 0u64;
        let mut cpu_count = 0;

        // resonance_map.get(&0, 0) повертає PerCpuValues<u64>
        if let Ok(values) = resonance_map.get(&0, 0) {
            for cpu_val in values.iter() {
                global_entropy = global_entropy.wrapping_add(*cpu_val);
                cpu_count += 1;
            }
        }

        let avg_entropy = if cpu_count > 0 { global_entropy / cpu_count as u64 } else { 0 };
        let is_attack = avg_entropy > 0x8000_0000_0000_0000;
        
        policy_map.set(0, if is_attack { 1 } else { 0 }, 0)?;

        println!(
            "Entropy (Global Avg): {:016X} | Status: {}", 
            avg_entropy, 
            if is_attack { "🔥 BLOCKING" } else { "🟢 STABLE" }
        );

        thread::sleep(Duration::from_millis(500));
    }
}
