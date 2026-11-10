use super::Tokio;
use crate::docker::{DockerClient, models::ContainerInfo};
use gpui_kit::*;
use std::collections::HashSet;

#[derive(Clone, Copy)]
pub enum Action {
    Start,
    Unpause,
    Stop,
    Restart,
}

/// Estado de los contenedores. Las vistas lo observan con `cx.observe`.
pub struct ContainersStore {
    client: DockerClient,
    pub containers: Vec<ContainerInfo>,
    pub loading: bool,
    pub error: Option<String>,
    /// Ids con una acción en curso (para deshabilitar sus botones).
    pub busy: HashSet<String>,
}

impl ContainersStore {
    pub fn new(client: DockerClient) -> Self {
        Self {
            client,
            containers: Vec::new(),
            loading: false,
            error: None,
            busy: HashSet::new(),
        }
    }

    /// Carga la lista (rápido) y después CPU/memoria de los que corren (~1 s).
    pub fn reload(&mut self, cx: &mut Context<Self>) {
        self.loading = true;
        self.error = None;
        cx.notify();

        let runtime = cx.global::<Tokio>().0.handle().clone();
        let client = self.client.clone();
        let list = runtime.spawn({
            let client = client.clone();
            async move { client.containers().await }
        });

        cx.spawn(async move |this, cx| {
            // 1. La lista.
            let result = list.await;
            let running_ids = this.update(cx, |this, cx| {
                this.loading = false;
                match result {
                    Ok(Ok(list)) => this.containers = list,
                    Ok(Err(e)) => this.error = Some(e.to_string()),
                    Err(e) => this.error = Some(e.to_string()),
                }
                cx.notify();
                this.containers
                    .iter()
                    .filter(|c| c.is_running())
                    .map(|c| c.id.clone())
                    .collect::<Vec<_>>()
            })?;

            // 2. Las estadísticas, en una segunda pasada.
            let usage = runtime
                .spawn(async move { client.usage_many(running_ids).await })
                .await?;
            this.update(cx, |this, cx| {
                for (id, u) in usage {
                    if let Some(c) = this.containers.iter_mut().find(|c| c.id == id) {
                        c.cpu = Some(u.cpu);
                        c.mem_mib = Some(u.mem_mib);
                    }
                }
                cx.notify();
            })?;
            anyhow::Ok(())
        })
        .detach();
    }

    /// Varias acciones a la vez (p. ej. detener todo un proyecto de compose).
    pub fn run_many(&mut self, actions: Vec<(String, Action)>, cx: &mut Context<Self>) {
        for (id, action) in actions {
            self.run(id, action, cx);
        }
    }

    pub fn run(&mut self, id: String, action: Action, cx: &mut Context<Self>) {
        if !self.busy.insert(id.clone()) {
            return; // ya hay una acción en curso para este contenedor
        }
        cx.notify();

        let client = self.client.clone();
        let task = cx.global::<Tokio>().0.spawn({
            let id = id.clone();
            async move {
                match action {
                    Action::Start => client.start(&id).await,
                    Action::Unpause => client.unpause(&id).await,
                    Action::Stop => client.stop(&id).await,
                    Action::Restart => client.restart(&id).await,
                }
            }
        });

        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                this.busy.remove(&id);
                match result {
                    Ok(Ok(())) => {}
                    Ok(Err(e)) => this.error = Some(e.to_string()),
                    Err(e) => this.error = Some(e.to_string()),
                }
                this.reload(cx);
            })
            .ok();
        })
        .detach();
    }
}
