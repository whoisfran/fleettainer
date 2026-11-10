use crate::docker::DockerClient;
use crate::state::containers::ContainersStore;
use crate::ui::layout::{app_shell::AppShell, section_tabs::SectionTabs};
use crate::ui::theme::AppColors;
use crate::views::containers::ContainersView;
use gpui_kit::*;

pub struct RootView {
    section: usize,
    containers_view: Entity<ContainersView>,
}

impl RootView {
    pub fn new(client: DockerClient, window: &mut Window, cx: &mut Context<Self>) -> Self {
        // RootView crea el store (es su dueño) y se lo pasa a la vista.
        let store = cx.new(|_| ContainersStore::new(client));
        store.update(cx, |store, cx| store.reload(cx));

        let containers_view = cx.new(|cx| ContainersView::new(store, window, cx));

        Self {
            section: 0,
            containers_view,
        }
    }
}

impl Render for RootView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();
        let tabs = SectionTabs::new(
            vec![
                "Contenedores".into(),
            ],
            self.section,
            move |i, _, cx| {
                entity.update(cx, |this, cx| {
                    this.section = i;
                    cx.notify();
                })
            },
        );

        let content = match self.section {
            0 => self.containers_view.clone().into_any_element(),
            // Fase 1: solo contenedores. El resto de secciones llegará después.
            _ => div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .text_color(AppColors::get(cx).muted)
                .child("Próximamente")
                .into_any_element(),
        };

        AppShell::new().tabs(tabs).child(content)
    }
}
