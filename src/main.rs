use anyhow::Result;
use anyhow::anyhow;
use axum::Router;
use port_check::free_local_port_in_range;
use std::path::PathBuf;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

#[tokio::main]
async fn main() -> Result<()> {
    let path_to_serve = PathBuf::from("docs");
    run_server(&path_to_serve).await?;
    Ok(())
}

async fn run_server(path_to_serve: &PathBuf) -> Result<()> {
    let port = find_port()?;
    dbg!(&port);
    let service = ServeDir::new(path_to_serve).append_index_html_on_directories(true);
    let livereload = LiveReloadLayer::new();
    let app = Router::new().fallback_service(service).layer(livereload);
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

fn find_port() -> Result<u16> {
    free_local_port_in_range(5444..=6000).ok_or(anyhow!("Could not find port"))
}
