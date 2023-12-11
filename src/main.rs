/* 
1-Any live cell with fewer than two live neighbours dies, as if by underpopulation.
2-Any live cell with two or three live neighbours lives on to the next generation.
3-Any live cell with more than three live neighbours dies, as if by overpopulation.
4-Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction. 
*/

struct Board
{
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

enum Rules
{
    ONE,
    TWO,
    THREE,
    FOUR 
} 

fn on_render(board: &Board) 
{
    board.grid.iter().for_each(|y| {
        y.iter().for_each(|x| {
            let res = if *x>0 {"*"}    else {" "};            
            print!("[{}]", res);
        });
    
        println!("");
    });
}

// Cell must be in (y, x) and Return is in (y, x)
fn get_neighbors(board: &Board, cell: &(usize, usize)) -> Vec<u8>
{
    let mut result: Vec<u8> = Vec::new();    
    
    // Define the relative coordinates of the neighbors
    let relative_coords = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

    for (x, y) in relative_coords
    {
        let new_x = cell.1 as isize + x;
        let new_y = cell.0 as isize + y;
        
        if new_x >= 0 && new_x < board.width as isize &&
           new_y >= 0 && new_y < board.height as isize
        {
            result.push(board.grid[new_y as usize][new_x as usize]);
        } 
    }
    result
}

// Any live cell with fewer than two live neighbours dies, as if by underpopulation.
fn impl_rule_one(board: &Board, cell: &(usize, usize)) -> u8
{
    let neighbors = get_neighbors(board, cell);
    let alive = neighbors.iter().filter(|x| **x > 0).count();

    if alive < 2 {0} else {1}
}

fn on_update(board: &mut Board, state: &mut Rules)
{
    match state
    {
        Rules::ONE =>
        {
            
            *state = Rules::TWO;
        }
        Rules::TWO =>
        {
            
            *state = Rules::THREE;
        }
        Rules::THREE =>
        {
            
            *state = Rules::FOUR;
        }
        Rules::FOUR =>
        {
            
            *state = Rules::ONE;
        }
    }

}
fn main() 
{
    let grid = vec![
        vec![0, 1, 0, 1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 0, 1, 0, 1],
    ];

    let mut state = Rules::ONE; 
    let mut board: Board = Board { width: 8, height: 8, grid:  grid};
    on_update(&mut board, &mut state);
}
