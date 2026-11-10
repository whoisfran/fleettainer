//! Pure logic for the container list: filtering and compose grouping. No UI here.

use crate::docker::models::ContainerInfo;
use std::collections::BTreeMap;

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Filter {
    All,
    Running,
    Stopped,
}

impl Filter {
    pub(super) fn accepts(self, c: &ContainerInfo) -> bool {
        match self {
            Filter::All => true,
            Filter::Running => c.is_running(),
            Filter::Stopped => !c.is_running(),
        }
    }
}

/// What the table draws: a compose project with its containers, or a standalone container.
pub(super) enum Entry {
    Group { project: String, items: Vec<ContainerInfo> },
    Single(ContainerInfo),
}

/// Groups by compose project (alphabetical) and puts standalone containers last.
pub(super) fn build_entries(rows: Vec<ContainerInfo>) -> Vec<Entry> {
    let mut groups: BTreeMap<String, Vec<ContainerInfo>> = BTreeMap::new();
    let mut singles = Vec::new();
    for c in rows {
        match c.project.clone() {
            Some(project) => groups.entry(project).or_default().push(c),
            None => singles.push(c),
        }
    }
    groups
        .into_iter()
        .map(|(project, items)| Entry::Group { project, items })
        .chain(singles.into_iter().map(Entry::Single))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn container(name: &str, project: Option<&str>) -> ContainerInfo {
        ContainerInfo {
            id: name.into(),
            name: name.into(),
            image: "img".into(),
            status: "Up 2 hours".into(),
            state: "running".into(),
            project: project.map(Into::into),
            ports: None,
            cpu: None,
            mem_mib: None,
        }
    }

    #[test]
    fn groups_first_sorted_then_singles() {
        let entries = build_entries(vec![
            container("solo", None),
            container("web", Some("zeta")),
            container("db", Some("alpha")),
            container("cache", Some("zeta")),
        ]);

        let shape: Vec<String> = entries
            .iter()
            .map(|e| match e {
                Entry::Group { project, items } => format!("{project}:{}", items.len()),
                Entry::Single(c) => c.name.clone(),
            })
            .collect();
        assert_eq!(shape, ["alpha:1", "zeta:2", "solo"]);
    }
}
