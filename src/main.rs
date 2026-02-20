// =================================================================
// Project: TigerΔ (Tiger Delta)
// Module: main.rs
// Description: Platinum Core — Asynchronous Nerve Center (Ulenspiegel)
// Framework: Tokio (Async Runtime) / Tracing (Logging)
// =================================================================

mod lumis;
mod string_state;
mod lagrange;
mod atomic_core;
mod simul;

use crate::string_state::{StringState, to_float};
use crate::lagrange::LagrangeEquilibrium;
use crate::lumis::{LumisCore, PHI_INVERSE, PHI};
use crate::atomic_core::AtomicCore;
use crate::simul::SimulUnit;

use tokio::sync::mpsc;
use tokio::net::UdpSocket;
use tracing::{info, warn, error};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    info!("🐯 TigerΔ v3.3 \"Ulenspiegel\" — Platinum Core Online");

    // Channel between interceptor and cognitive core
    let (tx, mut rx) = mpsc::channel::<(Vec<i64>, std::net::SocketAddr)>(1024);

    // UDP socket
    let socket = Arc::new(UdpSocket::bind("0.0.0.0:8888").await?);
    let socket_responder = socket.clone();

    // =============================================================
    // BRAIN THREAD
    // =============================================================
    tokio::spawn(async move {
        let mut lumis = LumisCore::new();
        let mut atomic = AtomicCore::new(100);
        let mut simul = SimulUnit::new();

        let mut defense_mass = 1000.0;
        let mut lagrange = LagrangeEquilibrium::new(defense_mass);
        let mut state = StringState::new();

        while let Some((attrs_vec, addr)) = rx.recv().await {
            // -----------------------------------------------------
            // 1. Feature vector normalization
            // -----------------------------------------------------
            let mut attrs = [0i64; 10];
            for (i, &v) in attrs_vec.iter().take(10).enumerate() {
                attrs[i] = v;
            }

            // -----------------------------------------------------
            // 2. Impact energy (physical)
            // -----------------------------------------------------
            let raw_energy: f64 =
                attrs.iter().map(|&x| x as f64).sum::<f64>() * PHI_INVERSE;

            let impact_energy = (raw_energy / 1_000_000.0).clamp(0.0, 10.0);

            // -----------------------------------------------------
            // 3. Entropy estimation (informational)
            // -----------------------------------------------------
            let entropy_input =
                (attrs[3].abs() as f64 / (attrs[1].max(1) as f64))
                .clamp(0.0, 10.0);

            // -----------------------------------------------------
            // 4. Digital Twin pre-filter
            // -----------------------------------------------------
            if simul.project_impact(impact_energy) {
                warn!("⚠️ PREEMPTIVE BLOCK from {}", addr);
                let _ = socket_responder
                    .send_to(b"DELTA_SHIELD_PREEMPT", addr)
                    .await;
                continue;
            }

            // -----------------------------------------------------
            // 5. Atomic core processing
            // -----------------------------------------------------
            atomic.sharpen_angles(impact_energy);
            let _drift = atomic.find_the_middle(entropy_input);
            let threat_p = atomic.threat_probability(entropy_input);

            // -----------------------------------------------------
            // 6. Lagrange stabilization
            // -----------------------------------------------------
            let compact = state.compactify(&attrs);
            let compact_f = to_float(compact);
            let equilibrium = lagrange.stabilize(compact_f, impact_energy);

            let resonance =
                (1.0 - (equilibrium.unwrap_or(PHI) - PHI).abs() / PHI)
                .clamp(0.0, 1.0);

            // -----------------------------------------------------
            // 7. Lumis life-cycle update
            // -----------------------------------------------------
            lumis.tick_cycle(impact_energy, resonance, &mut defense_mass);
            defense_mass = defense_mass.clamp(100.0, 10_000.0);
            lagrange.update_mass(defense_mass);

            // -----------------------------------------------------
            // 8. Adaptive response logic
            // -----------------------------------------------------
            if atomic.is_critical {
                warn!(
                    "🧬 MUTATION ACTIVE | phase={} | scars={:.3}",
                    atomic.mutation_phase,
                    atomic.scars_energy
                );
            }

            if threat_p > 0.85 || equilibrium.is_none() {
                error!(
                    "🔥 ATTACK | src={} | p={:.2} | phase={} | scars={:.2}",
                    addr,
                    threat_p,
                    atomic.mutation_phase,
                    atomic.scars_energy
                );
                let _ = socket_responder
                    .send_to(b"DELTA_SHIELD_NULL", addr)
                    .await;
            } else {
                let decoy = simul.get_decoy_state() * resonance;
                if decoy > 0.6 {
                    let msg = format!("STATUS_OK_{:.2}", decoy);
                    let _ = socket_responder.send_to(msg.as_bytes(), addr).await;
                }
            }

            // -----------------------------------------------------
            // 9. Rest mode
            // -----------------------------------------------------
            if lumis.is_resting() {
                info!(
                    "🌙 LUMIS REST MODE | entropy={:.4}",
                    lumis.entropy_level()
                );
            }
        }
    });

    // =============================================================
    // UDP INTERCEPTOR
    // =============================================================
    info!("🛰 Paranoia Filter Active — UDP 8888");
    let mut buf = [0u8; 2048];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;

        if len < 8 || len > 1024 {
            continue;
        }

        let data = &buf[..len];
        let weight: i64 = data.iter().map(|&b| b as i64).sum();

        let attrs = vec![
            addr.port() as i64,
            len as i64,
            data[0] as i64,
            weight,
            weight % 111,
            (len as i64) % 7,
            data.iter().take(5).map(|&x| x as i64).sum(),
            addr.ip().to_string().len() as i64,
            0,
            0,
        ];

        if tx.try_send((attrs, addr)).is_err() {
            error!("QUEUE OVERFLOW | Negative Radius | {}", addr);
        }
    }
}
