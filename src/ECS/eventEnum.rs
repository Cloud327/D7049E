
pub enum EventEnum{
    takeDamageEvent{id: usize, damage: usize},  // Id of enemy tower
    towerAttackEvent{xTarget: f32, yTarget: f32, zTarget: f32},  
    spawnTowerEvent{x: usize, y: usize, z: usize},
    spawnEnemyEvent,
    spawnProjectileEvent{x: usize, y: usize, z: usize},
}
