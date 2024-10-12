use rand::{
    Rng,
    seq::SliceRandom,
};

#[derive(Clone)]
struct Cell {
    visited: bool,
    walls: [bool; 4], // top, right, bottom, left
}

fn generate_maze(width: usize, height:usize) -> Vec<Vec<Cell>> {
    let mut maze = vec![
        vec![
            Cell {visited: false, walls: [true; 4]};
            width
        ];
        height
    ];
    let mut rng = rand::thread_rng();

    fn dfs(x: usize, y: usize, maze: &mut Vec<Vec<Cell>>, rng: &mut rand::rngs::ThreadRng) {
        maze[y][x].visited = true;
        let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)]; // top, right, bottom, left
        let mut order: Vec<usize> = (0..4).collect();
        order.shuffle(rng); // Add randoness to the order of directions
    
        for &dir in order.iter() {
            let next_x = x as i32 + directions[dir].0;
            let next_y = y as i32 + directions[dir].1;
            
            if next_x >= 0 && next_x < maze[0].len() as i32 && next_y >= 0 && next_y < maze.len() as i32 {
                let next_x = next_x as usize;
                let next_y = next_y as usize;
                if !maze[next_y][next_x].visited {
                    // if the cell is not visited, remove the wall between the current cell and the next cell
                    maze[y][x].walls[dir] = false; // remove wall from current cell
                    maze[next_y][next_x].walls[(dir + 2) % 4] = false; // remove wall from next cell
                    dfs(next_x, next_y, maze, rng);
                }
            }
        }
    }

    // Begins the depth-first search algorithm from random cell
    dfs(rng.gen_range(0..width), rng.gen_range(0..height), &mut maze, &mut rng);
    maze
}

fn draw_maze(maze: &Vec<Vec<Cell>>) {
    for y in 0..maze.len() {
        // Draw top walls
        for x in 0..maze[0].len() {
            print!("{}", if maze[y][x].walls[0] { "+---" } else { "+   " });
        }
        println!("+");

        // Draw side walls and cells
        for x in 0..maze[0].len() {
            print!("{}", if maze[y][x].walls[3] { "|   " } else { "    " });
        }
        println!("|");
    }

    // Draw bottom wall
    for _ in 0..maze[0].len() {
        print!("+---");
    }
    println!("+");
}

fn main() {
    let width = 6;
    let height = 4;
    let maze = generate_maze(width, height);
    draw_maze(&maze);
}
