mod render;
mod state;

pub use render::{popup_rect, render};
pub use state::{
    OnlineAction, OnlineState, OnlineStep,
    handle_key, take_action,
};
pub(crate) use state::ONLINE_STATE;