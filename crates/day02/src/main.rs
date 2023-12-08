mod input;

fn main() {
    let input = input::input::get_part_one_data();

    let mut games: Vec<Game> = Vec::new();

    // Parse the games
    for line in input.lines() {
        let game = parse_game_line(line);
        games.push(game);
    }

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut valid_game_id_sum = 0;
    let mut power_sum = 0;

    for game in games {
        let mut valid = true;

        let mut game_max_red = 0;
        let mut game_max_green = 0;
        let mut game_max_blue = 0;

        for reveal in game.reveals {
            if reveal.r > max_red || reveal.g > max_green || reveal.b > max_blue {
                valid = false;
                //println!("Game {0} invalid", game.id);
            }

            if reveal.r > game_max_red {
                game_max_red = reveal.r;
            }
            if reveal.g > game_max_green {
                game_max_green = reveal.g;
            }
            if reveal.b > game_max_blue {
                game_max_blue = reveal.b;
            }
        }

        let power = game_max_blue * game_max_green * game_max_red;
        power_sum += power;

        if valid {
            valid_game_id_sum += game.id;
        }
    }

    println!("part1: {valid_game_id_sum}");
    println!("part2: {power_sum}");
}

fn parse_game_line(line: &str) -> Game {
    let (game_id, reveals_part) = sscanf::sscanf!(line, "Game {i32}: {str}").unwrap();
    let mut game_reveals: Vec<Reveal> = Vec::new();
    for reveal in reveals_part.split(";") {
        let parts: Vec<&str> = reveal.split(" ").collect();
        let mut r = 0i32;
        let mut g = 0i32;
        let mut b = 0i32;
        for index in 0..parts.len() {
            if let Ok(number) = parts[index].parse::<i32>() {
                let next_char = parts[index + 1].chars().next().unwrap();
                match next_char {
                    'r' => { r = number; }
                    'g' => { g = number; }
                    'b' => { b = number; }
                    _ => panic!("Unexpected character")
                }
            }
        }

        //println!("game {0} reveal {r}r {g}g {b}b", game_id);
        game_reveals.push(Reveal { r, g, b });
    }

    Game {
        id: game_id,
        reveals: game_reveals,
    }
}

struct Game {
    id: i32,
    reveals: Vec<Reveal>,
}

#[derive(Copy, Clone)]
struct Reveal {
    r: i32,
    g: i32,
    b: i32,
}