pub fn personalized_reply(user_vector: Vec<f32>) {
    let friendliness = user_vector[0];
    let complexity = user_vector[1];

    let greeting = if friendliness > 0.5 {
        "Hey there, great to see you again!"
    } else {
        "Hello. How can I assist you today?"
    };

    let info = if complexity > 0.5 {
        "Would you like a deep dive into the system’s features or advanced analytics?"
    } else {
        "Want me to tell you a fun fact or help with something simple?"

    };

    println!("🤖 Chatbot says:\n{}\n{}", greeting, info);
}
