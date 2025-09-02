use crate::config::*;
use anyhow::{Result, anyhow};
use axum::Router;
use axum::response::Html;
use axum::routing::get;
use port_check::free_local_port_in_range;
use std::process::Command;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use tracing::info;

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
    info!("Starting web server");
    let port = find_port()?;
    info!("Found port for web server: {}", &port);
    launch_browser(port.into())?;
    let service = ServeDir::new("docs")
      .append_index_html_on_directories(true)
      .not_found_service(get(|| missing_page()));
    let app =
      Router::new().fallback_service(service).layer(live_reload);
    let listener =
      tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
  }
}

fn find_port() -> Result<u16> {
  free_local_port_in_range(5444..=6000)
    .ok_or(anyhow!("Could not find port"))
}

fn launch_browser(port: usize) -> Result<()> {
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
