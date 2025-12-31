use aya::programs::{Xdp, XdpFlags};
use aya::maps::Array;
use aya::{include_bytes_aligned, Bpf};
use std::env;
use std::time::Duration;
use std::thread;

fn main() -> Result<(), anyhow::Error> {
    // ---------------------------------
    // Interface selection
    // ---------------------------------
    let iface = env::args()
        .nth(1)
        .expect("Usage: tiger_loader <INTERFACE>");

    // ---------------------------------
    // Load eBPF bytecode
    // ---------------------------------
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../kernel/tiger_delta_xdp.o"
    ))?;

    // ---------------------------------
    // Load & attach XDP program
    // ---------------------------------
    let program: &mut Xdp = bpf
        .program_mut("tiger_delta_xdp")?
        .try_into()?;

    program.load()?;
    program.attach(&iface, XdpFlags::SKB_MODE)?;

    println!("🐅 TigerΔ Resonance Core attached to {}", iface);
    println!("📡 Monitoring resonance state (Ctrl+C to exit)\n");

    // ---------------------------------
    // Access resonance state map
    // ---------------------------------
    let resonance_map: Array<_, u64> =
        Array::try_from(bpf.map("resonance_state")?)?;

    // ---------------------------------
    // Telemetry loop
    // ---------------------------------
    loop {
        let key = 0;
        if let Ok(val) = resonance_map.get(&key, 0) {
            let intensity = (val % 40) as usize;
            let bar = "█".repeat(intensity);
            println!(
                "Entropy: {:016X} | Resonance [{}]",
                val, bar
            );
        }

        thread::sleep(Duration::from_millis(200));
    }
}
