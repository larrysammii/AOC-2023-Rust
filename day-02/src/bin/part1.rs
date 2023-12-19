// Matching colours into enum.

#[derive(Debug)]
enum Colour {
    Red,
    Green,
    Blue,
}

impl From<&str> for Colour {
    fn from(s: &str) -> Self {
        match s {
            "red" => Colour::Red,
            "green" => Colour::Green,
            "blue" => Colour::Blue,
            _ => unimplemented!(),
        }
    }
}

// Grouping colours and their quantities into sets.
// e.g.
// input = "Game 1: 10 green, 5 blue; 1 red, 9 green, 10 blue"

#[derive(Debug)]
struct ColourSet {
    quantity: u32,
    colour: Colour,
}

impl From<&str> for ColourSet {
    fn from(s: &str) -> Self {
        let mut iter = s.split(" "); // "10 green" -> ["10", "green"]
        let quantity = iter
            .next() // "10"
            .unwrap()
            .parse() // "10" -> 10
            .unwrap();

        let colour = iter
            .next() // "green"
            .unwrap()
            .into(); // "green" -> Colour::Green

        Self { quantity, colour }
    }
}

// Colour sets in the same game get groupped into round...a list of colour sets.
#[derive(Debug)]
struct Round {
    colour_sets: Vec<ColourSet>,
}

impl From<&str> for Round {
    fn from(s: &str) -> Self {
        let colour_sets = s
            .split(", ") // "10 green, 5 blue" -> ["10 green", "5 blue"]
            .map(ColourSet::from) // [ColourSet { quantity: 10, colour: Colour::Green }, ColourSet { quantity: 5, colour: Colour::Blue }]
            .collect(); // Vec<ColourSet>
        Self { colour_sets }
    }
}

// Game is a list of all the rounds.
// So the wrapping sequence is:
// Game id -> Round -> ColourSets ( Quantity, Colour ).
#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let mut iter = s.split(": "); // ["Game 1", "10 green, 5 blue; 1 red, 9 green, 10 blue"]
        let id = iter
            .next() // "Game 1"
            .unwrap()
            .strip_prefix("Game ") // "Game 1" -> "1"
            .unwrap()
            .parse() // "1" -> 1
            .unwrap();

        let rounds = iter
            .next() // "10 green, 5 blue; 1 red, 9 green, 10 blue"
            .unwrap()
            .split("; ") // ["10 green, 5 blue", "1 red, 9 green, 10 blue"]
            .map(Round::from) // [Round { colour_sets: [ColourSet { quantity: 10, colour: Colour::Green }, ...]
            .collect();

        Self { id, rounds }
    }
}

fn main() {
    let input =
        std::fs::read_to_string("././input/input.txt")
            .unwrap();

    let game = input
        .lines() // ["Game 1: 10 green, 5 blue; 1 red, 9 green, 10 blue", ...]
        .map(Game::from)
        .collect::<Vec<_>>(); // Vec<Game>

    dbg!(game);
}
