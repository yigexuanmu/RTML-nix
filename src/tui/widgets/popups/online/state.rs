use std::sync::LazyLock;
use std::sync::{Arc, Mutex};

use crossterm::event::{KeyCode, KeyEvent};

use crate::tui::widgets::instances;

#[derive(Debug, Clone, PartialEq)]
pub enum OnlineStep {
    Menu,
    HostInput,
    Hosting,
    HostOk { room_code: String },
    JoinInput,
    Joining,
    Joined { url: String },
    Error(String),
}

impl Default for OnlineStep {
    fn default() -> Self {
        Self::Menu
    }
}

#[derive(Debug, Clone)]
pub struct OnlineState {
    pub step: OnlineStep,
    pub player_name: String,
    pub room_code_input: String,
    pub state_json: String,
}

impl Default for OnlineState {
    fn default() -> Self {
        Self {
            step: OnlineStep::Menu,
            player_name: "Player".to_owned(),
            room_code_input: String::new(),
            state_json: String::new(),
        }
    }
}

impl OnlineState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

pub(crate) static ONLINE_STATE: LazyLock<Arc<Mutex<OnlineState>>> =
    LazyLock::new(|| Arc::new(Mutex::new(OnlineState::default())));

pub(crate) static ONLINE_ACTION: LazyLock<Arc<Mutex<Option<OnlineAction>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(None)));

#[derive(Debug, Clone)]
pub enum OnlineAction {
    StartHost { player: String },
    JoinRoom { room_code: String },
    Disconnect,
}

pub fn handle_key(key_event: &KeyEvent, instances_state: &mut instances::State) {
    let mut state = match ONLINE_STATE.lock() {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("Online state lock poisoned: {}", e);
            instances_state.show_online_popup = false;
            return;
        }
    };

    match &state.step {
        OnlineStep::Menu => match key_event.code {
            KeyCode::Char('h') | KeyCode::Char('H') => {
                state.step = OnlineStep::HostInput;
            }
            KeyCode::Char('j') | KeyCode::Char('J') => {
                state.step = OnlineStep::JoinInput;
            }
            KeyCode::Esc => {
                state.reset();
                instances_state.show_online_popup = false;
            }
            _ => {}
        },
        OnlineStep::HostInput => match key_event.code {
            KeyCode::Esc => {
                state.step = OnlineStep::Menu;
            }
            KeyCode::Enter => {
                let player = state.player_name.clone();
                let action = OnlineAction::StartHost { player };
                if let Ok(mut slot) = ONLINE_ACTION.lock() {
                    *slot = Some(action);
                }
                state.step = OnlineStep::Hosting;
            }
            KeyCode::Char(c) => {
                if !c.is_control() {
                    state.player_name.push(c);
                }
            }
            KeyCode::Backspace => {
                state.player_name.pop();
            }
            _ => {}
        },
        OnlineStep::JoinInput => match key_event.code {
            KeyCode::Esc => {
                state.step = OnlineStep::Menu;
            }
            KeyCode::Enter => {
                let code = state.room_code_input.clone();
                let action = OnlineAction::JoinRoom { room_code: code };
                if let Ok(mut slot) = ONLINE_ACTION.lock() {
                    *slot = Some(action);
                }
                state.step = OnlineStep::Joining;
            }
            KeyCode::Char(c) => {
                if !c.is_control() {
                    state.room_code_input.push(c);
                }
            }
            KeyCode::Backspace => {
                state.room_code_input.pop();
            }
            _ => {}
        },
        OnlineStep::Hosting | OnlineStep::Joining | OnlineStep::HostOk { .. } | OnlineStep::Joined { .. } => {
            match key_event.code {
                KeyCode::Esc => {
                    if let Ok(mut slot) = ONLINE_ACTION.lock() {
                        *slot = Some(OnlineAction::Disconnect);
                    }
                    state.reset();
                    instances_state.show_online_popup = false;
                }
                _ => {}
            }
        }
        OnlineStep::Error(_) => {
            match key_event.code {
                KeyCode::Esc | KeyCode::Enter | KeyCode::Char(' ') => {
                    state.reset();
                    instances_state.show_online_popup = false;
                }
                _ => {}
            }
        }
    }
}

pub fn take_action() -> Option<OnlineAction> {
    match ONLINE_ACTION.lock() {
        Ok(mut slot) => slot.take(),
        Err(_) => None,
    }
}