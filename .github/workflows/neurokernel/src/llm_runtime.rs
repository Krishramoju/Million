use std::env;
use std::io::{self, Write};
use base64::{engine::general_purpose, Engine as _};
use chrono::Local;
use reqwest::Client;
use serde_json::json;
use std::process::Command;

#[tokio::main]
async fn main() {
    let github_token = env::var("GITHUB_TOKEN").expect("Set GITHUB_TOKEN");
    let github_user = env::var("GITHUB_USERNAME").expect("Set GITHUB_USERNAME");
    let openai_key = env::var("OPENAI_API_KEY").expect("Set OPENAI_API_KEY");

    print!("\u{1F4AC} Enter your coding prompt: ");
    io::stdout().flush().unwrap();
    let mut prompt = String::new();
    io::stdin().read_line(&mut prompt).unwrap();
    let prompt = prompt.trim();

    // 1. Generate code
    let code = generate_code(&openai_key, prompt).await;
    println!("\u{2705} Code generated:\n{}", code);

    // 2. Save code to temp file for correction
    let temp_path = "temp_code.py";
    std::fs::write(temp_path, &code).expect("Unable to write temp file");

    // 3. Auto-correct code using Ruff
    let fix_status = Command::new("ruff")
        .arg("--fix")
        .arg(temp_path)
        .status()
        .expect("Failed to run Ruff");
    if !fix_status.success() {
        eprintln!("\u{274C} Ruff failed to correct code");
    }

    // 4. Read corrected code
    let corrected_code = std::fs::read_to_string(temp_path).unwrap_or_else(|_| code.clone());
    println!("\n\u{1F527} Corrected Code:\n{}", corrected_code);

    // 5. Create GitHub repo
    let slug = sanitize(prompt);
    let repo_name = format!("{}-{}", slug, Local::now().format("%H%M%S"));
    create_github_repo(&github_token, &repo_name).await;

    // 6. Upload corrected code
    upload_code(&github_token, &github_user, &repo_name, "main.py", &corrected_code).await;

    println!("\n\u{1F517} GitHub Link: https://github.com/{}/{}", github_user, repo_name);
}

fn sanitize(text: &str) -> String {
    text.to_lowercase()
        .replace(|c: char| !c.is_ascii_alphanumeric(), "-")
        .trim_matches('-')
        .to_string()
}

async fn generate_code(api_key: &str, prompt: &str) -> String {
    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&json!({
            "model": "gpt-4o",
            "messages": [
                { "role": "system", "content": "Only return Python code." },
                { "role": "user", "content": prompt }
            ],
            "temperature": 0.2
        }))
        .send()
        .await
        .expect("OpenAI API failed");

    let json: serde_json::Value = response.json().await.unwrap();
    json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("# No code returned.")
        .to_string()
}

async fn create_github_repo(token: &str, name: &str) {
    let client = Client::new();
    let res = client
        .post("https://api.github.com/user/repos")
        .bearer_auth(token)
        .header("User-Agent", "rust-llm-push")
        .json(&json!({
            "name": name,
            "description": "Repo created by Rust AI bot",
            "auto_init": true
        }))
        .send()
        .await
        .unwrap();

    if res.status() != 201 {
        let err = res.text().await.unwrap();
        panic!("\u{274C} GitHub repo error: {}", err);
    }
}

async fn upload_code(token: &str, user: &str, repo: &str, path: &str, code: &str) {
    let url = format!("https://api.github.com/repos/{}/{}/contents/{}", user, repo, path);
    let encoded = general_purpose::STANDARD.encode(code);

    let client = Client::new();
    let res = client
        .put(&url)
        .bearer_auth(token)
        .header("User-Agent", "rust-llm-push")
        .json(&json!({
            "message": format!("Add {}", path),
            "content": encoded,
        }))
        .send()
        .await
        .unwrap();

    if res.status() != 201 {
        let err = res.text().await.unwrap();
        panic!("\u{274C} Upload error: {}", err);
    }
}
