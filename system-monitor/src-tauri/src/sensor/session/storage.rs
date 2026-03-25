use std::fs;
use std::path::Path;
use crate::sensor::session::session_data::{SessionResults,SessionHistory};

pub struct SessionStorage;

impl SessionStorage {
    const HISTORY_FILE: &'static str = "benchmark_history.xml";

    pub fn save(new_session: SessionResults) {
        let mut history = Self::load();
        history.insert(0, new_session);
        history.truncate(5);

        let container = SessionHistory { sessions: history };
        if let Ok(xml_string) = quick_xml::se::to_string(&container) {
            let _ = fs::write(Self::HISTORY_FILE, xml_string);
        }
    }

    pub fn load() -> Vec<SessionResults> {
        if Path::new(Self::HISTORY_FILE).exists() {
            if let Ok(xml_string) = fs::read_to_string(Self::HISTORY_FILE) {
                if let Ok(container) = quick_xml::de::from_str::<SessionHistory>(&xml_string) {
                    return container.sessions;
                }
            }
        }
        Vec::new()
    }
}