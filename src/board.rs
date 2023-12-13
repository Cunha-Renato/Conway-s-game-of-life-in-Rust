#[derive(Clone)]
pub struct Board
{
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<u8>>,
}

impl Board
{
    pub fn new(width: usize, height: usize) -> Self
    {
        Board{width, height, grid: Vec::new()}
    }

    // Cell must be in (y, x) and Return is in (y, x)
    pub fn get_neighbors(&self, cell: &(usize, usize)) -> Vec<u8>
    {
        let mut result: Vec<u8> = Vec::new();    
        
        // Define the relative coordinates of the neighbors
        let relative_coords = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

        for (x, y) in relative_coords
        {
            let new_x = cell.1 as isize + x;
            let new_y = cell.0 as isize + y;
            
            if new_x >= 0 && new_x < self.width as isize &&
               new_y >= 0 && new_y < self.height as isize
            {
                result.push(self.grid[new_y as usize][new_x as usize]);
            } 
        }
        result
    }
}
