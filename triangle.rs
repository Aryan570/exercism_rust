use std::ops::Add;
pub struct Triangle {
    is_equi : bool,
    is_iso : bool,
    is_sca : bool
}
pub trait Zero {
    const ZERO : Self;
}
impl Zero for u64 {
    const ZERO : Self = 0;
}
impl Zero for f64 {
    const ZERO : Self = 0.0;
}

impl Triangle {
    fn valid_triangle<T : Add + Copy + PartialEq + Zero>(sides: &[T; 3]) -> bool where <T as Add>::Output: PartialOrd<T>{
        let mut cnt = 0;
        if sides[0] + sides[1] >= sides[2]
            && sides[1] + sides[2] >= sides[0]
            && sides[2] + sides[0] >= sides[1]
        {
            cnt += 3;
        }
        cnt == 3 && !sides.contains(&T::ZERO)
    }
    pub fn build<T : Add + Copy + PartialEq + Zero>(sides: [T; 3]) -> Option<Triangle> where <T as Add>::Output: PartialOrd<T>{
        if !Self::valid_triangle(&sides) {
            return None;
        }
        match sides[0] == sides[1] && sides[1] == sides[2] {
            true => Some(Triangle { is_equi: true, is_iso: true, is_sca: false }),
            _ => {
                match sides[0]!=sides[1] && sides[0]!=sides[2] && sides[1]!=sides[2] {
                    true => Some(Triangle { is_equi: false, is_iso: false, is_sca: true }),
                    _ => Some(Triangle { is_equi: false, is_iso: true, is_sca: false })
                }
            }
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.is_equi
    }

    pub fn is_scalene(&self) -> bool {
        self.is_sca
    }

    pub fn is_isosceles(&self) -> bool {
        self.is_iso
    }
}
