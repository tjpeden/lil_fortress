mod fortress;

use fortress::{
    MapTile,
    Fortress,
};

use amethyst::{
    core::transform::TransformBundle,
    input::{
        InputBundle,
        StringBindings,
    },
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    tiles::{
        MortonEncoder,
        RenderTiles2D,
    },
    ui::{
        RenderUi,
        UiBundle,
    },
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
    .with_bundle(TransformBundle::new())?
    .with_bundle(InputBundle::<StringBindings>::new())?
    .with_bundle(UiBundle::<StringBindings>::new())?
    .with_bundle(
        RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)?
            .with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderTiles2D::<MapTile, MortonEncoder>::default())
        .with_plugin(RenderUi::default()),
    )?;

    let mut game = Application::new(assets_dir, Fortress, game_data)?;

    game.run();

    Ok(())
}
