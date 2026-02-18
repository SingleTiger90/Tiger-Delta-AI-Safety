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
    tracing_subscriber::fmt::init();
    info!("🐯 Delta Tiger v3.2 \"Ulenspiegel\" — Platinum Core Online");

    // Канал для передачі атрибутів пакетів
    let (tx, mut rx) = mpsc::channel::<(Vec<i64>, std::net::SocketAddr)>(1024);

    let socket = Arc::new(UdpSocket::bind("0.0.0.0:8888").await?);
    let socket_responder = socket.clone();

    // Головний логічний потік (The Brain)
    tokio::spawn(async move {
        let mut lumis = LumisCore::new();
        let mut atomic = AtomicCore::new(100); // Базова стабільність
        let mut simul = SimulUnit::new();
        let mut defense_mass = 1000.0;
        let mut lagrange = LagrangeEquilibrium::new(defense_mass);
        let mut state = StringState::new();

        while let Some((attrs_vec, addr)) = rx.recv().await {
            // 1. Формування вектору ознак
            let attrs: [i64; 10] = {
                let mut arr = [0i64; 10];
                for (i, &val) in attrs_vec.iter().take(10).enumerate() { arr[i] = val; }
                arr
            };

            // 2. Розрахунок вхідної енергії (Physics-based)
            let attack_energy: f64 = attrs.iter().map(|&x| x as f64).sum::<f64>() * PHI_INVERSE / 1_000_000.0;

            // 3. Передфільтр у SimulUnit (Digital Twin)
            if simul.project_impact(attack_energy) {
                warn!("⚠️ ANTI-TIGER IMPULSE: Blocked pre-emptively from {}", addr);
                let _ = socket_responder.send_to(b"DELTA_SHIELD_OVERLOAD", addr).await;
                continue; 
            }

            // 4. Квантова обробка в AtomicCore
            atomic.sharpen_angles(attack_energy);
            let drift = atomic.find_the_middle(attack_energy);
            let threat_p = atomic.threat_probability(attack_energy);

            // 5. Стабілізація Лагранжа
            let compact = state.compactify(&attrs);
            let compact_float = to_float(compact);
            let result = lagrange.stabilize(compact_float, attack_energy);

            // 6. Розрахунок резонансу
            let resonance = (1.0 - (result.unwrap_or(PHI) - PHI).abs() / PHI).clamp(0.0, 1.0);
            
            // 7. Життєвий цикл Lumis
            lumis.tick_cycle(attack_energy, resonance, &mut defense_mass);
            lagrange.update_mass(defense_mass); // Оновлюємо масу без перестворення об'єкта

            // 8. Реакція системи
            if threat_p > 0.85 || result.is_none() {
                warn!("🔥 ANNIHILATION: {} | Prob: {:.2} | Res: {:.4}", addr, threat_p, resonance);
                let _ = socket_responder.send_to(b"DELTA_SHIELD_NULL", addr).await;
            } else {
                // Відправляємо децептивний стан (Decoy), щоб заплутати атакуючого
                let decoy = simul.get_decoy_state();
                if decoy > 0.7 {
                    let _ = socket_responder.send_to(format!("STATUS_OK_{:.2}", decoy).as_bytes(), addr).await;
                }
            }

            if lumis.is_resting() {
                info!("🌙 LUMIS REST MODE — entropy: {:.4}", lumis.entropy_level());
            }
        }
    });

    info!("Paranoia Filter Active — Listening on UDP 8888");
    let mut buf = [0u8; 2048];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;

        // Валідація розміру (RFC захист)
        if len < 8 || len > 1024 { continue; }

        let packet_data = &buf[..len];
        let weight: i64 = packet_data.iter().map(|&b| b as i64).sum();

        // Формуємо 10 атрибутів для StringState
        let attrs = vec![
            addr.port() as i64,
            len as i64,
            packet_data[0] as i64,
            weight,
            (weight % 111), // 111-а група в дії :)
            (len as i64 % 7),
            packet_data.iter().take(5).map(|&x| x as i64).sum(),
            (addr.ip().to_string().len() as i64),
            0, 0,
        ];

        // Відправка в чергу (Negative Radius Core у нас реалізований через обмеження каналу mpsc)
        if let Err(_) = tx.try_send((attrs, addr)) {
            // Тут і відбувається переповнення черги — "Negative Radius"
            error!("QUEUE OVERFLOW: Packet dropped from {}", addr);
        }
    }
}
