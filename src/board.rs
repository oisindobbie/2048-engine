
#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const ROWS:u64 = 4;
const COLS:u64 = 4;
const BITS_PER_ROW:u64 = 16;
const BITS_PER_CELL:u64 = 4;
const CELL_MASK:u64 = 0b1111;

pub struct Board{
    cells: u64,
}

fn merge_line(line: Vec<u64>) -> Vec<u64>{
    let mut result = Vec::new();
    let mut i = 0;

    while i < line.len(){
        if line[i]  != 0{
            if i+1 < line.len() && line[i] == line[i+1] {
                result.push(line[i] + 1);
                i+=1;
            }
            else{
                result.push(line[i]);
            }
        }
        i+=1;
    }

    for _i in result.len()..line.len(){
        result.push(0);
    }
    
    if result == line{
        result
    }
    else{
        merge_line(result)
    }
}

impl Board{
    pub fn new()->Self {
        Self{
            cells: 0,
        }
    }
    pub fn get(&self, row: u64, col:u64) -> u64{
        (self.cells >> (row * BITS_PER_ROW + (BITS_PER_CELL * col))) & CELL_MASK
    }
    pub fn get_ind(&self, index:u64) -> u64{
        (self.cells & (CELL_MASK << index * BITS_PER_CELL)) >> index * BITS_PER_CELL
    }
    pub fn set_ind(&mut self, index:u64, val:u64){
        self.cells &= !(CELL_MASK << index * BITS_PER_CELL);
        self.cells |= val << index * BITS_PER_CELL;
    }
   
    pub fn set(&mut self, row:u64, col:u64, val:u64){
        // clear the cell so the new value can be added
        self.cells &= !(CELL_MASK << (row * BITS_PER_ROW + (BITS_PER_CELL * col)));

        self.cells |=  val << (row * BITS_PER_ROW + (BITS_PER_CELL * col));
    }

    pub fn print_board(&self){
        for i in 0..ROWS{
            println!("{}, {}, {}, {}", self.get_ind(i*ROWS), self.get_ind(i*ROWS+1), self.get_ind(i*ROWS+2), self.get_ind(i*ROWS + 3));
        }
    }
    pub fn make_move(&mut self, direction: Direction){
        match direction{
            Direction::Up => 
                for col in 0..COLS{
                    //creates a vector for the col
                    let mut line = Vec::new();
                    for row in 0..ROWS{
                        line.push(self.get(row, col));

                    }
                    // merges the vector in accordance to 2048 rules
                    let mut new_line = merge_line(line);

                    // update the board 
                    // makes sure it is replaced in the correct order
                    for row in 1..=ROWS{
                        match new_line.pop(){
                            Some(val) => self.set(ROWS - row, col, val),
                            None => panic!("length of result vec doesnt match")
                        }
                    }

                }, 

            Direction::Left => 
                for row in 0..ROWS{
                    //creates a vector for the row
                    let mut line = Vec::new();
                    for col in 0..COLS{
                        line.push(self.get(row, col));

                    }
                    // merges the vector in accordance to 2048 rules
                    let mut new_line = merge_line(line);

                    // updates the board
                    // makes sure it is replaced in the correct order
                    for col in 1..=COLS{
                        match new_line.pop(){
                            Some(val) => self.set(row, COLS - col, val),
                            None => panic!("length of result vec doesnt match")
                        }
                    }

                }, 
            Direction::Down => 
                for col in 0..COLS{
                    //creates a vector for the col
                    // vectore is built reversed so merge works in the correct direction
                    let mut line = Vec::new();
                    for row in (0..ROWS).rev(){
                        line.push(self.get(row, col));

                    }
                    // merges the vector in accordance to 2048 rules
                    let mut new_line = merge_line(line);

                    // update the board 
                    for row in 0 ..ROWS{
                        match new_line.pop(){
                            Some(val) => self.set(row, col, val),
                            None => panic!("length of result vec doesnt match")
                        }
                    }

                },
            Direction::Right =>
                for row in 0..ROWS{
                    //creates a vector for the row
                    // vectore is built reversed so merge works in the correct direction
                    let mut line = Vec::new();
                    for col in (0..COLS).rev(){
                        line.push(self.get(row, col));
                    }
                    // merges the vector in accordance to 2048 rules
                    let mut new_line = merge_line(line);

                    // updates the board
                    for col in 0..COLS{
                        match new_line.pop(){
                            Some(val) => self.set(row, col, val),
                            None => panic!("length of result vec doesnt match")
                        }
                    }

                }
        }
    }

}