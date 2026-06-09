use crossterm::cursor::MoveTo;
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::style::{Color, Stylize};
use crossterm::terminal::{Clear, ClearType, enable_raw_mode};
use rand::rngs::ThreadRng;
use rand::{RngExt, rng};
use std::io::{Write, stdout};
use std::thread;
use std::time::Duration;

struct Entity {
    x: i8,
    y: i8,
}

fn main() -> () {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    enable_raw_mode().unwrap();

    let mut input_char: Option<char> = None;
    let mut paused: bool = false;

    let mut player: Entity = Entity { x: 0, y: 0 };
    let mut enemy: Entity = Entity { x: 0, y: 0 };
    let mut loot: Entity = Entity { x: 0, y: 0 };

    let mut score: i16 = -10;

    let mut tick: i64 = 0;
    let mut health: u8 = 20;

    let mut my_rng: ThreadRng = rng();

    render(score, &player, &enemy, &loot, health);
    let _ = event::read();
    let _ = write!(stdout(), "{}", MoveTo(0, 0));
    let _ = write!(stdout(), "{}", Clear(ClearType::All));

    loop {
        let input: bool = event::poll(Duration::from_millis(0)).unwrap();

        if input {
            if let Event::Key(key_event) = event::read().unwrap() {
                if let KeyCode::Char(c) = key_event.code {
                    input_char = Some(c);
                    if key_event.code == KeyCode::Char('p') {
                        paused = !paused;
                    }
                }
            }
        }

        if !paused {
            if input {
                if let Some(c) = input_char {
                    r#move(c, &mut player)
                };
            }
            loot_logic(
                &mut player,
                &mut enemy,
                &mut loot,
                &mut score,
                &mut health,
                &mut tick,
                &mut paused,
                &mut my_rng,
            );
            enemy_logic(
                &mut player,
                &mut enemy,
                &mut loot,
                &mut score,
                &mut health,
                &mut tick,
                &mut paused,
            );
            render(score, &player, &enemy, &loot, health);
            tick += 1;
        }

        input_char = None;
        thread::sleep(Duration::from_millis(
            (1000 as u64) / (30 /* change here for FPS */ as u64),
        ));
    }
}

fn r#move(c: char, player: &mut Entity) -> () {
    match c {
        'w' => {
            player.y -= 1;
            if player.y < 0 {
                player.y = 9;
            }
        }
        'a' => {
            player.x -= 1;
            if player.x < 0 {
                player.x = 9;
            }
        }
        's' => {
            player.y += 1;
            if player.y > 9 {
                player.y = 0;
            }
        }
        'd' => {
            player.x += 1;
            if player.x > 9 {
                player.x = 0;
            }
        }
        _ => {}
    }
}

fn loot_logic(
    mut player: &mut Entity,
    mut enemy: &mut Entity,
    mut loot: &mut Entity,
    mut score: &mut i16,
    mut health: &mut u8,
    mut tick: &mut i64,
    mut paused: &mut bool,
    my_rng: &mut ThreadRng,
) -> () {
    if player.x != loot.x || player.y != loot.y {
        return;
    }
    loot.x = my_rng.random_range(0..10);
    loot.y = my_rng.random_range(0..10);

    *score += 10;

    if *score >= 500 {
        render(*score, player, enemy, loot, *health);
        println!("\nYou've won!!!");
        restart(
            &mut player,
            &mut loot,
            &mut enemy,
            &mut health,
            &mut score,
            &mut tick,
            &mut paused,
        );
    }
}

fn enemy_logic(
    mut player: &mut Entity,
    mut enemy: &mut Entity,
    mut loot: &mut Entity,
    mut score: &mut i16,
    mut health: &mut u8,
    mut tick: &mut i64,
    mut paused: &mut bool,
) -> () {
    if *tick % 10 != 0 {
        return;
    }

    let x_dist: i8 = enemy.x - player.x;
    let y_dist: i8 = enemy.y - player.y;

    if x_dist.abs() > y_dist.abs() {
        enemy.x -= x_dist.signum();
    } else {
        enemy.y -= y_dist.signum();
    }

    if enemy.x != player.x || enemy.y != player.y {
        return;
    }

    *score -= 5;
    *health -= 1;

    if *health == 0 {
        render(*score, player, enemy, loot, *health);
        println!("\nYou've lose :(");
        restart(
            &mut player,
            &mut loot,
            &mut enemy,
            &mut health,
            &mut score,
            &mut tick,
            &mut paused,
        );
    }
}

fn render(score: i16, player: &Entity, enemy: &Entity, loot: &Entity, health: u8) -> () {
    let mut buffer: [char; 155] = [
        '+', '+', '+', '+', '+', '+', '+', '+', '+', '+', '+', '+', '\n', '+', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', '+', '\n', '+', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', '+', '\n', '+', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '+', '\n', '+', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '+', '\n', '+', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', '+', '\n', '+', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '+',
        '\n', '+', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '+', '\n', '+', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', '+', '\n', '+', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', '+', '\n', '+', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '+', '\n', '+',
        '+', '+', '+', '+', '+', '+', '+', '+', '+', '+', '+',
    ];
    buffer[((player.x as usize) + 1) + 13 * ((player.y as usize) + 1)] = '@';
    buffer[((enemy.x as usize) + 1) + 13 * ((enemy.y as usize) + 1)] = '!';
    buffer[((loot.x as usize) + 1) + 13 * ((loot.y as usize) + 1)] = '$';
    println!(
        "{}, {}, {}, {}, {}, {}",
        player.x, player.y, enemy.x, enemy.y, loot.x, loot.y
    );

    let _ = write!(stdout(), "{}", MoveTo(0, 0));
    let _ = write!(stdout(), "{:<4}", score);

    print_coloured_px('r', health as usize);
    print_coloured_px('d', health as usize);

    println!("");

    for i in 0..buffer.len() {
        match buffer[i] {
            ' ' => {
                print!("  ");
            }
            '+' => {
                print_coloured_px('w', 2);
            }
            '@' => {
                print_coloured_px('g', 2);
            }
            '$' => {
                print_coloured_px('y', 2);
            }
            '!' => {
                print_coloured_px('r', 2);
            }
            '\n' => {
                println!("")
            }
            _ => {}
        }
    }
}

fn print_coloured_px(colour: char, len: usize) {
    let _ = write!(
        stdout().lock(),
        "{}",
        " ".repeat(len).on(match colour {
            'b' => Color::Black,
            'r' => Color::Red,
            'g' => Color::Green,
            'y' => Color::Yellow,
            'l' => Color::Blue,
            'm' => Color::Magenta,
            'c' => Color::Cyan,
            'w' => Color::White,
            _ => Color::Reset,
        }),
    );
}

fn restart(
    player: &mut Entity,
    loot: &mut Entity,
    enemy: &mut Entity,
    health: &mut u8,
    score: &mut i16,
    tick: &mut i64,
    paused: &mut bool,
) -> () {
    let _ = event::read();
    print!("\x1b[H\x1b[2J");
    stdout().flush().unwrap();

    *player = Entity { x: 0, y: 0 };
    *loot = Entity { x: 0, y: 0 };
    *enemy = Entity { x: 0, y: 0 };

    *score = -10;
    *tick = 0;
    *health = 20;

    *paused = false;

    render(*score, &player, &enemy, &loot, *health);
}
