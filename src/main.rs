mod grid;
use crate::grid::grid::Grid;
use macroquad::{self, window::{screen_height, screen_width, next_frame}, 
                shapes::{draw_rectangle_lines, draw_rectangle}, prelude::*};

const CELL_SIZE: f32 = 16.0;
const GRID_THICKNESS: f32 = 1.0;
const GRID_HIGHLIGHT_THICKNESS: f32 = 8.0;
const GRID_COLOR: Color = WHITE;
const GRID_HIGHLIGHT_COLOR: Color = RED;
const LIVING_CELL_COLOR: Color = WHITE;

#[macroquad::main("Conway's Game of Life")]
async fn main() {

    let mut cell_grid = Grid::new();
    let mut camera = Camera2D { 
        ..Default::default()
    };

    loop {
        draw_grid(&cell_grid, &camera);
        
        let (mouse_x, mouse_y) = mouse_position();
        draw_highlighted_cell((mouse_x, mouse_y));

        // INPUTS ///////////////////////////////////////////////////////////////////////
        if is_mouse_button_pressed(MouseButton::Left) {
            let new_cell_x = (mouse_x - (mouse_x % CELL_SIZE)) / CELL_SIZE;
            let new_cell_y = (mouse_y - (mouse_y % CELL_SIZE)) / CELL_SIZE;

            cell_grid.toggle_cell((new_cell_x as i32, new_cell_y as i32));
        }
        if is_key_down(KeyCode::Space) { cell_grid.compute(); }
        if is_key_pressed(KeyCode::Backspace) { cell_grid.clear_cells(); }


        if is_key_pressed(KeyCode::Up) { camera.offset.y -= 0.1; }
        if is_key_pressed(KeyCode::Down) { camera.offset.y += 0.1; }        
        if is_key_pressed(KeyCode::Left) { camera.offset.x -= 0.1; }        
        if is_key_pressed(KeyCode::Right) { camera.offset.x += 0.1; }        

        
        /////////////////////////////////////////////////////////////////////////////////
        
        next_frame().await
    }
}

fn draw_highlighted_cell(mouse_pos: (f32, f32)){
    let (mouse_x, mouse_y) = mouse_pos;
    let mouse_cell_x = mouse_x - (mouse_x % CELL_SIZE) - 1.;
    let mouse_cell_y = mouse_y - (mouse_y % CELL_SIZE) - 1.;

    draw_rectangle_lines(mouse_cell_x, 
                         mouse_cell_y, 
                         CELL_SIZE + 1., 
                         CELL_SIZE + 1., 
                         GRID_HIGHLIGHT_THICKNESS, 
                         GRID_HIGHLIGHT_COLOR);
}

fn draw_grid(grid: &Grid, camera: &Camera2D){
    for i in 0..((screen_height() / CELL_SIZE) as i32) + 1 {
        for j in 0..((screen_width() / CELL_SIZE) as i32) + 1 {
            let x = j as f32;
            let y = i as f32;
            let offset = camera.world_to_screen(vec2(x, y));
            draw_rectangle_lines(offset.x, 
                                 offset.y, 
                                 CELL_SIZE,
                                 CELL_SIZE, 
                                 GRID_THICKNESS,
                                 GRID_COLOR);
        }
    }
    
    for c in grid.get_cells(){
        draw_rectangle(c.0 as f32 * CELL_SIZE,
                       c.1 as f32 * CELL_SIZE,
                       CELL_SIZE,
                       CELL_SIZE,
                       LIVING_CELL_COLOR);
    }
}