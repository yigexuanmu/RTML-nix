use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::config::theme::THEME;
use super::state::OnlineStep;

fn span_styled(text: &str, color: ratatui::style::Color) -> Span<'static> {
    Span::styled(text.to_owned(), Style::default().fg(color))
}

fn line(text: &str) -> Line<'static> {
    Line::from(text.to_owned())
}

fn line_colored(text: &str, color: ratatui::style::Color) -> Line<'static> {
    Line::from(vec![Span::styled(text.to_owned(), Style::default().fg(color))])
}

fn line_accent(text: &str) -> Line<'static> {
    let c = THEME.as_ref().accent();
    Line::from(vec![Span::styled(text.to_owned(), Style::default().fg(c))])
}

fn line_bold(text: String, color: ratatui::style::Color) -> Line<'static> {
    Line::from(vec![Span::styled(text, Style::default().fg(color).add_modifier(Modifier::BOLD))])
}

pub fn popup_rect(frame: Rect) -> Rect {
    let w = 52.min(frame.width.saturating_sub(4));
    let h = 20.min(frame.height.saturating_sub(4));
    frame.centered(Constraint::Length(w), Constraint::Length(h))
}

pub fn render(frame: &mut ratatui::Frame, area: Rect) {
    let state = match super::state::ONLINE_STATE.lock() {
        Ok(s) => s.clone(),
        Err(_) => return,
    };

    let theme = THEME.as_ref();
    let block = Block::default()
        .title(" 联机 (Terracotta) ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.accent()));

    let lines: Vec<Line<'static>> = match &state.step {
        OnlineStep::Menu => menu_lines(),
        OnlineStep::HostInput => host_input_lines(&state),
        OnlineStep::Hosting => status_lines("正在创建房间...", "请确保 Minecraft 已开启并处于多人游戏界面"),
        OnlineStep::HostOk { room_code } => host_ok_lines(room_code),
        OnlineStep::JoinInput => join_input_lines(&state),
        OnlineStep::Joining => status_lines("正在加入房间...", "连接可能需要几秒钟"),
        OnlineStep::Joined { url } => joined_lines(url),
        OnlineStep::Error(msg) => error_lines(msg),
    };

    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(Text::from(lines)).block(block).wrap(Wrap { trim: false }),
        area,
    );
}

fn menu_lines() -> Vec<Line<'static>> {
    vec![
        line_bold("Terracotta 联机".to_string(), THEME.as_ref().text()),
        line(""),
        line("  房主：输入昵称后分享房间码给好友"),
        line("  访客：输入房主的房间码加入联机"),
        line(""),
        Line::from(vec![
            span_styled("  [H]", THEME.as_ref().accent()),
            span_styled(" 成为房主", THEME.as_ref().text()),
            span_styled("  [J]", THEME.as_ref().accent()),
            span_styled(" 加入房间", THEME.as_ref().text()),
            span_styled("  [Esc]", THEME.as_ref().accent()),
            span_styled(" 关闭", THEME.as_ref().text()),
        ]),
    ]
}

fn host_input_lines(state: &super::state::OnlineState) -> Vec<Line<'static>> {
    vec![
        line("输入你的游戏昵称："),
        line(""),
        line_accent(&format!(" > {}", state.player_name)),
        line(""),
        line("  [Enter] 开始  [Esc] 返回"),
    ]
}

fn status_lines(title: &str, msg: &str) -> Vec<Line<'static>> {
    let theme = THEME.as_ref();
    vec![
        line_colored(title, theme.info()),
        line(""),
        line(msg),
        line(""),
        line("  [Esc] 取消"),
    ]
}

fn host_ok_lines(room_code: &str) -> Vec<Line<'static>> {
    let theme = THEME.as_ref();
    vec![
        line_colored("房间已创建！", theme.success()),
        line(""),
        line("房间码："),
        line_bold(room_code.to_owned(), theme.accent()),
        line(""),
        line("将此房间码分享给好友即可联机"),
        line(""),
        line("  [Esc] 关闭并断开"),
    ]
}

fn join_input_lines(state: &super::state::OnlineState) -> Vec<Line<'static>> {
    vec![
        line("输入房主的房间码："),
        line(""),
        line_accent(&format!(" > {}", state.room_code_input)),
        line(""),
        line("格式示例：U/XXXX-XXXX-XXXX-XXXX"),
        line(""),
        line("  [Enter] 加入  [Esc] 返回"),
    ]
}

fn joined_lines(url: &str) -> Vec<Line<'static>> {
    let theme = THEME.as_ref();
    vec![
        line_colored("已加入房间！", theme.success()),
        line(""),
        line("在 Minecraft 多人游戏中连接："),
        line_bold(url.to_owned(), theme.accent()),
        line(""),
        line("  [Esc] 断开"),
    ]
}

fn error_lines(msg: &str) -> Vec<Line<'static>> {
    vec![
        line_colored("出错", THEME.as_ref().error()),
        line(""),
        line(msg),
        line(""),
        line("  [Esc/Enter] 关闭"),
    ]
}