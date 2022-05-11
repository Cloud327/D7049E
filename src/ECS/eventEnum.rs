
pub enum EventEnum{
    takeDamageEvent{id: usize, damage: usize},  // Id of enemy tower
    towerAttackEvent{xTarget: usize, yTarget: usize, zTarget: usize},  
    spawnTowerEvent{x: usize, y: usize, z: usize},
    spawnEnemyEvent,
    spawnProjectileEvent{x: usize, y: usize, z: usize},
}

