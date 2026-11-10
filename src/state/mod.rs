pub mod containers;

use gpui_kit::Global;

/// Runtime de tokio compartido por toda la app (bollard lo necesita).
/// Se lanza trabajo con `cx.global::<Tokio>().0.spawn(...)` y se espera
/// el JoinHandle desde `cx.spawn`.
pub struct Tokio(pub tokio::runtime::Runtime);

impl Tokio {
    pub fn new() -> Self {
        Self(tokio::runtime::Runtime::new().expect("No se pudo crear el runtime de tokio"))
    }
}

impl Global for Tokio {}
