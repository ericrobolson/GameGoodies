use ansi_term::Colour;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::{Ctx, DataResource, ResourceState};

// multiply smash values by 100 for conversion

/// Static character data sheet
#[derive(Deserialize, Debug)]
pub struct CharacterInfo {
    /// The name of the character
    pub name: String,
    /// The weight of the character
    pub weight: u16,
    /// The gravity modifier for the character
    pub gravity_modifier: i16,
    /// The walk speed of the character
    pub walk_speed: u16,
    /// The run speed of the character
    pub run_speed: u16,
    /// The air movement speed of the character
    pub air_speed: u16,
    /// The fast fall speed of the character
    pub fast_fall_speed: u16,
    /// The fall speed of the character
    pub fall_speed: u16,
}

#[derive(Deserialize, Debug)]
pub struct Frame {}

impl DataResource for CharacterInfo {
    fn validate(path: &PathBuf, ctx: &mut Ctx) -> bool {
        let mut state = ResourceState::new(path.clone(), Self::file_extension().into());

        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        // Validate the JSON
        let this: Self = match serde_json::from_reader(reader) {
            Ok(t) => t,
            Err(e) => {
                state.add_err(format!("{:?}", e));
                ctx.log_resource(state);

                return false;
            }
        };

        // Property validations
        if this.weight == 0 {
            state.add_err(numerical_err(
                "weight",
                this.weight.into(),
                1,
                u8::MAX.into(),
            ));
        }
        if this.walk_speed == 0 {
            state.add_err(numerical_err(
                "walk_speed",
                this.walk_speed.into(),
                1,
                u8::MAX.into(),
            ));
        }
        if this.run_speed == 0 {
            state.add_err(numerical_err(
                "run_speed",
                this.run_speed.into(),
                1,
                u8::MAX.into(),
            ));
        }
        if this.air_speed == 0 {
            state.add_err(numerical_err(
                "air_speed",
                this.air_speed.into(),
                1,
                u8::MAX.into(),
            ));
        }
        if this.fall_speed == 0 {
            state.add_err(numerical_err(
                "fall_speed",
                this.fall_speed.into(),
                1,
                u8::MAX.into(),
            ));
        }
        if this.fast_fall_speed == 0 {
            state.add_err(numerical_err(
                "fast_fall_speed",
                this.fast_fall_speed.into(),
                1,
                u8::MAX.into(),
            ));
        }

        let is_valid = state.is_valid();

        ctx.log_resource(state);

        is_valid
    }

    fn file_extension() -> &'static str {
        "character.json"
    }
}

fn property_err(property: &'static str, provided: &str, expected: &str) -> String {
    let property = Colour::Yellow.paint(property);
    let provided = Colour::Red.paint(provided);
    format!(
        "PROPERTY {}: provided {}, expected {}.",
        property, provided, expected
    )
}

fn numerical_err(property: &'static str, provided: i32, min: i32, max: i32) -> String {
    let c = |i: i32| format!("{}", i);

    let expected = format!(
        "value between {} and {}",
        Colour::Green.paint(&c(min)),
        Colour::Green.paint(&c(max))
    );

    property_err(property, &c(provided), &expected)
}
