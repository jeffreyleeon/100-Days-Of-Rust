use std::collections::{HashMap, HashSet};

mod models;
use models::TrieNode;

struct Solution;

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut result = HashSet::new();
        let mut root = TrieNode::new();

        // Build the trie
        for word in &words {
            let mut node = &mut root;
            for ch in word.chars() {
                node = node.children.entry(ch).or_insert(TrieNode::new());
            }
            // Mark the end of the word in the last character
            node.is_word = true;
        }

        let m = board.len();
        let n = board[0].len();
        let mut visited = vec![vec![false; n]; m];

        // Perform DFS on each cell
        for x in 0..m {
            for y in 0..n {
                Self::dfs(&board, x, y, &mut visited, &mut root, &mut String::new(), &mut result);
            }
        }
        result.into_iter().collect()
    }

    fn dfs(
        board: &Vec<Vec<char>>,
        x: usize,
        y: usize,
        visited: &mut Vec<Vec<bool>>,
        node: &mut TrieNode,
        current_word: &mut String,
        result: &mut HashSet<String>,
    ) {
        // Out of bounds
        if x < 0 || x >= board.len() || y < 0 || y >= board[0].len() || visited[x][y] {
            return;
        }

        let ch = board[x][y];
        if let Some(next_node) = node.children.get_mut(&ch) {
            visited[x][y] = true;
            current_word.push(ch);

            if next_node.is_word {
                result.insert(current_word.clone());
            }

            // Explore all 4 directions
            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            for (dx, dy) in directions.iter() {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;
                Self::dfs(
                    board,
                    new_x as usize,
                    new_y as usize,
                    visited,
                    next_node,
                    current_word,
                    result,
                );
            }

            visited[x][y] = false;
            current_word.pop();
        }
    }
}

fn main() {
    let board = vec![
        vec!['o', 'a', 'a', 'n'],
        vec!['e', 't', 'a', 'e'],
        vec!['i', 'h', 'k', 'r'],
        vec!['i', 'f', 'l', 'v'],
    ];
    let words = vec![
        "oath".to_string(),
        "pea".to_string(),
        "eat".to_string(),
        "rain".to_string(),
    ];

    let result = Solution::find_words(board, words);
    println!("{:?}", result);  // Output: ["eat", "oath"]
}