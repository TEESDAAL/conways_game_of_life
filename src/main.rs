use macroquad::prelude::*;
use std::collections::HashSet;

type Cell = (isize, isize);

fn draw_board(alive_cells: &HashSet<Cell>, grid_size: &f32, x_shift: &f32, y_shift: &f32) {
    let (screen_width, screen_height) = (screen_width(), screen_height());
    let smaller_dimension = screen_width.min(screen_height);

    let cell_size = smaller_dimension / *grid_size;

    for x in 0..((screen_width / cell_size) + 1.) as isize {
        draw_line(
            x as f32 * cell_size,
            0.0,
            x as f32 * cell_size,
            screen_height as f32,
            1.0,
            PURPLE,
        );
    }
    for y in 0..((screen_height / cell_size) + 1.) as isize {
        draw_line(
            0.0,
            y as f32 * cell_size,
            screen_width as f32,
            y as f32 * cell_size,
            1.0,
            PURPLE,
        );
    }
    let num_cells = (screen_width / cell_size) as isize + 1;


    for x in 0..num_cells {
        for y in 0..num_cells {
            if alive_cells.contains(&(x + *x_shift as isize, y + *y_shift as isize)) {
                draw_rectangle(
                    x as f32 * cell_size,
                    y as f32 * cell_size,
                    cell_size,
                    cell_size,
                    WHITE,
                )
            }
        }
    }
}

fn mouse_keyboard_events(
    alive_cells: &mut HashSet<Cell>,
    game_state: &mut GameState,
    grid_size: &mut f32,
    x_shift: &mut f32,
    y_shift: &mut f32,
) {
    let cell_size = screen_width().min(screen_height()) / *grid_size;
    let (mouse_x, mouse_y) = mouse_position();
    let cell = (
        (mouse_x / cell_size) as isize + *x_shift as isize,
        (mouse_y / cell_size) as isize + *y_shift as isize,
    );
    let movements = [(KeyCode::Left, -1., 0.), (KeyCode::Right, 1., 0.), (KeyCode::Up, 0., -1.), (KeyCode::Down, 0., 1.)];
 

    for (key, dx, dy) in &movements {
        if is_key_down(*key) {
            *x_shift += dx;
            *y_shift += dy;
        }
    }
    

    if is_mouse_button_down(MouseButton::Left) {
        alive_cells.insert(cell);
    } else if is_mouse_button_down(MouseButton::Right) {
        alive_cells.remove(&cell);
    }

    if is_key_released(KeyCode::Space) {
        game_state.toggle();
    }

    if is_key_down(KeyCode::LeftControl) {
        if is_key_pressed(KeyCode::Equal) {
            *grid_size /= 2.0;
        }
        if is_key_pressed(KeyCode::Minus) {
            *grid_size *= 2.0;
        }
    }

    if is_key_down(KeyCode::C) {
        alive_cells.clear();
    }
}

#[derive(PartialEq)]
enum GameState {
    Placing,
    Playing,
}

impl GameState {
    fn toggle(&mut self) {
        match self {
            GameState::Placing => *self = GameState::Playing,
            GameState::Playing => *self = GameState::Placing,
        }
    }
}

fn number_of_neighbors(alive_cells: &HashSet<Cell>, cell: &Cell) -> i32 {
    let (x_cell, y_cell) = *cell;
    let mut neighbors = 0;


    for x in -1..=1 {
        for y in -1..=1 {
            // Don't count the cell as one of it's own neighboors
            if (x, y) == (0, 0) {
                continue;
            }
            let neighbor_cell = ((x_cell + x), (y_cell + y));
            if alive_cells.contains(&neighbor_cell) {
                neighbors += 1;
            }
        }
    }

    neighbors
}

fn tick(alive_cells: &mut HashSet<Cell>) {
    let mut cells_to_check = HashSet::<Cell>::new();
    let mut cells_to_kill = HashSet::<Cell>::new();

    for cell in alive_cells.iter() {
        let neighbors = number_of_neighbors(&alive_cells, cell);
        if neighbors < 2 || neighbors > 3 {
            cells_to_kill.insert(cell.clone());
        }

        for x in -1..=1 {
            for y in -1..=1 {
                // Don't count the cell as one of it's own neighboors
                if (x, y) == (0, 0) {
                    continue;
                }
                if !alive_cells.contains(&(cell.0 + x, cell.1 + y)) {
                    cells_to_check.insert((cell.0 + x, cell.1 + y));
                }
            }
        }
    }

    let mut cells_to_birth = HashSet::<Cell>::new();
    for cell in cells_to_check.iter() {
        if number_of_neighbors(&mut *alive_cells, cell) == 3 {
            cells_to_birth.insert(cell.clone());
        }
    }

    for cell in cells_to_kill.iter() {
        alive_cells.remove(cell);
    }

    for cell in cells_to_birth.iter() {
        alive_cells.insert(cell.clone());
    }
}

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let mut x_shift = 0.0;
    let mut y_shift = 0.0;
    let mut game_state = GameState::Placing;
    let mut alive_cells = HashSet::new();
    let mut grid_size = 15.;
    loop {
        clear_background(BLACK);
        mouse_keyboard_events(
            &mut alive_cells,
            &mut game_state,
            &mut grid_size,
            &mut x_shift,
            &mut y_shift,
        );

        draw_board(&alive_cells, &grid_size, &x_shift, &y_shift);
        match game_state {
            GameState::Placing => {
                draw_text(
                    "Click to place cells, right click to remove them.",
                    10. + 10.,
                    10. + 20.,
                    20.,
                    PURPLE,
                );
            }
            GameState::Playing => {
                draw_text(
                    "Press space to toggle between placing and playing.",
                    10. + 10.,
                    10. + 20.,
                    20.,
                    PURPLE,
                );
                tick(&mut alive_cells);
            }
        }

        next_frame().await
    }
}
