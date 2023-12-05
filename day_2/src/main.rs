use std::fs;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn build() -> Self {
        Game {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn build_full(red: u32, green: u32, blue: u32) -> Self {
        Game { red, green, blue }
    }

    fn red(&mut self, val: u32) -> &Self {
        self.red = val;
        self
    }

    fn green(&mut self, val: u32) -> &Self {
        self.green = val;
        self
    }

    fn blue(&mut self, val: u32) -> &Self {
        self.blue = val;
        self
    }

    fn is_possible(&self, max_game: &Self) -> bool {
        self.red <= max_game.red && self.green <= max_game.green && self.blue <= max_game.blue
    }
}

#[derive(Debug, Clone)]
struct Session {
    id: u32,
    games: Vec<Game>,
}

impl Session {
    fn build(id: u32) -> Self {
        Session {
            id: id,
            games: vec![],
        }
    }

    fn set_games(&mut self, games: Vec<Game>) -> &Self {
        self.games = games;
        self
    }

    fn is_possible(&self, max_game: &Game) -> bool {
        !self.games.iter().any(|&g| !g.is_possible(max_game))
    }

    fn maximum_possible_game(&self) -> Game {
        let mut game = Game::build();

        for &g in self.games.iter() {
            if g.red > game.red {
                game.red(g.red);
            }
            if g.green > game.green {
                game.green(g.green);
            }
            if g.blue > game.blue {
                game.blue(g.blue);
            }
        }

        return game;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSessionError;

impl FromStr for Session {
    type Err = ParseSessionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (session_id_str, rest) = s
            .strip_prefix("Game ")
            .and_then(|s| s.split_once(":"))
            .unwrap();
        let session_id = session_id_str
            .parse::<u32>()
            .map_err(|_| ParseSessionError)?;
        let mut session = Session::build(session_id);

        let games = rest
            .split(";")
            .map(|s| s.split(',').map(|s| s.strip_prefix(" ")))
            .map(|game_iter| {
                let mut game = Game::build();
                for s in game_iter {
                    let split: Vec<&str> = s.expect("must be a string").split(" ").collect();
                    let num = split[0].parse::<u32>().unwrap();
                    let color = split[1];

                    game = match color {
                        "red" => *game.red(num),
                        "green" => *game.green(num),
                        "blue" => *game.blue(num),
                        _ => game,
                    };
                }
                return game;
            })
            .collect();
        session.set_games(games);

        return Ok(session);
    }
}

fn main() {
    let maximum_game: Game = Game::build_full(12, 13, 14);
    let input = fs::read_to_string("input.txt").expect("failed to open input file");

    let sessions: Vec<Session> = input
        .split("\n")
        .filter(|&s| !s.is_empty())
        .map(|line| line.parse::<Session>().unwrap())
        .collect();

    // Part 1 - Sum of possible games
    let sum = sessions
        .iter()
        .filter(|&s| s.is_possible(&maximum_game))
        .fold(0, |acc, s| acc + s.id);

    println!("{:?}", sum);

    // Part 2 - Sum of game powers
    let sum = sessions
        .iter()
        .map(|s| s.maximum_possible_game())
        .fold(0, |acc, g| acc + (g.red * g.green * g.blue));

    println!("{:?}", sum);
}
