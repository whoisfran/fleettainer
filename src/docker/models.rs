/// Modelo propio: la UI nunca ve los tipos de bollard.
#[derive(Clone, Debug)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    /// Texto libre de Docker, p. ej. "Up 2 hours" o "Exited (0) 2 days ago".
    pub status: String,
    /// Estado de máquina: "running", "exited", "paused", "restarting"...
    pub state: String,
    /// Proyecto de Docker Compose (etiqueta `com.docker.compose.project`), si lo tiene.
    pub project: Option<String>,
    /// Puertos ya formateados ("5432→5432, 8080→80"). `None` si no expone ninguno.
    pub ports: Option<String>,
    /// Uso de CPU en %. Solo para contenedores en ejecución (llega en una segunda pasada).
    pub cpu: Option<f64>,
    /// Memoria en MiB. Igual que `cpu`.
    pub mem_mib: Option<f64>,
}

impl ContainerInfo {
    pub fn is_running(&self) -> bool {
        self.state == "running"
    }

    pub fn is_paused(&self) -> bool {
        self.state == "paused"
    }

    /// Tiempo activo compacto ("3 h", "20 min"). Solo si el contenedor está arriba.
    pub fn uptime(&self) -> Option<String> {
        self.status
            .strip_prefix("Up ")
            .map(|rest| humanize(strip_parens(rest)))
    }
}

fn strip_parens(s: &str) -> &str {
    s.split(" (").next().unwrap_or(s).trim()
}

/// "3 hours" → "3 h", "About a minute" → "1 min".
fn humanize(s: &str) -> String {
    match s {
        "About an hour" | "an hour" => return "1 h".into(),
        "About a minute" | "a minute" => return "1 min".into(),
        "Less than a second" => return "<1 s".into(),
        _ => {}
    }
    let mut parts = s.split_whitespace();
    match (parts.next(), parts.next()) {
        (Some(n), Some(unit)) => {
            let unit = match unit.trim_end_matches('s') {
                "second" => "s",
                "minute" => "min",
                "hour" => "h",
                "day" => "d",
                "week" => "sem",
                "month" => "mes",
                "year" => "a",
                other => other,
            };
            format!("{n} {unit}")
        }
        _ => s.to_string(),
    }
}
