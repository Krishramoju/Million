use std::{
    fs::File,
    io::BufReader,
    process::Command,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use rodio::{Decoder, OutputStream, Sink};

pub enum TrackType {
    Local(String),
    Online(String),       // YouTube or Archive.org URL
    AI(String),           // Prompt to generate music
}

pub struct MusicDaemon {
    sink: Arc<Mutex<Sink>>,
}

impl MusicDaemon {
    pub fn new() -> Self {
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        MusicDaemon {
            sink: Arc::new(Mutex::new(sink)),
        }
    }

    pub fn play(&self, track: TrackType) {
        let sink = self.sink.clone();

        thread::spawn(move || {
            match track {
                TrackType::Local(path) => {
                    MusicDaemon::play_local(&sink, &path);
                }
                TrackType::Online(url) => {
                    let output = MusicDaemon::download_audio(&url);
                    MusicDaemon::play_local(&sink, &output);
                }
                TrackType::AI(prompt) => {
                    let path = MusicDaemon::generate_music_from_prompt(&prompt);
                    MusicDaemon::play_local(&sink, &path);
                }
            }
        });
    }

    fn play_local(sink: &Arc<Mutex<Sink>>, path: &str) {
        if let Ok(file) = File::open(path) {
            let source = Decoder::new(BufReader::new(file)).unwrap();
            let mut sink = sink.lock().unwrap();
            sink.stop();
            sink.append(source);
            sink.play();
        } else {
            eprintln!("⚠️ Could not play file: {}", path);
        }
    }

    fn download_audio(url: &str) -> String {
        println!("🌐 Downloading: {}", url);
        let output = Command::new("yt-dlp")
            .arg("-x")
            .arg("--audio-format")
            .arg("mp3")
            .arg(url)
            .output()
            .expect("❌ yt-dlp failed");

        // Simulate finding the downloaded file
        let filename = "downloaded_audio.mp3";
        std::fs::rename("*.mp3", filename).ok(); // Simplified
        filename.to_string()
    }

    fn generate_music_from_prompt(prompt: &str) -> String {
        println!("🤖 Generating music from prompt: '{}'", prompt);

        // Simulated AI music generation (replace with real API call)
        std::fs::write("ai_music.mp3", b"SILENCE-MOCK-MUSIC").unwrap();

        "ai_music.mp3".to_string()
    }

    pub fn loop_daemon(&self) {
        println!("🎶 Music Daemon running...");
        loop {
            thread::sleep(Duration::from_secs(10));
        }
    }
}
/*

mod music_daemon;
use music_daemon::{MusicDaemon, TrackType};

fn main() {
    let daemon = MusicDaemon::new();

    // Play historical music from Archive.org/YouTube
    daemon.play(TrackType::Online("https://youtube.com/watch?v=xxxxx".to_string()));

    // Play AI-generated future music
    daemon.play(TrackType::AI("generate ambient cyberpunk meditation theme".to_string()));

    // Play local `.mp3`
    daemon.play(TrackType::Local("local/beethoven.mp3".to_string()));

    daemon.loop_daemon();
}*/
