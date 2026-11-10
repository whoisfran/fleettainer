use gpui_kit::component::{Theme, ThemeMode};
use gpui_kit::*;

/// Las dos propuestas de color del canvas.
#[allow(dead_code)] // solo se usa la variante elegida en PALETTE
#[derive(Clone, Copy, PartialEq)]
pub enum Palette {
    /// Negro cálido con resplandor rojizo (propuesta C).
    Warm,
    /// Azul oscuro (propuesta D).
    Navy,
}

/// Cambia esta constante para probar la otra paleta.
pub const PALETTE: Palette = Palette::Navy;

/// Colores propios de la app que el tema del kit no cubre
/// (estados, números, fondos translúcidos).
#[derive(Clone, Copy)]
pub struct AppColors {
    pub bg: Hsla,
    /// Color hacia el que se funde el fondo (el "resplandor").
    pub glow: Hsla,
    pub bar: Hsla,
    pub panel: Hsla,
    pub line: Hsla,
    pub active: Hsla,
    pub select: Hsla,
    /// Fondo de la fila de un grupo de compose.
    pub group: Hsla,
    /// Línea guía de los hijos de un grupo.
    pub guide: Hsla,
    pub text: Hsla,
    pub muted: Hsla,
    pub faint: Hsla,
    pub number: Hsla,
    pub accent: Hsla,
    pub running: Hsla,
    pub paused: Hsla,
    pub restarting: Hsla,
    pub danger: Hsla,
}

impl Global for AppColors {}

impl AppColors {
    pub fn get(cx: &App) -> Self {
        *cx.global::<Self>()
    }

    /// Color del punto y el texto de estado.
    pub fn for_state(&self, state: &str) -> Hsla {
        match state {
            "running" => self.running,
            "paused" => self.paused,
            "restarting" => self.restarting,
            "dead" => self.danger,
            _ => self.muted,
        }
    }

    fn warm() -> Self {
        Self {
            bg: rgb(0x141216).into(),
            glow: rgb(0x3a1715).into(),
            bar: rgb(0x34181a).into(),
            panel: rgba(0x0a090c61).into(),
            line: rgba(0xffffff12).into(),
            active: rgba(0xffffff14).into(),
            select: rgba(0xffffff0d).into(),
            group: rgba(0xffffff08).into(),
            guide: rgba(0xffffff29).into(),
            text: rgb(0xece8ef).into(),
            muted: rgb(0xa9a1ad).into(),
            faint: rgb(0x8b8394).into(),
            number: rgb(0xe3d27a).into(),
            accent: rgb(0xf08a5d).into(),
            running: rgb(0x7fd48a).into(),
            paused: rgb(0xf1c75b).into(),
            restarting: rgb(0x8fb4e8).into(),
            danger: rgb(0xf38b8b).into(),
        }
    }

    fn navy() -> Self {
        Self {
            bg: rgb(0x0c1424).into(),
            glow: rgb(0x16295a).into(),
            bar: rgb(0x13244a).into(),
            panel: rgba(0x050a166b).into(),
            line: rgba(0xa0beff17).into(),
            active: rgba(0x8cafff1f).into(),
            select: rgba(0x8cafff12).into(),
            group: rgba(0x8cafff0a).into(),
            guide: rgba(0xa0beff38).into(),
            text: rgb(0xe6ecf7).into(),
            muted: rgb(0x9fadc6).into(),
            faint: rgb(0x8191ad).into(),
            number: rgb(0xf2d57e).into(),
            accent: rgb(0x6ea8ff).into(),
            running: rgb(0x7fd48a).into(),
            paused: rgb(0xf1c75b).into(),
            restarting: rgb(0x8fb4e8).into(),
            danger: rgb(0xf38b8b).into(),
        }
    }
}

/// Ajusta el tema de gpui-kit. Llamar en `main` justo después de `init(cx)`.
pub fn setup(cx: &mut App) {
    let colors = match PALETTE {
        Palette::Warm => AppColors::warm(),
        Palette::Navy => AppColors::navy(),
    };
    cx.set_global(colors);

    // 1. Modo oscuro (recarga los colores base del kit).
    Theme::change(ThemeMode::Dark, None, cx);

    // 2. Nuestros colores encima, en un `update` aparte.
    Theme::update(cx, |theme| {
        theme.background = colors.bg;
        theme.foreground = colors.text;
        theme.muted_foreground = colors.muted;
        theme.border = colors.line;
        theme.title_bar = colors.bar;
        theme.title_bar_border = colors.line;
        theme.secondary_hover = colors.active;
        theme.secondary_active = colors.active;
        theme.accent = colors.active;
        // Semantic colors: kit components (buttons, notifications…) use the same palette.
        theme.success = colors.running;
        theme.warning = colors.paused;
        theme.info = colors.restarting;
        theme.danger = colors.danger;
    });
}
