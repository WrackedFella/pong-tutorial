mod pong;
extern crate amethyst;

use crate::pong::Pong;
use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};
use amethyst::core::transform::TransformBundle;


fn main() {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir().unwrap();
    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path).unwrap().with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default()),
        ).unwrap()
        .with_bundle(TransformBundle::new()).unwrap();

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Pong, game_data).unwrap();
    game.run();
}
