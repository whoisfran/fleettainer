use gpui_kit::*;
use std::sync::{Arc, LazyLock};

/// The app isotype ("Pila F"). The SVG is embedded in the binary at compile time,
/// so it doesn't depend on any file being installed.
static LOGO: LazyLock<Arc<Image>> = LazyLock::new(|| {
    let bytes = include_bytes!("../../../assets/fleettainer-16.svg");
    Arc::new(Image::from_bytes(ImageFormat::Svg, bytes.to_vec()))
});

#[derive(IntoElement)]
pub struct AppLogo {
    size: Pixels,
}

impl AppLogo {
    pub fn new() -> Self {
        Self { size: px(18.) }
    }
}

impl RenderOnce for AppLogo {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        img(LOGO.clone()).size(self.size).flex_shrink_0()
    }
}
