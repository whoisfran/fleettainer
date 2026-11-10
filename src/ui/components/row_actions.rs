use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Disableable, IconName, Sizable, h_flex};
use gpui_kit::*;
use std::rc::Rc;

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// Pair of icon buttons for a table row: toggle (stop ■ / start ▶) and restart.
/// A button without a handler is shown disabled. Clicks don't bubble up,
/// so pressing a button doesn't also select the row.
#[derive(IntoElement)]
pub struct RowActions {
    id: ElementId,
    running: bool,
    busy: bool,
    toggle: Option<(SharedString, ClickHandler)>,
    restart: Option<(SharedString, ClickHandler)>,
}

impl RowActions {
    /// `running` decides the toggle icon: ■ when running, ▶ otherwise.
    pub fn new(id: impl Into<ElementId>, running: bool) -> Self {
        Self {
            id: id.into(),
            running,
            busy: false,
            toggle: None,
            restart: None,
        }
    }

    /// While busy both buttons are disabled and the toggle shows a spinner.
    pub fn busy(mut self, busy: bool) -> Self {
        self.busy = busy;
        self
    }

    pub fn on_toggle(
        mut self,
        tooltip: impl Into<SharedString>,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.toggle = Some((tooltip.into(), Rc::new(handler)));
        self
    }

    pub fn on_restart(
        mut self,
        tooltip: impl Into<SharedString>,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.restart = Some((tooltip.into(), Rc::new(handler)));
        self
    }
}

impl RenderOnce for RowActions {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let toggle_icon = if self.running { IconName::Square } else { IconName::Play };

        h_flex()
            .id(self.id)
            .gap(px(2.))
            .child(action_button("toggle", toggle_icon, self.toggle, self.busy).loading(self.busy))
            .child(action_button("restart", IconName::RotateCw, self.restart, self.busy))
    }
}

fn action_button(
    id: &'static str,
    icon: IconName,
    action: Option<(SharedString, ClickHandler)>,
    busy: bool,
) -> Button {
    let button = Button::new(id)
        .icon(icon)
        .ghost()
        .xsmall()
        .disabled(busy || action.is_none());

    match action {
        Some((tooltip, handler)) => button.tooltip(tooltip).on_click(move |event, window, cx| {
            cx.stop_propagation();
            handler(event, window, cx);
        }),
        None => button,
    }
}
