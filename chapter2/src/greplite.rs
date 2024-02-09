static  QUOTE: &str = "\
now is the time
for all good men
to come to the aid
of their party";

fn grep<'a>(term: &str, txt: &'a str) -> Option<(&'a str, u32)> {
    let mut result: Option<(&str, u32)> = None;
    let mut line_nbr = 0;
    for s in txt.lines() {
        if s.contains(term){
            result = Some((s, line_nbr));
        }
        line_nbr += 1;
    }
    result
}

fn grep2<'a>(term: &str, txt: &'a str) -> Option<(&'a str, usize)> {
    let mut result: Option<(&str, usize)> = None;
    for (i, s) in txt.lines().enumerate() {
        if s.contains(term){
            result = Some((s, i));
        }
    }
    result
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn grep_read_lines(){
        let mut count = 0;
        for _ in QUOTE.lines(){
            count += 1;
        }
        assert_eq!(count, 4);
    }
    #[test]
    fn grep_find_lines(){
        let term = "party";
        let mut found: Option<&str> = None;

        for line in QUOTE.lines(){
            if line.contains(term){
                found = Some(line);
            }
        }
        assert!(found.is_some());
        println!("{}", found.unwrap());
    }

    #[test]
    fn grep_test_grep() {
        let result = grep("men", QUOTE);
        assert!(result.is_some());
        assert_eq!(result.unwrap().0, "for all good men");
        assert_eq!(result.unwrap().1, 1);
    }

    #[test]
    fn grep_test_grep2() {
        let result = grep("aid", QUOTE);
        assert!(result.is_some());
        assert_eq!(result.unwrap().0, "to come to the aid");
        assert_eq!(result.unwrap().1, 2);
    }
}