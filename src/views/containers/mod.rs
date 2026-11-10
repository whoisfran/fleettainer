//! Container list: filters + search, table grouped by compose project, and status bar.
//!
//! Split by responsibility:
//! - `entries`: pure logic (filter, grouping). No UI.
//! - `toolbar`: filter tabs + search + reload.
//! - `table`:   header and row flattening.
//! - `rows`:    container row and compose group row.

mod entries;
mod rows;
mod table;
mod toolbar;

use crate::docker::{DockerClient, models::ContainerInfo};
use crate::state::containers::ContainersStore;
use crate::ui::components::status_bar::StatusBar;
use entries::{Filter, build_entries};
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::component::v_flex;
use gpui_kit::*;
use std::collections::HashSet;

/// Data lives in the store; this view only holds UI state.
pub struct ContainersView {
    store: Entity<ContainersStore>,
    search: Entity<InputState>,
    filter: Filter,
    selected: Option<String>,
    /// Compose projects the user has expanded. Groups start collapsed,
    /// so a project not in this set is shown folded.
    expanded: HashSet<String>,
    _subscriptions: Vec<Subscription>,
}

impl ContainersView {
    pub fn new(store: Entity<ContainersStore>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search = cx.new(|cx| {
            InputState::new(window, cx).placeholder("Buscar por nombre, imagen o proyecto…")
        });

        let _subscriptions = vec![
            cx.observe(&store, |_, _, cx| cx.notify()),
            cx.subscribe(&search, |_, _, event: &InputEvent, cx| {
                if let InputEvent::Change = event {
                    cx.notify();
                }
            }),
        ];

        Self {
            store,
            search,
            filter: Filter::All,
            selected: None,
            expanded: HashSet::new(),
            _subscriptions,
        }
    }

    /// Rows that pass the search box and the filter tab.
    fn visible_rows(&self, cx: &App) -> Vec<ContainerInfo> {
        let query = self.search.read(cx).value().to_lowercase();
        self.store
            .read(cx)
            .containers
            .iter()
            .filter(|c| self.filter.accepts(c))
            .filter(|c| {
                query.is_empty()
                    || c.name.to_lowercase().contains(&query)
                    || c.image.to_lowercase().contains(&query)
                    || c.project.as_ref().is_some_and(|p| p.to_lowercase().contains(&query))
            })
            .cloned()
            .collect()
    }

    fn select(&mut self, id: String, cx: &mut Context<Self>) {
        self.selected = Some(id);
        cx.notify();
    }

    fn is_expanded(&self, project: &str) -> bool {
        self.expanded.contains(project)
    }

    fn toggle_group(&mut self, project: String, cx: &mut Context<Self>) {
        if !self.expanded.remove(&project) {
            self.expanded.insert(project);
        }
        cx.notify();
    }
}

impl Render for ContainersView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entries = build_entries(self.visible_rows(cx));

        let (total, running, projects, loading, error, busy) = {
            let store = self.store.read(cx);
            let projects: HashSet<&String> =
                store.containers.iter().filter_map(|c| c.project.as_ref()).collect();
            (
                store.containers.len(),
                store.containers.iter().filter(|c| c.is_running()).count(),
                projects.len(),
                store.loading,
                store.error.clone(),
                store.busy.clone(),
            )
        };
        let connected = error.is_none();

        v_flex()
            .size_full()
            .child(self.render_toolbar(total, running, loading, cx))
            .child(self.render_table(entries, &busy, loading, error, cx))
            .child(
                StatusBar::new(DockerClient::endpoint(), connected)
                    .child(format!(
                        "{total} contenedores · {running} en ejecución · {projects} proyectos"
                    ))
                    .child(format!("v{}", crate::VERSION)),
            )
    }
}
