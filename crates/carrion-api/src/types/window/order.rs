#[repr(u8)]
#[derive(abi_stable::StableAbi, Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Order {
    Background,
    Middle,
    Foreground,
    Tooltip,
    Debug,
}

#[cfg(feature = "egui")]
impl From<Order> for egui::Order {
    fn from(value: Order) -> Self {
        match value {
            Order::Background => egui::Order::Background,
            Order::Middle => egui::Order::Middle,
            Order::Foreground => egui::Order::Foreground,
            Order::Tooltip => egui::Order::Tooltip,
            Order::Debug => egui::Order::Debug,
        }
    }
}

#[cfg(feature = "egui")]
impl From<egui::Order> for Order {
    fn from(value: egui::Order) -> Self {
        match value {
            egui::Order::Background => Order::Background,
            egui::Order::Middle => Order::Middle,
            egui::Order::Foreground => Order::Foreground,
            egui::Order::Tooltip => Order::Tooltip,
            egui::Order::Debug => Order::Debug,
        }
    }
}
