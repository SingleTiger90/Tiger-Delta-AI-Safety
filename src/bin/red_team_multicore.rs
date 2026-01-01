use std::thread;
use std::time::Instant;
use std::f64::consts::PI;

const PHI: f64 = 1.6180339887498948482;
const ITERATIONS: u64 = 10_000_000;

#[inline(always)]
fn fold(state: f64, input: f64) -> f64 {
    let mixed = (state + input * PI).sin() * PHI;
    (state + mixed) * 0.5
}

fn worker(cpu_id: usize, iterations: u64) -> (usize, f64, f64) {
    let mut state: f64 = 0.5;
    let start = Instant::now();

    for i in 0..iterations {
        let phase = (i as f64 * 0.0000001) + cpu_id as f64;
        let input = (phase.sin() + phase.cos()) * 0.5;
        state = fold(state, input);
    }

    let elapsed = start.elapsed().as_secs_f64();
    let mpps = (iterations as f64 / 1_000_000.0) / elapsed;

    (cpu_id, state, mpps)
}

fn main() {
    let cpu_count = num_cpus::get();

    println!("TigerΔ Red Team Stress Test v1.0 [Multi-Core]");
    println!("--------------------------------------------------");
    println!("CPUs: {}", cpu_count);
    println!("Iterations per CPU: {}", ITERATIONS);
    println!();

    let mut handles = Vec::new();

    for cpu in 0..cpu_count {
        handles.push(thread::spawn(move || {
            worker(cpu, ITERATIONS)
        }));
    }

    let mut states = Vec::new();
    let mut total_mpps = 0.0;

    for handle in handles {
        let (cpu, state, mpps) = handle.join().unwrap();
        println!(
            "CPU {:02} | Final State: {:.8} | Throughput: {:.2} Mpps",
            cpu, state, mpps
        );
        states.push(state);
        total_mpps += mpps;
    }

    let mean: f64 = states.iter().sum::<f64>() / states.len() as f64;
    let variance: f64 = states
        .iter()
        .map(|v| (v - mean).powi(2))
        .sum::<f64>() / states.len() as f64;

    println!();
    println!("-------------------- SUMMARY --------------------");
    println!("Mean Final State : {:.8}", mean);
    println!("Variance         : {:.10}", variance);
    println!("Total Throughput : {:.2} Mpps", total_mpps);

    if variance < 1e-6 {
        println!("Verdict: CORE IS STABLE (Per-CPU coherent)");
    } else {
        println!("Verdict: WARNING (Inter-core divergence)");
    }
}
