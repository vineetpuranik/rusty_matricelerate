use rayon::prelude::*;
use std::mem;
use std::time::Instant;

const ROWS: usize = 10000;
const COLS: usize = 10000;

fn main() {
    //vector to store a matrix of ROWS and COLS
    let mut matrix = vec![vec![0; COLS]; ROWS];

    //calculate total size of the matrix
    calculate_size(&matrix);

    //populate the matrix entries
    populate_matrix_serially(&mut matrix);

    //populate the matrix parallely
    populate_matrix_parallely(&mut matrix);
}

fn calculate_size(matrix: &Vec<Vec<i32>>) {
    let vec_size = mem::size_of_val(matrix);
    let element_size = mem::size_of::<i32>();
    let element_count = ROWS * COLS;
    let total_memory = vec_size + (element_size * element_count);
    println!("Total memory consumed by matrix {}", total_memory);
}

fn populate_matrix_serially(matrix: &mut [Vec<i32>]) {
    let start_time = Instant::now();

    // Generate random numbers and populate the matrix using iterators
    matrix.iter_mut().take(ROWS).for_each(|row| {
        row.iter_mut().for_each(|element| {
            // do something with each element
            // for example: assign a random value
            *element = rand::random::<i32>();
        });
    });

    let elapsed_time = start_time.elapsed();
    println!(
        "Elapsed time populate_matrix_serially : {}s {}ms",
        elapsed_time.as_secs(),
        elapsed_time.subsec_millis()
    );
}

fn populate_matrix_parallely(matrix: &mut [Vec<i32>]) {
    let start_time = Instant::now();

    //Generate random numbers and populate the matrix in parallel
    matrix.par_iter_mut().for_each(|row| {
        row.par_iter_mut().for_each(|element| {
            *element = rand::random::<i32>();
        });
    });

    let elapsed_time = start_time.elapsed();
    println!(
        "Elapsed time populate_matrix_parallely: {}s {}ms",
        elapsed_time.as_secs(),
        elapsed_time.subsec_millis()
    );
}
