use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ApplicantType {
    Human,
    Agent,
}

#[derive(Debug, Clone)]
pub struct ApplicantProfile {
    pub id: Uuid,
    pub name: String,
    pub kind: ApplicantType,
    pub capabilities: Vec<String>,
    pub intent_statement: String,
    pub portfolio_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Role {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub required_skills: Vec<String>,
    pub is_open: bool,
}

#[derive(Debug)]
pub struct RecruitmentLayer {
    pub applicants: HashMap<Uuid, ApplicantProfile>,
    pub roles: HashMap<Uuid, Role>,
    pub queue: VecDeque<(Uuid, Uuid)>, // (Applicant ID, Role ID)
}

impl RecruitmentLayer {
    pub fn new() -> Self {
        Self {
            applicants: HashMap::new(),
            roles: HashMap::new(),
            queue: VecDeque::new(),
        }
    }

    pub fn register_applicant(
        &mut self,
        name: String,
        kind: ApplicantType,
        capabilities: Vec<String>,
        intent_statement: String,
        portfolio_url: Option<String>,
    ) -> Uuid {
        let id = Uuid::new_v4();
        let profile = ApplicantProfile {
            id,
            name,
            kind,
            capabilities,
            intent_statement,
            portfolio_url,
        };
        self.applicants.insert(id, profile);
        id
    }

    pub fn post_role(
        &mut self,
        title: String,
        description: String,
        required_skills: Vec<String>,
    ) -> Uuid {
        let id = Uuid::new_v4();
        let role = Role {
            id,
            title,
            description,
            required_skills,
            is_open: true,
        };
        self.roles.insert(id, role);
        id
    }

    pub fn match_roles(&mut self) {
        for (applicant_id, profile) in &self.applicants {
            for (role_id, role) in &self.roles {
                if role.is_open && self.skill_match(&profile.capabilities, &role.required_skills) {
                    self.queue.push_back((*applicant_id, *role_id));
                }
            }
        }
    }

    fn skill_match(&self, skills: &[String], required: &[String]) -> bool {
        let skill_set: std::collections::HashSet<_> = skills.iter().collect();
        required.iter().all(|r| skill_set.contains(r))
    }

    pub fn next_assignment(&mut self) -> Option<(ApplicantProfile, Role)> {
        while let Some((aid, rid)) = self.queue.pop_front() {
            if let (Some(applicant), Some(role)) = (
                self.applicants.get(&aid).cloned(),
                self.roles.get_mut(&rid),
            ) {
                if role.is_open {
                    role.is_open = false;
                    return Some((applicant, role.clone()));
                }
            }
        }
        None
    }

    pub fn list_open_roles(&self) -> Vec<&Role> {
        self.roles.values().filter(|r| r.is_open).collect()
    }
}
