use std::fs::File;
use std::io::{Write, BufWriter};

#[derive(Debug)]
pub struct GameData {
    pub user_id: String,
    pub game_score: u32,
    pub level_reached: u32,
}

#[derive(Debug)]
pub struct ChatData {
    pub user_id: String,
    pub last_message: String,
    pub response_time_ms: u64,
}

#[derive(Debug)]
pub struct ApplicantData {
    pub name: String,
    pub email: String,
    pub qualification: String,
    pub score: f32,
}

#[derive(Debug)]
pub struct UserBigData {
    pub game: GameData,
    pub chat: ChatData,
    pub applicant: ApplicantData,
}

pub struct BigDataStore {
    pub entries: Vec<UserBigData>,
}

impl BigDataStore {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn add_entry(&mut self, entry: UserBigData) {
        self.entries.push(entry);
    }

    pub fn export_to_csv(&self, path: &str) {
        let file = File::create(path).expect("Unable to create file");
        let mut writer = BufWriter::new(file);

        writeln!(
            writer,
            "UserID,GameScore,Level,LastMessage,ResponseTime,Name,Email,Qualification,ApplicantScore"
        )
        .unwrap();

        for entry in &self.entries {
            writeln!(
                writer,
                "{},{},{},{},{},{},{},{},{}",
                entry.game.user_id,
                entry.game.game_score,
                entry.game.level_reached,
                entry.chat.last_message,
                entry.chat.response_time_ms,
                entry.applicant.name,
                entry.applicant.email,
                entry.applicant.qualification,
                entry.applicant.score
            )
            .unwrap();
        }
        println!("📊 Exported {} entries to {}", self.entries.len(), path);
    }
}
