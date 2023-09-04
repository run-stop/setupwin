mod setupwin;
mod templates;
use relm4::RelmApp;
use setupwin::*;

fn main() {
    let app = RelmApp::new("com.iodream.setupwin");
    app.run_async::<SetupWin>(SetupWinInit {});
}
