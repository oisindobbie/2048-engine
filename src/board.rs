
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

fn merge_line(mut line: Vec<u64>) -> Vec<u64> {
    let mut changed = true;

    while changed {
        changed = false;

        let mut result = Vec::new();
        let mut i = 0;

        while i < line.len() {
            if i + 1 < line.len() && line[i] == line[i + 1] {
                // Merge
                result.push(line[i] + 1);
                i += 2;
                changed = true;
            } else {
                result.push(line[i]);
                i += 1;
            }
        }
        for i in result.len()..line.len(){
            result.push(0);
        }
        line = result
    }
    line
    
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
                    let mut line = Vec::new();
                    for row in 0..ROWS{
                        line.push(self.get(row, col));

                    }
                    let mut new_line = merge_line(line);
                    for row in 0..ROWS{
                        match new_line.pop(){
                            Some(val) => self.set(row, col, val),
                            None => panic!("length of result vec doesnt match")
                        }
                    }

                }, 

            Direction::Left => // for each cell
                for row in 0..ROWS{
                    for col in 0..COLS{
                        let curr_val = self.get(row, col);
                        if curr_val != 0{
                            //check spots left of  the cell
                            for left in 1..=col{
                                let left_val = self.get(row, col - left);
                                // merge with the value
                                if curr_val == left_val{
                                    self.set(row, col , 0);
                                    self.set(row, col - left, curr_val+1);
                                    break;
                                // there is no value above
                                }else if (col - left == 0) & (left_val == 0){
                                    self.set(row, col, 0);
                                    self.set(row, 0, curr_val);
                                    break;
                                }
                                // there is a value blocking
                                else if left_val != 0{
                                    self.set(row, col, 0);
                                    self.set(row, col - left + 1, curr_val);
                                    break;
                                }
                            }
                    }
                    }
                }, 
            Direction::Down => // for each cell
                for row in (0..ROWS).rev(){
                    for col in (0..COLS).rev(){
                        let curr_val = self.get(row, col);
                        if curr_val != 0{
                            //check spots above the cell
                            for below in (row + 1)..ROWS{
                                let below_val = self.get(below, col);
                                // merge with the value
                                if curr_val == below_val{
                                    self.set(row, col , 0);
                                    self.set(below, col, curr_val+1);
                                    break;
                                // there is no value below
                                }else if (below == ROWS - 1) & (below_val == 0){
                                    self.set(row, col, 0);
                                    self.set(ROWS - 1, col, curr_val);
                                    break;
                                }
                                // there is a value blocking
                                else if below_val != 0{
                                    self.set(row, col, 0);
                                    self.set(below - 1, col , curr_val);
                                    break;
                                }
                            }
                    }
                    }
                },
            Direction::Right =>
            // for each cell
                for row in (0..ROWS).rev(){
                    for col in (0..COLS).rev(){
                        let mut curr_val = self.get(row, col);
                        if curr_val != 0{
                            //check spots above the cell
                            for right in (col + 1)..COLS{
                                let right_val = self.get(row, right);
                                // merge with the value
                                if curr_val == right_val{
                                    //update curr_val and reset the original spot 
                                    curr_val += 1;
                                    self.set(row, col , 0);
                                    self.set(row, right, curr_val+1);

                                // there is no value below
                                }else if (right == COLS - 1) & (right_val == 0){
                                    self.set(row, col, 0);
                                    self.set(row, COLS - 1, curr_val);
                                    break;
                                }
                                // there is a value blocking
                                else if right_val != 0{
                                    self.set(row, col, 0);
                                    self.set(row, right - 1 , curr_val);
                                    break;
                                }
                            }
                        }
                    }
                }
        }
    }

}