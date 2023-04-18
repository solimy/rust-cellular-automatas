extern crate termion;
extern crate clap;

use clap::{Arg, Command};


mod braille;
mod canvas;
mod cellular_automata;

use cellular_automata::{World, Rules};

fn main() {
    let matches = Command::new("Cellular Automata")
        .version("0.1.0")
        .author("Hadrien DAURES <hadrien.daures@gmail.com>")
        .about("Cellular Automata")
        .arg(Arg::new("width")
            .long("width")
            .value_name("WIDTH")
            .help("Sets the width of the world")
        )
        .arg(Arg::new("height")
            .long("height")
            .value_name("HEIGHT")
            .help("Sets the height of the world")
        )
        .arg(Arg::new("rules")
            .long("rules")
            .value_name("RULE")
            .help("Sets the rule of the world")
        )
        .arg(Arg::new("tbt")
            .long("tbt")
            .value_name("TIMEBETWEENTICKS")
            .help("Sets the time between ticks")
        )
        .arg(Arg::new("epoch")
            .long("epoch")
            .value_name("EPOC")
            .help("Sets the epoch of the world")
        )
        .arg(Arg::new("countdown")
            .long("countdown")
            .value_name("COUNTDOWN")
            .help("Sets the countdown before starting")
        )
        .arg(Arg::new("reset")
            .long("reset")
            .value_name("RESET")
            .help("Sets the reset of the world as an epoch modulo")
        )
        .get_matches();

    let width: usize = matches.get_one::<String>("width").unwrap_or(&"20".to_string()).parse().unwrap();
    let height: usize = matches.get_one::<String>("height").unwrap_or(&"20".to_string()).parse().unwrap();
    let tbt: u64 = matches.get_one::<String>("tbt").unwrap_or(&"100".to_string()).parse().unwrap();
    let rules: Rules = match matches.get_one::<String>("rules").unwrap_or(&"conway".to_string()).as_str() {
        "conway" => Rules::Conway,
        "highlife" => Rules::HighLife,
        "snow" => Rules::Gravity(true),
        "rain" => Rules::Gravity(false),
        rules => panic!("Unknown rule: {rules}"),
    };
    let epoch: u64 = matches.get_one::<String>("epoch").unwrap_or(&"1000".to_string()).parse().unwrap();
    let countdown: bool = matches.get_one::<String>("countdown").unwrap_or(&"true".to_string()).parse().unwrap();
    let reset: u64 = matches.get_one::<String>("reset").unwrap_or(&"0".to_string()).parse().unwrap();

    let mut world = World::new(
        rules.clone(),
        width,
        height,
    );
    
    // NOT DEBUG
    world.populate(0.2);
    world.revive(0, 0);
    world.revive(0, 1);

    if countdown {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        for count_down in (0..5).rev() {
            println!("{}{}", termion::cursor::Goto(1, 1), world);
            // DEBUG
            // println!("Rules: {:?}, width: {}, height: {}, tbt: {}", rules, width, height, tbt);
            println!("Starting in {}...", count_down+1);
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }

    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    // DEBUG
    // println!("{world}");
    for epoch in 0..epoch {
        world.tick();
        // DEBUG
        // println!("{world}");
        // NOT DEBUG
        print!("{}{}", termion::cursor::Goto(1, 1), world);
        std::thread::sleep(std::time::Duration::from_millis(tbt));
        if reset > 0 && epoch % reset == 0 {
            world = World::new(
                rules.clone(),
                width,
                height,
            );
        }
    }
}
