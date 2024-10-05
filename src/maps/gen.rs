use crate::core::{
    map::{find_path, generate_map, MapPosition, PathCost},
    scene::GameMap,
};
use anyhow::Result;
use rand::{rngs::OsRng, Rng};

fn always_find_path(start: MapPosition, goal: MapPosition) -> PathCost {
    find_path(
        &vec![vec![true; 8]; 8],
        start.clone().to_tuple(),
        goal.clone().to_tuple(),
        false,
    )
    .unwrap()
}

fn check_and_pave_path(
    walkables: &mut Vec<Vec<bool>>,
    map: &mut Vec<Vec<String>>,
    start: MapPosition,
    main_route_path: &[(usize, usize)],
    target_char: &str,
    rng: &mut rand::rngs::ThreadRng,
) {
    for row in 0..8 {
        for col in 0..8 {
            if map[row][col] == *target_char {
                let target = MapPosition { x: col, y: row };
                if let Ok(_path) = find_path(walkables, start.clone().to_tuple(), (col, row), false)
                {
                    // OK
                } else {
                    // If no sub-route found, randomly pick a node and pave the way
                    let node_index = rng.gen_range(1..main_route_path.len() - 1);
                    let (x, y) = main_route_path[node_index];

                    // Pave the way to the nearest target
                    let path_cost = always_find_path(MapPosition { x, y }, target.clone());
                    for (px, py) in path_cost.path {
                        if !walkables[py][px] {
                            walkables[py][px] = true;
                            "➖".clone_into(&mut map[py][px]);
                        }
                    }
                }
            }
        }
    }
}

// Refine the walkable map
fn refine_walkable_map(
    walkables: &mut Vec<Vec<bool>>,
    game_map: &mut GameMap,
    start: MapPosition,
    goal: MapPosition,
) -> GameMap {
    let mut rng = rand::thread_rng();
    let GameMap(map) = game_map;

    // Check if any node in the main route can walk from start to goal
    let main_route_path = match find_path(walkables, start.to_tuple(), goal.to_tuple(), false) {
        Ok(path_cost) => path_cost.path,
        _ => {
            // Find the main route
            let main_route_path = always_find_path(start.clone(), goal).path;
            // Pave the way
            for (px, py) in main_route_path.clone() {
                if !walkables[py][px] {
                    walkables[py][px] = true;
                    "➖".clone_into(&mut map[py][px]);
                }
            }

            main_route_path
        }
    };

    // Check and pave paths to "💰"
    check_and_pave_path(
        walkables,
        map,
        start.clone(),
        &main_route_path,
        "💰",
        &mut rng,
    );

    // Check and pave paths to "💀"
    check_and_pave_path(walkables, map, start, &main_route_path, "💀", &mut rng);

    game_map.clone()
}

#[allow(clippy::type_complexity)]
pub fn gen_map_from_public_key(
    pubkey: &str,
) -> Result<(Vec<Vec<bool>>, MapPosition, MapPosition, GameMap)> {
    let mut map = vec![vec!["➖".to_string(); 8]; 8];

    // Fill the edges with 🌳
    for i in 0..8 {
        map[0][i] = "🌳".to_string();
        map[7][i] = "🌳".to_string();
        map[i][0] = "🌳".to_string();
        map[i][7] = "🌳".to_string();
    }

    // Place 🌳 based on the rest of the characters
    for (i, ch) in pubkey.chars().enumerate().skip(2) {
        let row = (i % 7) + 1; // Rows 1 to 7
        let col = ch as u8 % 8;
        map[row][col as usize] = "🌳".to_string();
    }

    // Place the 🚪 gates
    let c = 1 + pubkey.chars().nth(0).unwrap() as u8 % 6;
    let a = 1 + pubkey.chars().nth(1).unwrap() as u8 % 6;
    map[0][c as usize] = "🚪".to_string();
    map[7][a as usize] = "🚪".to_string();

    // Place 🆒 and 🆕
    let mut rng = OsRng;
    loop {
        // Place 💰 and 💀 randomly ensuring no conflict with 🆒 and 🆕
        for _ in 0..1 {
            let row = rng.gen_range(1..=6);
            let col = rng.gen_range(1..=6);
            map[row][col] = "💰".to_string();
        }
        for _ in 0..1 {
            let row = rng.gen_range(1..=6);
            let col = rng.gen_range(1..=6);
            map[row][col] = "💀".to_string();
        }

        if map[1][c as usize] != "💰" && map[1][c as usize] != "💀" {
            map[1][c as usize] = "🆒".to_string();
            map[6][a as usize] = "🆕".to_string();
            break;
        }
    }

    let (walkables, start, goal) = generate_map(&map);

    Ok((walkables, start, goal, GameMap(map)))
}

#[test]
fn test_refine_walkable_map() {
    let pubkey = "gistmeAhMG7AcKSPCHis8JikGmKT9tRRyZpyMLNNULq";
    let (walkables, start, goal, game_map) = gen_map_from_public_key(pubkey).unwrap();
    let GameMap(map) = game_map;

    for y in 0..8 {
        for x in 0..8 {
            print!("{}", map[y][x]);
        }
        println!();
    }

    let mut walkables = walkables;

    let GameMap(map) = refine_walkable_map(
        &mut walkables,
        &mut GameMap(map.clone()),
        start.clone(),
        goal.clone(),
    );

    for y in 0..8 {
        for x in 0..8 {
            print!("{}", map[y][x]);
        }
        println!();
    }

    // Print the refined walkable map for verification
    for row in &walkables {
        for &cell in row {
            print!("{}", if cell { "✅" } else { "❌" });
        }
        println!();
    }

    // Assert that start to goal is walkable
    assert!(find_path(
        &walkables,
        start.clone().to_tuple(),
        goal.clone().to_tuple(),
        true
    )
    .is_ok());

    // Assert that start to 💰 is walkable
    for row in 0..8 {
        for col in 0..8 {
            if map[row][col] == *"💰" {
                assert!(find_path(&walkables, start.clone().to_tuple(), (col, row), true).is_ok());
            }
        }
    }

    // Assert that start to 💀 is walkable
    for row in 4..=6 {
        for col in 2..=6 {
            if map[row][col] == *"💀" {
                assert!(find_path(&walkables, start.clone().to_tuple(), (col, row), true).is_ok());
            }
        }
    }
}
