use yew::prelude::*;

#[function_component(SystemSettings)]
pub fn system_settings() -> Html {
    let font_size = use_state(|| 16);
    let theme = use_state(|| "Dark Mode".to_string());

    let send_usage_data = use_state(|| false);
    let private_mode = use_state(|| false);

    let smart_responses = use_state(|| true);
    let voice_feedback = use_state(|| false);
    let assistant_persona = use_state(|| "Professional".to_string());

    let battery_saver = use_state(|| false);
    let animation_speed = use_state(|| 50);

    html! {
        <div class="settings-container">
            <h1>{ "⚙️ Neuro OS – System Settings" }</h1>

            // Appearance Section
            <section class="settings-section">
                <h2>{ "🖼️ Appearance" }</h2>

                <div class="setting">
                    <label>{ "Theme" }</label>
                    <select onchange={Callback::from({
                        let theme = theme.clone();
                        move |e: Event| {
                            let input: HtmlSelectElement = e.target_unchecked_into();
                            theme.set(input.value());
                        }
                    })}>
                        <option value="Dark Mode">{ "Dark Mode" }</option>
                        <option value="Light Mode">{ "Light Mode" }</option>
                        <option value="Cyber Neon">{ "Cyber Neon" }</option>
                    </select>
                </div>

                <div class="setting">
                    <label>{ "Font Size" }</label>
                    <input type="range"
                        min="12"
                        max="24"
                        value={font_size.to_string()}
                        oninput={Callback::from({
                            let font_size = font_size.clone();
                            move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                if let Ok(val) = input.value().parse::<i32>() {
                                    font_size.set(val);
                                }
                            }
                        })}
                    />
                    <span class="range-label">{ format!("{}px", *font_size) }</span>
                </div>
            </section>

            // Privacy Section
            <section class="settings-section">
                <h2>{ "🔒 Privacy" }</h2>
                <div class="setting">
                    <label>{ "Send Usage Data" }</label>
                    <input type="checkbox"
                        checked={*send_usage_data}
                        onchange={Callback::from({
                            let send_usage_data = send_usage_data.clone();
                            move |_| send_usage_data.set(!*send_usage_data)
                        })}
                    />
                </div>

                <div class="setting">
                    <label>{ "Private Mode" }</label>
                    <input type="checkbox"
                        checked={*private_mode}
                        onchange={Callback::from({
                            let private_mode = private_mode.clone();
                            move |_| private_mode.set(!*private_mode)
                        })}
                    />
                </div>
            </section>

            // AI Section
            <section class="settings-section">
                <h2>{ "🤖 AI Assistant" }</h2>
                <div class="setting">
                    <label>{ "Enable Smart Responses" }</label>
                    <input type="checkbox"
                        checked={*smart_responses}
                        onchange={Callback::from({
                            let smart_responses = smart_responses.clone();
                            move |_| smart_responses.set(!*smart_responses)
                        })}
                    />
                </div>

                <div class="setting">
                    <label>{ "Voice Feedback" }</label>
                    <input type="checkbox"
                        checked={*voice_feedback}
                        onchange={Callback::from({
                            let voice_feedback = voice_feedback.clone();
                            move |_| voice_feedback.set(!*voice_feedback)
                        })}
                    />
                </div>

                <div class="setting">
                    <label>{ "Assistant Personality" }</label>
                    <select onchange={Callback::from({
                        let assistant_persona = assistant_persona.clone();
                        move |e: Event| {
                            let input: HtmlSelectElement = e.target_unchecked_into();
                            assistant_persona.set(input.value());
                        }
                    })}>
                        <option value="Professional">{ "Professional" }</option>
                        <option value="Friendly">{ "Friendly" }</option>
                        <option value="Minimal">{ "Minimal" }</option>
                    </select>
                </div>
            </section>

            // Performance Section
            <section class="settings-section">
                <h2>{ "⚡ Performance" }</h2>
                <div class="setting">
                    <label>{ "Animation Speed" }</label>
                    <input type="range"
                        min="0"
                        max="100"
                        value={animation_speed.to_string()}
                        oninput={Callback::from({
                            let animation_speed = animation_speed.clone();
                            move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                if let Ok(val) = input.value().parse::<i32>() {
                                    animation_speed.set(val);
                                }
                            }
                        })}
                    />
                    <span class="range-label">{ format!("{}%", *animation_speed) }</span>
                </div>

                <div class="setting">
                    <label>{ "Battery Saver Mode" }</label>
                    <input type="checkbox"
                        checked={*battery_saver}
                        onchange={Callback::from({
                            let battery_saver = battery_saver.clone();
                            move |_| battery_saver.set(!*battery_saver)
                        })}
                    />
                </div>
            </section>
        </div>
    }
}
