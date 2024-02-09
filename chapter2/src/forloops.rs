

fn fruits() -> Vec<String> {
    let mut list: Vec<String> = Vec::new();
    list.push("banana".to_string());
    list.push("apple".to_string());
    list.push("cherry".to_string());
    list
}

fn dwarfs() -> Vec<String> {
    let dwarfs = "happy,dopey,sleepy,grumpy,doc".split(",");

    let mut result = Vec::new();
    for dwarf in dwarfs {
        result.push(dwarf.to_string());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_fruit() {
        let list = dwarfs();
        assert_eq!(list.len(), 5);
        assert_eq!(fruits().len(), 3);
    }

    #[test]
    fn reuse_collection() {
        let list = dwarfs();    // the collection
        for dwarf in list {
            println!("{}", dwarf.len());
        }
        // list should not be valid anymore; for borrowed it!
        // println!("{}", list.len());

        let list = fruits();
        assert_eq!(list.len(), 3);
        for fruit in &list {
            println!("{fruit}");
        }
        println!("{:?}", list); // list is still valid; used & in for loop!
    }

    #[test]
    fn mutate_list() {
        let mut basket = fruits();
        for fruit in &mut basket {
            fruit.push_str("yum");
        }
        for item in &basket {
            assert!(item.contains("yum"));
        }
        println!("{:?}", basket);
    }

    #[test]
    fn simple_loop() {
        let mut counter = 0;
        for _ in 2..=5 {
            counter += 1;
        }
        assert_eq!(counter, 4 );
    }

    #[test]
    fn use_ref(){
        let needle = 0o204;
        let haystack = [1,1,2,5,15,52,132, 203,877,4140,21147];

        for item in &haystack {
            if *item == needle {
                println!("{}", item);
            }
        }
    }
}