use std::usize;

use grid::*;
use rand::Rng;
use minifb::{Key, Window, WindowOptions};





fn main() {
    let field_size: usize  = 300;
    let mut test_array: Grid<i32> = Grid::new(field_size,field_size);
    println!("Test {}", test_array[(0, 0)]);
    for i in 0..field_size {
        for j in 0..field_size {
            test_array[(i, j)] = rand::thread_rng().gen_range(0,2);
            //println!("{} = value at column {} and row {}", test_array[(i, j)], i, j);
        }
    }

    let mut window = Window::new(
        "Conway's Game of Life - Rust",
        field_size as usize,
        field_size as usize,
        WindowOptions::default(),
    )
    .expect("Unable to create window");

     let mut buffer: Vec<u32> = vec![0; field_size * field_size];

     // Main loop for visualization
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update the buffer based on the current state of the grid
        for i in 0..field_size {
            for j in 0..field_size {
                let color = if test_array[(i, j)] == 1 {
                    0xFFFFFF // White for alive cells
                } else {
                    0x000000 // Black for dead cells
                };
                buffer[i + j * field_size] = color;
            }
        }

        // Display the buffer
        window
            .update_with_buffer(&buffer, field_size as usize, field_size as usize)
            .expect("Unable to update window");

        // Update the grid based on the rules of Conway's Game of Life
        let mut second_array: Grid<i32> = test_array.clone();
        update_array(test_array.clone(), &mut second_array, field_size);
        test_array = second_array;
    }
}

//update this shit 
fn calculate_sum_around(current_state: &Grid<i32>, i: usize, j: usize, size: usize) -> i32 {
    let mut sum = 0;

    for x in i.saturating_sub(1)..=usize::min(i + 1, size - 1) {
        for y in j.saturating_sub(1)..=usize::min(j + 1, size - 1) {
            // Skip the current cell (i, j)
            if x != i || y != j {
                sum += current_state[(x, y)];
            }
        }
    }

    sum
}

fn update_entry( array_to_update: &mut Grid<i32>, i: usize, j: usize, sum: i32) {
    if sum > 3 {
        array_to_update[(i, j)] = 0;
    }
    else if sum  ==3 {
        array_to_update[(i, j)] = 1;
    }
    else if sum < 2 {
        array_to_update[(i, j)] = 0;
    }
}


fn update_array(current_state: Grid<i32>, array_to_update: &mut Grid<i32>, size: usize) {
    for i in 0..size {
        for j in 0..size{
            update_entry(array_to_update, i, j, calculate_sum_around(&current_state, i, j, size))
        }
    }
}
