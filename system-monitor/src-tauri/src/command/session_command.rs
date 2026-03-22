use tauri::{command, State};
use std::sync::{Arc, Mutex};
use crate::sensor::session_tracker::{SessionTracker, SessionResults};

#[command]
pub fn start_session(state: State<'_, Arc<Mutex<SessionTracker>>>) {
    let mut tracker = state.lock().unwrap();
    tracker.start(None); // Per ora nessun processo specifico, lo aggiungeremo in futuro!
    println!("🟢 Sessione di Benchmark Avviata!");
}

#[command]
pub fn stop_session(state: State<'_, Arc<Mutex<SessionTracker>>>) -> SessionResults {
    let mut tracker = state.lock().unwrap();
    println!("🔴 Sessione Fermata! Calcolo risultati in corso...");
    tracker.stop() // Ferma tutto, fa le medie e restituisce il JSON a React
}