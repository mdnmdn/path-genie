use crate::utils::AppError;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    error::Error,
    fs::{read_to_string, write},
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Clone)]
pub enum PathSection {
    Visited,
    Top,
}

pub struct PathStore {
    paths: Vec<PathInfo>,
    pub config_file: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PathInfo {
    pub path: String,
    pub section: PathSection,
}

#[derive(Serialize, Deserialize)]
struct PathStoreStructure<'a> {
    paths: Cow<'a, Vec<PathInfo>>,
}

impl PathStore {
    pub fn new() -> Self {
        PathStore {
            paths: vec![],
            config_file: None,
        }
    }

    pub fn add_path(&mut self, path: String) -> () {
        println!("add_path > {}", path);
        let pos = self.paths.iter().position(|p| p.path == path);
        match pos {
            None => self.paths.push(PathInfo {
                path,
                section: PathSection::Visited,
            }),
            _ => {}
        }
    }

    pub fn add_path_buf(&mut self, path: PathBuf) -> bool {
        if !path.exists() {
            return false;
        }
        let absolute_path = path
            .as_path()
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        self.add_path(absolute_path);
        true
    }

    pub fn list_sorted_paths(&self) -> Vec<&str> {
        self.paths.iter().map(|p| p.path.as_str()).collect()
    }

    pub fn load_from_config(&mut self) -> Result<(), Box<dyn Error>> {
        match &self.config_file {
            Some(path) => {
                let serialized_config = read_to_string(path)?;
                let config_data: PathStoreStructure = serde_yaml::from_str(&serialized_config)?;

                self.paths = match config_data.paths {
                    Cow::Owned(data) => data,
                    Cow::Borrowed(data) => data.to_owned(),
                };
                Ok(())
            }
            _ => Err(Box::new(AppError::new("Missing config file".to_string()))),
        }
    }

    pub fn persist(&self) -> Result<(), Box<dyn Error>> {
        if let Some(path) = &self.config_file {
            let data_to_serialize = PathStoreStructure {
                paths: Cow::Borrowed(&self.paths),
            };
            let serialized = serde_yaml::to_string(&data_to_serialize)?;
            write(path, serialized)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::PathStore;
    #[test]
    fn add_paths_to_store() {
        let mut store = PathStore::new();
        store.add_path("pippo".to_owned());
        store.add_path("pluto".to_owned());

        assert_eq!(2, store.paths.len());
    }
}
