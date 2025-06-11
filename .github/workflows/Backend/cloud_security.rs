use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};
use uuid::Uuid;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum App {
    ChatBot,
    BrainTeaser,
    TeamNeuroOSRecruitment,
}

#[derive(Debug)]
pub struct SessionToken {
    pub token: String,
    pub user_id: String,
    pub valid_until: Instant,
    pub apps_allowed: HashSet<App>,
}

#[derive(Debug)]
pub struct CloudSecurity {
    pub sessions: HashMap<String, SessionToken>,
    pub users: HashMap<String, String>, // user_id -> hashed_password
    pub access_log: Vec<String>,
}

impl CloudSecurity {
    pub fn new() -> Self {
        CloudSecurity {
            sessions: HashMap::new(),
            users: HashMap::new(),
            access_log: Vec::new(),
        }
    }

    /// Registers a new user with a hashed password
    pub fn register_user(&mut self, user_id: &str, password: &str) {
        let hashed = Self::hash_password(password);
        self.users.insert(user_id.to_string(), hashed);
    }

    /// Authenticates a user and returns a session token
    pub fn authenticate(&mut self, user_id: &str, password: &str, apps: Vec<App>) -> Result<String, String> {
        let stored_hash = self.users.get(user_id).ok_or("User not found")?;
        let input_hash = Self::hash_password(password);

        if &input_hash != stored_hash {
            return Err("Invalid credentials".to_string());
        }

        let token = Uuid::new_v4().to_string();
        let session = SessionToken {
            token: token.clone(),
            user_id: user_id.to_string(),
            valid_until: Instant::now() + Duration::from_secs(3600),
            apps_allowed: apps.into_iter().collect(),
        };

        self.sessions.insert(token.clone(), session);
        Ok(token)
    }

    /// Checks if a session token is valid and has permission to access the given app
    pub fn validate_access(&mut self, token: &str, app: &App) -> Result<(), String> {
        let session = self.sessions.get(token).ok_or("Invalid session token")?;

        if Instant::now() > session.valid_until {
            return Err("Session expired".to_string());
        }

        if !session.apps_allowed.contains(app) {
            return Err("Access denied to this app".to_string());
        }

        self.log_access(&session.user_id, app);
        Ok(())
    }

    /// Logs all access attempts
    fn log_access(&mut self, user_id: &str, app: &App) {
        let entry = format!(
            "[{}] User '{}' accessed {:?} at {:?}",
            Uuid::new_v4(),
            user_id,
            app,
            Instant::now()
        );
        self.access_log.push(entry);
    }

    /// Helper function to hash passwords
    fn hash_password(password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Expires sessions manually (e.g. for admin or security reason)
    pub fn revoke_session(&mut self, token: &str) -> bool {
        self.sessions.remove(token).is_some()
    }
}
