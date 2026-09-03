mod solution;
use solution::Solution as sol;

fn main() {
    let s =  vec![-1,0,1,2,-1,-4];
    let _ = sol::three_sum(s);
}
