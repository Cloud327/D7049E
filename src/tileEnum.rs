#[derive(Clone, Copy)]
pub enum TileEnum{
    startTile{},        // where enemies spawn
    endTile{},          // where unkilled enemies despawn and wreak havoc
    pathTile{},         // the path enemies will follow
    placableTile{},     // tiles towers can be placed on
    unplacableTile{},   // tiles towers cannot be placed on, either occupied by som obstacle or other tower
}