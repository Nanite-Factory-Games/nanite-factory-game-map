use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: String::from(
                    "Tiled map editor example",
                ),
                ..Default::default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()));
    
    nanite_factory_map::register(&mut app);
    app.run();
}
