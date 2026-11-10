//! Building blocks for monospaced data tables (header cells, value cells, tree guides).

use crate::ui::theme::AppColors;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

/// Base cell: fixed width, or flexible (shares the remaining space) when `width` is `None`.
/// Returns a plain `Div` so callers can keep styling it.
pub fn cell(width: Option<f32>) -> Div {
    let base = div().h_full().px_3().flex().items_center();
    match width {
        Some(w) => base.w(px(w)).flex_shrink_0(),
        None => base.flex_1().min_w(px(0.)),
    }
}

/// Header cell with an optional small unit next to the label ("memoria MIB").
#[derive(IntoElement)]
pub struct HeaderCell {
    label: SharedString,
    unit: Option<SharedString>,
    width: Option<f32>,
}

impl HeaderCell {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            unit: None,
            width: None,
        }
    }

    pub fn unit(mut self, unit: impl Into<SharedString>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }
}

impl RenderOnce for HeaderCell {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = AppColors::get(cx);
        cell(self.width)
            .gap(px(6.))
            .child(self.label)
            .when_some(self.unit, |el, unit| {
                el.child(div().text_size(px(10.5)).text_color(colors.faint).child(unit))
            })
    }
}

/// Numeric value in the "number" color; italic `NULL` when there is no value,
/// or "…" while it is still being measured (`pending`).
#[derive(IntoElement)]
pub struct ValueCell {
    width: f32,
    value: Option<SharedString>,
    pending: bool,
}

impl ValueCell {
    pub fn new(width: f32, value: Option<impl Into<SharedString>>) -> Self {
        Self {
            width,
            value: value.map(Into::into),
            pending: false,
        }
    }

    pub fn pending(mut self, pending: bool) -> Self {
        self.pending = pending;
        self
    }
}

impl RenderOnce for ValueCell {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = AppColors::get(cx);
        let base = cell(Some(self.width)).text_color(colors.number);
        match self.value {
            Some(v) => base.child(v),
            None if self.pending => base.text_color(colors.faint).child("…"),
            None => base.italic().child("NULL"),
        }
    }
}

/// Vertical guide line that marks a row as a child of the group above it.
#[derive(IntoElement)]
pub struct TreeGuide;

impl RenderOnce for TreeGuide {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = AppColors::get(cx);
        div()
            .h_full()
            .w(px(14.))
            .ml(px(10.))
            .mr(px(12.))
            .flex_shrink_0()
            .border_l_1()
            .border_color(colors.guide)
    }
}
