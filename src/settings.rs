use clap::{
    builder::{EnumValueParser, PossibleValue},
    Parser, ValueEnum,
};

use super::cellular_automata::Rules;

impl ValueEnum for Rules {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Rules::Conway,
            Rules::HighLife,
            Rules::Gravity(true),
            Rules::Gravity(false),
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Rules::Conway => Some(PossibleValue::new("conway")),
            Rules::HighLife => Some(PossibleValue::new("highlife")),
            Rules::Gravity(true) => Some(PossibleValue::new("snow")),
            Rules::Gravity(false) => Some(PossibleValue::new("rain")),
        }
    }

    // Provided method
    fn from_str(input: &str, _: bool) -> Result<Self, String> {
        match input {
            "conway" => Ok(Rules::Conway),
            "highlife" => Ok(Rules::HighLife),
            "snow" => Ok(Rules::Gravity(true)),
            "rain" => Ok(Rules::Gravity(false)),
            _ => Err(format!("Unknown rules: {}", input)),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Cellular Automata", long_about = None)]
pub struct CommandLineProvidedSettings {
    #[arg(long, default_value_t = 30, help = "Width of the world in cells")]
    pub width: usize,

    #[arg(long, default_value_t = 30, help = "Height of the world in cells")]
    pub height: usize,

    #[arg(
        long,
        default_value_t = 5,
        help = "Scale of the world in pixels. Only works with GUI"
    )]
    pub scale: usize,

    #[arg(
        long,
        default_value_t = false,
        help = "Prints the world as text instead of using a GUI"
    )]
    pub text: bool,

    #[arg(
        long,
        default_value_t = 100,
        help = "Time between ticks in milliseconds"
    )]
    pub tbt: u64,

    #[arg(long, default_value = "conway", value_parser = EnumValueParser::<Rules>::new(), help = "Rules to use")]
    pub rules: Rules,

    #[arg(
        long,
        default_value_t = 1_000_000,
        help = "Number of epochs to run before exiting. Only works with text"
    )]
    pub epoch: u64,

    #[arg(
        long,
        default_value_t = 0,
        help = "epoch % reset == 0 => reset the world"
    )]
    pub reset: u64,
}

impl Default for CommandLineProvidedSettings {
    fn default() -> Self {
        Self::parse()
    }
}
