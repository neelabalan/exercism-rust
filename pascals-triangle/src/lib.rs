use std::array;

// http://people.uncw.edu/norris/133/counting/BinomialExpansion1.htm
pub struct PascalsTriangle {
    size: u32
}


pub fn factorial(num: u32) -> u32 {
    match num {
        0  => 1,
        1.. => (1..num+1).product(),
    }
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        return Self {size: row_count}
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut arows = vec![];
        for i in 0..self.size {
            let mut row = vec![];
            for j in 0..=i{
                row.push(
                    factorial(i)/(factorial(i-j)*factorial(j))
                );
            }
            arows.push(row);
        }
        return arows;
    }
}
