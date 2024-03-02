use chrono::DateTime;
use chrono::Local;
use clap::Arg;
use clap::Command;

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    fn set() -> ! {
        unimplemented!()
    }
}

fn main() {
    let app = Command::new("clock")
        .arg(
            Arg::new("action")
                .short('a')
                .value_parser(["get", "set"])
                .default_value("get"),
        )
        .arg(
            Arg::new("standard")
                .short('s')
                .long("standard")
                .value_parser(["timestamp", "rfc2822", "rfc3339"])
                .default_value("rfc2822"),
        );

    let args = app.get_matches();

    let action = args.get_one::<String>("action").unwrap().as_str();
    let std = args.get_one::<String>("standard").unwrap().as_str();

    match action {
        "get" => {
            let now = Clock::get();
            match std {
                "timestamp" => println!("{}", now.timestamp()),
                "rfc2822" => println!("{}", now.to_rfc2822()),
                "rfc3339" => println!("{}", now.to_rfc3339()),
                _ => unreachable!(),
            }
        }
        "set" => unimplemented!(),
        _ => unimplemented!(),
    }
}

#[cfg(test)]

mod tests {

    use clap::Arg;
    use clap::Command;

    #[test]
    fn clap_test_1() {
        let cli = Command::new("clock")
            .arg(
                Arg::new("action")
                    .value_parser(["get", "set"])
                    .default_value("get"),
            )
            .arg(
                Arg::new("standard")
                    .short('s')
                    .long("standard")
                    .value_parser(["timestamp", "rfc2822", "rfc3339"])
                    .default_value("timestamp"),
            );
        let args = cli.get_matches_from(vec!["myprog", "-s", "rfc3339"]);
        assert!(args.contains_id("action"));
        assert_eq!(args.get_one::<String>("action").unwrap(), "get");
        println!("{}", args.get_one::<String>("standard").unwrap());
    }

    #[test]
    fn reftest() {
        let a = String::from("hello");
        let ra = &a;
        let ba = Box::new(&a);
        println!("a: {}, ra: {}, ba: {}", a, ra, ba);
        let bba = ba.clone();
        println!("bba: {}", bba);
    }
}
