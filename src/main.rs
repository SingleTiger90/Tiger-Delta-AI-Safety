// src/main.rs

use std::io::{self, BufRead};

mod lumis;
mod simul;
mod string_state;

use lumis::LumisCore;
use simul::SimulUnit;
use string_state::StringState;

fn main() {
    // Initialize core components
    let mut lumis = LumisCore::new();
    let mut simul = SimulUnit::new();
    let mut state = StringState::new();

    println!("Lumis Defense System Initialized");
    println!("-------------------------------");

    // Example normalized attributes (simulating packet data: IP, port, etc. as fixed-point)
    let attrs: [i64; 10] = [
        ((0.1 * string_state::FIXED_SCALE as f64) as i64),
        ((0.23 * string_state::FIXED_SCALE as f64) as i64),
        ((0.45 * string_state::FIXED_SCALE as f64) as i64),
        ((0.67 * string_state::FIXED_SCALE as f64) as i64),
        ((0.89 * string_state::FIXED_SCALE as f64) as i64),
        ((0.12 * string_state::FIXED_SCALE as f64) as i64),
        ((0.34 * string_state::FIXED_SCALE as f64) as i64),
        ((0.56 * string_state::FIXED_SCALE as f64) as i64),
        ((0.78 * string_state::FIXED_SCALE as f64) as i64),
        ((0.90 * string_state::FIXED_SCALE as f64) as i64),
    ];

    // Simulate incoming threat (e.g., from network packet)
    let threat_impact = 0.75; // Normalized threat force (e.g., from anomaly detection)

    // Step 1: Simulate projection in SimulUnit
    if simul.project_impact(threat_impact) {
        println!("SIMUL ALERT: Potential overload detected. Accumulating entropy in LumisCore.");

        // Step 2: Accumulate entropy in LumisCore
        lumis.accumulate_entropy(threat_impact);

        // Step 3: Compute dynamic threshold
        let threshold = lumis.dynamic_threshold();
        println!("Dynamic Threshold: {:.4}", threshold);

        // Step 4: Compactify data in StringState (fixed-point folding)
        let compact = state.compactify(&attrs);
        let compact_float = compact as f64 / string_state::FIXED_SCALE as f64;
        println!("Compact Scalar: {:.10}", compact_float);

        // Decision logic example: If compact > threshold, trigger defense
        if compact_float > threshold {
            println!("DEFENSE TRIGGERED: Generating deception response.");
            // Here: Generate noise, tarpit, or redirect
        } else {
            println!("Traffic accepted under current threshold.");
        }

        // Step 5: Dissipate entropy for homeostasis
        lumis.dissipate_entropy();
        println!("Post-dissipation Entropy: {:.4}", lumis.entropy_level());
    } else {
        println!("No alert from simulation. System stable.");
    }

    // Optional: Interactive mode for testing (simulate multiple impacts)
    println!("\nEnter 'impact <value>' to simulate threats, or 'exit' to quit.");
    let stdin = io::stdin();
    for line in stdin.lines() {
        match line {
            Ok(input) => {
                let parts: Vec<&str> = input.trim().split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }
                match parts[0] {
                    "impact" => {
                        if parts.len() > 1 {
                            if let Ok(impact) = parts[1].parse::<f64>() {
                                simul.project_impact(impact);
                                lumis.accumulate_entropy(impact);
                                let threshold = lumis.dynamic_threshold();
                                let compact = state.compactify(&attrs);
                                let compact_float = compact as f64 / string_state::FIXED_SCALE as f64;
                                println!("Impact: {}, Threshold: {:.4}, Compact: {:.10}", impact, threshold, compact_float);
                                lumis.dissipate_entropy();
                            } else {
                                println!("Invalid impact value.");
                            }
                        }
                    }
                    "exit" => break,
                    _ => println!("Unknown command."),
                }
            }
            Err(_) => break,
        }
    }
}
