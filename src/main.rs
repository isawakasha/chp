use regex::Regex;
use std::env;

fn calculate_hiddenpower(gen: &String) -> String {
    let re = Regex::new(r"\d+").unwrap();
    let gens = re
        .captures_iter(&gen)
        .map(|x| x.get(0).map_or("", |m| m.as_str()));
    let mut g: Vec<f32> = Vec::new();

    for gen in gens {
        let tmp: f32 = gen.parse().expect("ERROR");
        g.push(tmp % 2 as f32);
    }

    let hp_value: f32 =
        ((g[0] + 2.0 * g[1] + 4.0 * g[2] + 8.0 * g[3] + 16.0 * g[4] + 32.0 * g[5]) * 15.0 / 63.0)
            .floor();

    match hp_value as i32 {
        0 => return String::from("Bug"),
        1 => return String::from("Dark"),
        2 => return String::from("Dragon"),
        3 => return String::from("Electric"),
        4 => return String::from("Fight"),
        5 => return String::from("Fire"),
        6 => return String::from("Fly"),
        7 => return String::from("Ghost"),
        8 => return String::from("Grass"),
        9 => return String::from("Ground"),
        10 => return String::from("Ice"),
        11 => return String::from("Water"),
        12 => return String::from("Poison"),
        13 => return String::from("Psychic"),
        14 => return String::from("Rock"),
        15 => return String::from("Steel"),
        _ => return String::from("Error"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let gens = args[1..].iter();

    for gen in gens {
        let hidden_power = calculate_hiddenpower(&gen);
        println!("{} {}", gen, hidden_power);
    }
}
