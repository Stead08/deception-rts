use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::renderer::StrategyCamera;

pub fn zoom_camera(
    mut wheel: MessageReader<MouseWheel>,
    mut cameras: Query<&mut Transform, With<StrategyCamera>>,
) {
    let amount: f32 = wheel.read().map(|event| event.y).sum();
    if amount.abs() < f32::EPSILON {
        return;
    }

    for mut transform in &mut cameras {
        let forward = transform.forward();
        let candidate = transform.translation + forward * amount * 1.5;
        if (8.0..=45.0).contains(&candidate.y) {
            transform.translation = candidate;
        }
    }
}
