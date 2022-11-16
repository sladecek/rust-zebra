use crate::backtracking::State;

#[derive(Debug, Clone)]
pub struct QueensState {
    d: [u8;8]
}

impl State for QueensState {
    
    fn is_valid(&mut self)->bool {
        if self.has_two_queens_in_any_row() {
            return false;
        }

        let ur: Vec<Option<i32>> = self.d.into_iter().map(|r| QueensState::contains_only_one_queen(r)).collect();
        for y1 in 0 ..7 {
            for y2 in y1+1..8 {
                if ur[y1].is_some() && ur[y2].is_some() {
                let delta_y = (y2 - y1) as i32;
                let x1 = ur[y1].unwrap();
                let x2 = ur[y2].unwrap();
                let delta_x = if x1 > x2 { x1-x2 } else {x2-x1} ;
                if delta_x == 0 || delta_x == delta_y {
                    return false
                }
            }
            }
        }
        true
    }
    fn split(&self)->Vec<QueensState> {
        let mut result : Vec<QueensState> = vec!();
       for i in 0..8 {
            let row = self.d[i];
           if row !=0 && QueensState::contains_only_one_queen(row).is_none() {
               for j in 0..8 {
                   if row & (1 << j) != 0 {
                       let mut s = self.clone();
                       s.d[i] = 1 << j;
                       for k in i+1..8 {
                        s.d[k] &= !(1 << j);
                       }
                        result.push(s);

                   }
               }
               break;
           }
       }
       result
    }
}

impl QueensState {
    fn init() -> Self {
        QueensState{ d: [255u8; 8]}
    }
    
    pub fn contains_only_one_queen(value: u8) -> Option<i32> {
        for i in 0..8 {
            if value == 1 << i {
                return Some(i)
            }
        }
        None
    }

    pub fn rows_with_one_queen(&self) -> Vec<i32> {
        let mut r = vec!();
        for i in self.d {
            let j = Self::contains_only_one_queen(i);
            if j.is_some() {
                r.push(j.unwrap());
            }
        }
        r
    }

    pub fn has_two_queens_in_any_row(&self) -> bool {
        self.d.into_iter().filter(|i| *i!=0u8).any(|i| QueensState::contains_only_one_queen(i).is_none())
    }

}

#[test]
fn test_contains_only_one_queen() {
    assert_eq!(None, QueensState::contains_only_one_queen(0u8));
    assert_eq!(None, QueensState::contains_only_one_queen(3u8));
    assert_eq!(Some(0), QueensState::contains_only_one_queen(1u8));
    assert_eq!(Some(7), QueensState::contains_only_one_queen(128u8));
}

#[test]
fn test_unique_rows() {
    let s = QueensState{ d: [3u8, 1u8, 2u8, 5u8, 9u8, 10u8, 3u8, 3u8]};
    let u = QueensState::rows_with_one_queen(&s);
    assert_eq!(vec![0,1], u);
}

#[test]
fn test_is_valid_empty() {
    let s = QueensState{ d: [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]};
    assert_eq!(true, s.is_valid());
}

#[test]
fn test_is_valid_row() {
    let s = QueensState{ d: [0u8, 129u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]};
    assert_eq!(false, s.is_valid());
}

#[test]
fn test_is_valid_column() {
    let s = QueensState{ d: [1u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]};
    assert_eq!(false, s.is_valid());
}


#[test]
fn test_is_valid_main_diag() {
    let s = QueensState{ d: [4u8, 0u8, 16u8, 0u8, 0u8, 0u8, 0u8, 0u8]};
    assert_eq!(false, s.is_valid());
}


#[test]
fn test_is_valid_other_diag() {
    let s = QueensState{ d: [0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 4u8]};
    assert_eq!(false, s.is_valid());
}

#[test]
fn test_is_valid_solution() {
    let s = QueensState{ d: [1u8, 16u8, 128u8, 32u8, 4u8, 64u8, 2u8, 8u8]};
    assert_eq!(true, s.is_valid());
}


#[test]
fn test_split() {
    let s = QueensState{ d: [0u8, 0u8, 0u8, 0u8, 0u8, 17u8, 0u8, 4u8]};
    let h = s.split();
    assert_eq!(2, h.len());
    assert_eq!([0u8, 0u8, 0u8, 0u8, 0u8, 16u8, 0u8, 4u8], h[1].d);

    assert_eq!([0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 4u8], h[0].d);
}
