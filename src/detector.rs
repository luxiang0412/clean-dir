use std::fmt;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectArtifact {
    PythonVenv,
    NodeModules,
    JavaTarget,
}

impl fmt::Display for ProjectArtifact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectArtifact::PythonVenv => write!(f, "Python"),
            ProjectArtifact::NodeModules => write!(f, "Node"),
            ProjectArtifact::JavaTarget => write!(f, "Java"),
        }
    }
}

impl ProjectArtifact {
    pub fn from_name(name: &str, parent: &Path) -> Option<Self> {
        match name {
            ".venv" => Some(ProjectArtifact::PythonVenv),
            "node_modules" => Some(ProjectArtifact::NodeModules),
            "target" if is_java_target(parent) => Some(ProjectArtifact::JavaTarget),
            _ => None,
        }
    }

    pub fn matches_filter(&self, filter: &[String]) -> bool {
        filter.iter().any(|f| match f.to_lowercase().as_str() {
            "python" => *self == ProjectArtifact::PythonVenv,
            "node" => *self == ProjectArtifact::NodeModules,
            "java" => *self == ProjectArtifact::JavaTarget,
            _ => false,
        })
    }
}

fn is_java_target(parent: &Path) -> bool {
    const MARKERS: &[&str] = &["pom.xml", "build.gradle", "build.gradle.kts"];
    MARKERS.iter().any(|m| parent.join(m).exists())
}
