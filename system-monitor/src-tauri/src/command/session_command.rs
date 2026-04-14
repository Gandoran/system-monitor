use tauri::{command, State};
use std::sync::{Arc, Mutex};
use crate::sensor::session::storage::SessionStorage;
use crate::sensor::session::session_data::SessionResults;
use crate::sensor::session::tracker::SessionTracker;

#[command]
pub fn get_session_history() -> Vec<SessionResults> {
    SessionStorage::load()
}

#[command]
pub fn start_session(state: State<'_, Arc<Mutex<SessionTracker>>>) {
    let mut tracker = state.lock().unwrap();
    tracker.start(None);
}

#[command]
pub fn stop_session(state: State<'_, Arc<Mutex<SessionTracker>>>) -> SessionResults {
    let mut tracker = state.lock().unwrap();
    let session = tracker.stop();
    SessionStorage::save(session.clone());
    session
}

#[command]
pub fn delete_session(index: usize) -> Vec<SessionResults> {
    SessionStorage::delete(index);
    SessionStorage::load() 
}