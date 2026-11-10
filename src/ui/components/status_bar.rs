use crate::ui::components::dot::Dot;
use crate::ui::theme::AppColors;
use gpui_kit::component::h_flex;
use gpui_kit::*;

/// Bottom bar: connection indicator + label on the left, children on the right.
#[derive(IntoElement)]
pub struct StatusBar {
    label: SharedString,
    connected: bool,
    children: Vec<AnyElement>,
}

impl StatusBar {
    pub fn new(label: impl Into<SharedString>, connected: bool) -> Self {
        Self {
            label: label.into(),
            connected,
            children: Vec::new(),
        }
    }
}

impl ParentElement for StatusBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for StatusBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = AppColors::get(cx);
        let dot_color = if self.connected { colors.running } else { colors.danger };

        h_flex()
            .h(px(32.))
            .flex_shrink_0()
            .px_3()
            .gap_4()
            .border_t_1()
            .border_color(colors.line)
            .bg(colors.panel)
            .text_xs()
            .text_color(colors.muted)
            .child(
                h_flex()
                    .gap_2()
                    .text_color(colors.text)
                    .child(Dot::new(dot_color).size(px(7.)))
                    .child(self.label),
            )
            .child(div().flex_1())
            .children(self.children)
    }
}
