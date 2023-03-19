mod character;
mod weapon;

use crate::character::{
    Character,
    WeaponState
};

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    input::Key,
    run, Graphics, Input, Result, Settings, Window,
};

// use crate::character::{
//     Character,
//     WeaponState
// };



fn main() {
    run(
        Settings {
            title: "Input Example",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    
    let mut player = Character::new();
    let enemy_position = Vector::new(600.0, 300.0);
    
    loop {
        while let Some(_) = input.next_event().await {}
        
        let mut enemy_color = Color::RED;
        let mut player_color = Color::BLUE;

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

        let enemy_shape = Rectangle::new(enemy_position, Vector::new(24.0, 24.0));

        if player.collides_with(enemy_shape) {
            player_color = Color::RED;
        }

        if player.weapon().collides_with(enemy_shape) && player.weapon_state() == WeaponState::Attack {
            enemy_color = Color::BLUE;
        }
        
        gfx.clear(Color::WHITE);    
        // Draw player
        gfx.fill_rect(&player.sprite(), player_color);

        // Draw weapon
        gfx.fill_rect(&player.weapon().sprite(), Color::ORANGE);

        //Draw enemy
        gfx.fill_rect(&enemy_shape, enemy_color);

        gfx.present(&window)?;
    }
}