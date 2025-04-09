mod graphics;

use graphics::window::create_window;

pub async fn run() {
    create_window().await;
}
