/*
1. render grid of cells with x width and y height
2. toggle each square "on"/"off" based on random value
3. loop through grid and draw line based on config (which squares are "on" or "off")
*/
use std::vec;
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
    let width = 100;
    let height = 100;
    let grid = build_grid(width, height);

    for x in 0..grid.len() - 2 { //- 2 since each square takes into account the next row/column
        for y in 0..grid[0].len() - 2{
            let i = x as usize;
            let j = y as usize;

            let square = Square {
                bot_left: grid[i][j],
                bot_right: grid[i + 1][j],
                top_right: grid[i + 1][j + 1],
                top_left: grid[i][j + 1]
            };

            spawn_dot(&mut commands, &square.bot_left);
            spawn_dot(&mut commands, &square.bot_right);
            spawn_dot(&mut commands, &square.top_right);
            spawn_dot(&mut commands, &square.top_left);
            draw_contour_lines(&mut commands, square);
        }
    }
}

fn draw_contour_lines(commands: &mut Commands, square: Square) {
    let contour_lines = get_contour_lines(&square);
    for line in contour_lines.iter() {
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

fn spawn_dot(commands: &mut Commands, cell: &Cell) {
    let c = shapes::Circle {
        radius: 1.,
        ..shapes::Circle::default()
    };

    let rgb: f32 = if cell.state {
        0. //white
    } else {
        1. //black
    };
    commands.spawn(GeometryBuilder::build_as(
        &c,
        DrawMode::Fill(
            FillMode::color(Color::rgb(rgb, rgb, rgb)),
        ),
        Transform::from_xyz(cell.x as f32 * TILE_SIZE, cell.y as f32 * TILE_SIZE, 0.),
    ));
}

fn build_grid(width: i32, height: i32) -> Vec<Vec<Cell>> {
    let w = width as usize;
    let h = height as usize;
    let empty_cell = Cell { x: 0., y: 0., state: false };
    let mut grid: Vec<Vec<Cell>> = vec![vec![empty_cell; w]; h];

    for x in 0..width - 1 {
        for y in 0..height - 1 {
            grid[x as usize][y as usize] = gen_cell(x, y);
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
    if case == 0 || case == 15 {
        //no contour line
        return vec![vec![Vec2::ZERO, Vec2::ZERO]];
    } else if case == 1 || case == 14 {
        let rx = square.bot_left.x * TILE_SIZE;
        let ry = (square.bot_left.y * TILE_SIZE + square.top_left.y * TILE_SIZE) / 2.;
        let ax = (square.bot_left.x * TILE_SIZE + square.bot_right.x * TILE_SIZE) / 2.;
        let ay = square.bot_left.y * TILE_SIZE;
        return vec![vec![Vec2::new(rx, ry), Vec2::new(ax, ay)]];
    } else if case == 2 || case == 13 {
        let rx = (square.bot_left.x * TILE_SIZE + square.bot_right.x * TILE_SIZE) / 2.;
        let ry = square.bot_left.y * TILE_SIZE;
        let ax = square.bot_right.x * TILE_SIZE;
        let ay = (square.bot_right.y * TILE_SIZE + square.top_right.y * TILE_SIZE) / 2.;
        return vec![vec![Vec2::new(rx, ry), Vec2::new(ax, ay)]];
    } else if case == 3 || case == 12 {
        let rx  = square.bot_left.x * TILE_SIZE;
        let ry = (square.bot_left.y * TILE_SIZE + square.top_left.y * TILE_SIZE) / 2.;
        let ax = square.bot_right.x * TILE_SIZE;
        let ay = (square.bot_right.y * TILE_SIZE + square.top_right.y * TILE_SIZE) / 2.;
        return vec![vec![Vec2::new(rx, ry), Vec2::new(ax, ay)]];
    } else if case == 4 || case == 11 {
        let rx = (square.bot_left.x * TILE_SIZE  + square.bot_right.x * TILE_SIZE) / 2.;
        let ry = square.top_left.y * TILE_SIZE;
        let ax = square.bot_right.x * TILE_SIZE;
        let ay = (square.bot_right.y * TILE_SIZE + square.top_right.y * TILE_SIZE) / 2.;
        return vec![vec![Vec2::new(rx, ry), Vec2::new(ax, ay)]];
    } 
    else if case == 5 || case == 10 {
        //only ambiguous case
        if case == 5 {
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
        } else {
            //case 10
            let rx1 = square.bot_left.x * TILE_SIZE;
            let ry1 = (square.bot_left.y * TILE_SIZE + square.top_left.y * TILE_SIZE) / 2.;
            let ax1 = (square.bot_left.x * TILE_SIZE + square.bot_right.x * TILE_SIZE) / 2.;
            let ay1 = square.bot_left.y * TILE_SIZE;
            let line1 = vec![Vec2::new(rx1, ry1), Vec2::new(ax1, ay1)];

            let rx2 = (square.bot_left.x * TILE_SIZE + square.bot_right.x * TILE_SIZE) / 2.;
            let ry2 = square.top_left.y * TILE_SIZE;
            let ax2 = square.bot_right.x * TILE_SIZE;
            let ay2 = (square.bot_right.y * TILE_SIZE + square.top_right.y * TILE_SIZE) / 2.;
            let line2 = vec![Vec2::new(rx2, ry2), Vec2::new(ax2, ay2)];
            return vec![line1, line2];
        }
    } else if case == 6 || case == 9 {
        let rx = (square.bot_left.x * TILE_SIZE + square.bot_right.x * TILE_SIZE) / 2.;
        let ry = square.bot_left.y * TILE_SIZE;
        let ax = (square.bot_left.x * TILE_SIZE + square.bot_right.x * TILE_SIZE) / 2.;
        let ay = square.top_left.y * TILE_SIZE;
        return vec![vec![Vec2::new(rx, ry), Vec2::new(ax, ay)]];
    } else if case == 7 || case == 8 {
        let rx = square.bot_left.x * TILE_SIZE;
        let ry = (square.bot_left.y * TILE_SIZE + square.top_left.y * TILE_SIZE) / 2.;
        let ax = (square.top_left.x * TILE_SIZE + square.top_right.x * TILE_SIZE) / 2.;
        let ay = square.top_left.y * TILE_SIZE;
        return vec![vec![Vec2::new(rx, ry), Vec2::new(ax, ay)]];
    } else {
        panic!("{} is not a valid case!", case);
    }
}
