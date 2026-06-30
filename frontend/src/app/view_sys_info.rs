use crate::app::App;
use yew::prelude::*;

impl App {
    pub fn view_sys_info_card(&self) -> Html {
        if let Some(stats) = &self.stats {
            let uptime_str = {
                let seconds = stats.uptime;
                let days = seconds / 86400;
                let hours = (seconds % 86400) / 3600;
                let minutes = (seconds % 3600) / 60;
                let secs = seconds % 60;
                if days > 0 {
                    format!("{}d {}h {}m", days, hours, minutes)
                } else if hours > 0 {
                    format!("{}h {}m {}s", hours, minutes, secs)
                } else {
                    format!("{}m {}s", minutes, secs)
                }
            };

            let os_ascii = self.get_os_ascii(&stats.os_name);

            html! {
                <div class="hud-metric-card sys-info-card" title="System Information">
                    <div class="sys-info-left">
                        <h3>{"SYSTEM INFO"}</h3>
                        <div class="card-metric-block sys-details">
                            <div class="sys-detail-row">
                                <span class="sys-detail-label">{"HOST:"}</span>
                                <span class="sys-detail-val hostname-glow">{ &stats.hostname }</span>
                            </div>
                            <div class="sys-detail-row">
                                <span class="sys-detail-label">{"OS:"}</span>
                                <span class="sys-detail-val">{ format!("{} {}", stats.os_name, stats.os_version).trim().to_string() }</span>
                            </div>
                            <div class="sys-detail-row">
                                <span class="sys-detail-label">{"KERNEL:"}</span>
                                <span class="sys-detail-val">{ &stats.kernel_version }</span>
                            </div>
                            <div class="sys-detail-row">
                                <span class="sys-detail-label">{"UPTIME:"}</span>
                                <span class="sys-detail-val">{ uptime_str }</span>
                            </div>
                        </div>
                    </div>
                    <div class="sys-info-right ascii-container">
                        <pre>{ os_ascii }</pre>
                    </div>
                </div>
            }
        } else {
            html! {
                <div class="hud-metric-card sys-info-card">
                    <div class="sys-info-left">
                        <h3>{"SYSTEM INFO"}</h3>
                        <div class="card-metric-block">
                            <div class="card-loading">{"Connecting..."}</div>
                        </div>
                    </div>
                </div>
            }
        }
    }

    fn get_os_ascii(&self, os_name: &str) -> &'static str {
        let os = os_name.to_lowercase();
        if os.contains("nixos") {
            "      ▗▟▙▖  ▗▟▙▖\n  ▗▞▘   ▐▌   ▝▚▖\n  ▐▌  ▗▞▘ ▝▚▖  ▐▌\n▗▄▟▙▄▖       ▗▄▟▙▄▖\n  ▐▌  ▝▚▖ ▗▞▘  ▐▌\n  ▝▚▖   ▐▌   ▗▞▘\n      ▝▘▀▘  ▝▀▘"
        } else if os.contains("ubuntu") {
            "     ▗▄███▄▖\n   ▗██▀▀ ▀▀██▖\n  ▐██▌ ▗▄▖ ▐██▌\n  ▐██▌ ▜█▛ ▐██▌\n   ▝██▄▄ ▄▄██▘\n     ▝▀███▀▘"
        } else if os.contains("debian") {
            "     ▗▄▄▄▄▖\n    ▞▀    ▀▚\n   ▐▌  ▗▄▄▖ ▐▌\n   ▐▌ ▐█▀▀  ▐▌\n    ▝▚▄▄▄▄▞▀\n     ▝▀▀▀▘"
        } else if os.contains("arch") {
            "      /\\\n     /  \\\n    /\\   \\\n   /  __  \\\n  /  (  )  \\\n /  ▞    ▜  \\\n/____________\\"
        } else if os.contains("fedora") {
            "      ▗▄▄▄▄▖\n     ▞▀  ▗▞▘\n    ▐▌  ▗▛▘\n   ▗▜▀▀▀▛▚▖\n   ▐▌  ▐▌ ▐▌\n    ▝▚▄▞▘▞▘"
        } else {
            "     ▗▄███▄▖\n    ▗█▀ ▜ ▀█▖\n    ▐▌ ▗▟▙▖ ▐▌\n   ▗█▌ ▐██▌ ▐█▖\n   ▐██▄▄██▄▄██▌\n    ▝▀█████▀▘"
        }
    }
}
