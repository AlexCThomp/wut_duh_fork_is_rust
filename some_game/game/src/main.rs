use game_objects::{character::{
    Character,
    WeaponState
}, GameObject};

use quicksilver::{
    geom::{Vector},
    graphics::Color,
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
    
    let mut player = Character::new(
        Vector::new(300.0, 300.0), 
        Vector::new(32.0,32.0), 
        Color::BLUE,
    );

    let mut enemy = Character::new(
        Vector::new(600.0, 300.0), 
        Vector::new(40.0, 40.0), 
        Color::RED,
    );
    
    loop {
        while let Some(_) = input.next_event().await {}
        
        player.set_color(Color::BLUE);
        enemy.set_color(Color::RED);

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
            player.set_color(enemy.color());
        }

        if player.weapon().collides_with(&enemy) && player.weapon_state() == WeaponState::Attack {
            enemy.set_color(player.color());
        }
        
        enemy.move_towards(player.position());

        gfx.clear(Color::WHITE);    
        // Draw player
        gfx.fill_rect(&player.sprite(), player.color());

        // Draw weapon
        gfx.fill_rect(&player.weapon().sprite(), player.weapon().color());

        //Draw enemy
        gfx.fill_rect(&enemy.sprite(), enemy.color());

        gfx.present(&window)?;
    }
}