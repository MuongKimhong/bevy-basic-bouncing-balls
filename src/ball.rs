use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;

pub const BALL_RADIUS: f32 = 20.0;
pub const PURPLE_BLOOM: Color = Color::srgb(4.0, 0.0, 4.0);
pub const BLUE_BLOOM: Color = Color::srgb(4.0, 0.0, 3.5);
pub const PINK_BLOOM: Color = Color::srgb(4.0, 1.0, 4.0);
pub const CYAN_BLOOM: Color = Color::srgb(0.0, 4.0, 4.0);
pub const BLOOM_COLORS: [Color; 4] = [PURPLE_BLOOM, BLUE_BLOOM, PINK_BLOOM, CYAN_BLOOM];
const BALL_TEXT_FONT_SIZE: f32 = 20.0;

#[derive(Component)]
pub struct BallCountText;

#[derive(Resource)]
pub struct TotalBallCount(pub u32);

pub fn get_random_color() -> Color {
    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..BLOOM_COLORS.len());
    BLOOM_COLORS[random_index]
}

pub fn create_ball(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    total_balls: &mut ResMut<TotalBallCount>,
    x: f32,
    y: f32,
) -> MaterialMesh2dBundle<ColorMaterial> {
    let ball = MaterialMesh2dBundle {
        mesh: meshes.add(Circle::new(BALL_RADIUS)).into(),
        material: materials.add(get_random_color()),
        transform: Transform::from_translation(Vec3::new(x, y, 0.)),
        ..default()
    };
    total_balls.0 += 1;
    ball
}

fn ball_text(asset_server: &Res<AssetServer>) -> TextSection {
    TextSection::new(
        " Balls: ",
        TextStyle {
            font: asset_server.load("Menlo-Regular.ttf"),
            font_size: BALL_TEXT_FONT_SIZE,
            ..default()
        },
    )
}

fn ball_count_text(asset_server: &Res<AssetServer>) -> TextSection {
    TextSection::new(
        "0",
        TextStyle {
            font: asset_server.load("Menlo-Regular.ttf"),
            font_size: BALL_TEXT_FONT_SIZE,
            ..default()
        },
    )
}

pub fn ball_text_bundle(asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        text: Text::from_sections([ball_text(asset_server), ball_count_text(asset_server)]),
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(25.0),
            ..default()
        },
        ..default()
    }
}

pub fn update_ball_count_system(
    total_balls: Res<TotalBallCount>,
    mut query: Query<&mut Text, With<BallCountText>>,
) {
    for mut text in &mut query {
        let count = total_balls.0;
        text.sections[1].value = format!("{count}");
    }
}
