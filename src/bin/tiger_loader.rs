/*
 * TigerΔ Control Plane (Loader)
 * Language: Rust
 * * Manages eBPF lifecycle and implements the Active Shield decision logic.
 */

use aya::programs::{Xdp, XdpFlags};
use aya::maps::Array;
use aya::{include_bytes_aligned, Bpf};
use std::env;
use std::time::Duration;
use std::thread;

fn main() -> Result<(), anyhow::Error> {
    let iface = env::args()
        .nth(1)
        .expect("Usage: tiger_loader <INTERFACE>");

    // Load pre-compiled eBPF object
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../src/kernel/tiger_delta_xdp.o"
    ))?;

    // Attach XDP program to the network interface
    let program: &mut Xdp = bpf
        .program_mut("tiger_delta_xdp")?
        .try_into()?;

    program.load()?;
    program.attach(&iface, XdpFlags::SKB_MODE)?;

    println!("🐅 TigerΔ Resonance Engine Active on: {}", iface);
    println!("🛡️ Mode: Autonomous Active Shield (Monitoring entropy spikes)\n");

    // Access the shared BPF maps
    let resonance_map: Array<_, u64> = Array::try_from(bpf.map("resonance_state")?)?;
    let mut policy_map: Array<_, u32> = Array::try_from(bpf.map("policy_map")?)?;

    let key = 0;
    // Decision threshold: crossing 50% of u64 capacity indicates entropy flood
    let attack_threshold: u64 = 0x8000000000000000;

    loop {
        if let Ok(current_resonance) = resonance_map.get(&key, 0) {
            let is_attack = current_resonance > attack_threshold;
            
            // Trigger Active Shield in the Kernel
            let mode: u32 = if is_attack { 1 } else { 0 };
            policy_map.set(&key, mode, 0)?;

            // Console Visualization
            let intensity = (current_resonance % 40) as usize;
            let bar = "█".repeat(intensity);
            let status = if is_attack { "🔥 BLOCKING" } else { "🟢 STABLE" };

            println!(
                "[{}] Entropy: {:016X} | Resonance: {}",
                status, current_resonance, bar
            );
        }

        thread::sleep(Duration::from_millis(150));
    }
}
