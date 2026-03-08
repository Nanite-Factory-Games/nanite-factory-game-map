

fn main() -> anyhow::Result<()> {
    // We don't want to do anything if this is targeted to wasm
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    {
        // Get the address and port from the cli arguments, defaulting to 127.0.0.1:8080

        use nanite_factory_game_map::client::{MapConfiguration, configure};
        let args: Vec<String> = std::env::args().collect();
        let default_address = "ws://127.0.0.1:8080".to_string();
        let address = args.get(1).unwrap_or(&default_address).clone();

        let mut app = configure(MapConfiguration::new(true, None), None);
        app.run();
    }
    Ok(())
}
