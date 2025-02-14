use std::vec;
use rand::Rng;

use crate::{toimage};


pub fn generate_maze(mut size: usize, show_animation: bool, anim_scale: usize, anim_speed_mult: usize, save_maze: bool)
{
    //Make sure maze has odd size
    if size % 2 == 0
    {
        size += 1;
    }

    let n = size;
    let m = size;    
    let mut m:Vec<Vec<u8>> = vec![vec![2; n]; m];
    m = prime_matrix(m, size);

    growing_tree(m, size, show_animation, anim_scale, anim_speed_mult, save_maze);
}

//Growing Tree Algorithm for implementing a "perfect" maze i.e. only one solution
fn growing_tree(mut mtx: Vec<Vec<u8>>, size: usize, show_animation: bool, anim_scale: usize, anim_speed_mult: usize, save_maze: bool)
{
    println!("Generating Maze");
    let cell_size = (size - 1)/2;
    let mut x = rand::thread_rng().gen_range(0..cell_size)*2 + 1;
    let mut y = rand::thread_rng().gen_range(0..cell_size)*2 + 1;
    let mut x_list = Vec::new();
    let mut y_list = Vec::new();
    x_list.push(x); //push the starting point onto the stack
    y_list.push(y);
    mtx[x][y] = 0;

    let s: u128 = size as u128;
    let itt: u128 = (s - 3)*(s + 1)/4;
    for _k in 0..itt
    {
        x = *x_list.last().unwrap();
        y = *y_list.last().unwrap();


        //Check if current tile can go farther. If not, remove it from the list and try the next one
        let mut travel = travel_to(&mtx, size, &x, &y);
        while travel.is_empty() && !x_list.is_empty()
        {
            x_list.pop();
            y_list.pop();
            if !x_list.is_empty()
            {
                x = *x_list.last().unwrap();
                y = *y_list.last().unwrap();
            }
            travel = travel_to(&mtx, size, &x, &y);
        }

        let n = rand::thread_rng().gen_range(0..travel.len());

        if !x_list.is_empty()
        {
            //movement up, right, down, or left
            match travel[n]
            {
                1 => {mtx[x][y-2] = 0;
                    mtx[x][y-1] = 0;
                    x_list.push(x);
                    y_list.push(y-2)},

                2 => {mtx[x+2][y] = 0;
                    mtx[x+1][y] = 0;
                    x_list.push(x+2);
                    y_list.push(y)},

                3 => {mtx[x][y+2] = 0;
                    mtx[x][y+1] = 0;
                    x_list.push(x);
                    y_list.push(y+2)},

                4 => {mtx[x-2][y] = 0;
                    mtx[x-1][y] = 0;
                    x_list.push(x-2);
                    y_list.push(y)},

                _ => println!("Something terrible has happened"),
            }
        }
    }

    //print_matrix(&mtx, size);
    if save_maze
    {
        toimage::mtx_to_img(&mtx, size, "unsolved.png".to_string());
    }

    println!("Maze Generation Complete");
    crate::solve::solve_maze(mtx, size, show_animation, anim_scale, anim_speed_mult, save_maze)
}

//Function to determine if current cell has any adjacent, non-visited cells
fn travel_to(mtx: &[Vec<u8>], size: usize, x: &usize, y: &usize) -> Vec<i32>
{
    let mut dirs = vec![1, 2, 3, 4];


    //look up
    if *y == 1 || mtx[*x][*y-2] == 0
    {
        dirs = remove_value(dirs, 1);
    }

    //look right
    if *x == size - 2 || mtx[*x+2][*y] == 0
    {
        dirs = remove_value(dirs, 2);
    }

    //look down
    if *y == size - 2 || mtx[*x][*y+2] == 0
    {
        dirs = remove_value(dirs, 3);
    }

    //look left
    if *x == 1 || mtx[*x-2][*y] == 0
    {
        dirs = remove_value(dirs, 4);
    }

    dirs
}

//Function to remove value from vector
fn remove_value(mut values: Vec<i32>, value: i32) -> Vec<i32>
{
    for i in 0..values.len()
    {
        if values[i] == value
        {
            values.remove(i);
            break;
        }
    }

    values
}


//Initialize the zero matrix in a "cell" format which surrounds even entries with "walls"
fn prime_matrix(mut mtx: Vec<Vec<u8>>, size: usize) -> Vec<Vec<u8>>
{   
    for i in 0..size
    {
        for j in 0..size
        {
            if (i+1) % 2 != 0 || (j+1) % 2 != 0
            {
                mtx[i][j] = u8::MAX;
            }
        }
    }

    mtx
}

