/**
 * Map and Tiles
 */
use rltk::{ RGB, Rltk, RandomNumberGenerator };
use super::{Rect};
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

///Makes a map with a solid boundary and 400 randomly placed walls.
///No guarentees that it won't look awful
fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80*50]; // initialize vector to all Floors

    // Make the boundaries Wall tiles
    for x in 0..80 {
        let _ = map[xy_idx(x, 0)] == TileType::Wall;
        let _ = map[xy_idx(x, 49)] == TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // Randomly Place Walls For Illustration
    // Get an instance of RNG
    let mut rng = RandomNumberGenerator::new();
    
    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);

        // Add a wall as long as it's not where the player starts
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

/// Makes a map of solid walls
pub fn new_map_rooms_and_corridors() -> Vec<TileType> {
    let mut map = vec![TileType::Wall; 80*50];

    let room1 = Rect::new(20, 15, 10, 15);
    let room2 = Rect::new(35, 15, 10, 15);

    apply_room_to_map(&room1, &mut map);
    apply_room_to_map(&room2, &mut map);

    map
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1 ..= room.y2 {
        for x in room.x1 + 1 ..= room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}


/*
 * In the declaration, we pass the map as `&[TileType]` rather than `&Vec<TileType>`; this
 * allows us to pass in "slices" (parts of) a map if we so choose.
 */
pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {

    // by incrementally looping we aren't repeatedly reading all over 
    // the array - which can get slow
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        // Render Tile based on TileType
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            }
        }

        // Move the coordinates (increment the values being set)
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

