use crate::{
    core::{event::EventFn, window::render_fn::RenderFn, window::ui::UiHandle},
    event::Event,
};

pub(crate) struct FnHandler<F>(pub(crate) F);

impl<F: Fn(Event) + Send + Sync + 'static> EventFn for FnHandler<F> {
    fn _call(&self, e: Event) {
        (self.0)(e)
    }
}

impl<F: Fn(&mut UiHandle<'_>) + Send + Sync + 'static> RenderFn for FnHandler<F> {
    fn _render(&self, ui: &mut UiHandle<'_>) {
        (self.0)(ui)
    }
}
