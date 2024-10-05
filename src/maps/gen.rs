use crate::core::{
    map::{find_path, generate_map, MapPosition, PathCost},
    scene::GameMap,
};
use anyhow::Result;
use rand::{rngs::OsRng, Rng};

fn always_find_path(start: &MapPosition, goal: &MapPosition) -> PathCost {
    find_path(
        &vec![vec![true; 8]; 8],
        start.to_tuple(),
        goal.to_tuple(),
        false,
    )
    .expect("Expected PathCost")
}

fn check_and_pave_path(
    walkables: &mut [Vec<bool>],
    map: &mut [Vec<String>],
    start: &MapPosition,
    main_route_path: &[(usize, usize)],
    target_char: &str,
    rng: &mut rand::rngs::ThreadRng,
) {
    for row in 0..8 {
        for col in 0..8 {
            if map[row][col] == target_char {
                let target = MapPosition { x: col, y: row };
                if let Ok(_path) = find_path(walkables, start.to_tuple(), (col, row), false) {
                    // OK
                } else {
                    // If no sub-route found, randomly pick a node and pave the way
                    let node_index = rng.gen_range(1..main_route_path.len() - 1);
                    let (x, y) = main_route_path[node_index];

                    // Pave the way to the nearest target
                    let path_cost = always_find_path(&MapPosition { x, y }, &target);
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
#[allow(unused)]
fn refine_walkable_map(
    walkables: &mut [Vec<bool>],
    game_map: &mut GameMap,
    start: &MapPosition,
    goal: &MapPosition,
) -> (GameMap, Vec<Vec<bool>>) {
    let mut rng = rand::thread_rng();
    let GameMap(map) = game_map;

    // Check if any node in the main route can walk from start to goal
    let main_route_path = match find_path(walkables, start.to_tuple(), goal.to_tuple(), false) {
        Ok(path_cost) => path_cost.path,
        _ => {
            // Find the main route
            let main_route_path = always_find_path(start, goal).path;
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
    check_and_pave_path(walkables, map, start, &main_route_path, "💰", &mut rng);

    // Check and pave paths to "💀"
    check_and_pave_path(walkables, map, start, &main_route_path, "💀", &mut rng);

    (game_map.clone(), walkables.to_vec())
}

#[allow(clippy::type_complexity)]
#[allow(unused)]
pub fn gen_map_from_public_key(
    public_key: &str,
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
    for (i, ch) in public_key.chars().enumerate().skip(2) {
        let row = (i % 7) + 1; // Rows 1 to 7
        let col = ch as u8 % 8;
        map[row][col as usize] = "🌳".to_string();
    }

    // Place the 🚪 gates
    #[allow(clippy::iter_nth_zero)]
    let c = 1 + public_key
        .chars()
        .nth(0)
        .expect("Expected valid public key") as u8
        % 6;
    let a = 1 + public_key
        .chars()
        .nth(1)
        .expect("Expected valid public key") as u8
        % 6;
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

    #[allow(clippy::needless_range_loop)]
    for y in 0..8 {
        for x in 0..8 {
            print!("{}", map[y][x]);
        }
        println!();
    }
    println!();

    let mut walkables = walkables;

    let (refined_game_map, refined_walkables) =
        refine_walkable_map(&mut walkables, &mut GameMap(map), &start, &goal);

    let GameMap(map) = refined_game_map;

    #[allow(clippy::needless_range_loop)]
    for y in 0..8 {
        for x in 0..8 {
            print!("{}", map[y][x]);
        }
        println!();
    }
    println!();

    // Print the refined walkable map for verification
    for row in &refined_walkables {
        for &cell in row {
            print!("{}", if cell { "✅" } else { "❌" });
        }
        println!();
    }

    // Assert that start to goal is walkable
    assert!(find_path(&refined_walkables, start.to_tuple(), goal.to_tuple(), true).is_ok());

    // Assert that start to 💰 is walkable
    #[allow(clippy::needless_range_loop)]
    for row in 0..8 {
        for col in 0..8 {
            if map[row][col] == *"💰" {
                assert!(find_path(&refined_walkables, start.to_tuple(), (col, row), true).is_ok());
            }
        }
    }

    // Assert that start to 💀 is walkable
    #[allow(clippy::needless_range_loop)]
    for row in 4..=6 {
        for col in 2..=6 {
            if map[row][col] == *"💀" {
                assert!(find_path(&refined_walkables, start.to_tuple(), (col, row), true).is_ok());
            }
        }
    }
}
