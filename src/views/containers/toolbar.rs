use super::{ContainersView, entries::Filter};
use crate::ui::components::filter_tab::FilterTab;
use crate::ui::theme::AppColors;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Escape, Input};
use gpui_kit::component::{IconName, Sizable, h_flex};
use gpui_kit::*;

impl ContainersView {
    /// Filter tabs on the left; search and reload on the right.
    pub(super) fn render_toolbar(
        &self,
        total: usize,
        running: usize,
        loading: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        let colors = AppColors::get(cx);
        let store = self.store.clone();

        h_flex()
            .h(px(48.))
            .flex_shrink_0()
            .px_3()
            .gap_1()
            .border_b_1()
            .border_color(colors.line)
            .child(self.filter_tab("Todos", total, Filter::All, cx))
            .child(self.filter_tab("En ejecución", running, Filter::Running, cx))
            .child(self.filter_tab("Detenidos", total - running, Filter::Stopped, cx))
            .child(div().flex_1())
            .child(
                div()
                    .w(px(280.))
                    // GPUI only moves focus when you click something focusable,
                    // so clicking the table would leave the input focused.
                    // Drop the focus on a click outside, or with Escape.
                    .on_mouse_down_out(cx.listener(|this, _, window, cx| this.blur_search(window, cx)))
                    .on_action(cx.listener(|this, _: &Escape, window, cx| this.blur_search(window, cx)))
                    .child(
                        Input::new(&self.search)
                            .cleanable(true)
                            .prefix(IconName::Search)
                            .small(),
                    ),
            )
            .child(
                Button::new("reload")
                    .icon(IconName::RefreshCw)
                    .ghost()
                    .small()
                    .loading(loading)
                    .tooltip("Recargar")
                    .on_click(move |_, _, cx| store.update(cx, |s, cx| s.reload(cx))),
            )
    }

    /// Removes focus from the search box (only if it has it, so we
    /// don't steal focus from anything else).
    fn blur_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.search.read(cx).focus_handle(cx).is_focused(window) {
            window.blur(cx);
        }
    }

    fn filter_tab(
        &self,
        label: &'static str,
        count: usize,
        filter: Filter,
        cx: &mut Context<Self>,
    ) -> FilterTab {
        FilterTab::new(label, label)
            .count(count)
            .active(self.filter == filter)
            .on_click(cx.listener(move |this, _, _, cx| {
                this.filter = filter;
                cx.notify();
            }))
    }
}
