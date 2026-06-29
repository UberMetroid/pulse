use crate::app::App;
use crate::app::Msg;
use yew::prelude::*;

impl App {
    pub fn view_login(&self, ctx: &Context<Self>) -> Html {
        let pin_len = self.pin_length;
        let error_html = self.error_message.as_ref().map(|err| {
            html! { <p id="pin-error" class="pin-error" style="display: block;">{err}</p> }
        });

        html! {
            <div class="login-container">
                <div class="login-box">
                    <div class="login-header" style="margin-bottom: 2rem;">
                        <div class="login-icon-frame" style="display: flex; justify-content: center; margin-bottom: 1rem;">
                            <img src="/favicon.svg" class="login-app-icon" alt="Pulse" style="width: 64px; height: 64px;" />
                        </div>
                        <h2 style="font-size: 1.5rem; font-weight: 500; color: var(--text); opacity: 0.8; line-height: 1.2;">
                            {"ENTER SECURITY PIN"}
                        </h2>
                    </div>
                    <form id="pin-form" onsubmit={ctx.link().callback(|e: SubmitEvent| { e.prevent_default(); Msg::SubmitPin })}>
                        <div class="pin-wrapper">
                            <input
                                type="password"
                                class="pin-input-field"
                                value={self.pin_input.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    Msg::PinInputChanged(input.value())
                                })}
                                placeholder={"• ".repeat(pin_len).trim().to_string()}
                                maxlength={pin_len.to_string()}
                                autofocus=true
                            />
                        </div>
                    </form>
                    <div class="pin-status">
                        {error_html}
                    </div>
                </div>
            </div>
        }
    }
}
