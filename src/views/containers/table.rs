use super::{ContainersView, entries::Entry};
use crate::ui::components::table::{HeaderCell, cell};
use crate::ui::theme::AppColors;
use gpui_kit::component::{ActiveTheme, h_flex, v_flex};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;
use std::collections::HashSet;

/// Fixed column widths (name and image share the rest).
pub(super) const COL_NUM: f32 = 44.;
pub(super) const COL_STATE: f32 = 140.;
pub(super) const COL_PORTS: f32 = 150.;
pub(super) const COL_CPU: f32 = 80.;
pub(super) const COL_MEM: f32 = 100.;
pub(super) const COL_UPTIME: f32 = 100.;
pub(super) const COL_ACTIONS: f32 = 84.;

impl ContainersView {
    pub(super) fn render_table(
        &self,
        entries: Vec<Entry>,
        busy_ids: &HashSet<String>,
        loading: bool,
        error: Option<String>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        let colors = AppColors::get(cx);
        let mono = cx.theme().mono_font_family.clone();

        let header = h_flex()
            .h(px(38.))
            .flex_shrink_0()
            .border_b_1()
            .border_color(colors.line)
            .text_size(px(12.5))
            .child(cell(Some(COL_NUM)).justify_center().text_color(colors.faint).child("#"))
            .child(HeaderCell::new("Nombre"))
            .child(HeaderCell::new("Imagen").unit("TAG"))
            .child(HeaderCell::new("Estado").width(COL_STATE))
            .child(HeaderCell::new("Puertos").unit("TCP").width(COL_PORTS))
            .child(HeaderCell::new("CPU").unit("%").width(COL_CPU))
            .child(HeaderCell::new("Memoria").unit("MIB").width(COL_MEM))
            .child(HeaderCell::new("Activo").width(COL_UPTIME))
            .child(cell(Some(COL_ACTIONS)).justify_end().text_color(colors.faint).child("acciones"));

        // Flatten groups and singles into rows. Numbering also counts collapsed rows.
        let mut rows: Vec<AnyElement> = Vec::new();
        let mut n = 0;
        for (gix, entry) in entries.iter().enumerate() {
            match entry {
                Entry::Group { project, items } => {
                    rows.push(self.render_group_row(gix, project, items, busy_ids, cx));
                    if self.is_expanded(project) {
                        for c in items {
                            n += 1;
                            rows.push(self.render_row(n, c, true, busy_ids.contains(&c.id), cx));
                        }
                    } else {
                        n += items.len();
                    }
                }
                Entry::Single(c) => {
                    n += 1;
                    rows.push(self.render_row(n, c, false, busy_ids.contains(&c.id), cx));
                }
            }
        }

        let body = if rows.is_empty() && !loading {
            div()
                .p_6()
                .text_color(colors.muted)
                .child("No hay contenedores")
                .into_any_element()
        } else {
            v_flex().children(rows).into_any_element()
        };

        v_flex()
            .flex_1()
            .min_h(px(0.))
            .font_family(mono)
            .text_size(px(13.))
            .child(header)
            .when_some(error, |el, e| {
                el.child(
                    div()
                        .px_4()
                        .py_2()
                        .border_b_1()
                        .border_color(colors.line)
                        .text_color(colors.danger)
                        .child(format!("Docker: {e}")),
                )
            })
            .child(div().id("rows").flex_1().overflow_y_scroll().child(body))
    }
}
