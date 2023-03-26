use game_objects::game_map::GameMap;
use game_objects::game_object::{
    GameObject, 
    WeaponState,
};

use quicksilver::{
    geom::{Vector},
    graphics::{Color, Image},
    input::Key,
    run, Graphics, Input, Result, Settings, Window,
};

fn main() {
    run(
        Settings {
            title: "some_game",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    let player_image = Image::load(&gfx, r"green_circle.png").await?;
    let enemy_image = Image::load(&gfx, r"green_circle.png").await?;
    let weapon_image = Image::load(&gfx, r"green_circle.png").await?;
    let death_image = Image::load(&gfx, r"red_x.png").await?;
    let wall_image = Image::load(&gfx, r"wall.png").await?;
    let floor_image = Image::load(&gfx, r"floor.png").await?;

    let game_map = GameMap::new(wall_image, floor_image);
    
    let mut player = GameObject::new(
        Vector::new(32.0, 32.0),  
        player_image.clone(),
        weapon_image,
    );

    let mut enemy = GameObject::new_no_weapon(
        Vector::new(600.0, 300.0), 
        enemy_image.clone(),
        Vector::new(32.0, 32.0),
        0.0,
        true,
    );
    
    loop {
        while let Some(_) = input.next_event().await {}
        
        player.set_image(player_image.clone());
        // enemy.set_image(enemy_image.clone());

        player.un_attack();

        if input.key_down(Key::A) {
            player.move_left(game_map.map());
        }
        if input.key_down(Key::D) {
            player.move_right(game_map.map());
        }
        if input.key_down(Key::W) {
            player.move_up(game_map.map());
        }
        if input.key_down(Key::S) {
            player.move_down(game_map.map());
        }
        if input.key_down(Key::Space) {
            player.attack();
        }

        if player.collides_with(&enemy) {
            player.set_image(death_image.clone());
        }

        if player.weapon().collides_with(&enemy) && player.weapon_state() == WeaponState::Attack {
            enemy.set_image(death_image.clone());
        }
        
        // enemy.move_towards(player.position(), game_map.map());

        gfx.clear(Color::WHITE);
        // Draw Map
        for tile in game_map.map() {
            gfx.draw_image(tile.image(), tile.sprite())
        }

        // Draw player
        gfx.draw_image(&player.image(), player.sprite());

        // Draw weapon
        gfx.draw_image(&player.weapon().image() ,player.weapon().sprite());

        //Draw enemy
        gfx.draw_image(&enemy.image(), enemy.sprite());

        gfx.present(&window)?;
    }
}