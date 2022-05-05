
pub enum EventEnum{
    takeDamageEvent{id: usize, damage: usize},  // Id of enemy t
    towerAttackEvent{x: usize, y: usize, z: usize},  
    spawnTowerEvent{x: usize, y: usize, z: usize},
    spawnEnemyEvent{x: usize, y: usize, z: usize},
    spawnProjectileEvent{x: usize, y: usize, z: usize},
}

