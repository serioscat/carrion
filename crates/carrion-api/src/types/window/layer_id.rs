use std::num::NonZeroU64;

use crate::types::window::Order;

#[repr(C)]
#[derive(abi_stable::StableAbi, Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LayerId {
    pub order: Order,
    pub id: NonZeroU64,
}

#[cfg(feature = "egui")]
impl From<LayerId> for egui::LayerId {
    fn from(value: LayerId) -> Self {
        egui::LayerId {
            order: egui::Order::from(value.order),
            id: unsafe { egui::Id::from_high_entropy_bits(value.id.get()) },
        }
    }
}

#[cfg(feature = "egui")]
impl From<egui::LayerId> for LayerId {
    fn from(value: egui::LayerId) -> Self {
        LayerId {
            order: Order::from(value.order),
            id: NonZeroU64::new(value.id.value()).unwrap(),
        }
    }
}
