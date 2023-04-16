use bevy::prelude::*;
use rand::prelude::*;

const UNIT_WIDTH: u32 = 40;
const UNIT_HEIGHT: u32 = 40;

const X_LENGTH: u32 = 10;
const Y_LENGTH: u32 = 18;

const SCREEN_WIDTH: u32 = UNIT_WIDTH * X_LENGTH;
const SCREEN_HEIGHT: u32 = UNIT_HEIGHT * Y_LENGTH;

fn main() {
    App::new()
        .init_resource::<Materials>()
        .init_resource::<BlockPatterns>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetris!".into(),
                resolution: (SCREEN_WIDTH as f32,  SCREEN_HEIGHT as f32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_block_element)
        // .add_system(position_transform)
        .add_system(position_translation)
        .add_system(size_scaling)
        .run();
}

#[derive(Component)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Resource)]
pub struct Materials {
    colors: Vec<Color>
}

struct NewBlockEvent;

impl Default for Materials {
    fn default() -> Materials {
        Materials {
            colors: vec![
                Color::rgb_u8(64, 230, 100),
                Color::rgb_u8(220, 64, 90),
                Color::rgb_u8(70, 150, 210),
                Color::rgb_u8(220, 230, 70),
                Color::rgb_u8(35, 220, 241),
                Color::rgb_u8(240, 140, 70),
            ]
        }
    }
}

#[derive(Resource)]
pub struct BlockPatterns(Vec<Vec<(i32, i32)>>);

impl Default for BlockPatterns {
    fn default() -> BlockPatterns {
        BlockPatterns (vec![
            vec![(0, 0), (0, -1), (0, 1), (0, 2)],  // I
            vec![(0, 0), (0, -1), (0, 1), (-1, 1)], // L
            vec![(0, 0), (0, -1), (0, 1), (1, 1)],  // 逆L
            vec![(0, 0), (0, -1), (1, 0), (1, 1)],  // Z
            vec![(0, 0), (1, 0), (0, 1), (1, -1)],  // 逆Z
            vec![(0, 0), (0, 1), (1, 0), (1, 1)],   // 四角
            vec![(0, 0), (-1, 0), (1, 0), (0, 1)],  // T
        ])
    }
}

pub fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_block_element(
    mut commands: Commands,
    materials: Res<Materials>,
    block_patterns: Res<BlockPatterns>,
){
    let new_block = next_block(&block_patterns.0);
    let new_color = next_color(&materials.colors);

    // ブロックの初期位置
    let initial_x = X_LENGTH / 2;
    let initial_y = Y_LENGTH - 4;

    for (r_x, r_y) in new_block.iter() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: new_color,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(80.0, -90.0, 0.0),
                    scale: Vec3::new(10.0, 10.0, 10.0),
                    ..default()
                },
                ..default()
            },
            Position {
                x: (initial_x as i32 + r_x),
                y: (initial_y as i32 + r_y),
            }
           ));
    }
}

fn next_color(colors: &Vec<Color>) -> Color {
    let mut rng = rand::thread_rng();
    let mut color_index: usize = rng.gen();
    color_index %= colors.len();

    colors[color_index].clone()
}

fn next_block(blocks: &Vec<Vec<(i32, i32)>>) -> Vec<(i32, i32)> {
    let mut rng = rand::thread_rng();
    let mut block_index: usize = rng.gen();
    block_index %= blocks.len();

    blocks[block_index].clone()
}

pub fn position_transform(
    mut position_query: Query<(&Position, &mut Transform)>
) {
    let origin_x = UNIT_WIDTH as i32 / 2 - SCREEN_WIDTH as i32 / 2;
    let origin_y = UNIT_HEIGHT as i32 / 2 - SCREEN_HEIGHT as i32 / 2;
    position_query
        .iter_mut()
        .for_each(|(pos, mut transform)| {
            transform.translation = Vec3::new(
                (origin_x + pos.x as i32 * UNIT_WIDTH as i32) as f32,
                (origin_y + pos.y as i32 * UNIT_HEIGHT as i32) as f32,
                0.0,
            );
            transform.scale = Vec3::new(UNIT_WIDTH as f32, UNIT_HEIGHT as f32, 0.0);
        });
}

pub fn size_scaling (
    mut transform_query: Query<&mut Transform, With<Position>>,
) {
    for mut transform in transform_query.iter_mut() {
        transform.scale = Vec3::new(UNIT_WIDTH as f32, UNIT_HEIGHT as f32, 0.0);
    }
}

pub fn position_translation(
    mut position_query: Query<(&Position, &mut Transform)>
){
    let origin_x = UNIT_WIDTH as i32 / 2 - SCREEN_WIDTH as i32 / 2;
    let origin_y = UNIT_HEIGHT as i32 / 2 - SCREEN_HEIGHT as i32 / 2;

    for( posision, mut transform) in position_query.iter_mut() {
        transform.translation = Vec3::new(
            (origin_x + posision.x as i32 * UNIT_WIDTH as i32) as f32,
            (origin_y + posision.y as i32 * UNIT_HEIGHT as i32) as f32,
            0.0,
        );
    }

}