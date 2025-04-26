//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically, with per-frame speeds.
use bevy::prelude::*;
use std::ops::RangeInclusive;

/// Animation component that holds frame range, per-frame durations, and whether the animation loops
#[derive(Component)]
struct Animation {
    frames: RangeInclusive<usize>,
    frame_durations: Vec<f32>, // seconds for each frame
    looped: bool,
}

/// Simple timer wrapper for animation updates #[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .run();
}

/// Advances sprite animations frame-by-frame depending on their specific per-frame durations
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut Sprite, &Animation)>,
) {
    for (mut timer, mut sprite, animation) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                // Move to next frame
                let next_index = if atlas.index == *animation.frames.end() {
                    if animation.looped {
                        *animation.frames.start()
                    } else {
                        atlas.index // stay at last frame
                    }
                } else {
                    atlas.index + 1
                };

                atlas.index = next_index;

                // Update the timer duration for the new frame
                if let Some(&new_duration) = animation.frame_durations.get(next_index) {
                    timer.set_duration(std::time::Duration::from_secs_f32(new_duration));
                    timer.reset();
                }
            }
        }
    }
}

/// Sets up the animated sprite
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("gabe-idle-run.png"); // replace with your sprite sheet
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 1, // Start at frame 1
            },
        ),
        Transform::from_scale(Vec3::splat(6.0)),
        Animation {
            frames: 1..=6, // frames 1 to 6 inclusive
            frame_durations: vec![
                0.1,  // frame 1
                0.15, // frame 2
                0.3,  // frame 3
                0.3,  // frame 4
                0.15, // frame 5
                0.1,  // frame 6
            ],
            looped: true,
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
