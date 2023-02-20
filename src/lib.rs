mod colliders;
mod components;
mod contacts;
mod phases;

use bevy::{prelude::*, time::FixedTimestep};
use colliders::*;
use components::*;
use contacts::*;
use phases::*;

pub mod prelude {
    pub use crate::{colliders::*, components::*, contacts::*, PhysicsPlugin};
}

pub const DELTA_TIME: f32 = 1. / 60.;
pub const BOUNDS_EPS: f32 = 0.01;

pub struct PhysicsPlugin {
    pub gravity: Vec3,
}

impl Default for PhysicsPlugin {
    fn default() -> Self {
        Self {
            gravity: Gravity::default().0,
        }
    }
}

#[derive(Resource, Debug)]
pub struct Gravity(pub Vec3);

impl Default for Gravity {
    fn default() -> Self {
        Self(Vec3::new(0., -9.81, 0.))
    }
}

// #[derive(Resource, Default, Debug)]
// pub struct DynamicContacts(pub Vec<(Entity, Entity, Vec3)>);

// #[derive(Resource, Default, Debug)]
// pub struct StaticContacts(pub Vec<(Entity, Entity, Vec3)>);

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum Step {
    UpdateAABB,
    BroadPhase,
    Integrate,
    SolvePositions,
    UpdateVelocities,
    SolveVelocities,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(self.gravity))
            .add_event::<BroadContactStatic>()
            .add_event::<BroadContactDynamic>()
            .add_event::<ContactDynamic>()
            .add_event::<ContactStatic>()            
            .add_asset::<Collider>()
            .add_stage_before(
                CoreStage::Update,
                FixedUpdateStage,
                SystemStage::parallel()
                    .with_run_criteria(FixedTimestep::step(DELTA_TIME as f64))
                    .with_system_set(
                        SystemSet::new()
                            .label(Step::UpdateAABB)
                            .with_system(update_aabb)
                            .with_system(update_aabb_static),
                        
                    )
                    .with_system(broad_phase.label(Step::BroadPhase).after(Step::UpdateAABB))
                    .with_system(integrate.label(Step::Integrate).after(Step::BroadPhase))
                    .with_system_set(
                        SystemSet::new()
                            .label(Step::SolvePositions)
                            .after(Step::Integrate)
                            .with_system(solve_pos)
                            .with_system(solve_pos_statics),
                    )
                    .with_system(
                        update_vel
                            .label(Step::UpdateVelocities)
                            .after(Step::SolvePositions),
                    )
                    .with_system_set(
                        SystemSet::new()
                            .label(Step::SolveVelocities)
                            .after(Step::UpdateVelocities)
                            .with_system(solve_vel)
                            .with_system(solve_vel_statics),
                    ),
            );
    }
}

fn integrate(
    mut query: Query<(
        &mut Transform,
        &mut PrevPos,
        &mut Velocity,
        &mut PreSolveVelocity,
        &Mass,
    )>,
    gravity: Res<Gravity>,
) {
    for (mut trans, mut prev_pos, mut vel, mut pre_solve_vel, mass) in query.iter_mut() {
        prev_pos.0 = trans.translation;

        let gravitation_force = mass.0 * gravity.0;
        let external_forces = gravitation_force;
        vel.linear += DELTA_TIME * external_forces / mass.0;
        trans.translation += DELTA_TIME * vel.linear;
        pre_solve_vel.linear = vel.linear;
    }
}

fn update_vel(mut query: Query<(&Transform, &PrevPos, &mut Velocity)>) {
    for (trans, prev_pos, mut vel) in query.iter_mut() {
        vel.linear = (trans.translation - prev_pos.0) / DELTA_TIME;
    }
}

fn solve_vel(
    query: Query<(
        &mut Velocity,
        &PreSolveVelocity,
        &Mass,
        &Restitution,
    )>,
    mut contacts: EventReader<ContactDynamic>,
) {
    for c in contacts.iter() {
        let (
            (mut vel_a, pre_solve_vel_a, mass_a, restitution_a),
            (mut vel_b, pre_solve_vel_b, mass_b, restitution_b),
        ) = unsafe {
            // Ensure safety
            assert!(c.entity_a != c.entity_b);
            (
                query.get_unchecked(c.entity_a).unwrap(),
                query.get_unchecked(c.entity_b).unwrap(),
            )
        };
        // Make sure velocities are reflected and restitution/friction calculated
        let pre_solve_relative_vel = pre_solve_vel_a.linear - pre_solve_vel_b.linear;
        let pre_solve_normal_vel = pre_solve_relative_vel.dot(c.normal);

        let relative_vel = vel_a.linear - vel_b.linear;
        let normal_vel = relative_vel.dot(c.normal);
        // averaging restitution
        let restitution = (restitution_a.0 + restitution_b.0) / 2.;

        let w_a = 1. / mass_a.0;
        let w_b = 1. / mass_b.0;
        let w_sum = w_a + w_b;

        vel_a.linear += c.normal * (-normal_vel - restitution * pre_solve_normal_vel) * w_a / w_sum;
        vel_b.linear -= c.normal * (-normal_vel - restitution * pre_solve_normal_vel) * w_b / w_sum;
    }
}

fn solve_vel_statics(
    mut dynamics: Query<(&mut Velocity, &PreSolveVelocity, &Restitution), With<Mass>>,
    statics: Query<&Restitution, Without<Mass>>,    
    mut contacts: EventReader<ContactStatic>,
) {
    for c in contacts.iter() {
        let (mut vel_a, pre_solve_vel_a, restitution_a) = dynamics.get_mut(c.entity).unwrap();
        let restitution_b = statics.get(c.static_entity).unwrap();
        let pre_solve_normal_vel = pre_solve_vel_a.linear.dot(c.normal);
        let normal_vel = vel_a.linear.dot(c.normal);
        let restitution = (restitution_a.0 + restitution_b.0) / 2.;
        vel_a.linear += c.normal * (-normal_vel - restitution * pre_solve_normal_vel);
    }
}
