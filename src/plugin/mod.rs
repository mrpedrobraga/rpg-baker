use serde::{Deserialize, Serialize};

/// Stores the loaded references to plugins (sourced from a plugin folder)
#[derive(Debug, Default)]
pub struct PluginDatabase {
    pub plugins: Vec<PluginRef>,
}

/// A reference to a plugin
pub struct PluginRef {
    metadata: PluginMetadata,
    active: bool,
}

impl std::fmt::Debug for PluginRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Plugin {} version {:?} ({})",
            self.metadata.id,
            self.metadata.version,
            if self.active { "active" } else { "inactive" }
        )
    }
}

/// Metadata of a plugin for information purposes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginMetadata {
    name: String,
    id: String,
    description: String,
    version: semver::Version,
    authors: Vec<String>,
    plugin_contributions: PluginContributions,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginContributions {}
