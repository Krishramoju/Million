use std::collections::HashMap;
use std::time::{SystemTime, Duration};

#[derive(Debug, Clone)]
pub enum ThreatType {
    UnauthorizedAccess,
    DataLeakAttempt,
    TamperingDetected,
    BruteForce,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ThreatEvent {
    pub ip: String,
    pub user_id: String,
    pub threat_type: ThreatType,
    pub severity: u8, // 1 (Low) to 10 (Critical)
    pub timestamp: SystemTime,
    pub description: String,
}

#[derive(Debug)]
pub struct ThreatLog {
    pub history: Vec<ThreatEvent>,
    pub failed_attempts: HashMap<String, usize>, // user_id -> count
}

impl ThreatLog {
    pub fn new() -> Self {
        ThreatLog {
            history: Vec::new(),
            failed_attempts: HashMap::new(),
        }
    }

    pub fn log_event(
        &mut self,
        ip: &str,
        user_id: &str,
        threat_type: ThreatType,
        severity: u8,
        description: &str,
    ) {
        let event = ThreatEvent {
            ip: ip.to_string(),
            user_id: user_id.to_string(),
            threat_type,
            severity,
            timestamp: SystemTime::now(),
            description: description.to_string(),
        };

        println!("🚨 SECURITY ALERT: {:?}", event); // log to console
        self.history.push(event);
    }

    pub fn detect_repeated_failures(&mut self, user_id: &str, ip: &str) {
        let count = self.failed_attempts.entry(user_id.to_string()).or_insert(0);
        *count += 1;

        if *count >= 5 {
            self.log_event(
                ip,
                user_id,
                ThreatType::BruteForce,
                9,
                "Repeated failed access attempts detected",
            );
        }
    }

    pub fn check_data_tampering(&mut self, original_hash: &str, current_hash: &str, user_id: &str, ip: &str) {
        if original_hash != current_hash {
            self.log_event(
                ip,
                user_id,
                ThreatType::TamperingDetected,
                10,
                "Hash mismatch detected - data integrity compromised",
            );
        }
    }

    pub fn check_unauthorized_access(&mut self, user_id: &str, ip: &str, expected_role: &str, actual_role: &str) {
        if expected_role != actual_role {
            self.log_event(
                ip,
                user_id,
                ThreatType::UnauthorizedAccess,
                7,
                &format!("Expected role: {}, but got {}", expected_role, actual_role),
            );
        }
    }
}
