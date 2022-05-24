use ansi_term::Colour;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::{Ctx, DataResource, ResourceState};

macro_rules! component {
    ($id:ident [$($y:ident : $z:ty),*]) => {
        #[serde(deny_unknown_fields)]
        #[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
        pub struct $id {
            $($y : $z),*
        }
    };

    ($id:ident [$($y:ident : $z:ty),*], no-copy) => {
        #[serde(deny_unknown_fields)]
        #[derive(Deserialize, Debug, PartialEq, Clone)]
        pub struct $id {
            $($y : $z),*
        }
    };
}

#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum UnitType {
    Small,
    Ground,
}

#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum Attributes {
    Biological,
    Light,
}

component!(Unit[
    types: [UnitType; 10],
    attributes: [Attributes;10]
]);
component!(Description[desc: String], no - copy);
component!(Cost[money: u32, gas:u32,time:u32,population:u32]);
component!(Input []);
component!(Hp[hp: u32]);
component!(Model[model_tag: u32]);
component!(Camera[]);
component!(Transform[x: u32, y: u32,z:u32]);

#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug)]
pub struct Entity {
    pub hit_points: Option<Hp>,
    pub input: Option<Input>,
    pub moveable: Option<bool>,
    pub camera_tracked: Option<bool>,
}

impl DataResource for Entity {
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

        let is_valid = state.is_valid();

        ctx.log_resource(state);

        is_valid
    }

    fn file_extension() -> &'static str {
        "entity"
    }
}
