#[repr(C)]
#[derive(abi_stable::StableAbi, Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sense(u8);

bitflags::bitflags! {
    impl Sense: u8 {
        const HOVER = 0;
        const CLICK = 1 << 0;
        const DRAG = 1 << 1;
        const FOCUSABLE = 1 << 2;
    }
}

impl Sense {
    #[inline]
    pub fn focusable_noninteractive() -> Self {
        Self::FOCUSABLE
    }

    #[inline]
    pub fn click() -> Self {
        Self::CLICK | Self::FOCUSABLE
    }

    #[inline]
    pub fn drag() -> Self {
        Self::DRAG | Self::FOCUSABLE
    }

    #[inline]
    pub fn click_and_drag() -> Self {
        Self::CLICK | Self::FOCUSABLE | Self::DRAG
    }

    #[inline]
    pub fn interactive(&self) -> bool {
        self.intersects(Self::CLICK | Self::DRAG)
    }

    #[inline]
    pub fn senses_click(&self) -> bool {
        self.contains(Self::CLICK)
    }

    #[inline]
    pub fn senses_drag(&self) -> bool {
        self.contains(Self::DRAG)
    }

    #[inline]
    pub fn is_focusable(&self) -> bool {
        self.contains(Self::FOCUSABLE)
    }
}

#[cfg(feature = "egui")]
impl From<Sense> for egui::Sense {
    fn from(value: Sense) -> Self {
        convert_bitflags!(value, Sense => egui::Sense {
            HOVER, CLICK, DRAG, FOCUSABLE
        })
    }
}

#[cfg(feature = "egui")]
impl From<egui::Sense> for Sense {
    fn from(value: egui::Sense) -> Self {
        convert_bitflags!(value, egui::Sense => Sense {
            HOVER, CLICK, DRAG, FOCUSABLE
        })
    }
}
