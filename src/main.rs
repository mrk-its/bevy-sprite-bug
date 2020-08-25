use bevy::prelude::*;
use bevy::sprite::TextureAtlas;

const SCALE: f32 = 3.0;

pub struct Player;
pub struct Wall;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(keyboard_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
        asset_server.load("assets/mini_atlas.png").unwrap(),
        Vec2::new(64.0, 32.0),
        2,
        1,
    ));
    let sprite = |index, x, y| SpriteSheetComponents {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(index),
        translation: Translation(Vec3::new(
            x as f32 * 32.0 * SCALE,
            y as f32 * 32.0 * SCALE,
            0.0,
        )),
        scale: Scale(SCALE),
        ..Default::default()
    };
    commands
        .spawn(Camera2dComponents::default())
        .spawn(sprite(0, 0, 0))
        .with(Player)
        .spawn(sprite(1, 1, 0))
        .with(Wall)
        .spawn(sprite(1, 1, 1))
        .with(Wall)
    ;
}

pub fn keyboard_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<With<Player, Entity>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for entity in &mut query.iter() {
            commands.despawn(entity);
        }
    }
}
