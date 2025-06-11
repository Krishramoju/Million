mod data_store;
use data_store::{BigDataStore, GameData, ChatData, ApplicantData, UserBigData};

fn main() {
    let mut store = BigDataStore::new();

    let user_entry = UserBigData {
        game: GameData {
            user_id: "user123".to_string(),
            game_score: 4200,
            level_reached: 12,
        },
        chat: ChatData {
            user_id: "user123".to_string(),
            last_message: "Tell me a joke".to_string(),
            response_time_ms: 230,
        },
        applicant: ApplicantData {
            name: "Krish Ramoju".to_string(),
            email: "krish@example.com".to_string(),
            qualification: "B.Tech AI".to_string(),
            score: 91.4,
        },
    };

    store.add_entry(user_entry);

    // Add more entries or loop for input...

    store.export_to_csv("neuro_os_big_data.csv");
}
