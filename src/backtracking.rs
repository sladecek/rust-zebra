use std::vec::Vec;
use std::fmt::Debug;

pub trait State: Sized {
    fn is_valid(&mut self)->bool;
    fn split(&self)->Vec<Self>;
}

pub static mut COUNTER: i32 = 0;

pub fn find_first<T>(s: T) -> Option<T> 
where T: State+Debug
{
    find_first_from(s, 0)
}

fn find_first_from<T>(s:  &mut T, level:i32) -> Option<T> 
    where T: State+Debug
{
    // println!("{}:{:?}",level, s);
    unsafe {
    COUNTER += 1;
    }
    if s.is_valid() {
        return Some(*s)
    } else {
        let children = s.split();
        for c in children {
            let cs = find_first_from(&mut c, level+1);
            if cs.is_some() {
                return cs
            }
        }
        None
    }
}