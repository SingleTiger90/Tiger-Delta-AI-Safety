use tokio::net::UdpSocket;
use std::io::{self, Write};
use std::str;
use std::time::Duration;

// Connect the "Soul" module (Your lumis.rs file)
mod lumis;
use lumis::LumisCore;

#[tokio::main]
async fn main() -> io::Result<()> {
    // --- 1. SETUP (Initialization) ---
    // Listen on port 8888 (Standard testing port)
    let addr = "0.0.0.0:8888";
    let socket = UdpSocket::bind(addr).await?;

    println!("\n==================================================");
    println!("🛡️  TIGER DELTA ENGINE (Rust Core) ACTIVATED");
    println!("📡  Membrane listening on UDP: {}", addr);
    println!("🩸  Biological Protocol: OODA Loop via Phi-Harmonics");
    println!("==================================================\n");

    let mut buf = [0; 1024];
    let mut lumis = LumisCore::new();
    let mut packet_count: u64 = 0;

    // --- 2. THE ETERNAL LOOP (Life Cycle) ---
    loop {
        // A. OBSERVE (The Membrane)
        // Non-blocking async wait for kinetic impact
        let (len, addr) = socket.recv_from(&mut buf).await?;
        packet_count += 1;

        // "Matrix Protocol" Check (Wake Up)
        // If packet is small and contains the keyphrase -> Activate
        if len < 50 {
            if let Ok(payload) = str::from_utf8(&buf[..len]) {
                if payload.trim() == "WAKE_UP_NEO" {
                    println!("\n🐇 MATRIX PROTOCOL INITIATED from {}", addr);
                    println!("   System entering AWAKENED state...\n");
                }
            }
        }

        // B. ORIENT (Math Calculation)
        // Calculate the current "Living Phi" threshold
        let current_phi = lumis.compute_living_phi();
        
        // C. DECIDE (Kinetic Weight Calculation)
        // Simplified model: packet size = kinetic energy
        let packet_energy = len as f64 / 100.0;

        if packet_energy > current_phi {
            // D. ACT (Absorb Entropy)
            // The packet is too "heavy" or off-beat. 
            // We do not block. We absorb the stress.
            lumis.absorb_stress(0.05); 
        } 

        // E. HOMEOSTASIS (Metabolic Cycle)
        // Attempt to cool down every cycle
        lumis.weep_reset();

        // --- F. VISUALIZATION (The Hum) ---
        // Visualizing the system heartbeat (Metabolic State)
        if packet_count % 50 == 0 || lumis.get_entropy() > 0.5 {
            let entropy = lumis.get_entropy();
            
            if entropy < 0.2 {
                // Flow State (Calm)
                 print!("\r[FLOW] |||||||||||||||||| (Entropy: {:.4})   ", entropy);
            } else if entropy < 0.7 {
                // Adaptation (Stress)
                 print!("\r[ADAPT] ~~~zzZZZzz~~~      (Entropy: {:.4})   ", entropy);
            } else {
                // Phase Shift (Critical)
                 print!("\r[WEEP]  !!! PHASE SHIFT !!! (Entropy: {:.4})   ", entropy);
            }
            
            // Force flush to console
            io::stdout().flush().unwrap();
        }
    }
}
