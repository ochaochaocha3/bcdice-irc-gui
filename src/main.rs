mod app;

use crate::app::Application;

/// BCDice IRC GUIのメイン処理。
fn main() {
    let app = Application::new();
    app.run();
}
