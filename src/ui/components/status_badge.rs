use crate::ui::components::dot::Dot;
use crate::ui::theme::AppColors;
use gpui_kit::component::h_flex;
use gpui_kit::*;

/// Colored dot + state text ("● running"), using the state's theme color.
#[derive(IntoElement)]
pub struct StatusBadge {
    state: SharedString,
}

impl StatusBadge {
    pub fn new(state: impl Into<SharedString>) -> Self {
        Self { state: state.into() }
    }
}

impl RenderOnce for StatusBadge {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = AppColors::get(cx).for_state(&self.state);
        h_flex()
            .gap_2()
            .text_color(color)
            .child(Dot::new(color))
            .child(self.state)
    }
}
