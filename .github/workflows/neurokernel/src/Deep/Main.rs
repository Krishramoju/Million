mod deep_learning;
mod chatbot;

fn main() {
    println!("🧠 Booting Neuro OS AI...");

    // Sample values: BrainTeaserScore, ChatFrequency, RecruitmentFormScore
    let user_vector = deep_learning::get_user_vector(0.7, 0.9, 0.6);

    println!("🧬 Generated User Vector: {:?}", user_vector);

    // Generate personalized reply using vector
    chatbot::personalized_reply(user_vector);
}
