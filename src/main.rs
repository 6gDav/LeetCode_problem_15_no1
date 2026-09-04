mod solution;
mod unit_tests;
use solution::Solution as sol;

fn main() {
    let s =  vec![0,0,0];
    let res = sol::three_sum(s);
    println!("{:?}", res)
}
