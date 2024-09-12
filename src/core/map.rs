use std::fs;

use anyhow::Result;
use csv::*;

use crate::pathfinder::astar::Grid;

use super::scene::GameMap;

pub fn load_map_from_csv(file_path: &str) -> Result<(Grid, GameMap)> {
    // Read the CSV file
    let file_content = fs::read_to_string(file_path)?;
    let mut rdr = Reader::from_reader(file_content.as_bytes());

    // Initialize the grid
    let mut grid = Grid::new(8, 8);
    let mut map = vec![vec![String::new(); 8]; 8];

    // Parse the CSV data and set obstacles
    for (y, result) in rdr.records().enumerate() {
        let record = result?;
        for (x, cell) in record.iter().enumerate() {
            map[y][x] = cell.to_string();
            match cell {
                "🌳" => grid.set_obstacle(x, y),
                "🚪" => grid.set_start(x, y),
                "💰" => grid.set_goal(x, y),
                _ => (),
            }
        }
    }

    Ok((grid, GameMap(map)))
}

#[test]
fn test_map_csv() {
    use crate::pathfinder::astar::Heuristic;

    let (mut grid, _) = load_map_from_csv("assets/map.csv").unwrap();
    grid.solve(&Heuristic::Manhattan);
    if let Some(path) = grid.path.as_mut() {
        path.reverse();
    }

    println!("Goal: {:?}", grid.goal.unwrap());
    println!("Path: {:?}", grid.path);
}