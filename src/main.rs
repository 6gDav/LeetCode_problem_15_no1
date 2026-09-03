mod solution;

use solution::Solution as sol;

fn main() {
    let s =  vec![1,2,4,5,6,7];
    let res = sol::three_sum(s);
    println!("{:?}", res)
}
