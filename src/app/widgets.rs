use super::{HIGHLIGHT_STYLE, TITLE_STYLE};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::Widget,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ListTitle<'a>(pub(super) &'a str);

impl Widget for ListTitle<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Line::styled(self.0, TITLE_STYLE).render(area, buf);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct RadioOption<'a> {
    pub(super) label: &'a str,
    pub(super) checked: bool,
    pub(super) focused: bool,
}

impl Widget for RadioOption<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let wdgt = MarkedOption {
            mark: if self.checked { "(X)" } else { "( )" },
            label: self.label,
            focused: self.focused,
        };
        wdgt.render(area, buf);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct MultiOption<'a> {
    pub(super) label: &'a str,
    pub(super) checked: bool,
    pub(super) focused: bool,
}

impl Widget for MultiOption<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let wdgt = MarkedOption {
            mark: if self.checked { "[X]" } else { "[ ]" },
            label: self.label,
            focused: self.focused,
        };
        wdgt.render(area, buf);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MarkedOption<'a> {
    mark: &'a str,
    label: &'a str,
    focused: bool,
}

impl Widget for MarkedOption<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut line = Line::from(format!("{} {}", self.mark, self.label));
        if self.focused {
            line = line.style(HIGHLIGHT_STYLE);
        }
        line.render(area, buf);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct OkButton(pub(super) bool);

impl Widget for OkButton {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut wdgt = Span::from("<OK>");
        if self.0 {
            wdgt = wdgt.style(HIGHLIGHT_STYLE);
        }
        wdgt.into_centered_line().render(area, buf);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct CancelButton(pub(super) bool);

impl Widget for CancelButton {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut wdgt = Span::from("<Cancel>");
        if self.0 {
            wdgt = wdgt.style(HIGHLIGHT_STYLE);
        }
        wdgt.into_centered_line().render(area, buf);
    }
}
