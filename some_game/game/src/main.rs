use game_objects::game_map::GameMap;
use game_objects::game_object::{
    GameObject, 
    WeaponState, Direction,
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
    let arrow_up = Image::load(&gfx, r"arrow_up.png").await?;
    let arrow_left = Image::load(&gfx, r"arrow_left.png").await?;
    let arrow_down = Image::load(&gfx, r"arrow_down.png").await?;
    let arrow_right = Image::load(&gfx, r"arrow_right.png").await?;

    let circle_image = Image::load(&gfx, r"circle.png").await?;
    // let death_image = Image::load(&gfx, r"x.png").await?;
    let wall_image = Image::load(&gfx, r"barrier.png").await?;
    let floor_image = Image::load(&gfx, r"ice.png").await?;

    let game_map = GameMap::new(wall_image, floor_image);

    let mut player = GameObject::new_with_direction(
        Vector::new(32.0, 32.0),  
        arrow_up,
        arrow_left,
        arrow_down,
        arrow_right,
    );

    let mut enemies: Vec<GameObject> = Vec::new();
    enemies.push(
        GameObject::new(
            Vector::new(600.0, 300.0), 
            circle_image.clone(),
            Vector::new(32.0, 32.0),
            Vector::new(0.0,0.0),
            0.0,
            WeaponState::Attack,
            true,
        )
    );

    let mut bullets: Vec<GameObject> = Vec::new();
    
    loop {
        while let Some(_) = input.next_event().await {}

        // moves
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

        // direction changes
        if input.key_down(Key::Left) {
            player.set_direction(Direction::Left);
        }
        if input.key_down(Key::Right) {
            player.set_direction(Direction::Right);
        }
        if input.key_down(Key::Up) {
            player.set_direction(Direction::Up);
        }
        if input.key_down(Key::Down) {
            player.set_direction(Direction::Down);
        }
        if input.key_down(Key::Space) {
            player.shoot(&mut bullets);
        }

        // cull bullets
        bullets.retain(|bullet| {
            let delete = {
                bullet.position().x > (32.0*32.0) ||
                bullet.position().x < 0.0 ||
                bullet.position().y > (24.0*32.0) ||
                bullet.position().y < 0.0
            };
            !delete
        });

        player.carry_momentum(game_map.map());
        for bullet in bullets.iter_mut(){
            bullet.carry_momentum(game_map.map());
        }
        // player.carry_momentum(game_map.map());

        gfx.clear(Color::WHITE);
        // Draw Map
        for tile in game_map.map() {
            gfx.draw_image(tile.image(), tile.sprite())
        }

        // Draw player
        gfx.draw_image(&player.image(), player.sprite());

        // Draw bullets
        for bullet in bullets.iter_mut(){
            gfx.draw_image(&bullet.image(), bullet.sprite());
        }

        // Draw enemies
        for enemy in enemies.iter_mut() {
            gfx.draw_image(&enemy.image(), enemy.sprite());
        }

        gfx.present(&window)?;
    }
}