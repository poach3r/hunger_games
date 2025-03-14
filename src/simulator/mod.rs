use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;

use crate::player::status::*;
use crate::player::*;
use crate::scenario::*;
use log::info;
use rand::seq::SliceRandom;

/// Simulates an entire game with predefined scenarios
/// and players. This will be made much more modular
/// in the future but this is more useful for
/// rapid prototyping.
pub fn simulate_game<'a, 'b>(players: &'a mut Vec<Player<'b>>, scenarios: &Vec<Vec<Scenario>>) {
    let mut rng = rand::rng();
    let mut round = 0usize;

    // progress rounds
    while get_available_players(&players, 0).len() > 1 {
        println!("Day {}:", round + 1);
        players.shuffle(&mut rng);
        println!("{}", simulate_round(players, scenarios, &mut rng, round).0);
        println!();
        round += 1;
    }

    match get_available_players(players, 0).first() {
        Some(winner) => println!("Winner: {}", players[*winner].name),
        _ => println!("Nobody won."),
    };

    info!("Simulation ended with following result:\n{players:#?}");
}

/// Simulates a single round.
/// Returns a string containing all
/// the scenarios that occurred and
/// whether the game ended.
pub fn simulate_round<'a, 'b, 'c>(
    players: &'a mut Vec<Player<'c>>,
    scenarios: &Vec<Vec<Scenario>>,
    rng: &'b mut ThreadRng,
    round: usize,
) -> (String, bool) {
    let mut string = String::new();
    let mut index: usize = 0;
    while index < players.len() {
        if let Status::Dead = players[index].status {
            index += 1;
            continue;
        }
        if players[index].moved {
            index += 1;
            continue;
        }

        let indices = get_available_players(players, index);
        let (scenario, arity) = get_scenario(scenarios, rng, players, &indices, round);

        string.push_str(scenario.run(players, &indices, arity).as_str());
        string.push('\n');
        index += 1;
    }

    for player in players.iter_mut() {
        player.moved = false;
    }

    let living = get_available_players(&players, 0);
    if living.len() == 0 {
        string.push_str("Nobody won.");
        (string, true)
    } else if living.len() == 1 {
        string.push_str(format!("{} won.", players[living[0]].name).as_str());
        (string, true)
    } else {
        string.pop(); // remove extra newline
        (string, false)
    }
}

/// Returns the indices of all players capable of moving.
fn get_available_players(players: &Vec<Player>, index: usize) -> Vec<usize> {
    let mut indices = Vec::<usize>::new();
    let mut n = index;
    while n < players.len() {
        if let Status::Dead = players[n].status {
            n += 1;
            continue;
        }
        if players[n].moved {
            n += 1;
            continue;
        }

        indices.push(n);
        n += 1;
    }

    indices
}

/// Gets a random scenario from `scenarios` based on the provided `players` and `indices`.
fn get_scenario<'a, 'b>(
    scenarios: &'a Vec<Vec<Scenario>>,
    rng: &'b mut ThreadRng,
    players: &Vec<Player>,
    indices: &Vec<usize>,
    round: usize,
) -> (&'a Scenario, usize) {
    loop {
        let mut random_scenes: Vec<(&Scenario, usize)> = Vec::new();

        for i in 0..scenarios.len() {
            if i >= indices.len() {
                continue;
            }

            let possible_scene = match scenarios[i].choose(rng) {
                Some(s) => s,
                None => {
                    panic!(
                        "Failed to choose a scenario. Please send this full message to poacher. \nrng: {rng:?}\nscenarios: {scenarios:?}"
                    );
                }
            };

            if round < possible_scene.possible_after || round >= possible_scene.impossible_after {
                continue;
            }

            if (possible_scene.condition)(players, indices) {
                random_scenes.push((possible_scene, i));
            }
        }

        match random_scenes.choose(rng) {
            Some(s) => return *s,
            None => continue,
        }
    }
}
