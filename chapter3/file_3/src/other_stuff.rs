
#[allow(unused)]
pub const V1:&str = "Hello";

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn other_clone() {
        let hush = String::from(V1);
        let mut v1 = vec![1,2,3];
        let mut v2 = vec![4,5,6];
        v1.append(&mut v2);
        println!("v1 now {:?}", v1);
        println!("v2 now {:?}", v2);
    }

    use std::io::Write;

    #[test]
    fn other_write() -> std::io::Result<()> {
        let mut dest: Vec<u8>  = Vec::new();
        write!(&mut dest, "test")?;
        assert_eq!(dest, b"test");

        write!(&mut dest, "{}", "happy")?;
        assert_eq!(dest, b"testhappy");
        Ok(())
    }
}