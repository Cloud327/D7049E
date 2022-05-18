use kiss3d::scene::SceneNode;
use rapier3d::prelude::ColliderHandle;

use super::{typeEnum::TypeEnum, Components::{typeComponent::TypeComponent, renderableComponent::RenderableComponent}};


pub enum EventEnum{
    takeDamageEvent{id: usize, damage: usize},  // Id of enemy tower
    towerAttackEvent{xTarget: f32, yTarget: f32, zTarget: f32},  
    //spawnTowerEvent{x: usize, y: usize, z: usize},
    //spawnEnemyEvent,
    spawnProjectileEvent{x: usize, y: usize, z: usize},
    removeObjectEvent{id: usize, colliderHandle: ColliderHandle},
}
