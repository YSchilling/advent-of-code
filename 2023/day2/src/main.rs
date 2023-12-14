use std::fs;

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
    valid: bool,
}

#[derive(Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    const RED_MAX: u32 = 12;
    const GREEN_MAX: u32 = 13;
    const BLUE_MAX: u32 = 14;

    //read file and get lines
    let file_path = "/mnt/e/dev/github/advent-of-code/2023/day2/input.txt";
    let file_content = fs::read_to_string(file_path).expect("Could not read file");
    let lines = file_content.split("\n");

    // convert lines to games
    let mut games = Vec::new();
    for line in lines {
        // split : game other
        // split ; draw draw draw
        // split , value color

        // extract id
        let mut colon_split = line.split(":");
        let game_str = colon_split.next().unwrap();
        let other_str = colon_split.next().unwrap();

        let id: u32 = game_str.split(" ").nth(1).unwrap().parse().unwrap();

        // extract draws
        let draws_str = other_str.split(";");
        let mut draws = Vec::new();
        for draw in draws_str {
            let splitted_draw = draw
                .split(",")
                .map(|split| split.strip_prefix(" ").unwrap());

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for pair in splitted_draw {
                let mut splitted_pair = pair.split(" ");
                let value = splitted_pair.next().unwrap().parse::<u32>().unwrap();
                let color = splitted_pair.next().unwrap();

                match color {
                    "red" => red = value,
                    "green" => green = value,
                    "blue" => blue = value,
                    _ => panic!(),
                }
            }

            draws.push(Draw { red, green, blue });
        }

        let game = Game {
            id,
            draws,
            valid: true,
        };
        games.push(game);
    }

    let mut sum = 0;
    for mut game in games {
        for draw in game.draws {
            if draw.red > RED_MAX || draw.green > GREEN_MAX || draw.blue > BLUE_MAX {
                game.valid = false;
            }
        }
        if game.valid {
            sum += game.id;
        }
    }
    println!("{sum}");
}
