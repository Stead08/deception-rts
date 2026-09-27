use bevy::{
    math::primitives::{Cuboid, Plane3d},
    prelude::*,
};

use crate::SimulationState;

#[derive(Component)]
pub struct StrategyCamera;

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    simulation: Res<SimulationState>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(40.0, 40.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.18, 0.28, 0.16))),
    ));

    let hull_mesh = meshes.add(Cuboid::new(2.4, 0.7, 1.5));
    let turret_mesh = meshes.add(Cuboid::new(1.1, 0.45, 1.0));
    let barrel_mesh = meshes.add(Cuboid::new(0.25, 0.25, 1.8));
    let tank_material = materials.add(Color::srgb(0.22, 0.36, 0.20));
    let accent_material = materials.add(Color::srgb(0.42, 0.50, 0.26));

    for unit in &simulation.0.units {
        let position = Vec3::new(unit.position.x, 0.55, unit.position.y);
        commands.spawn((
            Mesh3d(hull_mesh.clone()),
            MeshMaterial3d(tank_material.clone()),
            Transform::from_translation(position),
        ));
        commands.spawn((
            Mesh3d(turret_mesh.clone()),
            MeshMaterial3d(accent_material.clone()),
            Transform::from_translation(position + Vec3::Y * 0.55),
        ));
        commands.spawn((
            Mesh3d(barrel_mesh.clone()),
            MeshMaterial3d(accent_material.clone()),
            Transform::from_translation(position + Vec3::new(0.0, 0.62, -1.25)),
        ));
    }

    commands.spawn((
        DirectionalLight {
            illuminance: 12_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(14.0, 20.0, 22.0).looking_at(Vec3::ZERO, Vec3::Y),
        StrategyCamera,
    ));
}
