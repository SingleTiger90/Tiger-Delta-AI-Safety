use aya::programs::{Xdp, XdpFlags};
use aya::maps::Array;
use aya::{include_bytes_aligned, Bpf};
use std::{
    env,
    thread,
    time::{Duration, Instant},
};
use rand::{Rng, thread_rng};

fn main() -> Result<(), anyhow::Error> {
    let iface = env::args()
        .nth(1)
        .expect("Usage: tiger_loader <INTERFACE>");

    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../src/kernel/tiger_delta_xdp.o"
    ))?;

    let program: &mut Xdp = bpf.program_mut("tiger_delta_xdp")?.try_into()?;
    program.load()?;
    program.attach(&iface, XdpFlags::SKB_MODE)?;

    let mut config_map: Array<_, u64> =
        Array::try_from(bpf.map("config_map")?)?;
    let resonance_map: Array<_, u64> =
        Array::try_from(bpf.map("resonance_state")?)?;
    let mut policy_map: Array<_, u32> =
        Array::try_from(bpf.map("policy_map")?)?;

    let mut rng = thread_rng();
    let mut last_rotation = Instant::now();

    println!("TigerΔ active (time-quantized mode)");

    loop {
        /* --- Dynamic salt rotation (30s) --- */
        if last_rotation.elapsed() >= Duration::from_secs(30) {
            let phi: u64 = rng.gen();
            let pi: u64  = rng.gen();

            config_map.set(0, phi, 0)?;
            config_map.set(1, pi, 0)?;

            last_rotation = Instant::now();
            println!("Salt rotation applied");
        }

        /* --- Resonance monitoring --- */
        if let Ok(res) = resonance_map.get(&0, 0) {
            let attack = res > 0x8000_0000_0000_0000;
            policy_map.set(0, attack as u32, 0)?;

            println!(
                "Entropy: {:016X} | {}",
                res,
                if attack { "DROP" } else { "PASS" }
            );
        }

        thread::sleep(Duration::from_millis(500));
    }
}
