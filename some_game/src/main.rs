mod quick_maffs;


use quicksilver::{
    geom::{Circle, Rectangle, Vector},
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
    
    let mut player_position = Vector::new(300.0, 300.0);
    let enemy_position = Vector::new(600.0, 300.0);
    
    
    loop {
        while let Some(_) = input.next_event().await {}
        
        const SPEED: f32 = 2.0;
        const RANGE: f32 = 24.0;
        let mut weapon_position = Vector::new(player_position.x+32.0, player_position.y-12.0);
        let mut enemy_color: Color = Color::RED;
        let mut player_color: Color = Color::BLUE;

        if input.key_down(Key::A) {
            player_position.x -= SPEED;
        }
        if input.key_down(Key::D) {
            player_position.x += SPEED;
        }
        if input.key_down(Key::W) {
            player_position.y -= SPEED;
        }
        if input.key_down(Key::S) {
            player_position.y += SPEED;
        }
        if input.key_down(Key::Space) {
            weapon_position.x += RANGE;
        }

        let player_shape: Circle = Circle::new(player_position, 32.0);
        let enemy_shape: Rectangle = Rectangle::new(enemy_position, Vector::new(24.0, 24.0));
        let weapon_shape: Rectangle = Rectangle::new(weapon_position, Vector::new(24.0, 24.0));

        let weapon_hit_enemy: bool =
            weapon_position.y <= enemy_position.y+24.0
            && enemy_position.y <= weapon_position.y+24.0
            && weapon_position.x <= enemy_position.x+24.0
            && enemy_position.x <= weapon_position.x+24.0
            && input.key_down(Key::Space);
        


        if quick_maffs::collision_rectangle_circle(player_shape, enemy_shape) {
            player_color = Color::RED;
        }

        if weapon_hit_enemy {
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