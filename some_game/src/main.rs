mod quick_maffs;

// Example 8: Input
// Respond to user keyboard and mouse input onscreen
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
    // Keep track of the position of the square
    let mut player_position = Vector::new(300.0, 300.0);
    let enemy_position = Vector::new(600.0, 300.0);
    
    
    loop {
        while let Some(_) = input.next_event().await {}
        // Check the state of the keys, and move the square accordingly
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

        let weapon_hit_enemy: bool =
            weapon_position.y <= enemy_position.y+24.0
            && enemy_position.y <= weapon_position.y+24.0
            && weapon_position.x <= enemy_position.x+24.0
            && enemy_position.x <= weapon_position.x+24.0
            && input.key_down(Key::Space);
        
        let enemy_hit_player: bool = (
                quick_maffs::distance(player_position, enemy_position) <= 32.0
            )
            ||
            (quick_maffs::distance(
                    player_position, 
                    Vector::new(enemy_position.x+24.0, enemy_position.y+24.0)
                ) <= 32.0)
            ||
            (quick_maffs::distance(
                player_position, 
                Vector::new(enemy_position.x+24.0, enemy_position.y)
            ) <= 32.0)
            ||
            (quick_maffs::distance(
                player_position, 
                Vector::new(enemy_position.x, enemy_position.y+24.0)
            ) <= 32.0);

        if weapon_hit_enemy {
            enemy_color = Color::BLUE;
        }
        if enemy_hit_player {
            player_color = Color::RED;
        }
        
        gfx.clear(Color::WHITE);    
        // Draw player
        gfx.fill_circle(&Circle::new(player_position, 32.0), player_color);

        // Draw weapon
        gfx.fill_rect(
            &Rectangle::new(weapon_position, Vector::new(24.0, 24.0)),
            Color::ORANGE,
        );

        //Draw enemy
        gfx.fill_rect(
            &Rectangle::new(enemy_position, Vector::new(24.0, 24.0)),
            enemy_color,
        );

        gfx.present(&window)?;
    }
}