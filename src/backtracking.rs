use std::fmt::Display;
use std::vec::Vec;

pub trait State: Sized {
    fn is_solution(&self) -> bool;
    fn split(&self) -> Vec<Self>;
    fn apply_facts(&mut self, change_counter: &mut i32) -> bool;
    fn apply_predicates(&mut self) -> bool;
    fn apply_permutations(&mut self, change_counter: &mut i32) -> bool;
}

pub static mut COUNTER: i32 = 0;

pub fn find_first<T>(s: T) -> Option<T>
where
    T: State + Display + Clone,
{
    find_first_from(s, 0)
}

fn find_first_from<T>(ss: T, level: i32) -> Option<T>
where
    T: State + Display + Clone,
{
    let mut s = ss.clone();
    unsafe {
        COUNTER += 1;
    }

    loop {
        let mut change_counter = 0;
        let mut valid = s.apply_facts(&mut change_counter);
        valid = valid && s.apply_predicates();
        valid = valid && s.apply_permutations(&mut change_counter);
        if !valid {
            return None;
        }
        if change_counter == 0 {
            break;
        }
    }
    if s.is_solution() {
        return Some(s);
    }

    let children = s.split();
    for c in children {
        let cs = find_first_from(c, level + 1);
        if cs.is_some() {
            return cs;
        }
    }
    None
}
