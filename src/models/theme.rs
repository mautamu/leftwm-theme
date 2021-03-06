use crate::models::Config;
use std::path::PathBuf;

/// Contains information about a theme contained within themes.toml (or known.toml upstream).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub description: Option<String>,
    pub directory: Option<PathBuf>,
    pub repository: Option<String>,
    pub commit: Option<String>,
    pub version: Option<String>,
    pub leftwm_versions: Option<String>,
    pub current: Option<bool>,
    pub dependencies: Option<Vec<DependencyL>>,
    #[serde(skip)]
    pub source: Option<String>,
}

/// Contains a vector of themes used for processing.
#[derive(Debug, Deserialize)]
pub struct TempThemes {
    pub theme: Vec<Theme>,
}

/// Contains information pertaining to a program dependency (name, required/optional, package).
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DependencyL {
    pub program: String,
    pub optional: Option<bool>,
    pub package: Option<String>,
}

impl Default for DependencyL {
    fn default() -> DependencyL {
        DependencyL {
            program: String::from("leftwm"),
            optional: None,
            package: None,
        }
    }
}

impl Theme {
    #[must_use]
    pub fn new(name: &str, description: Option<String>, directory: Option<PathBuf>) -> Self {
        Theme {
            name: name.to_string(),
            description,
            directory,
            repository: None,
            commit: None,
            version: Some("0.0.0".to_string()),
            leftwm_versions: Some("*".to_string()),
            dependencies: None,
            current: Some(false),
            source: None,
        }
    }

    #[must_use]
    pub fn is_installed(&self) -> bool {
        match &self.directory {
            Some(_dir) => true,
            None => false,
        }
    }

    pub fn find(config: &mut Config, name: &str) -> Option<Theme> {
        config
            .themes(false)
            .iter()
            .find(|p| name == p.name)
            .cloned()
    }

    pub fn find_installed(config: &mut Config, name: &str) -> Option<Theme> {
        config
            .themes(false)
            .iter()
            .find(|p| name == p.name && p.directory.is_some())
            .cloned()
    }

    pub fn find_all(config: &mut Config, name: &str) -> Option<Vec<Theme>> {
        let (themes, _) = config
            .themes(false)
            .iter()
            .cloned()
            .partition::<Vec<Theme>, _>(|p| name == p.name);
        Some(themes)
    }

    pub fn find_mut<'a>(
        config: &'a mut Config,
        name: &str,
        repo_name: &str,
    ) -> Option<&'a mut Theme> {
        match config.repos.iter_mut().find(|ref p| repo_name == p.name) {
            Some(reposit) => reposit.themes.iter_mut().find(|ref o| name == o.name),
            None => None,
        }
    }

    pub fn source(&mut self, name: String) -> &mut Theme {
        self.source = Some(name);
        self
    }

    pub fn current(&mut self, currency: bool) {
        self.current = if currency { Some(true) } else { None }
    }
}
