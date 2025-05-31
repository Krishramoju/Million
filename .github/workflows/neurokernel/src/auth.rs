//! neurokernel/src/auth.rs
//! Authentication and biometric scaffold for Neurokernel OS.

use std::collections::HashMap;

pub enum AuthError {
    UserNotFound,
    InvalidPassword,
    Unauthorized,
}

pub struct User {
    pub username: String,
    pub password_hash: String, // For simplicity, store hashed password as string
    pub biometric_enabled: bool,
}

pub struct AuthManager {
    users: HashMap<String, User>,
    logged_in_user: Option<String>,
}

impl AuthManager {
    pub fn new() -> Self {
        AuthManager {
            users: HashMap::new(),
            logged_in_user: None,
        }
    }

    pub fn register_user(&mut self, username: &str, password_hash: &str) -> bool {
        if self.users.contains_key(username) {
            return false; // User already exists
        }
        self.users.insert(
            username.to_string(),
            User {
                username: username.to_string(),
                password_hash: password_hash.to_string(),
                biometric_enabled: false,
            },
        );
        true
    }

    pub fn login(&mut self, username: &str, password_hash: &str) -> Result<(), AuthError> {
        match self.users.get(username) {
            Some(user) if user.password_hash == password_hash => {
                self.logged_in_user = Some(username.to_string());
                Ok(())
            }
            Some(_) => Err(AuthError::InvalidPassword),
            None => Err(AuthError::UserNotFound),
        }
    }

    pub fn logout(&mut self) {
        self.logged_in_user = None;
    }

    pub fn enable_biometric(&mut self, username: &str) -> Result<(), AuthError> {
        match self.users.get_mut(username) {
            Some(user) => {
                user.biometric_enabled = true;
                Ok(())
            }
            None => Err(AuthError::UserNotFound),
        }
    }

    pub fn is_biometric_enabled(&self, username: &str) -> bool {
        self.users
            .get(username)
            .map_or(false, |user| user.biometric_enabled)
    }

    pub fn current_user(&self) -> Option<&String> {
        self.logged_in_user.as_ref()
    }
}
