use anyhow::Result;
use tokio::sync::mpsc::Sender;
use watchexec::Watchexec;
use watchexec_signals::Signal;

pub async fn run_watcher(tx: Sender<bool>) -> Result<()> {
    println!("Starting watcher");
    tx.send(true).await.unwrap();
    let wx = Watchexec::default();
    wx.config.pathset(vec!["content", "data", "scripts"]);
    wx.config.on_action(move |mut action| {
        let tx2 = tx.clone();
        tokio::spawn(async move {
            tx2.send(true).await.unwrap();
        });
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
