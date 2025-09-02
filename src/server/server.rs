use crate::config::*;
use anyhow::Result;
use tower_livereload::LiveReloadLayer;

pub struct Server {
  config: Config,
}

impl Server {
  pub fn new(config: Config) -> Server {
    Server { config }
  }

  pub async fn start(
    &self,
    live_reload: LiveReloadLayer,
  ) -> Result<()> {
    Ok(())
  }
}
