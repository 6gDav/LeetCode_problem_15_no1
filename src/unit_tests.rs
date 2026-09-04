#[cfg(test)]
mod tests {
    use crate::solution::Solution as sol;

    #[test]
    fn revint_exp1() {
        assert_eq!(sol::three_sum(vec![-1,0,1,2,-1,-4]), [[-1,-1,2],[-1,0,1]]);
    }

    #[test]
    fn revint_exp2() {
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(sol::three_sum(vec![0,1,1]), expected);
    }

    #[test]
    fn revint_exp3() {
        let expected: Vec<Vec<i32>> = vec![vec![0, 0, 0]];
        assert_eq!(sol::three_sum(vec![0, 0, 0]), expected);
    }
}