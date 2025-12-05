

fn main() -> anyhow::Result<()> {
    // We don't want to do anything if this is targeted to wasm
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    {
        use nanite_factory_game_map::{app::{get_assets_recursively, start_from_server_info}};
        // Get the address and port from the cli arguments, defaulting to 127.0.0.1:8080
        let args: Vec<String> = std::env::args().collect();
        let default_address = "127.0.0.1:8080".to_string();
        let address = args.get(1).unwrap_or(&default_address).clone();

        start_from_server_info(address, None, None)
            .map_err(|e| anyhow::anyhow!("Error starting from server info: {:?}", e))?;
    }
    Ok(())
}
