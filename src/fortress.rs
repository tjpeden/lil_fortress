use amethyst::{
    assets::{
        AssetStorage,
        Loader,
    },
    core::{
        math::{
            Point3,
            Vector3,
        },
        Transform,
    },
    input::{
        is_close_requested,
        is_key_down,
    },
    prelude::*,
    renderer::{
        camera::Camera,
        formats::texture::ImageFormat,
        sprite::{
            SpriteSheet,
            SpriteSheetFormat,
            SpriteSheetHandle,
        },
        Texture,
    },
    tiles::{
        MortonEncoder,
        Tile,
        TileMap,
    },
    window::ScreenDimensions,
    winit::VirtualKeyCode,
};

#[derive(Default, Clone)]
pub struct MapTile;

impl Tile for MapTile {
    fn sprite(&self, _: Point3<u32>, _: &World) -> Option<usize> {
        Some(0xDB)
    }
}

pub struct Fortress;

impl SimpleState for Fortress {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        create_map(world);
        create_camera(world);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }
}

fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    let loader = world.read_resource::<Loader>();
    let texture_store = world.read_resource::<AssetStorage<Texture>>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    let texture_handle = loader.load(
        png_path,
        ImageFormat::default(),
        (),
        &texture_store,
    );

    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn create_map(world: &mut World) {
    let map_sprite_sheet_handle = load_sprite_sheet(
        world,
        "aesomatica_16x16.png",
        "aesomatica_16x16.ron",
    );
    let map = TileMap::<MapTile, MortonEncoder>::new(
        Vector3::new(32, 32, 1),
        Vector3::new(16, 16, 1),
        Some(map_sprite_sheet_handle),
    );
    let transform = Transform::default();

    world
    .create_entity()
    .with(map)
    .with(transform)
    .named("Map")
    .build();
}

fn create_camera(world: &mut World) {
    let (width, height) = {
        let dimensions = world.read_resource::<ScreenDimensions>();

        (dimensions.width(), dimensions.height())
    };
    let transform = Transform::from(Vector3::new(0.5, 0.5, 0.5));

    world
    .create_entity()
    .with(Camera::standard_2d(width, height))
    .with(transform)
    .named("Camera")
    .build();
}
