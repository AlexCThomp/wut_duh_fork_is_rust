use game_objects::{character::{
    Character,
    WeaponState
}, GameObject};

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
    
    let mut player = Character::new(
        Vector::new(300.0, 300.0), 
        Vector::new(32.0,32.0), 
        player_image.clone(),
        weapon_image,
    );

    let mut enemy = Character::new_no_weapon(
        Vector::new(600.0, 300.0), 
        Vector::new(40.0, 40.0), 
        enemy_image.clone(),
    );
    
    loop {
        while let Some(_) = input.next_event().await {}
        
        player.set_image(player_image.clone());
        enemy.set_image(enemy_image.clone());

        player.un_attack();

        if input.key_down(Key::A) {
            player.move_left();
        }
        if input.key_down(Key::D) {
            player.move_right();
        }
        if input.key_down(Key::W) {
            player.move_up();
        }
        if input.key_down(Key::S) {
            player.move_down();
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
        
        enemy.move_towards(player.position());

        gfx.clear(Color::WHITE);    
        // Draw player
        gfx.draw_image(&player.image(), player.sprite());

        // Draw weapon
        gfx.draw_image(&player.weapon().image() ,player.weapon().sprite());

        //Draw enemy
        gfx.draw_image(&enemy.image(), enemy.sprite());

        gfx.present(&window)?;
    }
}