pub mod game_object;
pub mod game_map;


#[derive(PartialEq, Copy, Clone)]
pub enum GameObjectType {
    Enemy,
    Player,
    Weapon,
    Wall,
    Floor,
}