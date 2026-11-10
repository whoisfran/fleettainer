use crate::ui::theme::AppColors;
use gpui_kit::component::h_flex;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;
use std::rc::Rc;

type SelectHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// Pestañas de sección dentro de la barra de título.
/// Sin estado propio: recibe la activa y avisa con `on_select`.
#[derive(IntoElement)]
pub struct SectionTabs {
    items: Vec<SharedString>,
    selected: usize,
    on_select: SelectHandler,
}

impl SectionTabs {
    pub fn new(
        items: Vec<SharedString>,
        selected: usize,
        on_select: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            items,
            selected,
            on_select: Rc::new(on_select),
        }
    }
}

impl RenderOnce for SectionTabs {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = AppColors::get(cx);

        h_flex()
            .gap(px(2.))
            .children(self.items.into_iter().enumerate().map(|(i, label)| {
                let on_select = self.on_select.clone();
                let active = i == self.selected;
                div()
                    .id(("section", i))
                    .h(px(26.))
                    .px_3()
                    .flex()
                    .items_center()
                    .rounded_md()
                    .text_sm()
                    .cursor_pointer()
                    .text_color(if active { colors.text } else { colors.muted })
                    .when(active, |el| el.bg(colors.active).font_weight(FontWeight::MEDIUM))
                    .hover(move |s| s.text_color(colors.text))
                    .child(label)
                    .on_click(move |_, window, cx| on_select(i, window, cx))
            }))
    }
}
