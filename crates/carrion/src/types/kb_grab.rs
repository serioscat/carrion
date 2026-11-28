use carrion_api::event::{Event, KeyboardEvent, SystemEvent};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};
use std::{ffi::c_void, sync::Arc};
use x11_dl::xlib::{self, Xlib};

use crate::send_event;

pub struct KeyboardGrab {
    xlib: Xlib,
    display: *mut c_void,
    xid: u64,
}

impl KeyboardGrab {
    pub fn new(window: &winit::window::Window) -> Option<Arc<Self>> {
        let display = match window.display_handle().ok()?.as_raw() {
            RawDisplayHandle::Xlib(d) => d.display?.as_ptr(),
            _ => return None,
        };
        let xid = match window.window_handle().ok()?.as_raw() {
            RawWindowHandle::Xlib(h) => h.window,
            _ => return None,
        };
        Some(Arc::new(Self {
            xlib: Xlib::open().ok()?,
            display,
            xid,
        }))
    }

    pub fn acquire(&self) {
        unsafe {
            let d = self.display as *mut xlib::Display;
            (self.xlib.XSync)(d, 0);

            (self.xlib.XGrabKeyboard)(
                d,
                self.xid,
                1,
                xlib::GrabModeAsync,
                xlib::GrabModeAsync,
                xlib::CurrentTime,
            );
            (self.xlib.XSync)(d, 0);

            (self.xlib.XSetInputFocus)(d, self.xid, xlib::RevertToParent, xlib::CurrentTime);
            (self.xlib.XFlush)(d);

            send_event(Event::System(SystemEvent::Keyboard(KeyboardEvent::Grabbed)));
        }
    }

    pub fn release(&self) {
        unsafe {
            let d = self.display as *mut xlib::Display;
            (self.xlib.XUngrabKeyboard)(d, xlib::CurrentTime);
            (self.xlib.XFlush)(d);

            send_event(Event::System(SystemEvent::Keyboard(
                KeyboardEvent::Ungrabbed,
            )));
        }
    }
}

unsafe impl Send for KeyboardGrab {}
unsafe impl Sync for KeyboardGrab {}