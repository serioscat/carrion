#[cfg(feature = "egui")]
macro_rules! convert_bitflags {
    ($value:expr, $from:path => $to:path { $($flag:ident),* $(,)? }) => {{
        let mut result = <$to>::empty();
        $(
            if $value.contains(<$from>::$flag) {
                result |= <$to>::$flag;
            }
        )*

        result
    }};
}

pub mod align;
pub mod layer_id;
pub mod order;
pub mod response;
pub mod sense;
pub mod text_wrap_mode;
pub mod theme;
pub mod widgets;

pub use align::*;
pub use layer_id::*;
pub use order::*;
pub use response::*;
pub use sense::*;
pub use text_wrap_mode::*;
pub use theme::*;
pub use widgets::*;
