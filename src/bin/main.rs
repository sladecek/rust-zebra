use rust8queens::backtracking::{find_first, COUNTER};
//use rust8queens::queens::QueensState;
use rust8queens::zebra::Zebra;
use rust8queens::myzebra::init_my_zebra;

fn main() {
    //let qs = QueensState::init();
    //let sol: Option<QueensState> = find_first(qs);
    let zs = init_my_zebra();
    println!("{}",zs);
    let sol: Option<Zebra>= find_first(zs);
    if sol.is_some() {
        unsafe {
        println!("Solution: {:?} {}", sol.unwrap(), COUNTER);
        }
    } else {
        println!("No solution");
    }
}
