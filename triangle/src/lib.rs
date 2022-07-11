pub struct Triangle {
    x: u64,
    y: u64,
    z: u64, 
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        // For a shape to be a triangle at all, all sides have to be of length > 0,
        // and the sum of the lengths of any two sides must be greater than or equal to
        // the length of the third side
        if sides.iter().all(|&x| x > 0)
            && (sides[0] + sides[1] >= sides[2]
                && sides[1] + sides[2] >= sides[0]
                && sides[2] + sides[0] >= sides[1])
        {
            return Some(
                Triangle {
                    x: sides[0],
                    y: sides[1],
                    z: sides[2],
                }
            );
        } 
        else {
            return None;
        }
    }

    pub fn is_equilateral(&self) -> bool {
        return self.x == self.y && self.y == self.z;
    }

    pub fn is_scalene(&self) -> bool {
        return !(self.is_equilateral() || self.is_isosceles());
    }

    pub fn is_isosceles(&self) -> bool {
        return self.x == self.y || self.y == self.z || self.z == self.x;
    }
}
