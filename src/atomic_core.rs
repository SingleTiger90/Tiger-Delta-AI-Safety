// =================================================================
// Project: TigerΔ (Tiger Delta)
// Module: atomic_core.rs
// Description: Core resonance engine with Quantum Hysteresis and FFI.
// Version: 3.3 Gold/Platinum (Stabilized, EN)
// =================================================================

use std::f64::consts::{PI, TAU};

/// Irrational constants for aperiodic drift (Fractal Resilience).
const PHI: f64 = 1.6180339887498948;
const EQUILIBRIUM: f64 = PHI / PI;
const SEPTIMAL_SHIFT: f64 = 1.777;

/// Lower energy bound (anti-collapse safeguard).
const MIN_VALENCE: f64 = 0.001;

#[repr(C)]
pub struct AtomicCore {
    /// Proton count — stability threshold (core mass).
    pub proton_count: u32,

    /// Electron cloud (Ψ²) — probabilistic zone of legitimate signal presence.
    pub electron_cloud: f64,

    /// Valence energy — energy available for processing.
    pub valence_energy: f64,

    /// Scars Energy — accumulated experience gained through stress.
    pub scars_energy: f64,

    /// Mutation phase (Septachord logic: 0..6).
    pub mutation_phase: u8,

    /// Critical state flag (used by TigerCore / OODA loop).
    pub is_critical: bool,
}

impl AtomicCore {
    pub fn new(stability: u32) -> Self {
        Self {
            proton_count: stability.max(1),
            electron_cloud: 0.5,
            valence_energy: 1.0,
            scars_energy: 0.0,
            mutation_phase: 0,
            is_critical: false,
        }
    }

    // --------------------------------------------------------------
    // Sharpen Angles
    // Converts linear external impact (e.g. interference)
    // into angular momentum (nonlinear compression).
    // --------------------------------------------------------------
    pub fn sharpen_angles(&mut self, impact: f64) {
        let abs_impact = impact.abs();

        let mutation_multiplier = 1.0 + (self.mutation_phase as f64 * 0.1);
        let resistance = (self.proton_count as f64 * PHI * mutation_multiplier).max(1.0);

        // Arctangent compresses any energy into a safe nonlinear range
        let angle = (abs_impact / resistance).atan();

        // Normalized electron cloud (bounded probability field)
        self.electron_cloud =
            ((self.electron_cloud + angle).cos().abs()).min(1.0);

        // Accumulate usable energy
        self.valence_energy += angle * 0.1;

        // Strong impact generates scars energy
        if abs_impact > resistance {
            self.scars_energy += angle * 0.5;
        }
    }

    // --------------------------------------------------------------
    // Find the Middle (AADA / Golden Ratio Logic)
    // Returns system drift from equilibrium.
    // --------------------------------------------------------------
    pub fn find_the_middle(&mut self, input_entropy: f64) -> f64 {
        if self.valence_energy < MIN_VALENCE {
            self.valence_energy = MIN_VALENCE;
        }

        let ratio = input_entropy / self.valence_energy;
        let drift = ratio - EQUILIBRIUM;

        if drift.abs() > 0.08 {
            self.is_critical = true;

            // Adaptive contraction / amplification
            self.valence_energy *= 1.0 - (drift.signum() * 0.02);

            // Mutation trigger under extreme chaos
            if drift.abs() > 0.15 {
                self.trigger_mutation();
            }
        } else {
            // Harmonic relaxation
            self.valence_energy = self.valence_energy * 0.95 + 0.05;
            self.is_critical = false;
        }

        // Hard floor protection
        if self.valence_energy < MIN_VALENCE {
            self.valence_energy = MIN_VALENCE;
        }

        drift
    }

    // --------------------------------------------------------------
    // Threat Probability (Wave Function Collapse)
    // Returns probability in range [0, 1].
    // --------------------------------------------------------------
    pub fn threat_probability(&self, input_entropy: f64) -> f64 {
        if self.valence_energy <= 0.0 {
            return 1.0;
        }

        let drift = (input_entropy / self.valence_energy) - EQUILIBRIUM;
        (drift * PI * SEPTIMAL_SHIFT).sin().powi(2)
    }

    // --------------------------------------------------------------
    // Septachord Mutation Logic
    // --------------------------------------------------------------
    fn trigger_mutation(&mut self) {
        self.mutation_phase = (self.mutation_phase + 1) % 7;
        self.scars_energy *= 0.9;
        self.valence_energy += 0.2;
    }
}

// =================================================================
// C-FFI BINDINGS — Bridges for Python / TigerCore
// =================================================================

#[no_mangle]
pub extern "C" fn core_create(stability: u32) -> *mut AtomicCore {
    Box::into_raw(Box::new(AtomicCore::new(stability)))
}

#[no_mangle]
pub extern "C" fn core_process_impact(
    core_ptr: *mut AtomicCore,
    impact: f64,
    entropy: f64,
) -> f64 {
    if core_ptr.is_null() {
        return 0.0;
    }

    let core = unsafe { &mut *core_ptr };
    core.sharpen_angles(impact);
    core.find_the_middle(entropy);
    core.threat_probability(entropy)
}

#[no_mangle]
pub extern "C" fn core_is_critical(core_ptr: *const AtomicCore) -> bool {
    if core_ptr.is_null() {
        return false;
    }
    unsafe { (*core_ptr).is_critical }
}

#[no_mangle]
pub extern "C" fn core_get_scars_energy(core_ptr: *const AtomicCore) -> f64 {
    if core_ptr.is_null() {
        return 0.0;
    }
    unsafe { (*core_ptr).scars_energy }
}

#[no_mangle]
pub extern "C" fn core_get_mutation_phase(core_ptr: *const AtomicCore) -> u8 {
    if core_ptr.is_null() {
        return 0;
    }
    unsafe { (*core_ptr).mutation_phase }
}

/// NEW: Electron cloud getter (signal legitimacy / masking field)
#[no_mangle]
pub extern "C" fn core_get_electron_cloud(core_ptr: *const AtomicCore) -> f64 {
    if core_ptr.is_null() {
        return 0.0;
    }
    unsafe { (*core_ptr).electron_cloud }
}

#[no_mangle]
pub extern "C" fn core_destroy(core_ptr: *mut AtomicCore) {
    if !core_ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(core_ptr);
        }
    }
}
