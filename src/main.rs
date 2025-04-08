use bevy::prelude::*;
use bevy_quick_response::QuickResponsePlugin;


fn main() {
    let mut app = App::new();
    app
        // Create the window
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: String::from(
                    "Nanite Factory Game Map",
                ),
                ..Default::default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        // Should improve respinsiveness at the cost of performance. Should probably be removed.
        .add_plugins(QuickResponsePlugin::default().with_no_default_plugins());

    
    nanite_factory_game_map::register(&mut app);
    app.run();
}
