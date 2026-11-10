use egui::{FontData, FontDefinitions, ViewportId};
use egui_wgpu::{RendererOptions, WgpuConfiguration, winit::Painter};
use winit::{application::ApplicationHandler, dpi::{PhysicalPosition, PhysicalSize}, event::{ElementState, WindowEvent}, event_loop::ActiveEventLoop, keyboard::{KeyCode, ModifiersState, PhysicalKey}, platform::x11::WindowAttributesExtX11, window::{Window, WindowAttributes}};

use std::sync::Arc;
use tracing::{error, debug};

use crate::{CONFIG, OVERLAY_STATE, overlay::keyboard_grab::KeyboardGrab};

mod keyboard_grab;
pub mod state;

pub struct Overlay {
    window: Option<Arc<Window>>,
    egui_ctx: egui::Context,
    egui_state: Option<egui_winit::State>,
    painter: Option<Painter>,

    mods: ModifiersState
}

impl Overlay {
    pub fn new() -> Self {
        Self {
            window: None,
            egui_ctx: egui::Context::default(),
            egui_state: None,
            painter: None,

            mods: ModifiersState::empty()
        }
    }
}

impl ApplicationHandler for Overlay {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let monitor = event_loop.primary_monitor()
            .or_else(|| event_loop.available_monitors().next());

        let (size, pos) = monitor
            .map(|m| (m.size(), m.position()))
            .unwrap_or((PhysicalSize::new(1920, 1080), PhysicalPosition::new(0, 0)));

        let attrs = WindowAttributes::default()
            .with_inner_size(size)
            .with_position(pos)
            .with_blur(true)
            .with_override_redirect(true)
            .with_name("carrion", "carrion")
            .with_title("Carrion Overlay");

        let window = match event_loop.create_window(attrs) {
            Ok(w) => Arc::new(w),
            Err(e) => {
                error!("Failed to create overlay widnow: {e}");
                event_loop.exit();
                return;
            }
        };

        window.set_visible(false);
        OVERLAY_STATE.lock().unwrap().kb_grab = KeyboardGrab::new(&window);

        let state = egui_winit::State::new(
            self.egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &window,
            None,
            None,
            None,
        );

        let mut fonts = FontDefinitions::default();

        let bytes = std::fs::read(&CONFIG.lock().unwrap().font_path).unwrap();

        fonts.font_data.insert(
            "default".into(),
            Arc::new(FontData::from_owned(bytes))
        );

        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "default".into());
        self.egui_ctx.set_fonts(fonts);



        let mut painter = pollster::block_on(Painter::new(
            self.egui_ctx.clone(),
            WgpuConfiguration::default(),
            true,
            RendererOptions::default()
        ));
        pollster::block_on(painter.set_window(egui::ViewportId::ROOT, Some(window.clone()))).unwrap();

        self.painter = Some(painter);
        self.egui_state = Some(state);

        window.request_redraw();
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    )
    {
        let (Some(window), Some(state), Some(painter)) = (&self.window, &mut self.egui_state, &mut self.painter) else {
            return;
        };

        let r = state.on_window_event(window, &event);
        if r.repaint {
            window.request_redraw();
        }

        match event {
            WindowEvent::CloseRequested => {
                debug!("Close request ignored");
            }
            WindowEvent::Destroyed => {
                error!("Window destroyed");
                event_loop.exit()
            }
            WindowEvent::RedrawRequested => {
                let mut st = OVERLAY_STATE.lock().unwrap();
                if st.pending_grab {
                    st.pending_grab = false;
                    if let Some(g) = &st.kb_grab {
                        let _ = window.set_cursor_hittest(st.visible);
                        g.acquire()
                    }
                }
                drop(st);

                let raw = state.take_egui_input(window);
                let mut out = self.egui_ctx.run_ui(raw, |ctx| {
                    let p = ctx.layer_painter(egui::LayerId::new(egui::Order::Background, egui::Id::unique("overlay")));
                    p.text(
                        egui::pos2(20., 20.),
                        egui::Align2::LEFT_TOP,
                        "Hello, Carrion!",
                        egui::FontId::proportional(14.),
                        egui::Color32::WHITE
                    );

                    if ctx.button("Close win").clicked() {
                        event_loop.exit();
                    }
                });

                state.handle_platform_output(window, out.platform_output);
                let prims = self.egui_ctx.tessellate(out.shapes, out.pixels_per_point);
                painter.paint_and_update_textures(
                    ViewportId::ROOT,
                    out.pixels_per_point,
                    [0., 0., 0., 0.],
                    &prims,
                    &mut out.textures_delta,
                    vec![],
                    window
                );

            }
            WindowEvent::ModifiersChanged(m) => self.mods = m.state(),
            WindowEvent::KeyboardInput { event: ref key, .. } => {
                if key.state == ElementState::Pressed && !key.repeat {
                    if let PhysicalKey::Code(code) = key.physical_key {
                        if (code == KeyCode::KeyG && self.mods.super_key())
                            || (code == KeyCode::Escape && OVERLAY_STATE.lock().unwrap().visible) {
                            let mut st = OVERLAY_STATE.lock().unwrap();
                            let v = !st.visible;

                            st.set_visible(window, v);
                            let _ = window.set_cursor_hittest(v);
                            
                            if let Some(kb_grab) = &st.kb_grab {
                                if v {
                                    kb_grab.acquire();
                                } else {
                                    kb_grab.release();
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }


    fn user_event(&mut self, _: &ActiveEventLoop, _: ()) {
        if let Some(w) = &self.window {
            let mut st = OVERLAY_STATE.lock().unwrap();
            let v = !st.visible;

            st.set_visible(w, v);
            let _ = w.set_cursor_hittest(v);
            
            if let Some(kb_grab) = &st.kb_grab {
                if v {
                    kb_grab.acquire();
                } else {
                    kb_grab.release();
                }
            }
        }
    }
}