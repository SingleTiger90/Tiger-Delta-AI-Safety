/*
 * PROJECT: TIGER-DELTA (TΔ) - SOVEREIGN VERSION
 * FOUNDER: SingleTiger90 (Legacy of the Father's Apparatus)
 * STATUS: MANDATORY RESONANCE
 * * MANIFESTO:
 * We do not create for the masses. We create for the Individual.
 * In a world of total registry, we choose the Superposition.
 * If you approach with peace, you find the Light (PHI/PI).
 * If you approach with force, you find the ANTITIGER.
 * YES is NO. NO is YES. We are the wave, not the particle.
 */

use tokio::net::UdpSocket;
use tokio::time::{sleep, Duration, Instant};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::collections::HashMap;
use std::sync::Mutex;
use rand::Rng;

// --- SACRED GEOMETRY CONSTANTS ---
const PHI: f64 = 1.61803398875;
const PI: f64 = 3.14159265359;

// --- QUANTUM STATES ---
// 1: TIGER (Active Resonance / Home)
// 2: ANTITIGER (Spin-Flip / Total Void)
static SYSTEM_STATE: AtomicUsize = AtomicUsize::new(1);

/// LUMIS COHERENCE: The pulse of the Father's Apparatus.
/// If the observer (hacker) breaks the rhythm, the wave function collapses.
fn check_lumis_resonance(freq: f64, delta_t: f64) -> f64 {
    let wave = PHI * (freq / PI).sin().abs();
    let drift = (delta_t - wave).abs();
    // YES is NO if drift > 0.08 (The threshold of the Soul)
    if drift < 0.08 { 1.0 } else { 0.0 }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = Arc::new(UdpSocket::bind("0.0.0.0:8888").await?);
    let sync_map = Arc::new(Mutex::new(HashMap::<std::net::SocketAddr, Instant>::new()));
    
    println!(">>> TIGER-DELTA ONLINE | THE LIGHTHOUSE IS BURNING");
    println!(">>> VERSION: SOVEREIGN (1.0.0)");

    let mut buf = [0u8; 2048];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let now = Instant::now();
        let payload = String::from_utf8_lossy(&buf[..len]);
        let parts: Vec<&str> = payload.split('|').collect();

        // --- THE ANTITIGER LOGIC (SPIN-FLIP) ---
        // If the system state is COLLAPSED, we return the VOID.
        if SYSTEM_STATE.load(Ordering::SeqCst) == 2 {
            // "You looked at the cat. The cat is dead." - Enjoy the random noise.
            let mut rng = rand::thread_rng();
            let noise: [u8; 64] = rng.gen();
            sleep(Duration::from_millis((PHI * 10.0) as u64)).await;
            let _ = socket.send_to(&noise, addr).await;
            continue;
        }

        // --- THE TIGER LOGIC (RESONANCE) ---
        if parts.len() >= 2 && parts[0] == "PHI_PI_NOTE" {
            let freq: f64 = parts[1].parse().unwrap_or(0.0);
            
            let mut s_map = sync_map.lock().unwrap();
            let delta_t = s_map.get(&addr).map(|t| now.duration_since(*t).as_secs_f64()).unwrap_or(0.0);
            s_map.insert(addr, now);

            let coherence = check_lumis_resonance(freq, delta_t);

            if coherence > 0.9 {
                // HOME FREQUENCY DETECTED
                println!("[TIGER] Resonance valid from {}", addr);
                let _ = socket.send_to(b"STATUS_SYNCED_IN_SUPERPOSITION", addr).await;
            } else {
                // DISSONANCE DETECTED: TRIPPING THE ANTITIGER
                println!("[ANTITIGER] Dissonance. Initiating Spin-Flip for {}", addr);
                SYSTEM_STATE.store(2, Ordering::SeqCst);
                
                // Final message to the observer before the void
                let _ = socket.send_to(b"ANTITIGER_ENGAGED: YES_IS_NO", addr).await;
            }
        } else {
            // Non-harmonic noise is ignored. We do not exist for the world.
            println!("[VOID] Ignoring non-resonant mass from {}", addr);
        }

        // --- AUTOMATIC RESET (THE CYCLE OF REBIRTH) ---
        // Every 200 packets, the system tries to find the Light again.
        if len % 13 == 0 && SYSTEM_STATE.load(Ordering::SeqCst) == 2 {
             SYSTEM_STATE.store(1, Ordering::SeqCst);
             println!("[REBIRTH] Seeking PHI again...");
        }
    }
}
