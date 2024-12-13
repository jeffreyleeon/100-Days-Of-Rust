use std::collections::VecDeque;

// Define the directions for moving horizontally and vertically
const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn color_maze(sequence: Vec<char>, maze: Vec<Vec<char>>) -> Option<Vec<(usize, usize)>> {
    let rows = maze.len();
    let cols = maze[0].len();
    let mut visited = vec![vec![false; cols]; rows];

    // Find the starting point on the bottom row
    let mut start = None;
    for (j, &color) in maze[rows - 1].iter().enumerate() {
        if color == sequence[0] {
            start = Some((rows - 1, j));
            break;
        }
    }

    if start.is_none() {
        return None;
    }

    let start = start.unwrap();

    // Perform DFS
    let mut path = VecDeque::new();
    path.push_back(start);
    visited[start.0][start.1] = true;

    fn dfs(
        current: (usize, usize),
        sequence: &Vec<char>,
        maze: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        path: &mut VecDeque<(usize, usize)>,
        index: usize,
    ) -> Option<Vec<(usize, usize)>> {
        let rows = maze.len();
        let cols = maze[0].len();
        if index == sequence.len() - 1 && current.0 == 0 {
            // If we reached the top row and the end of the sequence, return the path
            return Some(path.iter().cloned().collect());
        }

        for &(dx, dy) in &DIRECTIONS {
            let nx = current.0 as isize + dx;
            let ny = current.1 as isize + dy;

            if nx >= 0 && nx < rows as isize && ny >= 0 && ny < cols as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                if !visited[nx][ny] && maze[nx][ny] == sequence[index + 1] {
                    visited[nx][ny] = true;
                    path.push_back((nx, ny));

                    if let Some(solution) = dfs((nx, ny), sequence, maze, visited, path, index + 1) {
                        return Some(solution);
                    }

                    // Backtrack
                    path.pop_back();
                    visited[nx][ny] = false;
                }
            }
        }

        None
    }

    dfs(start, &sequence, &maze, &mut visited, &mut path, 0)
}

fn main() {
    let sequence = vec!['R', 'O', 'Y', 'P', 'O'];
    let maze = vec![
        vec!['R', 'R', 'B', 'R', 'R', 'R', 'B', 'P', 'Y', 'G', 'P', 'B', 'B', 'B', 'G', 'P', 'B', 'P', 'P', 'R', 'B', 'G', 'Y', 'P', 'R', 'P', 'Y', 'Y', 'O', 'R', 'Y', 'P', 'P', 'Y', 'Y', 'R', 'R', 'R', 'P', 'P', 'B', 'P', 'G', 'R', 'O', 'P', 'Y', 'G', 'R', 'Y', 'Y', 'G', 'P', 'O', 'R', 'Y', 'P', 'B', 'O', 'O', 'R', 'B', 'B', 'O', 'R', 'P', 'Y', 'O', 'O', 'Y', 'R', 'P', 'B', 'R', 'G', 'R', 'B', 'G', 'P', 'G', 'R', 'P', 'Y', 'G', 'G', 'G', 'P', 'Y', 'P', 'Y', 'O', 'G', 'B', 'O', 'R', 'Y', 'P', 'B', 'Y', 'O', 'O', 'R', 'B', 'G', 'B', 'Y', 'B', 'P', 'G', 'R', 'P', 'Y', 'R', 'O', 'G', 'Y', 'G', 'Y', 'R', 'P', 'B', 'G', 'O', 'O', 'O', 'G', 'B', 'B', 'R', 'O', 'Y', 'Y', 'Y', 'Y', 'P', 'B', 'Y', 'Y', 'G', 'G', 'P', 'P', 'G', 'B', 'O', 'P', 'Y', 'G', 'B', 'R', 'O', 'G', 'B', 'G', 'R', 'O', 'Y', 'R', 'B', 'R', 'Y', 'Y', 'P', 'P', 'R', 'B', 'Y', 'B', 'P', 'O', 'O', 'G', 'P', 'Y', 'R', 'P', 'P', 'Y', 'R', 'Y', 'P', 'O', 'O', 'B', 'B', 'B', 'G', 'O', 'Y', 'G', 'O', 'P', 'B', 'G', 'Y', 'R', 'R', 'Y', 'R', 'B', 'P', 'P', 'Y', 'R', 'B', 'O', 'O', 'R', 'O', 'R', 'Y', 'B', 'G', 'B', 'G', 'O', 'O', 'P', 'B', 'Y', 'B', 'B', 'R', 'G', 'Y', 'G', 'P', 'Y', 'G', 'P', 'R', 'R', 'P', 'Y', 'G', 'O', 'O', 'Y', 'R', 'R', 'O', 'G', 'R', 'Y', 'B', 'P', 'Y', 'O', 'P', 'B', 'R', 'Y', 'B', 'G', 'P', 'G', 'O', 'O', 'B', 'P', 'R', 'Y', 'G', 'P', 'G', 'G', 'O', 'R', 'Y', 'O', 'O', 'G', 'R', 'G', 'P', 'P', 'Y', 'P', 'B', 'G', 'P', 'Y', 'P', 'R', 'O', 'O', 'R', 'O', 'Y', 'R', 'P', 'O', 'R', 'Y', 'P', 'Y', 'B', 'B', 'Y', 'R', 'O', 'Y', 'P', 'G', 'R', 'P', 'R', 'G', 'P', 'O', 'B', 'B', 'R', 'B', 'O', 'B', 'Y', 'Y', 'B', 'P', 'B', 'Y', 'Y', 'P', 'O', 'Y', 'O', 'Y', 'O', 'R', 'B', 'R', 'G', 'G', 'Y', 'G', 'R', 'G', 'Y', 'G', 'Y', 'B', 'Y', 'Y', 'G', 'B', 'R', 'R', 'O', 'B', 'O', 'P', 'P', 'O', 'B', 'O', 'R', 'R', 'R', 'P', 'P', 'O', 'O', 'O', 'P', 'Y', 'G', 'G', 'Y', 'P', 'O', 'G', 'P', 'O', 'B', 'G', 'P', 'R', 'P', 'B', 'R', 'B', 'B', 'R', 'R', 'R', 'R', 'B', 'B', 'B', 'Y', 'O', 'B', 'G', 'P', 'G', 'G', 'O', 'O', 'Y'],
        vec!['G', 'O', 'R', 'O', 'Y', 'O', 'R', 'B', 'C', 'R', 'G', 'O', 'G', 'O', 'G', 'Y', 'G', 'B', 'Y', 'G', 'R', 'O', 'R', 'B', 'R'],
        vec!['B', 'O', 'R', 'O', 'Y', 'O', 'R', 'B', 'G', 'R', 'B', 'O', 'G', 'O', 'Y', 'Y', 'G', 'B', 'Y', 'G', 'R', 'O', 'R', 'B', 'R'],
        vec!['G', 'O', 'R', 'O', 'Y', 'O', 'R', 'B', 'G', 'R', 'G', 'O', 'G', 'O', 'G', 'Y', 'G', 'B', 'Y', 'G', 'R', 'O', 'R', 'B', 'R'],
    ];

    match color_maze(sequence, maze) {
        Some(path) => println!("Solution: {:?}", path),
        None => println!("No solution!"),
    }
}
