//! neurokernel/src/brainteaser.rs
//! Interactive brain teaser engine for cognitive stimulation

use rand::seq::SliceRandom;
use std::io::{self, Write};

struct BrainTeaser {
    question: &'static str,
    answer: &'static str,
}

pub struct BrainTeaserModule {
    teasers: Vec<BrainTeaser>,
}

impl BrainTeaserModule {
    pub fn new() -> Self {
        BrainTeaserModule {
            teasers: vec![
                BrainTeaser {
                    question: "What comes next in the sequence: 2, 3, 5, 7, 11, ?",
                    answer: "13",
                },
                BrainTeaser {
                    question: "I speak without a mouth and hear without ears. What am I?",
                    answer: "echo",
                },
                BrainTeaser {
                    question: "What gets wetter the more it dries?",
                    answer: "towel",
                },
                BrainTeaser {
                    question: "What number do you get if you multiply all of the numbers on a telephone keypad?",
                    answer: "0",
                },
                BrainTeaser {
                    question: "What five-letter word becomes shorter when you add two letters to it?",
                    answer: "short",
                },
            ],
        }
    }

    pub fn challenge(&self) {
        let mut rng = rand::thread_rng();
        if let Some(teaser) = self.teasers.choose(&mut rng) {
            println!("\n🧩 Brain Teaser:\n{}
", teaser.question);
            print!("Your answer: ");
            io::stdout().flush().unwrap();

            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).unwrap();

            if user_input.trim().eq_ignore_ascii_case(teaser.answer) {
                println!("✅ Correct!\n");
            } else {
                println!("❌ Incorrect. The correct answer is '{}'.\n", teaser.answer);
            }
        }
    }
}
