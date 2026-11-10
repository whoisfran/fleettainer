use crate::ui::components::app_logo::AppLogo;
use crate::ui::theme::AppColors;
use gpui_kit::component::{TitleBar, h_flex, v_flex};
use gpui_kit::*;

/// Layout principal: barra de título del kit (con pestañas) + contenido.
#[derive(IntoElement)]
pub struct AppShell {
    tabs: Option<AnyElement>,
    children: Vec<AnyElement>,
}

impl AppShell {
    pub fn new() -> Self {
        Self {
            tabs: None,
            children: Vec::new(),
        }
    }

    pub fn tabs(mut self, tabs: impl IntoElement) -> Self {
        self.tabs = Some(tabs.into_any_element());
        self
    }
}

impl ParentElement for AppShell {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AppShell {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let colors = AppColors::get(cx);

        v_flex()
            .size_full()
            .text_color(colors.text)
            // Fondo con un resplandor suave hacia la esquina superior derecha.
            .bg(linear_gradient(
                225.,
                linear_color_stop(colors.glow, 0.),
                linear_color_stop(colors.bg, 0.55),
            ))
            // TitleBar del kit: controles de ventana nativos en macOS, propios en Linux/Windows.
            .child(
                TitleBar::new().child(
                    h_flex()
                        .w_full()
                        .pr_3()
                        .justify_between()
                        .children(self.tabs)
                        .child(
                            h_flex()
                                .gap_2()
                                .text_xs()
                                .text_color(colors.muted)
                                .child(AppLogo::new())
                                .child("Fleettainer"),
                        ),
                ),
            )
            .child(v_flex().flex_1().min_h(px(0.)).children(self.children))
    }
}
