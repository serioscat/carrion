#![allow(non_local_definitions)]

use abi_stable::{sabi_trait, std_types::RBox};

#[sabi_trait]
pub trait MathApi: Send + Sync {
    fn add(&self, a: i32, b: i32) -> i32;
}

pub type Math = MathApi_TO<'static, RBox<()>>;