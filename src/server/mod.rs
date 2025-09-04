use crate::config::*;
use anyhow::Result;
use axum::Router;
use axum::response::Html;
use axum::routing::get;
use std::process::Command;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use tracing::info;

pub struct Server {
  config: Config,
  port: u16,
}

impl Server {
  pub fn new(
    config: Config,
    port: u16,
  ) -> Server {
    Server { config, port }
  }

  pub async fn start(
    &self,
    live_reload: LiveReloadLayer,
  ) -> Result<()> {
    info!("Starting web server");
    launch_browser(self.port)?;
    let service = ServeDir::new(&self.config.output_root)
      .append_index_html_on_directories(true)
      .not_found_service(get(missing_page));
    let app = Router::new()
      .fallback_service(service)
      .layer(live_reload);
    let listener = tokio::net::TcpListener::bind(
      format!("127.0.0.1:{}", self.port),
    )
    .await
    .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
  }
}

fn launch_browser(port: u16) -> Result<()> {
  if !cfg!(debug_assertions) {
    let args: Vec<String> =
      vec![format!("http://localhost:{}", port)];
    Command::new("open").args(args).output()?;
  }
  Ok(())
}

async fn missing_page() -> Html<&'static str> {
  Html(
    r#"<!DOCTYPE html>
<html lang="en">
<head><style>body { background: black; color: white;}</style></head>
<body>Page Not Found</sody>
</html>"#,
  )
}
