mod colliders;
mod components;
mod contacts;
mod debug;
mod intersect;
mod math;
mod phases;

use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_inspector_egui::prelude::*;
use colliders::*;
use components::*;
use contacts::*;
use phases::*;
use prelude::PrevPos;

pub mod prelude {
    pub use crate::{
        colliders::*, components::*, contacts::*, debug::*, PhysicsBundle, PhysicsPlugin,
    };
}

pub struct PhysicsPlugin {
    pub gravity: Vec3,
    pub number_substeps: u32,
    pub number_position_iterations: u32,
    pub delta_time: f32,
    pub k: f32,
}

impl Default for PhysicsPlugin {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0., -9.81, 0.),
            number_substeps: 20,
            number_position_iterations: 1,
            delta_time: 1. / 60.,
            k: 2.0,
        }
    }
}

#[derive(Bundle, Default)]
pub struct PhysicsBundle {
    pub mode: PhysicsMode,
    pub mass: Mass,
    pub collider: Handle<Collider>,
    pub velocity: Velocity,
    pub restitution: Restitution,

    // Should not be set by user
    pub inverse_mass: InverseMass,
    pub inertia_tensor: InertiaTensor,
    pub inverse_inertia_tensor: InverseInertiaTensor,
    pub aabb: Aabb,
    pub prev_pos: PrevPos,
    pub prev_rot: PrevRot,
    pub pre_solve_velocity: PreSolveVelocity,
}

#[derive(Resource, InspectorOptions, Debug)]
//#[reflect(Resource, InspectorOptions)]
pub struct PhysicsConfig {
    pub number_substeps: u32,
    pub number_position_iterations: u32,
    pub delta_time: f32,
    pub sub_delta_time: f32, // h in the paper
    #[inspector(min = 1.0)]
    pub k: f32,
}

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct SubstepContacts(pub Vec<Contact>);

#[derive(Resource, Debug, Deref, Default, DerefMut)]
pub struct CollisionPairs(pub Vec<contacts::CollisionPair>);

#[derive(Resource, Debug)]
//#[reflect(Resource)]
pub struct Gravity(pub Vec3);

// This will be based on Algorith 2 (page 5) in https://github.com/matthias-research/pages/blob/master/publications/PBDBodies.pdf
//  while simulating do
//      CollectCollisionPairs();                                    broad phase
//      ℎ ← Δ𝑡/numSubsteps;
//      for numSubsteps do                                          intergrate phase
//          for 𝑛 bodies and particles do
//              xprev ← x;
//              v ← v + ℎ fext /𝑚;
//              x ← x + ℎ v;
//              qprev ← q;
//              𝜔 ← 𝜔 + ℎ I−1 (𝜏ext − (𝜔 × (I𝜔)));
//              q ← q + ℎ 21 [𝜔 𝑥 , 𝜔 𝑦 , 𝜔 𝑧 , 0] q;
//              q ← q/|q|;
//          end
//          for numPosIters do                                      solve positions phase
//              SolvePositions(x1 , . . . x𝑛 , q1 , . . . q𝑛 );
//          end
//          for 𝑛 bodies and particles do                           update velocities phase
//              v ← (x − xprev )/ℎ;
//              Δq ← q qprev-1
//              𝝎 ← 2[Δq 𝑥 , Δq 𝑦 , Δq 𝑧 ]/ℎ;
//              𝝎 ← Δ𝑞 𝑤 ≥ 0 ? 𝝎 : −𝝎;
//          end
//          SolveVelocities(v1 , . . . v𝑛 , 𝜔1 , . . . 𝜔 𝑛 );       solve velocities phase
//      end
//  end

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum Step {
    Setup,
    CollisionPairs,
    Integrate,
    SolvePositions,
    UpdateVelocities,
    SolveVelocities,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register the resources
            //.register_type::<PhysicsConfig>()
            //.add_plugin(ResourceInspectorPlugin::<PhysicsConfig>::default())
            //.register_type::<Gravity>()
            // Register the components
            .register_type::<Mass>()
            .register_type::<InverseMass>()
            .register_type::<InertiaTensor>()
            .register_type::<InverseInertiaTensor>()
            .register_type::<Aabb>()
            .register_type::<Restitution>()
            .register_type::<Velocity>()
            .register_type::<PreSolveVelocity>()
            .register_type::<PrevPos>()
            .register_type::<PrevRot>()
            // Add Asset
            .add_asset::<Collider>()
            // Add Resources
            .insert_resource(Gravity(self.gravity))
            .insert_resource(PhysicsConfig {
                number_substeps: self.number_substeps,
                number_position_iterations: self.number_position_iterations,
                delta_time: self.delta_time,
                sub_delta_time: self.delta_time / self.number_substeps as f32,
                k: self.k,
            })
            .init_resource::<LoopState>()
            .init_resource::<SubstepContacts>()
            .init_resource::<CollisionPairs>()
            // Add Events
            //.add_event::<CollisionPair>()
            //.add_event::<Contact>()
            // Add Systems
            .add_stage_before(
                CoreStage::Update,
                FixedUpdateStage,
                SystemStage::parallel()
                    .with_run_criteria(run_criteria)
                    .with_system_set(
                        SystemSet::new()
                            .label(Step::Setup)
                            .with_run_criteria(first_substep)
                            .with_system(setup_prev_pos)
                            .with_system(setup_mass_and_inertia)
                            .with_system(update_aabb),
                    )
                    .with_system_set(
                        SystemSet::new()
                            .label(Step::CollisionPairs)
                            .after(Step::Setup)
                            .with_run_criteria(first_substep)
                            .with_system(collision_pairs),
                    )
                    .with_system(integrate.label(Step::Integrate).after(Step::CollisionPairs))
                    .with_system_set(
                        SystemSet::new()
                            .label(Step::SolvePositions)
                            .after(Step::Integrate)
                            .with_system(solve_pos),
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
                            .with_system(solve_vel),
                    ),
            );
    }
}

#[derive(Resource)]
pub struct LoopState {
    pub accumulator: f32,
    pub substepping: bool,
    pub current_substep: u32,
    pub has_added_time: bool,
}

impl Default for LoopState {
    fn default() -> Self {
        Self {
            accumulator: 0.,
            substepping: false,
            current_substep: 0,
            has_added_time: false,
        }
    }
}

fn run_criteria(
    time: Res<Time>,
    mut state: ResMut<LoopState>,
    config: Res<PhysicsConfig>,
) -> ShouldRun {
    if !state.has_added_time {
        state.has_added_time = true;
        state.accumulator += time.delta_seconds();
    }

    if state.substepping {
        state.current_substep += 1;

        if state.current_substep < config.number_substeps {
            return ShouldRun::YesAndCheckAgain;
        } else {
            // We finished a whole step
            state.accumulator -= config.delta_time;
            state.current_substep = 0;
            state.substepping = false;
        }
    }

    if state.accumulator >= config.delta_time {
        state.substepping = true;
        state.current_substep = 0;
        ShouldRun::YesAndCheckAgain
    } else {
        state.has_added_time = false;
        ShouldRun::No
    }
}

fn first_substep(state: Res<LoopState>) -> ShouldRun {
    if state.current_substep == 0 {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

#[allow(dead_code)]
fn last_substep(state: Res<LoopState>, config: Res<PhysicsConfig>) -> ShouldRun {
    if state.current_substep == config.number_substeps - 1 {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
