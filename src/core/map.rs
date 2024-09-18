use std::fs;

use anyhow::{bail, Result};
use bevy::prelude::Transform;
use csv::*;
use pathfinding::prelude::*;

use super::scene::GameMap;

#[derive(Clone, Default, Debug)]
pub struct PathCost {
    pub path: Vec<(usize, usize)>,
    pub cost: usize,
}

fn successors(
    // TOFIX
    #[allow(clippy::ptr_arg)] walkables: &Vec<Vec<bool>>,
    &(x, y): &(usize, usize),
) -> Vec<((usize, usize), usize)> {
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        .filter_map(|(nx, ny)| walkables[ny][nx].then_some(((nx, ny), 1)))
        .collect()
}

fn distance(&(x1, y1): &(usize, usize), &(x2, y2): &(usize, usize)) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

pub fn find_path(
    walkables: &Vec<Vec<bool>>,
    start: (usize, usize),
    goal: (usize, usize),
) -> Result<PathCost> {
    let mut counter = 0;
    let (path, cost) = astar(
        &start,
        |n| {
            counter += 1;
            successors(walkables, n)
        },
        |n| distance(n, &goal),
        |n| n == &goal,
    )
    .expect("path not found");

    Ok(PathCost { path, cost })
}

pub fn load_map_from_csv(file_path: &str) -> Result<(Vec<Vec<bool>>, PathCost, GameMap)> {
    // Read the CSV file
    let file_content = fs::read_to_string(file_path)?;
    let mut rdr = Reader::from_reader(file_content.as_bytes());

    // Initialize the grid
    let mut walkables = vec![vec![false; 8]; 8];
    let mut map = vec![vec![String::new(); 8]; 8];
    let mut start = (0, 0);
    let mut goal = (0, 0);

    // Parse the CSV data and set obstacles
    for (y, result) in rdr.records().enumerate() {
        let record = result?;
        for (x, cell) in record.iter().enumerate() {
            let inverted_y = 7 - y; // Invert the y-coordinate
            map[inverted_y][x] = cell.to_string();
            match cell {
                "_" => walkables[inverted_y][x] = true,
                "▶️" => {
                    start = (x, inverted_y);
                    println!("start:{:?}", start);
                    walkables[inverted_y][x] = true;
                }
                "⏹" => {
                    goal = (x, inverted_y);
                    println!("goal:{:?}", goal);
                    walkables[inverted_y][x] = true;
                }
                _ => {
                    walkables[inverted_y][x] = false;
                }
            }
        }
    }

    // Find the path
    match find_path(&walkables, start, goal) {
        Ok(path_cost) => {
            println!("{:?}", path_cost);
            Ok((walkables, path_cost, GameMap(map)))
        }
        Err(error) => bail!(error),
    }
}

pub fn convert_map_to_screen(map_coord: String) -> Option<(usize, usize)> {
    if map_coord.len() < 2 {
        return None;
    }

    let x = match map_coord.chars().next().unwrap().to_ascii_lowercase() {
        'a'..='h' => map_coord.chars().next().unwrap().to_ascii_lowercase() as usize - 'a' as usize,
        _ => return None,
    };

    let y = match map_coord.chars().nth(1).unwrap().to_digit(10) {
        Some(digit) if (1..=8).contains(&digit) => digit as usize - 1,
        _ => return None,
    };

    Some((x, y))
}

pub fn get_position_from_map(
    cell_size: usize,
    half_width: f32,
    half_height: f32,
    offset_x: f32,
    offset_y: f32,
    x: usize,
    y: usize,
) -> Transform {
    Transform::from_xyz(
        cell_size as f32 * x as f32 - half_width + offset_x,
        cell_size as f32 * y as f32 - half_height + offset_y,
        0.0,
    )
}

#[allow(dead_code)]
pub fn get_map_from_position(
    cell_size: usize,
    half_width: f32,
    half_height: f32,
    offset_x: f32,
    offset_y: f32,
    transform: Transform,
) -> (usize, usize) {
    // Extract the x and y coordinates from the transform
    let pos_x = transform.translation.x;
    let pos_y = transform.translation.y;

    // Reverse the transformation to get the map coordinates
    let x = ((pos_x + half_width - offset_x) / cell_size as f32).round() as usize;
    let y = ((pos_y + half_height - offset_y) / cell_size as f32).round() as usize;

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_map_to_screen() {
        // Test cases for the function
        assert_eq!(convert_map_to_screen("a1".to_string()), Some((0, 0)));
        assert_eq!(convert_map_to_screen("b1".to_string()), Some((1, 0)));
        assert_eq!(convert_map_to_screen("c1".to_string()), Some((2, 0)));
        assert_eq!(convert_map_to_screen("d1".to_string()), Some((3, 0)));
        assert_eq!(convert_map_to_screen("e1".to_string()), Some((4, 0)));
        assert_eq!(convert_map_to_screen("f1".to_string()), Some((5, 0)));
        assert_eq!(convert_map_to_screen("g1".to_string()), Some((6, 0)));
        assert_eq!(convert_map_to_screen("h1".to_string()), Some((7, 0)));

        assert_eq!(convert_map_to_screen("a2".to_string()), Some((0, 1)));
        assert_eq!(convert_map_to_screen("b2".to_string()), Some((1, 1)));
        assert_eq!(convert_map_to_screen("c2".to_string()), Some((2, 1)));
        assert_eq!(convert_map_to_screen("d2".to_string()), Some((3, 1)));
        assert_eq!(convert_map_to_screen("e2".to_string()), Some((4, 1)));
        assert_eq!(convert_map_to_screen("f2".to_string()), Some((5, 1)));
        assert_eq!(convert_map_to_screen("g2".to_string()), Some((6, 1)));
        assert_eq!(convert_map_to_screen("h2".to_string()), Some((7, 1)));

        assert_eq!(convert_map_to_screen("a8".to_string()), Some((0, 7)));
        assert_eq!(convert_map_to_screen("b8".to_string()), Some((1, 7)));
        assert_eq!(convert_map_to_screen("c8".to_string()), Some((2, 7)));
        assert_eq!(convert_map_to_screen("d8".to_string()), Some((3, 7)));
        assert_eq!(convert_map_to_screen("e8".to_string()), Some((4, 7)));
        assert_eq!(convert_map_to_screen("f8".to_string()), Some((5, 7)));
        assert_eq!(convert_map_to_screen("g8".to_string()), Some((6, 7)));
        assert_eq!(convert_map_to_screen("h8".to_string()), Some((7, 7)));

        // Test case for invalid input
        assert_eq!(convert_map_to_screen("i1".to_string()), None);
        assert_eq!(convert_map_to_screen("a9".to_string()), None);
        assert_eq!(convert_map_to_screen("a".to_string()), None);
        assert_eq!(convert_map_to_screen("".to_string()), None);
    }

    // #[test]
    // fn test_map_csv() {
    //     use crate::pathfinder::astar::Heuristic;

    //     let (mut grid, _) = load_map_from_csv("assets/map.csv").unwrap();
    //     grid.solve(&Heuristic::Manhattan);

    //     println!("Goal: {:?}", grid.goal.unwrap());
    //     println!("Path: {:?}", grid.path);
    // }

    #[test]
    fn test_flip_map_csv() {
        let map = [
            "a,b,c,d,e,f,g,h",
            "🌳,⛩️,🌳,🌳,🌳,🌳,🌳,🌳",
            "🌳,1,1,1,1,1,1,🌳",
            "🌳,🌳,🌳,🌳,1,1,1,🌳",
            "🌳,💰,1,1,💀,1,1,🌳",
            "🌳,🌳,🌳,🌳,1,1,1,🌳",
            "🌳,🦀,1,1,1,1,1,🌳",
            "🌳,🌳,🌳,🌳,🌳,1,🌳,🌳",
            "🌳,🌳,🌳,🌳,🌳,🚪,🌳,🌳",
        ];

        // Extract the header
        let header = &map[0];

        // Extract the rows to be flipped
        let rows_to_flip = &map[1..];

        // Reverse the rows
        let flipped_rows: Vec<&str> = rows_to_flip.iter().rev().cloned().collect();

        // Print the header
        println!("{}", header);

        // Print the flipped rows
        for line in flipped_rows {
            println!("{}", line);
        }
    }

    #[test]
    fn test_conversion_back_and_forth() {
        let cell_size = 46usize;
        let half_width = 320. / 2.;
        let half_height = 320. / 2.;
        let offset_x = 0.;
        let offset_y = 0.;

        let x = 5;
        let y = 7;

        // Convert map coordinates to position
        let transform =
            get_position_from_map(cell_size, half_width, half_height, offset_x, offset_y, x, y);

        // Convert position back to map coordinates
        let (map_x, map_y) = get_map_from_position(
            cell_size,
            half_width,
            half_height,
            offset_x,
            offset_y,
            transform,
        );

        // Assert that the original map coordinates are recovered
        assert_eq!((x, y), (map_x, map_y));
    }
}
