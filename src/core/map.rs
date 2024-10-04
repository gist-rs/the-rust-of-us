use std::fs;

use anyhow::Result;
use bevy::{math::Vec2, prelude::Transform};
use csv::*;
use pathfinding::prelude::*;

use super::scene::GameMap;

#[derive(Clone, Default, Debug)]
pub struct PathCost {
    #[allow(unused)]
    pub path: Vec<(usize, usize)>,
    #[allow(unused)]
    pub cost: usize,
}

// TOFIX
#[allow(clippy::ptr_arg)]
fn successors(
    walkables: &Vec<Vec<bool>>,
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

// TOFIX
#[allow(clippy::ptr_arg)]
fn is_walkable_line(
    walkables: &Vec<Vec<bool>>,
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
) -> bool {
    let dx = (x2 as isize - x1 as isize).abs();
    let dy = (y2 as isize - y1 as isize).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;
    let mut x = x1 as isize;
    let mut y = y1 as isize;

    loop {
        if !walkables[y as usize][x as usize] {
            return false;
        }
        if x == x2 as isize && y == y2 as isize {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
    true
}

fn smooth_path(walkables: &Vec<Vec<bool>>, path: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut smoothed_path = vec![path[0]];
    let mut i = 0;

    while i < path.len() - 1 {
        let mut j = i + 1;
        while j < path.len() && is_walkable_line(walkables, path[i], path[j]) {
            j += 1;
        }
        smoothed_path.push(path[j - 1]);
        i = j - 1;
    }

    smoothed_path
}

pub fn find_path(
    walkables: &Vec<Vec<bool>>,
    start: (usize, usize),
    goal: (usize, usize),
    is_smooth: bool,
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

    if is_smooth {
        let smoothed_path = smooth_path(walkables, path);

        Ok(PathCost {
            path: smoothed_path,
            cost,
        })
    } else {
        Ok(PathCost { path, cost })
    }
}

#[derive(Default, Debug, Clone)]
pub struct MapPosition {
    pub x: usize,
    pub y: usize,
}

impl MapPosition {
    #[allow(unused)]
    pub fn to_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

pub fn load_map_from_csv(
    file_path: &str,
) -> Result<(Vec<Vec<bool>>, MapPosition, MapPosition, GameMap)> {
    // Read the CSV file
    let file_content = fs::read_to_string(file_path)?;
    let mut rdr = Reader::from_reader(file_content.as_bytes());

    // Initialize the grid
    let mut map = vec![vec![String::new(); 8]; 8];

    // Parse the CSV data and set obstacles
    for (y, result) in rdr.records().enumerate() {
        let record = result?;
        for (x, cell) in record.iter().enumerate() {
            map[y][x] = cell.to_string();
        }
    }

    let (walkables, start, goal) = generate_map(&map);

    Ok((walkables, start, goal, GameMap(map)))
}

#[allow(clippy::ptr_arg)]
pub fn generate_map(map: &Vec<Vec<String>>) -> (Vec<Vec<bool>>, MapPosition, MapPosition) {
    let mut walkables = vec![vec![false; 8]; 8];
    let mut start = MapPosition::default();
    let mut goal = MapPosition::default();

    for y in 0..8 {
        for x in 0..8 {
            match map[y][x].as_str() {
                "🆕" => {
                    start = MapPosition { x, y };
                    walkables[y][x] = true;
                }
                "🆒" => {
                    goal = MapPosition { x, y };
                    walkables[y][x] = true;
                }
                "🚪" | "🌳" => {
                    walkables[y][x] = false;
                }
                _ => {
                    walkables[y][x] = true;
                }
            }
        }
    }

    (walkables, start, goal)
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

pub struct MapConfig {
    cell_size: usize,
    half_width: f32,
    half_height: f32,
    offset: (f32, f32),
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            cell_size: 46,
            half_width: 320. / 2.,
            half_height: 320. / 2.,
            offset: (0., 0.),
        }
    }
}

pub fn get_position_from_map(x: usize, y: usize, map_config: Option<MapConfig>) -> Transform {
    let map_config = map_config.unwrap_or_default();
    let (offset_x, offset_y) = map_config.offset;
    Transform::from_xyz(
        map_config.cell_size as f32 * x as f32 - map_config.half_width + offset_x,
        -1.0 * (map_config.cell_size as f32 * y as f32 - map_config.half_height + offset_y),
        0.0,
    )
}

#[allow(dead_code)]
pub fn get_map_from_position(xy: Vec2, map_config: Option<MapConfig>) -> (usize, usize) {
    // Use the provided map_config or create a default instance
    let map_config = map_config.unwrap_or_default();

    // Reverse the transformation to get the map coordinates
    let x = ((xy.x + map_config.half_width - map_config.offset.0) / map_config.cell_size as f32)
        .round() as usize;
    let y = ((-1.0 * xy.y + map_config.half_height - map_config.offset.1)
        / map_config.cell_size as f32)
        .round() as usize;

    (x, y)
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec3Swizzles;

    use super::*;
    #[test]
    fn test_find_path() {
        let (walkables, ..) = load_map_from_csv("assets/map.csv").unwrap();

        let start = MapPosition { x: 1, y: 1 };
        let goal = MapPosition { x: 1, y: 3 };

        // Find the path
        let result = find_path(
            &walkables,
            start.clone().to_tuple(),
            goal.clone().to_tuple(),
            false,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_find_path_smooth() {
        // Define a 3x3 grid with walkable and non-walkable cells
        let walkables = vec![
            vec![false, false, false, false, false, false],
            vec![false, true, true, true, true, false],
            vec![false, true, false, true, true, false],
            vec![false, true, true, true, true, false],
            vec![false, true, true, true, true, false],
            vec![false, false, false, false, false, false],
        ];

        // Define the start and goal positions
        let start = (1, 1);
        let goal = (4, 4);

        // Find the path
        let result = find_path(&walkables, start, goal, true);

        // Assert that the path is found
        assert!(result.is_ok());

        // Get the path and cost
        let path_cost = result.unwrap();

        // Define the expected smoothed path
        let expected_path = vec![(1, 1), (4, 2), (4, 4)];

        // Assert that the path matches the expected path
        println!("path_cost.path:{:?}", path_cost.path);
        assert_eq!(path_cost.path, expected_path);
    }

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

    #[test]
    fn test_conversion_back_and_forth() {
        let x = 1;
        let y = 3;

        // Convert map coordinates to position
        let transform = get_position_from_map(x, y, None);

        // Convert position back to map coordinates
        let (map_x, map_y) = get_map_from_position(transform.translation.xy(), None);

        // Assert that the original map coordinates are recovered
        assert_eq!((x, y), (map_x, map_y));
    }
}
