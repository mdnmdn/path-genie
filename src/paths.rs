
pub struct PathStore {
  paths: Vec<PathInfo>,
  pub config_file: Option<String>, 
}

pub struct PathInfo {
  pub path: String,
}

impl PathStore {
  pub fn new() -> Self {
    PathStore {
      paths: vec![],
      config_file: None,
    }
  }

  pub fn add_path(&mut self, path: String ) -> () {
    self.paths.push(PathInfo{
      path,
    })
  }

  pub fn list_sorted_paths(&self) -> Vec<&str> {
    self.paths.iter().map(|p| p.path.as_str()).collect()
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



