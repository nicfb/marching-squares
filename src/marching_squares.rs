/*
1. render grid of squares with x width and y height
2. toggle each square "on"/"off" based on random value
3. loop through grid and draw line based on config (which squares are "on" or "off")
*/

use bevy::prelude::*;
use rand::prelude::*;
use bevy_prototype_lyon::prelude::{*, tess::path::commands};

#[derive(Clone, Copy)]
struct Cell {
    x: f32,
    y: f32,
    state: bool,
}

#[derive(Clone, Copy)]
struct Square {
    bot_left: Cell,
    bot_right: Cell,
    top_right: Cell,
    top_left: Cell,
}

pub struct MarchingSquaresPlugin;

impl Plugin for MarchingSquaresPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(ShapePlugin)
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    let width = 50;
    let height = 50;
    
    let tile_size = 20.;

    let s = shapes::Rectangle {
        extents: Vec2::new(5., 5.),
        ..shapes::Rectangle::default()
    };

    let grid = build_grid(width, height);    

    for square in grid.iter() {
        spawn_square(&mut commands, &square.bot_right);
        spawn_square(&mut commands, &square.bot_left);
        spawn_square(&mut commands, &square.top_right);
        spawn_square(&mut commands, &square.top_left);

        let rx = (square.bot_left.x * tile_size / 2.) + (square.bot_right.x * tile_size / 2.);
        let ry = (square.bot_left.y * tile_size / 2.) + (square.bot_right.y * tile_size / 2.);
        let ax = (square.bot_left.x * tile_size / 2.) + (square.top_left.x * tile_size / 2.);
        let ay = (square.bot_left.y * tile_size / 2.) + (square.top_left.y * tile_size / 2.);
        
        let mut path_builder = PathBuilder::new();
        path_builder.move_to(Vec2::new(rx, ry)); //midway between x,y and x+1,y
        path_builder.line_to(Vec2::new(ax, ay)); //midway between x,y and x,y+1

        let line = path_builder.build();
        commands.spawn(GeometryBuilder::build_as(
            &line,
            DrawMode::Stroke(StrokeMode::new(Color::BLACK, 0.8)),
            Transform::default(),
        ));
    }
}

fn spawn_square(commands: &mut Commands, cell: &Cell) {
    let tile_size = 20.;

    let s = shapes::Rectangle {
        extents: Vec2::new(5., 5.),
        ..shapes::Rectangle::default()
    };

    let rgb: f32 = if cell.state {
        1.        
    } else {
        0.
    };
    commands.spawn(GeometryBuilder::build_as(
        &s,
        DrawMode::Fill(
            FillMode::color(Color::rgb(rgb, rgb, rgb)),
        ),
        Transform::from_xyz(cell.x as f32 * tile_size, cell.y as f32 * tile_size, 0.),
    ));
}

fn build_grid(width: i32, height: i32) -> Vec<Square> {
    let w = width as usize;
    let h = height as usize;
    let empty_cell = Cell { x: 0., y: 0., state: false };
    let empty_square = Square {
        bot_left: empty_cell,
        bot_right: empty_cell,
        top_right: empty_cell,
        top_left: empty_cell,
    };
    let size = (w * h) / 2. as usize;
    let mut grid: Vec<Square> = vec![empty_square; size];

    for x in 0..width {
        for y in 0..height {
            if x % 2 == 0 && y % 2 == 0 {
                let bot_left = gen_cell(x, y);
                let bot_right = gen_cell(x + 1, y);
                let top_right = gen_cell(x + 1, y + 1);
                let top_left = gen_cell(x, y + 1);

                grid.push(Square {
                    bot_left,
                    bot_right,
                    top_right,
                    top_left,
                });
            }
        }
    }
    return grid;
}

fn gen_cell(x: i32, y: i32) -> Cell {
    let mut rng: ThreadRng = rand::thread_rng();
    let n: f32 = rng.gen();
    let state: bool = n.round() == 1.;

    return Cell {
        x: x as f32,
        y: y as f32,
        state,
    };
}
