/*
1. render grid of squares with x width and y height
2. toggle each square "on"/"off" based on random value
3. loop through grid and draw line based on config (which squares are "on" or "off")
*/

use bevy::prelude::*;
use rand::prelude::*;
use bevy_prototype_lyon::prelude::*;

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

impl Square {
    fn get_config(&self) -> i8 {
        let bl = self.bot_left.state as i8;
        let br = self.bot_right.state as i8;
        let tr = self.top_right.state as i8;
        let tl = self.top_left.state as i8;
        let config = tl << 3 | tr << 2 | br << 1 | bl << 0;
        return config;
    }
}

const TILE_SIZE: f32 = 20.;

pub struct MarchingSquaresPlugin;

impl Plugin for MarchingSquaresPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(ShapePlugin)
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    let width = 10;
    let height = 10;

    let grid = build_grid(width, height);    

    for vertex in grid.iter() {
        spawn_dot(&mut commands, &vertex.bot_right);
        spawn_dot(&mut commands, &vertex.bot_left);
        spawn_dot(&mut commands, &vertex.top_right);
        spawn_dot(&mut commands, &vertex.top_left);

        let contour_line = get_contour_lines(vertex);
        for line in contour_line.iter() {
            let line_start = line[0];
            let line_end = line[1];
    
            let mut path_builder = PathBuilder::new();
            path_builder.move_to(line_start);
            path_builder.line_to(line_end);
    
            let line = path_builder.build();
            commands.spawn(GeometryBuilder::build_as(
                &line,
                DrawMode::Stroke(StrokeMode::new(Color::BLACK, 1.5)),
                Transform::default(),
            ));
        }
    }
}

fn spawn_dot(commands: &mut Commands, cell: &Cell) {
    let s = shapes::Circle {
        radius: 1.,
        ..shapes::Circle::default()
    };

    let rgb: f32 = if cell.state {
        0. //white
    } else {
        1. //black
    };
    commands.spawn(GeometryBuilder::build_as(
        &s,
        DrawMode::Fill(
            FillMode::color(Color::rgb(rgb, rgb, rgb)),
        ),
        Transform::from_xyz(cell.x as f32 * TILE_SIZE, cell.y as f32 * TILE_SIZE, 0.),
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

    //debugging
    // grid.push(Square{
    //     bot_left: Cell{
    //         x: 0.,
    //         y: 0.,
    //         state: false
    //     },
    //     bot_right: Cell{
    //         x: 1.,
    //         y: 0.,
    //         state: true
    //     },
    //     top_right: Cell{
    //         x: 1.,
    //         y: 1.,
    //         state: true
    //     },
    //     top_left: Cell{
    //         x: 0.,
    //         y: 1.,
    //         state: true
    //     },
    // });

    for x in 0..width - 1 {
        for y in 0..height - 1 {
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

fn get_contour_lines(square: &Square) -> Vec<Vec<Vec2>> {
    let case: i8 = square.get_config();
    if case == 0 {
        //no contour line
        return vec![vec![Vec2::ZERO, Vec2::ZERO]];
    } else if case == 1 {
        let rx = square.bot_left.x * TILE_SIZE;
        let ry = (square.bot_left.y * TILE_SIZE + square.top_left.y * TILE_SIZE) / 2.;
        let ax = (square.bot_left.x * TILE_SIZE + square.bot_right.x * TILE_SIZE) / 2.;
        let ay = square.bot_left.y * TILE_SIZE;
        return vec![vec![Vec2::new(rx, ry), Vec2::new(ax, ay)]];
    } else if case == 2 {
        let rx = (square.bot_left.x * TILE_SIZE + square.bot_right.x * TILE_SIZE) / 2.;
        let ry = square.bot_left.y * TILE_SIZE;
        let ax = square.bot_right.x * TILE_SIZE;
        let ay = (square.bot_right.y * TILE_SIZE + square.top_right.y * TILE_SIZE) / 2.;
        return vec![vec![Vec2::new(rx, ry), Vec2::new(ax, ay)]];
    } else if case == 3 {
        let rx  = square.bot_left.x * TILE_SIZE;
        let ry = (square.bot_left.y * TILE_SIZE + square.top_left.y * TILE_SIZE) / 2.;
        let ax = square.bot_right.x * TILE_SIZE;
        let ay = (square.bot_right.y * TILE_SIZE + square.top_right.y * TILE_SIZE) / 2.;
        return vec![vec![Vec2::new(rx, ry), Vec2::new(ax, ay)]];
    } else if case == 4 {
        let rx = (square.bot_left.x * TILE_SIZE  + square.bot_right.x * TILE_SIZE) / 2.;
        let ry = square.top_left.y * TILE_SIZE;
        let ax = square.bot_right.x * TILE_SIZE;
        let ay = (square.bot_right.y * TILE_SIZE + square.top_right.y * TILE_SIZE) / 2.;
        return vec![vec![Vec2::new(rx, ry), Vec2::new(ax, ay)]];
    } 
    else if case == 5 {
        let rx1 = square.bot_left.x * TILE_SIZE;
        let ry1 = (square.bot_left.y * TILE_SIZE + square.top_left.y * TILE_SIZE) / 2.;
        let ax1 = (square.top_left.x * TILE_SIZE + square.top_right.x * TILE_SIZE) / 2.;
        let ay1 = square.top_left.y * TILE_SIZE;
        let line1 = vec![Vec2::new(rx1, ry1), Vec2::new(ax1, ay1)];

        let rx2 = (square.bot_left.x * TILE_SIZE + square.bot_right.x * TILE_SIZE) / 2.;
        let ry2 = square.bot_left.y * TILE_SIZE;
        let ax2 = square.bot_right.x * TILE_SIZE;
        let ay2 = (square.bot_right.y * TILE_SIZE + square.top_right.y * TILE_SIZE) / 2.;
        let line2 = vec![Vec2::new(rx2, ry2), Vec2::new(ax2, ay2)];

        return vec![line1, line2];
    } else if case == 6 {
        let rx = (square.bot_left.x * TILE_SIZE + square.bot_right.x * TILE_SIZE) / 2.;
        let ry = square.bot_left.y * TILE_SIZE;
        let ax = (square.bot_left.x * TILE_SIZE + square.bot_right.x * TILE_SIZE) / 2.;
        let ay = square.top_left.y * TILE_SIZE;
        return vec![vec![Vec2::new(rx, ry), Vec2::new(ax, ay)]];
    } else if case == 7 {
        let rx = square.bot_left.x * TILE_SIZE;
        let ry = (square.bot_left.y * TILE_SIZE + square.top_left.y * TILE_SIZE) / 2.;
        let ax = (square.top_left.x * TILE_SIZE + square.top_right.x * TILE_SIZE) / 2.;
        let ay = square.top_left.y * TILE_SIZE;
        return vec![vec![Vec2::new(rx, ry), Vec2::new(ax, ay)]];
    } else if case == 8 {
        //same as 7
        let rx = square.bot_left.x * TILE_SIZE;
        let ry = (square.bot_left.y * TILE_SIZE + square.top_left.y * TILE_SIZE) / 2.;
        let ax = (square.top_left.x * TILE_SIZE + square.top_right.x * TILE_SIZE) / 2.;
        let ay = square.top_left.y * TILE_SIZE;
        return vec![vec![Vec2::new(rx, ry), Vec2::new(ax, ay)]];
    } else if case == 9 {
        //same as 6
        let rx = (square.bot_left.x * TILE_SIZE + square.bot_right.x * TILE_SIZE) / 2.;
        let ry = square.bot_left.y * TILE_SIZE;
        let ax = (square.bot_left.x * TILE_SIZE + square.bot_right.x * TILE_SIZE) / 2.;
        let ay = square.top_left.y * TILE_SIZE;
        return vec![vec![Vec2::new(rx, ry), Vec2::new(ax, ay)]];
    } else if case == 10 {
        //1 and 4 combined
        let rx1 = square.bot_left.x * TILE_SIZE;
        let ry1 = (square.bot_left.y * TILE_SIZE + square.top_left.y * TILE_SIZE) / 2.;
        let ax1 = (square.bot_left.x * TILE_SIZE + square.bot_right.x * TILE_SIZE) / 2.;
        let ay1 = square.bot_left.y * TILE_SIZE;
        let line1 = vec![Vec2::new(rx1, ry1), Vec2::new(ax1, ay1)];

        let rx2 = (square.bot_left.x * TILE_SIZE  + square.bot_right.x * TILE_SIZE) / 2.;
        let ry2 = square.top_left.y * TILE_SIZE;
        let ax2 = square.bot_right.x * TILE_SIZE;
        let ay2 = (square.bot_right.y * TILE_SIZE + square.top_right.y * TILE_SIZE) / 2.;
        let line2 = vec![Vec2::new(rx2, ry2), Vec2::new(ax2, ay2)];

        return vec![line1, line2];
    }
    else if case == 11 {
        //same as 4
        let rx = (square.bot_left.x * TILE_SIZE  + square.bot_right.x * TILE_SIZE) / 2.;
        let ry = square.top_left.y * TILE_SIZE;
        let ax = square.bot_right.x * TILE_SIZE;
        let ay = (square.bot_right.y * TILE_SIZE + square.top_right.y * TILE_SIZE) / 2.;
        return vec![vec![Vec2::new(rx, ry), Vec2::new(ax, ay)]];
    } else if case == 12 {
        //same as case 3
        let rx  = square.bot_left.x * TILE_SIZE;
        let ry = (square.bot_left.y * TILE_SIZE + square.top_left.y * TILE_SIZE) / 2.;
        let ax = square.bot_right.x * TILE_SIZE;
        let ay = (square.bot_right.y * TILE_SIZE + square.top_right.y * TILE_SIZE) / 2.;
        return vec![vec![Vec2::new(rx, ry), Vec2::new(ax, ay)]];
    } else if case == 13 {
        //same as case 2
        let rx = (square.bot_left.x * TILE_SIZE + square.bot_right.x * TILE_SIZE) / 2.;
        let ry = square.bot_left.y * TILE_SIZE;
        let ax = square.bot_right.x * TILE_SIZE;
        let ay = (square.bot_right.y * TILE_SIZE + square.top_right.y * TILE_SIZE) / 2.;
        return vec![vec![Vec2::new(rx, ry), Vec2::new(ax, ay)]];
    } else if case == 14 {
        //same as case 1
        let rx = square.bot_left.x * TILE_SIZE;
        let ry = (square.bot_left.y * TILE_SIZE + square.top_left.y * TILE_SIZE) / 2.;
        let ax = (square.bot_left.x * TILE_SIZE + square.bot_right.x * TILE_SIZE) / 2.;
        let ay = square.bot_left.y * TILE_SIZE;
        return vec![vec![Vec2::new(rx, ry), Vec2::new(ax, ay)]];
    } else if case == 15 {
        //no contour line
        return vec![vec![Vec2::ZERO, Vec2::ZERO]];
    } else {
        panic!("{} is not a valid case!", case);
    }
}
