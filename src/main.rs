pub mod board;
use board::*; 
use std::thread::sleep;
use std::time::Duration;

fn on_render(board: &Board) 
{
    board.grid.iter().for_each(|y| {
        y.iter().for_each(|x| {
            let res = if *x>0 {"$"}    else {" "};            
            print!("[{}]", res);
        });
    
        println!("");
    });
}

/*
1-Any live cell with fewer than two live neighbours dies, as if by underpopulation.
2-Any live cell with two or three live neighbours lives on to the next generation.
3-Any live cell with more than three live neighbours dies, as if by overpopulation.
4-Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
*/
fn impl_rules(board: &Board, cell: &(usize, usize)) -> u8
{
    let neighbors = board.get_neighbors(cell);
    let alive = neighbors.iter().filter(|x| { **x > 0 }).count();
    let mut current_cell = board.grid[cell.0][cell.1];

    if current_cell == 1 && (alive < 2 || alive > 3) { current_cell = 0; }
    if current_cell == 0 && alive == 3               { current_cell = 1; }

    return current_cell;
}

fn on_update(board: &mut Board)
{
    let mut new_board = board.clone();

    board.grid.iter().enumerate().for_each(|(y, val_y)| {
        val_y.iter().enumerate().for_each(|(x, _)| {
            new_board.grid[y][x] = impl_rules(board, &(y, x));
        })
    });

    *board = new_board;
}

fn main() 
{
    let grid = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 0],
        vec![1, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let mut board: Board = Board { width: 8, height: 8, grid:  grid};

    loop 
    {
        on_render(&board);
        on_update(&mut board);
        print!("\x1B[2J");
        sleep(Duration::from_millis(500));
    }
}
