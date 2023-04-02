use std::num::NonZeroUsize;

use game_objects::game_map::GameMap;
use game_objects::game_object::{
    GameObject, 
    Direction,
};

use quicksilver::Timer;
use quicksilver::input::{Event, GamepadAxis, GamepadButton};
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

    let game_map = GameMap::new(&wall_image, &floor_image);

    let mut enemy_spawn_timer = Timer::time_per_second(0.2);


    let mut player = GameObject::new_with_weapon(
        Vector::new(32.0, 32.0),  
        &arrow_up,
        &arrow_left,
        &arrow_down,
        &arrow_right,
        &circle_image
    );

    let mut enemies: Vec<GameObject> = Vec::new();
    for _ in 0..1 {
        enemies.push(GameObject::new_random_enemy(&circle_image));
    }

    let mut bullets: Vec<GameObject> = Vec::new();

    let mut left_stick = Vector::new(0.0, 0.0);
    let mut right_stick = Vector::new(0.0, 0.0);

    loop {
        while let Some(event) = input.next_event().await {
            match event {
                Event::GamepadAxis(axis_event) => {
                    if axis_event.axis() == GamepadAxis::RightStickX {
                        right_stick.x = axis_event.value();
                    }
                    if axis_event.axis() == GamepadAxis::RightStickY {
                        right_stick.y = -axis_event.value();
                    }
                    if axis_event.axis() == GamepadAxis::LeftStickX {
                        left_stick.x = axis_event.value();
                    }
                    if axis_event.axis() == GamepadAxis::LeftStickY {
                        left_stick.y = -axis_event.value();
                    }
                    // println!("left_stick x: {} y: {}", left_stick.x, left_stick.y);
                    player.set_acceleration(left_stick);
                    player.set_direction(right_stick);
                },
                Event::GamepadButton(button_event) =>{
                    if button_event.button() == GamepadButton::RightTrigger {
                        player.shoot(&mut bullets);
                    }
                },
                _ => (),
            }
        }
  
        //move
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

        // let acceleration = player.acceleration();
        // println!("player.acceleration x: {} y: {}", acceleration.x, acceleration.y);

        player.accelerate();
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
        // gfx.draw_image(&player.weapon().image(), player.weapon().sprite());
        // Draw Aim Line
        gfx.stroke_path(&[player.center(), player.weapon().center()], Color::RED);

        // Draw bullets
        for bullet in bullets.iter_mut(){
            gfx.draw_image(&bullet.image(), bullet.sprite());
        }

        // cull dead enemies
        enemies.retain(|enemy| {
            let delete = {enemy.got_shot(&bullets)};
            !delete
        });

        // spawn new enemies
        if enemy_spawn_timer.exhaust() >= NonZeroUsize::new(1) {
            enemies.push(GameObject::new_random_enemy(&circle_image));
            enemy_spawn_timer.reset();
        }
        
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
            enemy.patrol_for(&player);
            enemy.carry_momentum(game_map.map());
            gfx.draw_image(&enemy.image(), enemy.sprite());
        }

        gfx.present(&window)?;
    }
}