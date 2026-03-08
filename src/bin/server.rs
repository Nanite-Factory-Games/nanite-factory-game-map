use std::time::Duration;

use nanite_factory_game_map::server::configure;

fn main() -> anyhow::Result<()> {

    let mut app = configure(Duration::from_millis(100));
    app.run();
    Ok(())
}