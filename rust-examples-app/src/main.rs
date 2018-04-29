#[macro_use] extern crate nickel;
// extern crate chrono;
// use chrono::prelude::*;
use nickel::Nickel;

fn say_hello() -> &'static str {
    //let lastchecked = Instant::now();
    //let a = String::from(lastchecked);
    //let b = String::from("Last checked: ");
    //let c = String::
    //"<!DOCTYPE html><title>EPIC STATUS</title><body>EPIC IS UP! Last checked {}.</body>", (lastcheck);
    "hello"
}

// pub fn get_unix_timestamp_us() -> i64 {
//     let now = Utc::now();
//     let seconds: i64 = now.timestamp();
//     let nanoseconds: i64 = now.nanosecond() as i64;
//     (seconds * 1000 * 1000) + (nanoseconds / 1000)
// }

fn main() {
    // let newtimestamp = get_unix_timestamp_us();
    // println!("{:?}", newtimestamp);
    let mut server = Nickel::new();
            // let d = SystemTime::now();
                // let datetime = DateTime::<Utc>::from(newtimestamp);
    // Formats the combined date and time with the specified format string.
    // let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    // println!{"{}",timestamp_str};

    server.utilize(router! {
        get "**" => |_req, _res| {

            //timestamp_str
            //say_hello()
            "made it"
        }
    });

    server.listen("127.0.0.1:6767");
}