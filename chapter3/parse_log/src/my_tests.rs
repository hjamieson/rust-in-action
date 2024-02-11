#[cfg(test)]

fn times_2(nums: &[i32]) -> Vec<i32> {
    let mut res = Vec::new();
    for n in nums {
        res.push(n * 2);
    }
    res
}

mod tests {
    use super::*;

    #[test]
    fn my_tests_1() {
        let v1 = vec![1,2,3];
        let v2 = times_2(&v1);
        assert_eq!(v2[0], 2);
    }
}