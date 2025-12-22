/*
 * PROJECT: TIGER-DELTA (TΔ)
 * MODULE: SOVEREIGN_GUARDIAN_CORE
 * VERSION: 1.0.0 (Global Release)
 * COMPLIANCE: NATO-ACD / EU-Sovereignty / NIST Resilience
 * LICENSE: TΔ-S (Sovereign) - See LICENSE.md for terms.
 * * DESCRIPTION: Active Cyber Defense system utilizing harmonic resonance 
 * and kinetic energy reflection to ensure signal integrity and protection.
 * Based on the legacy of the Father's Original Apparatus.
 */

use tokio::net::UdpSocket;
use tokio::time::{sleep, Duration, Instant};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::collections::HashMap;
use std::sync::Mutex;
use rand::Rng;

// --- HARMONIC CONSTANTS (The Core) ---
const PHI: f64 = 1.61803398875;
const PI: f64 = 3.14159265359;

// --- GLOBAL STATE CONTROL ---
// 1: STABLE (Normal Resonance)
// 2: SHADOW_MODE (Active Interference/Defense)
static SYSTEM_STATE: AtomicUsize = AtomicUsize::new(1);

/// LUMIS (Luminous Universal Mutual Interdependence Shield)
/// Evaluates temporal and frequency coherence based on PHI-sinusoidal drift.
fn lumis_coherence_check(freq: f64, delta_t: f64) -> f64 {
    let expected = PHI * (freq / PI).sin().abs();
    let drift = (delta_t - expected).abs();
    
    // Industrial standard drift tolerance: < 0.08s (80ms)
    if drift < 0.08 { 1.0 } else { (1.0 - (drift * 1.5)).max(0.0) }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = Arc::new(UdpSocket::bind("0.0.0.0:8888").await?);
    
    // Tracking maps for synchronization and threat intelligence
    let sync_map = Arc::new(Mutex::new(HashMap::<std::net::SocketAddr, Instant>::new()));
    let threat_map = Arc::new(Mutex::new(HashMap::<std::net::SocketAddr, Vec<String>>::new()));

    println!("--- TIGER-DELTA CORE v1.0 ONLINE ---");
    println!("[INFO] LUMIS Monitoring: ENGAGED");
    println!("[INFO] Kinetic Membrane: READY (Sovereign Status)");

    let mut buf = [0u8; 4096];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let now = Instant::now();
        let payload = String::from_utf8_lossy(&buf[..len]).to_string();
        let parts: Vec<&str> = payload.split('|').collect();

        // 1. AUTHORIZED SIGNAL VALIDATION (The Beacon Sync)
        if parts.len() >= 2 && parts[0] == "PHI_PI_NOTE" {
            let freq: f64 = parts[1].parse().unwrap_or(0.0);
            
            let mut s_map = sync_map.lock().unwrap();
            let delta_t = s_map.get(&addr).map(|t| now.duration_since(*t).as_secs_f64()).unwrap_or(0.0);
            s_map.insert(addr, now);

            let resonance = lumis_coherence_check(freq, delta_t);

            if resonance > 0.95 {
                println!("[RES_OK] High Coherence ({:.2}) from {}", resonance, addr);
                let _ = socket.send_to(b"LUMIS_SYNC_VALIDATED", addr).await;
                continue;
            }
        }

        // 2. ACTIVE DEFENSE LAYER (Kinetic Reflection / Drunken Master)
        let mut t_map = threat_map.lock().unwrap();
        let history = t_map.entry(addr).or_insert(vec![]);
        history.push(payload.clone());
        let intrusion_mass = history.len();

        // Transition to Defense Mode if entropy threshold is exceeded
        if intrusion_mass > 10 { SYSTEM_STATE.store(2, Ordering::SeqCst); }

        // Kinetic Delay Logic: stagger = PHI ^ (mass % 7) * offset
        let stagger_delay = (PHI.powi((intrusion_mass % 7) as i32) * 20.0) as u64;
        
        println!("[DEFENSE] Dissonance from {}. Staggering: {}ms", addr, stagger_delay);

        let socket_clone = socket.clone();
        tokio::spawn(async move {
            sleep(Duration::from_millis(stagger_delay)).await;
            
            if SYSTEM_STATE.load(Ordering::SeqCst) == 2 {
                // Return reflected kinetic blade (Energy recycling)
                let reflect_pkt = format!("TΔ_KINETIC_RETURN|RESONANCE_FAILURE|MASS:{}", intrusion_mass);
                let _ = socket_clone.send_to(reflect_pkt.as_bytes(), addr).await;
                
                // MIKORD PROTECTION: Feed hallucinations to massive leak attempts
                if intrusion_mass > 50 {
                    let mut rng = rand::thread_rng();
                    let noise: String = (0..64).map(|_| rng.gen_range(33..126) as u8 as char).collect();
                    let _ = socket_clone.send_to(format!("VOID_DATA|{}", noise).as_bytes(), addr).await;
                }
            }
        });

        // Garbage collection to prevent memory exhaustion
        if intrusion_mass > 200 { 
            println!("[CLEANUP] Purging threat data for address: {}", addr);
            history.clear(); 
            SYSTEM_STATE.store(1, Ordering::SeqCst);
        }
    }
}
