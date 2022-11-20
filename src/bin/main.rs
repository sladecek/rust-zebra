use rust8queens::backtracking::{find_first, COUNTER};
use rust8queens::myzebra::init_my_zebra;
use rust8queens::zebra::Zebra;

fn main() {
    let zs = init_my_zebra();
    println!("{}", zs);
    let sol: Option<Zebra> = find_first(zs);
    if sol.is_some() {
        unsafe {
            println!("Solution: {} {}", sol.unwrap(), COUNTER);
        }
    } else {
        println!("No solution");
    }
}
