use abi_stable::{RMut, sabi_trait};

use crate::types::window::{Label, Response};


pub type UiHandle<'a> = UiApi_TO<'a, RMut<'a, ()>>;

#[sabi_trait]
pub trait UiApi {
    fn label(&mut self, l: Label) -> Response;
}


