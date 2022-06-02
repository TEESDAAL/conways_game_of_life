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

    for x in (0 + *x_shift as isize)..((screen_width / cell_size) + 1. + *x_shift) as isize {
        for y in (0 + *y_shift as isize)..((screen_height / cell_size) + 1. + *y_shift) as isize {
            if alive_cells.contains(&(x, y)) {
                draw_rectangle(
                    (x as f32 - *x_shift) * cell_size,
                    (y as f32 - *y_shift) * cell_size,
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

    if is_key_down(KeyCode::Left) {
        *x_shift += 1.0;
    }
    if is_key_down(KeyCode::Right) {
        *x_shift -= 1.0;
    }
    if is_key_down(KeyCode::Up) {
        *y_shift += 1.0;
    }
    if is_key_down(KeyCode::Down) {
        *y_shift -= 1.0;
    }
    if is_mouse_button_down(MouseButton::Left) {
        alive_cells.insert(cell);
    } else if is_mouse_button_down(MouseButton::Right) {
        alive_cells.remove(&cell);
    }

    if is_key_released(KeyCode::Space) {
        match &game_state {
            &GameState::Placing => *game_state = GameState::Playing,
            &GameState::Playing => *game_state = GameState::Placing,
        }
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

fn number_of_neighbors(alive_cells: &HashSet<Cell>, cell: &Cell) -> i32 {
    let (x, y) = cell;
    let mut neighbors = 0;
    for (x_offset, y_offset) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    {
        let neighbor_cell = ((*x as isize + x_offset), (*y as isize + y_offset));
        if alive_cells.contains(&neighbor_cell) {
            neighbors += 1;
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
        for (x, y) in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        {
            if !alive_cells.contains(&(cell.0 + x, cell.1 + y)) {
                cells_to_check.insert((cell.0 + x, cell.1 + y));
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
