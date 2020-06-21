extern crate tokio;

mod app;

/// BCDice IRC GUIのメイン処理。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run();

    let join = tokio::spawn(async {
        app::stop_rpc_server().await;
    });

    // 警告を抑えるために結果を束縛する
    let _ = join.await;

    Ok(())
}
