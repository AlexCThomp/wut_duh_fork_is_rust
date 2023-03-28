use std::collections::HashMap;

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

    let mut game_map = GameMap::new(wall_image, floor_image);

    let mut actors: HashMap<&str, Vec<GameObject>> = HashMap::from([
        ("players", Vec::new()),
        ("enemies", Vec::new()),
        ("bullets", Vec::new()),
    ]);

    actors.get_mut("players").expect("no players in actors map").push(
        GameObject::new_with_direction(
            Vector::new(32.0, 32.0),  
            arrow_up,
            arrow_left,
            arrow_down,
            arrow_right,
        )
    );

    actors.get_mut("enemies").expect("no enemies in actors map").push(
        GameObject::new(
            Vector::new(600.0, 300.0), 
            circle_image.clone(),
            Vector::new(32.0, 32.0),
            0.0,
            WeaponState::Attack,
            true,
        )
    );
    
    loop {
        while let Some(_) = input.next_event().await {}

        // moves
        if input.key_down(Key::A) {
            for player in actors.get_mut("players").expect("no players in actors map") {
                player.move_left();
            }
            
        }
        if input.key_down(Key::D) {
            for player in actors.get_mut("players").expect("no players in actors map") {
                player.move_right();
            }
        }
        if input.key_down(Key::W) {
            for player in actors.get_mut("players").expect("no players in actors map") {
                player.move_up();
            }
        }
        if input.key_down(Key::S) {
            for player in actors.get_mut("players").expect("no players in actors map") {
                player.move_down();
            }
        }

        // direction changes
        if input.key_down(Key::Left) {
            for player in actors.get_mut("players").expect("no players in actors map") {
                player.set_direction(Direction::Left);
            }
        }
        if input.key_down(Key::Right) {
            for player in actors.get_mut("players").expect("no players in actors map") {
                player.set_direction(Direction::Right);
            }
        }
        if input.key_down(Key::Up) {
            for player in actors.get_mut("players").expect("no players in actors map") {
                player.set_direction(Direction::Up);
            }
        }
        if input.key_down(Key::Down) {
            for player in actors.get_mut("players").expect("no players in actors map") {
                player.set_direction(Direction::Down);
            }
        }
        // if input.key_down(Key::Space) {
        //     player.shoot(); TODO: Implement shoot action for player
        // }
        
        for things in actors.values_mut() {
            for thing in things {
                thing.carry_momentum(game_map.map());
            }
        }
        // player.carry_momentum(game_map.map());

        gfx.clear(Color::WHITE);
        // Draw Map
        for tile in game_map.map() {
            gfx.draw_image(tile.image(), tile.sprite())
        }

        // Draw actors
        for things in actors.values_mut() {
            for thing in things {
                gfx.draw_image(&thing.image(), thing.sprite());
            }
        }
        // Draw player
        // gfx.draw_image(&player.image(), player.sprite());

        // Draw weapon
        // gfx.draw_image(&player.weapon().image() ,player.weapon().sprite());

        //Draw enemy
        // gfx.draw_image(&enemy.image(), enemy.sprite());

        gfx.present(&window)?;
    }
}