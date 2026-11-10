use super::models::ContainerInfo;
use anyhow::{Context as _, Result};
use bollard::Docker;
use bollard::query_parameters::{
    InspectContainerOptions, ListContainersOptionsBuilder, LogsOptionsBuilder,
    RestartContainerOptions, StartContainerOptions, StatsOptionsBuilder, StopContainerOptions,
};
use futures_util::StreamExt;
use futures_util::future::join_all;
use tokio::{runtime::Handle, sync::mpsc};

/// Cliente de Docker. `Docker` es un Arc por dentro, así que clonar es barato.
#[derive(Clone)]
pub struct DockerClient {
    inner: Docker,
}

/// Uso de recursos de un contenedor en un instante.
pub struct Usage {
    pub cpu: f64,
    pub mem_mib: f64,
}

impl DockerClient {
    pub fn new() -> Result<Self> {
        Ok(Self {
            inner: Docker::connect_with_local_defaults()?,
        })
    }

    /// Dirección a la que nos conectamos, para la barra de estado.
    pub fn endpoint() -> String {
        std::env::var("DOCKER_HOST").unwrap_or_else(|_| "unix:///var/run/docker.sock".into())
    }

    pub async fn containers(&self) -> Result<Vec<ContainerInfo>> {
        let options = ListContainersOptionsBuilder::new().all(true).build();
        let containers = self.inner.list_containers(Some(options)).await?;

        Ok(containers
            .into_iter()
            .map(|c| {
                let mut ports: Vec<String> = Vec::new();
                for p in c.ports.unwrap_or_default() {
                    let text = match p.public_port {
                        Some(public) => format!("{public}:{}", p.private_port),
                        None => p.private_port.to_string(),
                    };
                    // Docker repite cada puerto para IPv4 e IPv6.
                    if !ports.contains(&text) {
                        ports.push(text);
                    }
                }

                let project = c
                    .labels
                    .as_ref()
                    .and_then(|l| l.get("com.docker.compose.project"))
                    .cloned();

                ContainerInfo {
                    project,
                    id: c.id.unwrap_or_default(),
                    name: c
                        .names
                        .and_then(|n| n.into_iter().next())
                        .map(|n| n.trim_start_matches('/').to_string())
                        .unwrap_or_else(|| "Sin nombre".into()),
                    image: c.image.unwrap_or_else(|| "Sin imagen".into()),
                    status: c.status.unwrap_or_default(),
                    state: c.state.map(|s| s.to_string()).unwrap_or_default(),
                    ports: (!ports.is_empty()).then(|| ports.join(", ")),
                    cpu: None,
                    mem_mib: None,
                }
            })
            .collect())
    }

    /// CPU y memoria de un contenedor. Docker tarda ~1 s porque mide dos muestras.
    pub async fn usage(&self, id: &str) -> Result<Usage> {
        let options = StatsOptionsBuilder::new().stream(false).build();
        let mut stream = Box::pin(self.inner.stats(id, Some(options)));
        let stats = stream.next().await.context("Docker no devolvió estadísticas")??;

        let cpu = match (&stats.cpu_stats, &stats.precpu_stats) {
            (Some(now), Some(before)) => {
                let total = |s: &bollard::models::ContainerCpuStats| {
                    s.cpu_usage.as_ref().and_then(|u| u.total_usage).unwrap_or(0) as f64
                };
                let cpu_delta = total(now) - total(before);
                let system_delta = now.system_cpu_usage.unwrap_or(0) as f64
                    - before.system_cpu_usage.unwrap_or(0) as f64;
                let cpus = now.online_cpus.unwrap_or(1) as f64;
                if cpu_delta > 0.0 && system_delta > 0.0 {
                    cpu_delta / system_delta * cpus * 100.0
                } else {
                    0.0
                }
            }
            _ => 0.0,
        };

        let mem_mib = stats
            .memory_stats
            .map(|m| {
                let usage = m.usage.unwrap_or(0);
                // Igual que `docker stats`: se descuenta la caché de archivos.
                let cache = m
                    .stats
                    .as_ref()
                    .and_then(|s| s.get("inactive_file").or_else(|| s.get("cache")).copied())
                    .unwrap_or(0);
                usage.saturating_sub(cache) as f64 / 1_048_576.0
            })
            .unwrap_or(0.0);

        Ok(Usage { cpu, mem_mib })
    }

    /// Estadísticas de varios contenedores en paralelo. Los que fallen se omiten.
    pub async fn usage_many(&self, ids: Vec<String>) -> Vec<(String, Usage)> {
        let results = join_all(ids.iter().map(|id| self.usage(id))).await;
        ids.into_iter()
            .zip(results)
            .filter_map(|(id, r)| r.ok().map(|u| (id, u)))
            .collect()
    }

    pub async fn start(&self, id: &str) -> Result<()> {
        self.inner
            .start_container(id, None::<StartContainerOptions>)
            .await?;
        Ok(())
    }

    pub async fn unpause(&self, id: &str) -> Result<()> {
        self.inner.unpause_container(id).await?;
        Ok(())
    }

    pub async fn stop(&self, id: &str) -> Result<()> {
        self.inner
            .stop_container(id, None::<StopContainerOptions>)
            .await?;
        Ok(())
    }

    pub async fn restart(&self, id: &str) -> Result<()> {
        self.inner
            .restart_container(id, None::<RestartContainerOptions>)
            .await?;
        Ok(())
    }

    #[allow(dead_code)] // para la vista de detalle
    pub async fn env_vars(&self, container_id: &str) -> Result<Vec<String>> {
        let inspect = self
            .inner
            .inspect_container(container_id, None::<InspectContainerOptions>)
            .await?;
        Ok(inspect.config.and_then(|c| c.env).unwrap_or_default())
    }

    #[allow(dead_code)] // para la vista de detalle
    /// Lanza el stream de logs en el runtime de tokio y devuelve un canal.
    pub fn logs(&self, runtime: &Handle, container_id: &str) -> mpsc::Receiver<String> {
        let (tx, rx) = mpsc::channel(100);
        let docker = self.inner.clone();
        let id = container_id.to_string();

        runtime.spawn(async move {
            let options = LogsOptionsBuilder::new()
                .follow(true)
                .stdout(true)
                .stderr(true)
                .tail("50")
                .build();
            let mut stream = Box::pin(docker.logs(&id, Some(options)));

            while let Some(item) = stream.next().await {
                let line = match item {
                    Ok(out) => out.to_string(),
                    Err(e) => format!("[error] {e}"),
                };
                if tx.send(line).await.is_err() {
                    break;
                }
            }
        });

        rx
    }
}
