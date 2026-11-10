use gpui_kit::*;

/// Small filled circle used as a status indicator.
/// The caller picks the color from the theme (e.g. `colors.running`).
#[derive(IntoElement)]
pub struct Dot {
    color: Hsla,
    size: Pixels,
}

impl Dot {
    pub fn new(color: Hsla) -> Self {
        Self { color, size: px(6.) }
    }

    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }
}

impl RenderOnce for Dot {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .flex_shrink_0()
            .size(self.size)
            .rounded_full()
            .bg(self.color)
    }
}
