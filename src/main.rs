use std::time::Instant;
use std::io;
use std::io::{stdin, stdout, Read, Write};
mod generate;
mod toimage;
mod solve;
mod utils;
mod algo_tremaux;

fn main()
{
    println!("----------------------------------------------------");
    println!("----------------Maze Cruncher v1.0.3----------------");
    println!("----------------------------------------------------");


    let mut size: usize = 75;
    let mut save_maze = true;
    let mut show_animation = true;
    let mut anim_scale = 4;
    let mut anim_speed = 2;


    let use_default = read_bool("Use default settings?\ny/n: ".to_string());

    if !use_default
    {
        save_maze = read_bool("Save the solved and unsolved maze?\ny/n".to_string());
        if save_maze
        {
            println!("A solution image will be saved to this directory");
        }
        size = read_int("Enter a maze size: ".to_string(), size);
        

        if size <= 2048
        {
            show_animation = read_bool("Do you want to display the solution animation? (takes longer but looks cool)\ny/n".to_string());

            if show_animation
            {
                anim_scale = read_int("Enter animation scale: ".to_string(), anim_scale);
                anim_speed = read_int("Enter animation speed: ".to_string(), anim_speed);
            }
        }
        else 
        {
            show_animation = false;
        }

    }

    println!("Initializing");

    let before = Instant::now();
    generate::generate_maze(size, show_animation, anim_scale, anim_speed, save_maze); //Solver is called from within generator
    println!("Elapsed time: {:.2?}", before.elapsed());

    println!("Done!");
    
    if save_maze
    {
        println!("Solved and unsolved images of this maze were saved to this directory");
    }
    pause();
}

fn read_int(message: String, default_value: usize) -> usize
{
    
    let mut input = String::new();
    println!();
    println!("{}", message);

    io::stdin().read_line(&mut input).expect("Failed to read");
    let trimmed = input.trim().to_string();
    match trimmed.parse::<u32>() {
        Ok(i) => return i as usize,
        Err(..) => println!("Invalid Input: {}", trimmed),
    };

    default_value
}

fn read_bool(message: String) -> bool
{
    let mut input = String::new();
    println!();
    println!("{}", message);

    io::stdin().read_line(&mut input).expect("Failed to read");
    let trimmed: String = input.trim().to_string().to_lowercase();
    trimmed == "y"
}

fn pause()
{
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}