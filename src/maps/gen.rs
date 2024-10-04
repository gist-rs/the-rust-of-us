use crate::core::{
    map::{find_path, generate_map, MapPosition},
    scene::GameMap,
};
use rand::Rng;

#[allow(clippy::type_complexity)]
pub fn gen_map_from_public_key(
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
    #[allow(clippy::iter_nth_zero)]
    let c = 1 + pubkey.chars().nth(0).unwrap() as u8 % 6;
    let a = 1 + pubkey.chars().nth(1).unwrap() as u8 % 6;
    map[0][c as usize] = '🚪'.to_string();
    map[7][a as usize] = '🚪'.to_string();

    // Place 🆒 and 🆕
    map[1][c as usize] = '🆒'.to_string();
    map[6][a as usize] = '🆕'.to_string();

    let (mut walkables, start, goal) = generate_map(&map);

    // Refine the walkable map
    fn refine_walkable_map(
        walkables: &mut Vec<Vec<bool>>,
        map: &mut GameMap,
        start: MapPosition,
        goal: MapPosition,
    ) {
        let mut rng = rand::thread_rng();
        let mut found_sub_route = false;

        // Find the main route
        let main_route = find_path(
            walkables,
            start.clone().to_tuple(),
            goal.clone().to_tuple(),
            false,
        )
        .unwrap()
        .path;

        // Check if any node in the main route can access 💰 or 🪦
        for &node in &main_route {
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
                        let distance =
                            (x as i32 - col as i32).abs() + (y as i32 - row as i32).abs();
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
                        let distance =
                            (x as i32 - col as i32).abs() + (y as i32 - row as i32).abs();
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

    refine_walkable_map(
        &mut walkables,
        &mut GameMap(map.clone()),
        start.clone(),
        goal.clone(),
    );

    Ok((walkables, start, goal, GameMap(map)))
}

#[test]
fn test_refine_walkable_map() {
    let pubkey = "gistmeAhMG7AcKSPCHis8JikGmKT9tRRyZpyMLNNULq";
    let (walkables, start, goal, game_map) = gen_map_from_public_key(pubkey).unwrap();

    // Print the refined walkable map for verification
    for row in &walkables {
        for &cell in row {
            print!("{} ", if cell { "1" } else { "0" });
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
    for row in 1..=3 {
        for col in 2..=6 {
            if game_map.0[row][col] == *"💰" {
                assert!(find_path(&walkables, start.clone().to_tuple(), (col, row), true).is_ok());
            }
        }
    }

    // Assert that start to 🪦 is walkable
    for row in 4..=6 {
        for col in 2..=6 {
            if game_map.0[row][col] == *"🪦" {
                assert!(find_path(&walkables, start.clone().to_tuple(), (col, row), true).is_ok());
            }
        }
    }
}
