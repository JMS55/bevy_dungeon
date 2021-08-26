use crate::actions::{Action, PrintEntityAction};
use crate::bundles::SpriteBundleExt;
use crate::components::{Actor, GridPosition, Health, TurnGroup};
use crate::world::ImmutableWorld;
use bevy::prelude::{Bundle, Entity, SpriteBundle};

#[derive(Bundle)]
pub struct SkeletonScout {
    position: GridPosition,
    health: Health,
    actor: Actor,
    #[bundle]
    sprite: SpriteBundle,
}

impl SkeletonScout {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            position: GridPosition::new(x, y),
            health: Health::new(20),
            actor: Actor::new(print_entity_brain, TurnGroup::Enemy),
            sprite: SpriteBundle::new("skeleton_scout.png", x, y),
        }
    }
}

fn print_entity_brain(entity: Entity, _: &mut ImmutableWorld) -> Option<Box<dyn Action>> {
    PrintEntityAction { entity }.to_brain_decision()
}
