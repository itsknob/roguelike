use specs_derive::*;
use specs::prelude::*;

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool
}

#[derive(Component, Debug)]
pub struct Monster {}
