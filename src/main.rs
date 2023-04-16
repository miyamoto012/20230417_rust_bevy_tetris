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
        .init_resource::<BlockFallTimer>()
        .init_resource::<GameBoard>()
        .add_event::<NewBlockEvent>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetris!".into(),
                resolution: (SCREEN_WIDTH as f32,  SCREEN_HEIGHT as f32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(send_new_block_event)
        .add_startup_system(spawn_camera)

        .add_system(bevy::window::close_on_esc)

        .add_system(tick_block_fall_timer)
        .add_system(spawn_block_element)
        .add_system(position_translation)

        .add_system(size_scaling)
        .add_system(block_fall)
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

pub struct NewBlockEvent;

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

pub const BLOCK_FALL_TIMER: f32 = 0.4;

#[derive(Resource)]
pub struct BlockFallTimer{
    pub timer: Timer,
}

impl Default for BlockFallTimer {
    fn default() -> Self {
        BlockFallTimer{
            timer: Timer::from_seconds(BLOCK_FALL_TIMER, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct FreeBlock{}

#[derive(Component)]
pub struct FixBlock{}

#[derive(Resource)]
pub struct GameBoard(Vec<Vec<bool>>);

impl Default for GameBoard {
    fn default() -> Self {
        GameBoard(vec![vec![false; 25]; 25])
    }
}




pub fn tick_block_fall_timer (
    mut block_fall_timer: ResMut<BlockFallTimer>,
    time: Res<Time>,
){
    block_fall_timer.timer.tick(time.delta());
}

pub fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}

pub fn send_new_block_event(
    mut new_block_event_reader: EventWriter<NewBlockEvent>
){
    new_block_event_reader.send(NewBlockEvent);
}

pub fn spawn_block_element(
    mut commands: Commands,
    materials: Res<Materials>,
    block_patterns: Res<BlockPatterns>,
    mut new_block_event_reader: EventReader<NewBlockEvent>,
){
    if new_block_event_reader.iter().count() == 0 {
        return;
    }

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
            },
            FreeBlock{}
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

pub fn block_fall(
    mut commands: Commands,
    mut block_query: Query<(Entity, &mut Position), With<FreeBlock>>,
    block_fall_timer: Res<BlockFallTimer>,
    mut game_board: ResMut<GameBoard>,
    mut new_block_events: EventWriter<NewBlockEvent>,
){
    if ! block_fall_timer.timer.finished() {
        return;
    }

    let cannot_fall = block_query.iter().any(|(_, position)| {
        if position.x as u32 >= X_LENGTH || position.y as u32 >= Y_LENGTH {
            return false;
        }

        position.y == 0 || game_board.0[(position.y - 1) as usize][position.x as usize]
    });

    if cannot_fall {
        for (entity, position) in block_query.iter() {
            commands.entity(entity)
                .remove::<FreeBlock>()
                .insert(FixBlock{});

            game_board.0[position.y as usize][position.x as usize] = true;
        }
        new_block_events.send(NewBlockEvent);
    } else {
        for(_ , mut block_position) in block_query.iter_mut() {
            block_position.y -= 1;
        }
    }
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