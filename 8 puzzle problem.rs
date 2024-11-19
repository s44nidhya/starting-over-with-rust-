use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    board: Vec<Vec<u8>>, // 3x3 board
    empty_pos: (usize, usize), // (row, col) of the empty space (0)
    moves: usize, // Number of moves so far
    cost: usize, // Heuristic cost + moves
}

// For the priority queue
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse for min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    // Starting state
    let start = vec![
        vec![1, 2, 3],
        vec![4, 0, 5],
        vec![7, 8, 6],
    ];

    // Goal state
    let goal = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 0],
    ];

    if let Some(solution) = solve_8_puzzle(start, goal) {
        println!("Solved in {} moves!", solution.len());
        for state in solution {
            print_board(&state.board);
        }
    } else {
        println!("No solution found.");
    }
}

// Solves the 8-puzzle using A* search
fn solve_8_puzzle(start: Vec<Vec<u8>>, goal: Vec<Vec<u8>>) -> Option<Vec<State>> {
    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();

    // Find the initial empty position
    let empty_pos = find_empty(&start)?;

    // Initial state
    open_set.push(State {
        board: start.clone(),
        empty_pos,
        moves: 0,
        cost: heuristic_cost(&start, &goal),
    });

    while let Some(current) = open_set.pop() {
        // Check if the current state is the goal
        if current.board == goal {
            return Some(reconstruct_path(current));
        }

        // Skip already visited states
        if closed_set.contains(&current.board) {
            continue;
        }
        closed_set.insert(current.board.clone());

        // Generate valid neighbors
        for neighbor in generate_neighbors(&current, &goal) {
            if !closed_set.contains(&neighbor.board) {
                open_set.push(neighbor);
            }
        }
    }

    None // No solution found
}

// Generate valid moves from the current state
fn generate_neighbors(state: &State, goal: &Vec<Vec<u8>>) -> Vec<State> {
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // Up, Down, Left, Right
    let mut neighbors = vec![];

    for &(dr, dc) in &directions {
        let new_row = state.empty_pos.0 as isize + dr;
        let new_col = state.empty_pos.1 as isize + dc;

        if new_row >= 0 && new_row < 3 && new_col >= 0 && new_col < 3 {
            let new_row = new_row as usize;
            let new_col = new_col as usize;

            // Create a new state by swapping tiles
            let mut new_board = state.board.clone();
            new_board[state.empty_pos.0][state.empty_pos.1] = new_board[new_row][new_col];
            new_board[new_row][new_col] = 0;

            neighbors.push(State {
                board: new_board,
                empty_pos: (new_row, new_col),
                moves: state.moves + 1,
                cost: state.moves + 1 + heuristic_cost(&state.board, goal),
            });
        }
    }

    neighbors
}

// Heuristic function: Manhattan distance
fn heuristic_cost(board: &Vec<Vec<u8>>, goal: &Vec<Vec<u8>>) -> usize {
    let mut cost = 0;
    for i in 0..3 {
        for j in 0..3 {
            let value = board[i][j];
            if value != 0 {
                let (goal_row, goal_col) = find_value(goal, value).unwrap();
                cost += (i as isize - goal_row as isize).abs() as usize
                    + (j as isize - goal_col as isize).abs() as usize;
            }
        }
    }
    cost
}

// Find the position of a value in the board
fn find_value(board: &Vec<Vec<u8>>, value: u8) -> Option<(usize, usize)> {
    for (i, row) in board.iter().enumerate() {
        if let Some(j) = row.iter().position(|&x| x == value) {
            return Some((i, j));
        }
    }
    None
}

// Find the position of the empty space (0)
fn find_empty(board: &Vec<Vec<u8>>) -> Option<(usize, usize)> {
    find_value(board, 0)
}

// Reconstruct the path from the final state
fn reconstruct_path(state: State) -> Vec<State> {
    let mut path = vec![state];
    path.reverse();
    path
}

// Print the board
fn print_board(board: &Vec<Vec<u8>>) {
    for row in board {
        println!("{:?}", row);
    }
    println!();
}
