use abi_stable::sabi_trait::TD_Opaque;
use carrion_api::core::window::UiApi_TO;
use egui::{FontData, FontDefinitions, ViewportId};
use egui_wgpu::{RendererOptions, WgpuConfiguration};
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, ModifiersState, PhysicalKey},
    platform::x11::WindowAttributesExtX11,
    window::{Window, WindowAttributes},
};

use std::sync::Arc;
use tracing::error;

use crate::{CONFIG, OVERLAY_DATA, core::window::ui::UiApiImpl, types::kb_grab::KeyboardGrab};

pub struct Overlay {
    window: Option<Arc<Window>>,
    egui_ctx: egui::Context,
    egui_state: Option<egui_winit::State>,
    painter: Option<egui_wgpu::winit::Painter>,

    mods: ModifiersState,
}

impl Overlay {
    pub fn new() -> Self {
        Self {
            window: None,
            egui_ctx: egui::Context::default(),
            egui_state: None,
            painter: None,

            mods: ModifiersState::empty(),
        }
    }
}

impl ApplicationHandler for Overlay {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let monitor = event_loop
            .primary_monitor()
            .or_else(|| event_loop.available_monitors().next());

        let (size, pos) = monitor
            .map(|m| (m.size(), m.position()))
            .unwrap_or((PhysicalSize::new(1920, 1080), PhysicalPosition::new(0, 0)));

        let attrs = WindowAttributes::default()
            .with_inner_size(size)
            .with_position(pos)
            .with_transparent(true)
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
        OVERLAY_DATA.lock().kb_grab = KeyboardGrab::new(&window);

        let state = egui_winit::State::new(
            self.egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &window,
            None,
            None,
            None,
        );

        let mut fonts = FontDefinitions::default();

        let bytes = std::fs::read(&CONFIG.lock().font_path).unwrap();

        fonts
            .font_data
            .insert("default".into(), Arc::new(FontData::from_owned(bytes)));

        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "default".into());
        self.egui_ctx.set_fonts(fonts);

        let mut painter = pollster::block_on(egui_wgpu::winit::Painter::new(
            self.egui_ctx.clone(),
            WgpuConfiguration::default(),
            true,
            RendererOptions::default(),
        ));
        pollster::block_on(painter.set_window(egui::ViewportId::ROOT, Some(window.clone())))
            .unwrap();

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
    ) {
        let (Some(window), Some(state), Some(painter)) =
            (&self.window, &mut self.egui_state, &mut self.painter)
        else {
            return;
        };

        let r = state.on_window_event(window, &event);
        if r.repaint {
            window.request_redraw();
        }

        match event {
            WindowEvent::RedrawRequested => {
                let mut data = OVERLAY_DATA.lock();
                if data.pending_grab {
                    data.pending_grab = false;
                    if let Some(kb_grab) = &data.kb_grab {
                        let _ = window.set_cursor_hittest(data.visible);
                        kb_grab.acquire()
                    }
                }

                let (windows, handlers) = (data.windows.clone(), data.render_handlers.clone());

                drop(data);

                let raw = state.take_egui_input(window);
                let mut out = self.egui_ctx.run_ui(raw, |ctx| {
                    ctx.layer_painter(egui::LayerId::background()).rect_filled(
                        ctx.content_rect(),
                        0.,
                        egui::Color32::from_black_alpha(140),
                    );

                    if ctx.button("Exit from overlay").clicked() {
                        event_loop.exit();
                    }

                    for window in windows.iter() {
                        let (_plugin_id, windows) = window;

                        for w in windows {
                            let (_window_id, win_data) = w;
                            let mut data = win_data.lock();

                            egui::Window::new(data.title.clone())
                                .resizable(data.resizable)
                                .default_pos([data.position.x, data.position.y])
                                .default_size(data.size)
                                .open(&mut data.open)
                                .show(&ctx, |ui| {
                                    let mut api = UiApiImpl { ui };
                                    let mut handle = UiApi_TO::from_ptr(&mut api, TD_Opaque);

                                    for handler in handlers.iter() {
                                        handler._render(&mut handle);
                                    }
                                });
                        }
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
                    window,
                );
            }
            WindowEvent::ModifiersChanged(m) => self.mods = m.state(),
            WindowEvent::KeyboardInput { event: ref key, .. } => {
                if key.state == ElementState::Pressed && !key.repeat {
                    let PhysicalKey::Code(code) = key.physical_key else {
                        return;
                    };
                    let is_visible = OVERLAY_DATA.lock().visible;

                    if (code == KeyCode::KeyG && self.mods.super_key())
                        || (code == KeyCode::Escape && is_visible)
                    {
                        let visible = !is_visible;

                        let mut data = OVERLAY_DATA.lock();
                        data.set_visible(window, visible);
                        let _ = window.set_cursor_hittest(visible);

                        let Some(kb_grab) = data.kb_grab.clone() else { return };
                        drop(data);

                        if visible {
                            kb_grab.acquire();
                        } else {
                            kb_grab.release();
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn user_event(&mut self, _: &ActiveEventLoop, _: ()) {
        let Some(w) = &self.window else { return };

        let mut data = OVERLAY_DATA.lock();
        let v = !data.visible;

        data.set_visible(w, v);
        let _ = w.set_cursor_hittest(v);

        let Some(kb_grab) = data.kb_grab.clone() else { return };
        drop(data);
        if v {
            kb_grab.acquire();
        } else {
            kb_grab.release();
        }
    }
}
