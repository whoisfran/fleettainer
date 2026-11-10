use crate::ui::theme::AppColors;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;
use std::rc::Rc;

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// Pill-shaped tab with an optional counter ("Running 3").
/// Stateless: the parent says whether it is active and reacts to `on_click`.
#[derive(IntoElement)]
pub struct FilterTab {
    id: ElementId,
    label: SharedString,
    count: Option<usize>,
    active: bool,
    on_click: Option<ClickHandler>,
}

impl FilterTab {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            count: None,
            active: false,
            on_click: None,
        }
    }

    pub fn count(mut self, count: usize) -> Self {
        self.count = Some(count);
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for FilterTab {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = AppColors::get(cx);

        div()
            .id(self.id)
            .h(px(28.))
            .px_3()
            .flex()
            .items_center()
            .gap(px(6.))
            .rounded_md()
            .text_sm()
            .cursor_pointer()
            .text_color(if self.active { colors.text } else { colors.muted })
            .when(self.active, |el| el.bg(colors.active).font_weight(FontWeight::MEDIUM))
            .hover(move |s| s.text_color(colors.text))
            .child(self.label)
            .when_some(self.count, |el, count| {
                el.child(div().text_color(colors.muted).child(count.to_string()))
            })
            .when_some(self.on_click, |el, handler| {
                el.on_click(move |event, window, cx| handler(event, window, cx))
            })
    }
}
