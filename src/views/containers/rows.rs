use super::ContainersView;
use super::table::{COL_ACTIONS, COL_CPU, COL_MEM, COL_NUM, COL_PORTS, COL_STATE, COL_UPTIME};
use crate::docker::models::ContainerInfo;
use crate::state::containers::Action;
use crate::ui::components::dot::Dot;
use crate::ui::components::row_actions::RowActions;
use crate::ui::components::status_badge::StatusBadge;
use crate::ui::components::table::{TreeGuide, ValueCell, cell};
use crate::ui::theme::AppColors;
use gpui_kit::component::{Icon, IconName, Sizable, h_flex};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;
use std::collections::HashSet;

impl ContainersView {
    /// Click handler that sends the actions to the store.
    fn run_on_click(
        &self,
        actions: Vec<(String, Action)>,
    ) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static + use<> {
        let store = self.store.clone();
        move |_, _, cx| store.update(cx, |s, cx| s.run_many(actions.clone(), cx))
    }

    /// Clickable area that folds/unfolds a compose group.
    fn group_toggle(&self, id: &'static str, project: &str, cx: &mut Context<Self>) -> Stateful<Div> {
        let project = project.to_string();
        div()
            .id(id)
            .h_full()
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| this.toggle_group(project.clone(), cx)))
    }

    pub(super) fn render_row(
        &self,
        n: usize,
        c: &ContainerInfo,
        in_group: bool,
        busy: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let colors = AppColors::get(cx);
        let is_selected = self.selected.as_ref() == Some(&c.id);
        let running = c.is_running();

        let (action, tip) = if running {
            (Action::Stop, "Detener")
        } else if c.is_paused() {
            (Action::Unpause, "Reanudar")
        } else {
            (Action::Start, "Iniciar")
        };
        let actions = RowActions::new("actions", running)
            .busy(busy)
            .on_toggle(tip, self.run_on_click(vec![(c.id.clone(), action)]))
            .when(running, |a| {
                a.on_restart("Reiniciar", self.run_on_click(vec![(c.id.clone(), Action::Restart)]))
            });

        let id = c.id.clone();
        h_flex()
            .id(("row", n))
            .h(px(40.))
            .flex_shrink_0()
            .border_b_1()
            .border_color(colors.line)
            .when(is_selected, |el| el.bg(colors.select))
            .hover(move |s| s.bg(colors.select))
            .on_click(cx.listener(move |this, _, _, cx| this.select(id.clone(), cx)))
            .child(cell(Some(COL_NUM)).justify_center().text_color(colors.faint).child(n.to_string()))
            .child(
                cell(None)
                    .when(in_group, |el| el.child(TreeGuide))
                    .child(div().min_w(px(0.)).truncate().child(c.name.clone())),
            )
            .child(cell(None).text_color(colors.muted).child(div().truncate().child(c.image.clone())))
            .child(cell(Some(COL_STATE)).child(StatusBadge::new(c.state.clone())))
            .child(ValueCell::new(COL_PORTS, c.ports.clone()))
            .child(ValueCell::new(COL_CPU, c.cpu.map(|v| format!("{v:.2}"))).pending(running))
            .child(ValueCell::new(COL_MEM, c.mem_mib.map(|v| format!("{v:.1}"))).pending(running))
            .child(
                cell(Some(COL_UPTIME))
                    .text_color(colors.muted)
                    .child(c.uptime().unwrap_or_else(|| "—".into())),
            )
            .child(cell(Some(COL_ACTIONS)).justify_end().child(actions))
            .into_any_element()
    }

    /// Compose project row: clicking the chevron or the name folds/unfolds it.
    pub(super) fn render_group_row(
        &self,
        gix: usize,
        project: &str,
        items: &[ContainerInfo],
        busy_ids: &HashSet<String>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let colors = AppColors::get(cx);
        let collapsed = !self.is_expanded(project);
        let running: Vec<&ContainerInfo> = items.iter().filter(|c| c.is_running()).collect();
        let any_running = !running.is_empty();
        let busy = items.iter().any(|c| busy_ids.contains(&c.id));

        // CPU/memory totals of the running ones.
        let cpu: f64 = items.iter().filter_map(|c| c.cpu).sum();
        let mem: f64 = items.iter().filter_map(|c| c.mem_mib).sum();

        // Group action: if anything runs, stop what runs; otherwise start everything.
        let (toggle_actions, tip): (Vec<_>, _) = if any_running {
            (running.iter().map(|c| (c.id.clone(), Action::Stop)).collect(), "Detener todo")
        } else {
            let start = items
                .iter()
                .map(|c| {
                    let a = if c.is_paused() { Action::Unpause } else { Action::Start };
                    (c.id.clone(), a)
                })
                .collect();
            (start, "Iniciar todo")
        };
        let restart_actions: Vec<_> = running.iter().map(|c| (c.id.clone(), Action::Restart)).collect();

        let actions = RowActions::new("actions", any_running)
            .busy(busy)
            .on_toggle(tip, self.run_on_click(toggle_actions))
            .when(any_running, |a| a.on_restart("Reiniciar todo", self.run_on_click(restart_actions)));

        let summary_color = if any_running { colors.running } else { colors.muted };

        h_flex()
            .id(("group", gix))
            .h(px(40.))
            .flex_shrink_0()
            .border_b_1()
            .border_color(colors.line)
            .bg(colors.group)
            // "#" column: the chevron.
            .child(
                self.group_toggle("chevron", project, cx)
                    .w(px(COL_NUM))
                    .flex_shrink_0()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        Icon::new(if collapsed { IconName::ChevronRight } else { IconName::ChevronDown })
                            .xsmall()
                            .text_color(colors.muted),
                    ),
            )
            // "name" column: folder + project name.
            .child(
                self.group_toggle("name", project, cx)
                    .flex_1()
                    .min_w(px(0.))
                    .px_3()
                    .flex()
                    .items_center()
                    .gap(px(10.))
                    .child(
                        Icon::new(if collapsed { IconName::FolderClosed } else { IconName::FolderOpen })
                            .small()
                            .text_color(colors.accent),
                    )
                    .child(div().font_weight(FontWeight::SEMIBOLD).truncate().child(project.to_string())),
            )
            .child(cell(None).text_color(colors.muted).child(format!("compose · {}", items.len())))
            .child(
                cell(Some(COL_STATE))
                    .gap_2()
                    .text_color(summary_color)
                    .child(Dot::new(summary_color))
                    .child(format!("{}/{} running", running.len(), items.len())),
            )
            .child(cell(Some(COL_PORTS)))
            .child(ValueCell::new(COL_CPU, any_running.then(|| format!("{cpu:.2}"))))
            .child(ValueCell::new(COL_MEM, any_running.then(|| format!("{mem:.1}"))))
            .child(
                cell(Some(COL_UPTIME))
                    .text_color(colors.muted)
                    .child(running.first().and_then(|c| c.uptime()).unwrap_or_else(|| "—".into())),
            )
            .child(cell(Some(COL_ACTIONS)).justify_end().child(actions))
            .into_any_element()
    }
}
