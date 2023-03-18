mod quick_maffs;

use std::collections::HashMap;

use quicksilver::{
    geom::{Circle, Rectangle, Vector, Shape},
    graphics::Color,
    input::Key,
    run, Graphics, Input, Result, Settings, Window,
};

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
    
    const SPEED: f32 = 2.0;
    const RANGE: f32 = 24.0;

    let mut player_position = Vector::new(300.0, 300.0);
    let enemy_position = Vector::new(600.0, 300.0);
    let mut weapon_position = Vector::new(player_position.x+32.0, player_position.y-12.0);
    let mut weapon_direction = "right";
    
    loop {
        while let Some(_) = input.next_event().await {}
        
        let mut enemy_color = Color::RED;
        let mut player_color = Color::BLUE;

        let weapon_directions = HashMap::from([
            ("up", HashMap::from([
                ("default", Vector::new(player_position.x-12.0, player_position.y-56.0)),
                ("attack", Vector::new(player_position.x-12.0, player_position.y-(56.0+RANGE))),
            ])),
            ("right", HashMap::from([
                ("default", Vector::new(player_position.x+32.0, player_position.y-12.0)),
                ("attack", Vector::new(player_position.x+32.0+RANGE, player_position.y-12.0)),
            ])),
            ("down", HashMap::from([
                ("default", Vector::new(player_position.x-12.0, player_position.y+32.0)),
                ("attack", Vector::new(player_position.x-12.0, player_position.y+32.0+RANGE)),
            ])),
            ("left", HashMap::from([
                ("default", Vector::new(player_position.x-56.0, player_position.y-12.0)),
                ("attack", Vector::new(player_position.x-(56.0+RANGE), player_position.y-12.0)),
            ])),
        ]);
        
        weapon_position = weapon_directions[weapon_direction]["default"];

        if input.key_down(Key::A) {
            player_position.x -= SPEED;
            weapon_direction = "left";
        }
        if input.key_down(Key::D) {
            player_position.x += SPEED;
            weapon_direction = "right";
        }
        if input.key_down(Key::W) {
            player_position.y -= SPEED;
            weapon_direction = "up";
        }
        if input.key_down(Key::S) {
            player_position.y += SPEED;
            weapon_direction = "down";
        }
        if input.key_down(Key::Space) {
            weapon_position = weapon_directions[weapon_direction]["attack"];
        }

        let player_shape = Circle::new(player_position, 32.0);
        let enemy_shape = Rectangle::new(enemy_position, Vector::new(24.0, 24.0));
        let weapon_shape = Rectangle::new(weapon_position, Vector::new(24.0, 24.0));

        if player_shape.overlaps_rectangle(&enemy_shape) {
            player_color = Color::RED;
        }

        if weapon_shape.overlaps_rectangle(&enemy_shape) && input.key_down(Key::Space) {
            enemy_color = Color::BLUE;
        }
        
        gfx.clear(Color::WHITE);    
        // Draw player
        gfx.fill_circle(&player_shape, player_color);

        // Draw weapon
        gfx.fill_rect(&weapon_shape, Color::ORANGE);

        //Draw enemy
        gfx.fill_rect(&enemy_shape, enemy_color);

        gfx.present(&window)?;
    }
}