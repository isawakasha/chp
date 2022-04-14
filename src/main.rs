use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let gens = args[1..].iter();

    for gen in gens {
        let hidden_power = calculate_hiddenpower(&gen);
        println!("{} {}", gen, hidden_power);
    }
}

fn calculate_hiddenpower(gen: &String) -> &str {
    fn parse_gens_from_gen_string(gen: &String) -> Vec<f32> {
        let re = Regex::new(r"\d+").unwrap();

        // Get gen values from the genocode string by regex
        let gen_values = re
            .captures_iter(&gen)
            .map(|x| x.get(0).map_or("", |m| m.as_str()));

        let mut gens: Vec<f32> = Vec::new();

        for gen in gen_values {
            let tmp: f32 = gen.parse().expect("Error");
            gens.push(tmp % 2 as f32);
        }
        return gens;
    }

    fn calculate_hiddenpower_value(gens: Vec<f32>) -> f32 {
        return ((gens[0]
            + 2.0 * gens[1]
            + 4.0 * gens[2]
            + 8.0 * gens[3]
            + 16.0 * gens[4]
            + 32.0 * gens[5])
            * 15.0
            / 63.0)
            .floor();
    }

    let gens_values = parse_gens_from_gen_string(&gen);
    let hiddenpower_value = calculate_hiddenpower_value(gens_values);

    match hiddenpower_value as i32 {
        0 => return "Bug",
        1 => return "Dark",
        2 => return "Dragon",
        3 => return "Electric",
        4 => return "Fight",
        5 => return "Fire",
        6 => return "Fly",
        7 => return "Ghost",
        8 => return "Grass",
        9 => return "Ground",
        10 => return "Ice",
        11 => return "Water",
        12 => return "Poison",
        13 => return "Psychic",
        14 => return "Rock",
        15 => return "Steel",
        _ => return "Error",
    }
}
