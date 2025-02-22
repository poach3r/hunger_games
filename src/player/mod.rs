pub mod pronouns;
pub mod status;

use log::warn;
use serde::{Deserialize, Serialize};

use pronouns::*;
use status::*;

/// Represents a player in the simulation.
#[derive(Debug, Clone)]
pub struct Player<'a> {
    pub name: String,
    pub status: Status,
    pub pronouns: Pronouns<'a>,
    pub moved: bool,
    pub district: u8,
}

/// Represents a player in JSON form.
#[derive(Serialize, Deserialize, Clone)]
pub struct JsonPlayer {
    name: String,
    gender: String,
    district: u8,
}

impl<'a> From<&JsonPlayer> for Player<'a> {
    fn from(value: &JsonPlayer) -> Self {
        let pronouns = match value.gender.to_lowercase().as_str() {
            "male" => MALE,
            "female" => FEMALE,
            "enby" => ENBY,
            _ => {
                warn!(
                    "Failed to deserialize {}'s gender, defaulting to ENBY.",
                    &value.name
                );
                ENBY
            }
        };
        Self::new(value.name.clone(), pronouns, value.district)
    }
}

impl<'a> From<JsonPlayer> for Player<'a> {
    fn from(value: JsonPlayer) -> Self {
        let pronouns = match value.gender.as_str() {
            "Male" | "male" | "MALE" => MALE,
            "Female" | "female" | "FEMALE" => FEMALE,
            _ => {
                warn!(
                    "Failed to deserialize {}'s gender, defaulting to ENBY.",
                    &value.name
                );
                ENBY
            }
        };
        Self::new(value.name, pronouns, value.district)
    }
}

impl<'a> Player<'a> {
    /// Constructs a player at the start of a simulator.
    pub fn new(name: String, pronouns: Pronouns<'a>, district: u8) -> Self {
        Self {
            name,
            status: Status::Alive(AliveStatus::Healthy),
            pronouns,
            moved: false,
            district,
        }
    }

    /// Changes the player's `status` to `Dead`.
    pub fn kill(&mut self) {
        self.status = Status::Dead;
    }

    /// Changes a player's `status` to `Alive(Injured)` if their
    /// current `status` is `Alive(Healthy), otherwise sets it
    /// to `Dead`.
    pub fn hurt(&mut self) {
        match self.status {
            Status::Alive(AliveStatus::Healthy) => {
                self.status = Status::Alive(AliveStatus::Injured);
            }
            _ => {
                self.kill();
            }
        }
    }

    /// Changes a player's `status` to `Alive(Healthy)`.
    pub fn heal(&mut self) {
        self.status = Status::Alive(AliveStatus::Healthy);
    }
}
