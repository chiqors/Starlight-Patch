mod theme;

use std::sync::atomic::Ordering;
use crate::ui::theme::{apply_theme, rgba};
use hudhook::imgui::{Context, Ui};
use hudhook::{imgui, ImguiRenderLoop, RenderContext};
use crate::config::{save_config, CONFIG, FROM_REMOTE};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_CONTROL, VK_L};
use crate::patches::fps;
use crate::patches::fps::update_frame_rate;

pub struct RenderLoop {
    pub ui_visible: bool,
    pub toggle_pressed: bool,
}

fn toggle_ui(ui_visible: &mut bool, was_pressed: &mut bool) {
    let ctrl = unsafe { GetAsyncKeyState(VK_CONTROL.0 as i32) < 0 };
    let l = unsafe { GetAsyncKeyState(VK_L.0 as i32) < 0 };

    let pressed = ctrl && l;

    if pressed && !*was_pressed {
        *ui_visible = !*ui_visible;
    }

    *was_pressed = pressed;
}

impl ImguiRenderLoop for RenderLoop {
    fn initialize<'a>(&'a mut self, ctx: &mut Context, _render_context: &'a mut dyn RenderContext) {
        apply_theme(ctx);
    }

    fn render(&mut self, ui: &mut Ui) {
        toggle_ui(&mut self.ui_visible, &mut self.toggle_pressed);

        if !self.ui_visible {
            return;
        }

        ui.window("Starlight - RSA Patch")
            .position([300., 300.], imgui::Condition::FirstUseEver)
            .size([700., 500.], imgui::Condition::FirstUseEver)
            .build(|| {
                let fonts = ui.fonts().fonts();

                if let Some(_) = ui.tab_bar("tabs") {
                    if let Some(_) = ui.tab_item("Info") {
                        {
                            let bold = fonts.get(1).unwrap();
                            let _font = ui.push_font(*bold);
                            ui.text("Starlight-Patch");
                        }

                        ui.text_colored(
                            rgba(150, 150, 160, 255),
                            "RSA patch utility for connecting to alternative servers.",
                        );

                        ui.spacing();

                        {
                            let bold = fonts.get(1).unwrap();
                            let _font = ui.push_font(*bold);
                            ui.text("Controls");
                        }

                        ui.text_colored(
                            rgba(150, 150, 160, 255),
                            "Press Ctrl+L to toggle the menu.",
                        );
                    }

                    if let Some(_) = ui.tab_item("Settings") {
                        {
                            let bold = fonts.get(1).unwrap();
                            let _font = ui.push_font(*bold);
                            ui.text("Patch Settings");
                        }

                        ui.text_colored(
                            rgba(150, 150, 160, 255),
                            "Configure the patch settings below.",
                        );

                        ui.separator();

                        if CONFIG.get().is_none() {
                            ui.text_colored(rgba(255, 204, 128, 255), "Loading settings...");
                        }

                        // (should, reset)
                        let mut should_update_fps = false;
                        let mut should_reset = false;

                        if let Ok(mut config) = CONFIG.get().unwrap().write() {
                            if ui.collapsing_header("Encryption", imgui::TreeNodeFlags::empty()) {
                                if FROM_REMOTE.load(Ordering::Relaxed) {
                                    ui.text_colored(
                                        rgba(255, 204, 128, 255),
                                        "Encryption settings are controlled by the server.",
                                    );
                                } else {
                                    {
                                        let bold = fonts.get(1).unwrap();
                                        let _font = ui.push_font(*bold);
                                        ui.text("RSA Configuration");
                                    }

                                    ui.checkbox(
                                        "Patch SDK RSA Encryption",
                                        &mut config.encryption.use_sdk_rsa,
                                    );

                                    ui.input_text(
                                        "SDK RSA Key",
                                        &mut config.encryption.sdk_key,
                                    ).build();

                                    ui.input_text(
                                        "CheckSign RSA Key",
                                        &mut config.encryption.sdk_key,
                                    ).build();
                                }
                            }

                            if ui.collapsing_header("Network", imgui::TreeNodeFlags::empty()) {
                                {
                                    let bold = fonts.get(1).unwrap();
                                    let _font = ui.push_font(*bold);
                                    ui.text("Connection");
                                }

                                ui.checkbox("Use HTTPS", &mut config.network.https);
                                ui.input_text("Host", &mut config.network.address).build();
                                ui.input_int("Port", &mut config.network.port).build();
                            }

                            if ui.collapsing_header("FPS", imgui::TreeNodeFlags::empty()) {
                                if fps::WORKS.load(Ordering::Relaxed) {
                                    {
                                        let bold = fonts.get(1).unwrap();
                                        let _font = ui.push_font(*bold);
                                        ui.text("FPS unlocker");
                                    }

                                    let mut value = match config.fps.target_max_fps {
                                        -1 => 0,
                                        _ => config.fps.target_max_fps,
                                    };

                                    if ui.checkbox("Enabled", &mut config.fps.enabled) {
                                        should_update_fps = true;
                                        should_reset = !config.fps.enabled;
                                    }

                                    let submitted = ui
                                        .input_int("Target frame rate", &mut value)
                                        .enter_returns_true(true)
                                        .build();

                                    ui.text_colored(
                                        rgba(150, 150, 160, 255),
                                        "Use \"0\" to unlock the framerate completely. Press Enter to apply.",
                                    );

                                    if value < 0 {
                                        ui.text_colored(
                                            rgba(255, 82, 82, 255),
                                            "Please input a positive number.",
                                        );
                                    } else if submitted {
                                        let new_value = if value == 0 {
                                            -1
                                        } else {
                                            value
                                        };

                                        if new_value != config.fps.target_max_fps {
                                            config.fps.target_max_fps = new_value;
                                            should_update_fps = true;
                                        }
                                    }
                                } else {
                                    ui.text_colored(
                                        rgba(255, 82, 82, 255),
                                        "Patch is outdated."
                                    )
                                }
                            }
                        } else {
                            ui.text_colored(
                                rgba(255, 82, 82, 255),
                                "Couldn't open a RwLock to CONFIG!",
                            );
                        }

                        if should_update_fps {
                            update_frame_rate(should_reset);
                        }

                        ui.spacing();

                        if ui.button("Save config to file") {
                            save_config().unwrap();
                        }
                    }
                }
            });
    }
}