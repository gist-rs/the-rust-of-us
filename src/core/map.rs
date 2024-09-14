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
            let inverted_y = 7 - y; // Invert the y-coordinate
            map[inverted_y][x] = cell.to_string();
            match cell {
                "🌳" => grid.set_obstacle(x, inverted_y),
                "🚪" => grid.set_start(x, inverted_y),
                "💰" => grid.set_goal(x, inverted_y),
                _ => (),
            }
        }
    }

    Ok((grid, GameMap(map)))
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
}

#[test]
fn test_map_csv() {
    use crate::pathfinder::astar::Heuristic;

    let (mut grid, _) = load_map_from_csv("assets/map.csv").unwrap();
    grid.solve(&Heuristic::Manhattan);

    println!("Goal: {:?}", grid.goal.unwrap());
    println!("Path: {:?}", grid.path);
}

#[test]
fn test_flip_map_csv() {
    let map = vec![
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