use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnalyticsApp {
    ChatBot,
    BrainTeaser,
    TeamRecruitment,
}

#[derive(Debug)]
pub struct ChatMetrics {
    pub message_count: usize,
    pub unique_words: HashSet<String>,
    pub word_freq: HashMap<String, usize>,
}

#[derive(Debug)]
pub struct BrainTeaserMetrics {
    pub total_puzzles: usize,
    pub solved: usize,
    pub avg_time_secs: f64,
}

#[derive(Debug)]
pub struct RecruitmentMetrics {
    pub total_applicants: usize,
    pub accepted: usize,
    pub rejected: usize,
    pub top_skills: HashMap<String, usize>,
}

#[derive(Debug)]
pub enum AppAnalytics {
    Chat(ChatMetrics),
    Brain(BrainTeaserMetrics),
    Recruit(RecruitmentMetrics),
}

pub struct DataAnalytics;

impl DataAnalytics {
    // --- CHAT ANALYSIS ---
    pub fn analyze_chat(messages: &[String]) -> ChatMetrics {
        let mut word_freq = HashMap::new();
        let mut unique_words = HashSet::new();
        let mut message_count = 0;

        for message in messages {
            message_count += 1;
            for word in message.split_whitespace() {
                let clean = word.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_string();
                if clean.is_empty() { continue; }

                *word_freq.entry(clean.clone()).or_insert(0) += 1;
                unique_words.insert(clean);
            }
        }

        ChatMetrics {
            message_count,
            unique_words,
            word_freq,
        }
    }

    // --- BRAIN TEASER ANALYSIS ---
    pub fn analyze_brain_teasers(times: &[f64], results: &[bool]) -> BrainTeaserMetrics {
        let total = results.len();
        let solved = results.iter().filter(|&&x| x).count();
        let avg_time = if !times.is_empty() {
            times.iter().sum::<f64>() / times.len() as f64
        } else {
            0.0
        };

        BrainTeaserMetrics {
            total_puzzles: total,
            solved,
            avg_time_secs: avg_time,
        }
    }

    // --- RECRUITMENT ANALYSIS ---
    pub fn analyze_applicants(applicants: &[(&str, bool, Vec<&str>)]) -> RecruitmentMetrics {
        let mut accepted = 0;
        let mut rejected = 0;
        let mut skills_map = HashMap::new();

        for (name, is_accepted, skills) in applicants {
            if *is_accepted {
                accepted += 1;
            } else {
                rejected += 1;
            }

            for skill in skills {
                *skills_map.entry(skill.to_lowercase()).or_insert(0) += 1;
            }
        }

        RecruitmentMetrics {
            total_applicants: applicants.len(),
            accepted,
            rejected,
            top_skills: skills_map,
        }
    }
}
