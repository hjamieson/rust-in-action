struct Hostname(String);

#[derive(Debug)]
struct Point(usize, usize);

fn connect(host: Hostname) {
    println!("connected to {}", host.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newtype_1() {
        let text = String::from("localhost");
        let host = Hostname(text.clone());
        connect(host);
    }

    #[test]
    fn newtype_2() {
        let pt = Point(12, 17);
        println!("{:?}", pt);
    }
}
