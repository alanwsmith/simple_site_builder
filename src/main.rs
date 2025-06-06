#![allow(unused)]
use anyhow::Result;
use anyhow::anyhow;
use axum::Router;
use axum::response::Html;
use axum::routing::get;
use minijinja::syntax::SyntaxConfig;
use minijinja::{Environment, Value, context};
use port_check::free_local_port_in_range;
use std::process::Command;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use tower_livereload::Reloader;
use watchexec::Watchexec;
use watchexec_signals::Signal;

async fn run_server(livereload: LiveReloadLayer) -> Result<()> {
    let port = find_port()?;
    println!("Starting web server on port: {}", port);
    let service = ServeDir::new("docs")
        .append_index_html_on_directories(true)
        .not_found_service(get(|| missing_page()));
    let app = Router::new().fallback_service(service).layer(livereload);
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())

    // let wx = Watchexec::default();
    // wx.config.pathset(vec!["content"]);
    // wx.config.on_action(move |mut action| {
    //     // reloader.reload();
    //     for event in action.events.iter() {
    //         eprintln!("EVENT: {event:?}");
    //     }
    //     if action
    //         .signals()
    //         .any(|sig| sig == Signal::Interrupt || sig == Signal::Terminate)
    //     {
    //         action.quit(); // Needed for Ctrl+c
    //     }
    //     action
    // });
    // println!("Starting watcher");
    // let _ = wx.main().await?;
    // http_handle.abort();
    // println!("Watcher done check");
    // println!("Process complete.");
}

async fn run_builder(mut rx: Receiver<bool>, reloader: Reloader) -> Result<()> {
    println!("Starting builder");
    while let Some(message) = rx.recv().await {
        println!("Building.");
        reloader.reload();
    }
    println!("Builder stopped.");
    Ok(())
}

async fn run_watcher(tx: Sender<bool>) -> Result<()> {
    println!("Starting watcher");
    let wx = Watchexec::default();
    wx.config.pathset(vec!["content"]);
    wx.config.on_action(move |mut action| {
        println!("----");
        let tx2 = tx.clone();
        tokio::spawn(async move {
            tx2.send(true).await.unwrap();
        });

        // reloader.reload();
        // for event in action.events.iter() {
        //     eprintln!("EVENT: {event:?}");
        // }
        if action
            .signals()
            .any(|sig| sig == Signal::Interrupt || sig == Signal::Terminate)
        {
            action.quit(); // Needed for Ctrl+c
        }
        action
    });
    let _ = wx.main().await?;
    println!("Watcher stopped.");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting up...");
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();
    let (watcher_tx, mut watcher_rx) = mpsc::channel::<bool>(32);
    let http_handle = tokio::spawn(async move {
        run_server(livereload).await;
    });
    let builder_handle = tokio::spawn(async move {
        run_builder(watcher_rx, reloader).await;
    });
    run_watcher(watcher_tx).await;
    http_handle.abort();
    builder_handle.abort();

    //let watcher_handle = tokio::spawn(async move {});

    //watcher_handle.await;

    println!("Process complete");

    // let port = find_port()?;
    // let service = ServeDir::new("docs")
    //     .append_index_html_on_directories(true)
    //     .not_found_service(get(|| missing_page()));
    // let livereload = LiveReloadLayer::new();
    // let reloader = livereload.reloader();
    // let app = Router::new().fallback_service(service).layer(livereload);
    // let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
    //     .await
    //     .unwrap();
    // let http_handle = tokio::spawn(async move {
    //     println!("Starting folder server on port {}", port);
    //     axum::serve(listener, app).await.unwrap();
    // });

    Ok(())
}

async fn builder() -> Result<()> {
    Ok(())
}

//
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

//async fn run_watcher() -> Result<()> {
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

//   Ok(())
//}
