use game_objects::game_map::GameMap;
use game_objects::game_object::{
    GameObject, 
    Direction,
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
    let death_image = Image::load(&gfx, r"x.png").await?;
    let wall_image = Image::load(&gfx, r"barrier.png").await?;
    let floor_image = Image::load(&gfx, r"ice.png").await?;

    let game_map = GameMap::new(wall_image, floor_image);

    let mut player = GameObject::new_with_weapon(
        Vector::new(32.0, 32.0),  
        arrow_up,
        arrow_left,
        arrow_down,
        arrow_right,
        circle_image.clone()
    );

    let mut enemies: Vec<GameObject> = Vec::new();
    for _ in 0..10 {
        enemies.push(GameObject::new_random_enemy(circle_image.clone()));
    }

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
            player.update_direction(Direction::Left);
        }
        if input.key_down(Key::Right) {
            player.update_direction(Direction::Right);
        }
        if input.key_down(Key::Up) {
            player.update_direction(Direction::Up);
        }
        if input.key_down(Key::Down) {
            player.update_direction(Direction::Down);
        }
        if input.key_down(Key::Space) {
            player.shoot(&mut bullets);
        }

        // cull bullets
        bullets.retain(|bullet| {
            let delete = {
                bullet.out_of_range()
            };
            !delete
        });

        player.carry_momentum(game_map.map());
        for bullet in bullets.iter_mut(){
            bullet.carry_momentum(game_map.map());
        }

        gfx.clear(Color::WHITE);
        // Draw Map
        for tile in game_map.map() {
            gfx.draw_image(tile.image(), tile.sprite())
        }

        // Draw player
        gfx.draw_image(&player.image(), player.sprite());

        // Draw weapon
        gfx.draw_image(&player.weapon().image(), player.weapon().sprite());

        // Draw bullets
        for bullet in bullets.iter_mut(){
            gfx.draw_image(&bullet.image(), bullet.sprite());
        }

        // cull dead enemies
        enemies.retain(|enemy| {
            let delete = {enemy.got_shot(&bullets)};
            !delete
        });

        // Draw enemies
        for enemy in enemies.iter_mut() {
            for bullet in bullets.iter() {
                if bullet.collides_with(enemy){
                    enemy.set_image(death_image.clone());
                }
            }
            if enemy.collides_with(&player) {
                player.set_image(death_image.clone());
            }
            enemy.move_towards(player.position());
            enemy.carry_momentum(game_map.map());
            gfx.draw_image(&enemy.image(), enemy.sprite());
        }

        gfx.present(&window)?;
    }
}