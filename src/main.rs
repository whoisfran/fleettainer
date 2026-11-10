mod docker;
mod state;
mod ui;
mod views;

use docker::DockerClient;
use gpui_kit::component::TitleBar;
use gpui_kit::*;
use state::Tokio;
use views::root::RootView;

/// App version, taken from `Cargo.toml` at compile time.
/// To release: bump `version` in Cargo.toml and push a matching tag (`v0.2.0`).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let client = DockerClient::new().expect("No se pudo conectar con Docker");

    application().with_assets(assets::Assets).run(move |cx| {
        init(cx);

        ui::theme::setup(cx);
        
        cx.set_global(Tokio::new());

        let options = WindowOptions {
            window_min_size: Some(size(px(1100.), px(600.))),
            // Must match the .desktop file name, so Wayland docks show our icon.
            app_id: Some("fleettainer".into()),
            ..TitleBar::window_options()
        };

        open_window(options, cx, |window, cx| {
            cx.new(|cx| RootView::new(client, window, cx))
        })
        .expect("Failed to open window");
    });
}
