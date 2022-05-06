pub enum EventEnum{
    takeDamageEvent{id: usize, damage: usize},  // Id of enemy tower
    towerAttackEvent{x: usize, y: usize, z: usize},  
    spawnTowerEvent{x: usize, y: usize, z: usize},
    spawnEnemyEvent,
    spawnProjectileEvent{x: usize, y: usize, z: usize},
}