use super::Scenario;
use crate::item;
use crate::status;

pub fn get() -> Vec<Scenario> {
    vec![
        Scenario {
            condition: |_, _| true,
            message: |players, indices| {
                format!(
                    "{} caught {} off guard and killed {}.",
                    players[indices[0]].name,
                    players[indices[1]].name,
                    players[indices[1]].pronouns.object
                )
            },
            actions: |players, indices| {
                players[indices[0]].kills += 1;
                players[indices[1]].kill();
            },
        },
        Scenario {
            condition: |_, _| true,
            message: |players, indices| {
                format!(
                    "{} got into a fistfight with {}.",
                    players[indices[0]].name, players[indices[1]].name
                )
            },
            actions: |players, indices| {
                players[indices[0]].hurt();
                players[indices[1]].hurt();
            },
        },
        Scenario {
            condition: |players, indices| {
                if let status::Status::Alive(status::AliveStatus::Injured) =
                    players[indices[1]].status
                {
                    true
                } else {
                    false
                }
            },
            message: |players, indices| {
                format!(
                    "{} tended to {}'s wounds.",
                    players[indices[0]].name, players[indices[1]].name
                )
            },
            actions: |players, indices| {
                players[indices[1]].heal();
            },
        },
        Scenario {
            condition: |_, _| true,
            message: |players, indices| {
                format!(
                    "{} and {} poisoned eachother.",
                    players[indices[0]].name, players[indices[1]].name
                )
            },
            actions: |players, indices| {
                players[indices[0]].kill();
                players[indices[1]].kill();
                players[indices[0]].kills += 1;
                players[indices[1]].kills += 1;
            },
        },
        Scenario {
            condition: |players, indices| players[indices[0]].get_item(&item::KNIFE).is_some(),
            message: |players, indices| {
                format!(
                    "{} hit {} with a long distance knife throw.",
                    players[indices[0]].name, players[indices[1]].name
                )
            },
            actions: |players, indices| {
                let i = players[indices[0]].get_item(&item::KNIFE).unwrap();
                players[indices[0]].inventory.remove(i);
                players[indices[0]].kills += 1;
                players[indices[1]].kill();
            },
        },
    ]
}
