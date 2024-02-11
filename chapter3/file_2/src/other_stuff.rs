#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn other_clone() {
        let mut v1 = vec![1,2,3];
        let mut v2 = vec![4,5,6];
        v1.append(&mut v2);
        println!("v1 now {:?}", v1);
        println!("v2 now {:?}", v2);
    }
}