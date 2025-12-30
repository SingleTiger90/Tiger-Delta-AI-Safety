// src/main.rs

mod lumis;
mod string_state;
mod lagrange;

use crate::string_state::{StringState, to_float};
use crate::lagrange::LagrangeEquilibrium;
use crate::lumis::{LumisCore, PHI_INVERSE};

use tokio::sync::mpsc;
use tokio::net::UdpSocket;
use tracing::{info, warn};

use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    info!("Delta Tiger v1.0 \"Ulenspiegel\" — System Online");

    let (tx, mut rx) = mpsc::channel::<(Vec<i64>, std::net::SocketAddr)>(1024);

    let socket = Arc::new(UdpSocket::bind("0.0.0.0:8888").await?);
    let socket_responder = socket.clone();

    tokio::spawn(async move {
        let mut lumis = LumisCore::new();
        let mut defense_mass = 1000.0;
        let mut lagrange = LagrangeEquilibrium::new(defense_mass);
        let mut state = StringState::new();

        while let Some((attrs_vec, addr)) = rx.recv().await {
            let attrs: [i64; 10] = {
                let mut arr = [0i64; 10];
                for (i, &val) in attrs_vec.iter().take(10).enumerate() {
                    arr[i] = val;
                }
                arr
            };

            let attack_energy: f64 = attrs.iter().map(|&x| x as f64).sum::<f64>() * PHI_INVERSE / 1_000_000.0;

            let compact = state.compactify(&attrs);
            let compact_float = to_float(compact);

            let result = lagrange.stabilize(compact_float, attack_energy);

            let resonance = (1.0 - (result.unwrap_or(PHI) - PHI).abs() / PHI).clamp(0.0, 1.0);
            info!("Resonance level: {:.4}", resonance);

            lumis.tick_cycle(attack_energy, resonance, &mut defense_mass);
            lagrange = LagrangeEquilibrium::new(defense_mass);

            match result {
                None => {
                    warn!("Annihilation from {} (resonance {:.4})", addr, resonance);
                    let _ = socket_responder.send_to(b"DELTA_SHIELD_NULL", addr).await;
                }
                Some(_) => {}
            }

            if lumis.is_resting() {
                info!("LUMIS REST MODE — deep recovery activated");
            }
        }
    });

    info!("Paranoia Filter Active — Listening on UDP 8888");
    let mut buf = [0u8; 2048];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;

        if len < 8 || len > 512 {
            continue;
        }

        let packet_data = &buf[..len];
        let weight: i64 = packet_data.iter().map(|&b| b as i64).sum();

        let attrs: Vec<i64> = vec![
            addr.ip().to_string().len() as i64,
            addr.port() as i64,
            len as i64,
            packet_data[0] as i64,
            weight,
            (len as i64 % 7),
            0, 0, 0, 0,
        ];

        if let Err(_) = tx.try_send((attrs, addr)) {}
    }
}
