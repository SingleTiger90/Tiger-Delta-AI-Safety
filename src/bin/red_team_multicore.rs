use std::thread;
use std::time::{Duration, Instant};
use std::sync::Arc;
use std::f64::consts::{PI};

const PHI: f64 = 1.6180339887498948482;
const ITERATIONS: u64 = 10_000_000;

#[inline(always)]
fn fold(state: f64, input: f64) -> f64 {
    let mixed = (state + input * PI).sin() * PHI;
    (state + mixed) * 0.5
}

fn worker(thread_id: usize, iterations: u64) -> (usize, f64, f64) {
    let mut state: f64 = 0.5;
    let start = Instant::now();

    for i in 0..iterations {
        // Adversarial coordinated pattern with slight phase drift
        let phase = (i as f64 * 0.0000001) + (thread_id as f64);
        let input = (phase.cos() + phase.sin()) * 0.5;
        state = fold(state, input);
    }

    let elapsed = start.elapsed().as_secs_f64();
    let mpps = (iterations as f64 / 1_000_000.0) / elapsed;

    (thread_id, state, mpps)
}

fn main() {
    let cpu_count = num_cpus::get();
    let iterations_per_thread = ITERATIONS;

    println!("TigerΔ Red Team Stress Test v1.0 [Multi-Core]");
    println!("--------------------------------------------------");
    println!("CPUs: {}", cpu_count);
    println!("Iterations per CPU: {}", iterations_per_thread);
    println!();

    let mut handles = Vec::with_capacity(cpu_count);
    let start_all = Instant::now();

    for id in 0..cpu_count {
        handles.push(thread::spawn(move || {
            worker(id, iterations_per_thread)
        }));
    }

    let mut states = Vec::new();
    let mut throughputs = Vec::new();

    for h in handles {
        let (id, state, mpps) = h.join().expect("Thread panicked");
        println!(
            "CPU {:02} | Final State: {:.8} | Throughput: {:.2} Mpps",
            id, state, mpps
        );
        states.push(state);
        throughputs.push(mpps);
    }

    let total_time = start_all.elapsed().as_secs_f64();

    let mean_state: f64 = states.iter().sum::<f64>() / states.len() as f64;
    let variance: f64 = states
        .iter()
        .map(|s| (s - mean_state).powi(2))
        .sum::<f64>() / states.len() as f64;

    let total_mpps: f64 = throughputs.iter().sum();

    println!();
    println!("-------------------- SUMMARY --------------------");
    println!("Mean Final State : {:.8}", mean_state);
    println!("Variance         : {:.10}", variance);
    println!("Total Throughput : {:.2} Mpps", total_mpps);
    println!("Wall Time        : {:.2} s", total_time);

    if variance < 1e-6 {
        println!("Verdict: CORE IS STABLE (Per-CPU coherent)");
    } else {
        println!("Verdict: WARNING (Inter-core divergence detected)");
    }
}
