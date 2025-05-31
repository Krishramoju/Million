//! neurokernel/src/setup.rs
//! Boot-time setup and initialization routines for Neurokernel OS.

use crate::auth::AuthManager;
use crate::fs::FileSystem;
use crate::notifier::Notifier;
use crate::llm_runtime::LLMEngine;
use crate::scheduler::Scheduler;
use crate::ml_monitor::MLMonitor;
use crate::net::NetManager;
use crate::vault_storage::Vault;
use crate::config::Config;
use crate::shell::Shell;

pub struct SystemCore {
    pub auth: AuthManager,
    pub fs: FileSystem,
    pub notifier: Notifier,
    pub llm: LLMEngine,
    pub scheduler: Scheduler,
    pub monitor: MLMonitor,
    pub net: NetManager,
    pub vault: Vault,
    pub config: Config,
    pub shell: Shell,
}

impl SystemCore {
    pub fn init() -> Self {
        println!("🧠 Neurokernel OS Booting...");

        let auth = AuthManager::new();
        let fs = FileSystem::new();
        let notifier = Notifier::new();
        let llm = LLMEngine::new();
        let scheduler = Scheduler::new();
        let monitor = MLMonitor::new();
        let net = NetManager::new();
        let vault = Vault::new();
        let config = Config::load_defaults();
        let shell = Shell::new();

        println!("✅ Core systems initialized.");
        notifier.push("Neurokernel OS setup complete.", crate::notifier::NotificationLevel::Info);

        SystemCore {
            auth,
            fs,
            notifier,
            llm,
            scheduler,
            monitor,
            net,
            vault,
            config,
            shell,
        }
    }

    pub fn boot_sequence(&mut self) {
        self.notifier.push("🟢 Running boot sequence...", crate::notifier::NotificationLevel::Info);

        self.vault.sync();
        self.net.establish_secure_tunnel();
        self.scheduler.run_boot_jobs();
        self.llm.warm_up();
        self.shell.welcome();

        self.notifier.push("🧬 System ready for interaction.", crate::notifier::NotificationLevel::Info);
    }
}
