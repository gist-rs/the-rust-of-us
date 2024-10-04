use rand::Rng;

use crate::core::{
    map::{find_path, generate_map, MapPosition},
    scene::GameMap,
};

fn refine_walkable_map(
    walkables: &mut Vec<Vec<bool>>,
    map: &mut GameMap,
    main_route: &Vec<(usize, usize)>,
) {
    let mut rng = rand::thread_rng();
    let mut found_sub_route = false;

    // Check if any node in the main route can access 💰 or 🪦
    for &node in main_route {
        let (x, y) = node;

        // Check for 💰 and 🪦
        for row in 1..=3 {
            for col in 2..=6 {
                if map.0[row][col] == *"💰" {
                    if let Ok(_path) = find_path(walkables, (x, y), (col, row), false) {
                        found_sub_route = true;
                        break;
                    }
                }
            }
        }

        for row in 4..=6 {
            for col in 2..=6 {
                if map.0[row][col] == *"🪦" {
                    if let Ok(_path) = find_path(walkables, (x, y), (col, row), false) {
                        found_sub_route = true;
                        break;
                    }
                }
            }
        }

        if found_sub_route {
            break;
        }
    }

    // If no sub-route found, randomly pick a node and pave the way
    if !found_sub_route {
        let node_index = rng.gen_range(0..main_route.len());
        let (x, y) = main_route[node_index];

        // Find the nearest 💰 or 🪦
        let mut nearest_treasure = None;
        let mut min_distance = usize::MAX;

        for row in 1..=3 {
            for col in 2..=6 {
                if map.0[row][col] == *"💰" {
                    let distance = (x as i32 - col as i32).abs() + (y as i32 - row as i32).abs();
                    if distance < min_distance.try_into().unwrap() {
                        min_distance = distance as usize;
                        nearest_treasure = Some((col, row));
                    }
                }
            }
        }

        for row in 4..=6 {
            for col in 2..=6 {
                if map.0[row][col] == *"🪦" {
                    let distance = (x as i32 - col as i32).abs() + (y as i32 - row as i32).abs();
                    if distance < min_distance.try_into().unwrap() {
                        min_distance = distance as usize;
                        nearest_treasure = Some((col, row));
                    }
                }
            }
        }

        if let Some((tx, ty)) = nearest_treasure {
            // Pave the way to the nearest treasure
            let path_cost = find_path(walkables, (x, y), (tx, ty), false).unwrap();
            for (px, py) in path_cost.path {
                walkables[py][px] = true;
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn gen_map_from_pubkey(
    pubkey: &str,
) -> anyhow::Result<(Vec<Vec<bool>>, MapPosition, MapPosition, GameMap)> {
    let mut map = vec![vec!['➖'.to_string(); 8]; 8];

    // Fill the edges with 🌳
    for i in 0..8 {
        map[0][i] = '🌳'.to_string();
        map[7][i] = '🌳'.to_string();
        map[i][0] = '🌳'.to_string();
        map[i][7] = '🌳'.to_string();
    }

    // Place 🌳 based on the rest of the characters
    for (i, ch) in pubkey.chars().enumerate().skip(2) {
        let row = (i % 7) + 1; // Rows 1 to 7
        let col = ch as u8 % 8;
        map[row][col as usize] = '🌳'.to_string();
    }

    // Place 💰 randomly between row 1-3, col 2-6
    let mut rng = rand::thread_rng();
    for _ in 0..3 {
        let row = rng.gen_range(1..=3);
        let col = rng.gen_range(2..=6);
        map[row][col] = '💰'.to_string();
    }

    // Place 🪦 randomly between row 4-6, col 2-6
    for _ in 0..3 {
        let row = rng.gen_range(4..=6);
        let col = rng.gen_range(2..=6);
        map[row][col] = '🪦'.to_string();
    }

    // Place the 🚪 gates
    let c = 1 + pubkey.chars().nth(0).unwrap() as u8 % 6;
    let a = 1 + pubkey.chars().nth(1).unwrap() as u8 % 6;
    map[0][c as usize] = '🚪'.to_string();
    map[7][a as usize] = '🚪'.to_string();

    // Place 🆒 and 🆕
    map[1][c as usize] = '🆒'.to_string();
    map[6][a as usize] = '🆕'.to_string();

    let (walkables, start, goal) = generate_map(&map);

    Ok((walkables, start, goal, GameMap(map)))
}

#[test]
fn test_refine_walkable_map() {
    let pubkey = "gistmeAhMG7AcKSPCHis8JikGmKT9tRRyZpyMLNNULq";
    let (mut walkables, start, goal, mut game_map) = gen_map_from_pubkey(pubkey).unwrap();

    // Find the main route
    let main_route = find_path(
        &vec![vec![true; 8]; 8],
        start.clone().to_tuple(),
        goal.clone().to_tuple(),
        false,
    )
    .unwrap();

    // Refine the walkable map
    refine_walkable_map(&mut walkables, &mut game_map, &main_route.path);

    // Print the refined walkable map for verification
    for row in &walkables {
        for &cell in row {
            print!("{} ", if cell { "1" } else { "0" });
        }
        println!();
    }

    // TODO assert that
    // 1. start to goal is walkable by result is_ok() from find_path(
    //     &vec![vec![true; 8]; 8],
    //     start.clone().to_tuple(),
    //     goal.clone().to_tuple(),
    //     true,
    // );
    // 2. start to 💰 is walkable by result is_ok() from find_path(
    // 3. start to 🪦 is walkable by result is_ok() from find_path(
}
