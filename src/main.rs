use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::*;
use std::cmp::{min, max};

// mod components;
// pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;
mod rect;
pub use rect::Rect;

pub struct State {
    ecs: World
}

impl State {
    fn run_systems(&mut self) {
        // Run Systems Here

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        // fetch - promise that you know the resource you are requesting
        // really does exist; will crash if it doesn't
        let map = self.ecs.fetch::<Vec<TileType>>(); // techically returns a `shred` type, (acts like ref)
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        // join (like db join) returns entities with both pos, and render properties
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }

    }
}

/** Components */ 
#[derive(Component)] 
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
pub struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component)]
pub struct Player {}


/**
 * Main
 */

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State{
        ecs: World::new()
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    let (rooms, map) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);
    let (player_x, player_y) = rooms[0].center();

    gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
    .build();
       
    // Randomly Generate a Starting Map
    gs.ecs.insert(new_map_rooms_and_corridors());

    rltk::main_loop(context, gs)
}
