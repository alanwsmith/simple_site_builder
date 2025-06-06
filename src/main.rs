use anyhow::Result;
use anyhow::anyhow;
use axum::Router;
use axum::response::Html;
use axum::routing::get;
use port_check::free_local_port_in_range;
use std::process::Command;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use watchexec::Watchexec;
use watchexec_signals::Signal;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting up...");
    let wx = Watchexec::default();
    let port = find_port()?;
    let service = ServeDir::new("docs")
        .append_index_html_on_directories(true)
        .not_found_service(get(|| missing_page()));
    let livereload = LiveReloadLayer::new();
    let app = Router::new().fallback_service(service).layer(livereload);
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    let http_handle = tokio::spawn(async move {
        println!("Starting folder server on port {}", port);
        axum::serve(listener, app).await.unwrap();
    });
    wx.config.pathset(vec!["content"]);
    wx.config.on_action(move |mut action| {
        if action.signals().any(|sig| sig == Signal::Interrupt) {
            action.quit(); // Needed for Ctrl+c
        } else {
            action.quit();
        }
        action
    });
    println!("Starting watcher");
    let _ = wx.main().await?;
    http_handle.abort();
    println!("Watcher done check");
    println!("Process complete.");
    Ok(())
}

fn find_port() -> Result<u16> {
    free_local_port_in_range(5444..=6000).ok_or(anyhow!("Could not find port"))
}

// async fn init() -> Result<()> {
//     tokio::spawn(async move {
//         let _ = run_server().await;
//     });
//     let _ = run_watcher().await;
//     Ok(())
// }

fn launch_browser(port: usize) -> Result<()> {
    let args: Vec<String> = vec![format!("http://localhost:{}", port)];
    Command::new("open").args(args).output()?;
    Ok(())
}

async fn missing_page() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html lang="en">
<head><style>body { background: black; color: white;}</style></head>
<body>Page Not Found</body>
</html>"#,
    )
}

// async fn run_server() -> Result<()> {
//     let port = find_port()?;
//     let service = ServeDir::new("docs")
//         .append_index_html_on_directories(true)
//         .not_found_service(get(|| missing_page()));
//     let livereload = LiveReloadLayer::new();
//     let reloader = livereload.reloader();
//     let app = Router::new().fallback_service(service).layer(livereload);
//     let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
//         .await
//         .unwrap();
//     tokio::spawn(async move {
//         let _ = run_watcher().await;
//     });
//     println!("Starting folder server on port {}", port);
//     // launch_browser(port.into())?;
//     axum::serve(listener, app).await.unwrap();
//     Ok(())
// }

async fn run_watcher() -> Result<()> {
    // let wx = Watchexec::default();
    // let port = find_port()?;
    // let service = ServeDir::new("docs")
    //     .append_index_html_on_directories(true)
    //     .not_found_service(get(|| missing_page()));
    // let livereload = LiveReloadLayer::new();
    // let app = Router::new().fallback_service(service).layer(livereload);
    // let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
    //     .await
    //     .unwrap();
    // let http_handle = tokio::spawn(async move {
    //     let _ = run_watcher().await;
    // });
    // println!("Starting folder server on port {}", port);
    // axum::serve(listener, app).await.unwrap();
    // wx.config.pathset(vec!["content"]);
    // wx.config.on_action(move |mut action| {
    //     if action.signals().any(|sig| sig == Signal::Interrupt) {
    //         action.quit(); // Needed for Ctrl+c
    //     } else {
    //         action.quit();
    //     }
    //     action
    // });
    // println!("Starting watcher");
    // // let _ = wx.main().await?;
    // // http_handle.abort();
    // println!("Watcher done check");

    Ok(())
}
