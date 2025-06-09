use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div style="display: flex; height: 100vh; background: linear-gradient(135deg, #1b1f3a, #2c2f61); color: white;">
            // Sidebar
            <div style="width: 80px; background: rgba(255,255,255,0.05); display: flex; flex-direction: column; align-items: center; padding-top: 20px;">
                <img src="https://cdn-icons-png.flaticon.com/512/942/942748.png" alt="Notepad" style="width: 40px; margin-bottom: 20px;" />
                <img src="https://cdn-icons-png.flaticon.com/512/727/727245.png" alt="Music" style="width: 40px; margin-bottom: 20px;" />
                <img src="https://cdn-icons-png.flaticon.com/512/786/786205.png" alt="Chatbot" style="width: 40px; margin-bottom: 20px;" />
                <img src="https://cdn-icons-png.flaticon.com/512/709/709496.png" alt="Join Us" style="width: 40px; margin-bottom: 20px;" />
                <img src="https://cdn-icons-png.flaticon.com/512/3135/3135715.png" alt="Teaser" style="width: 40px;" />
            </div>

            // Main Content Area
            <div style="flex: 1; padding: 30px; display: flex; flex-wrap: wrap; gap: 20px;">
                { render_app_window("📝 Notepad", "Type your notes here...") }
                { render_app_window("🎵 Music Player", "Now Playing: Neuro Beats") }
                { render_app_window("🤖 Chatbot", "NeuroBot is ready!") }
                { render_app_window("📥 Join Team Neuro", "Apply to join us!") }
                { render_app_window("🧠 Brain Teaser", "What has keys but can’t open locks?") }
            </div>
        </div>
    }
}

fn render_app_window(title: &str, content: &str) -> Html {
    html! {
        <div style="width: 250px; min-height: 150px; background: rgba(255,255,255,0.05); border-radius: 14px; padding: 10px; box-shadow: 0 0 10px rgba(0,0,0,0.3);">
            <div style="font-weight: bold; margin-bottom: 10px;">{ title }</div>
            <div>{ content }</div>
        </div>
    }
}
