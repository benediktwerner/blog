#![feature(phase)]
extern crate regex;
#[phase(plugin)] extern crate regex_macros;

fn main() {
    let re = regex!(r"(.*");
    println!("{}", re.find("Today's date is 2014-04-21."));
}
