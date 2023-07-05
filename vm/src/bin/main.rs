use lazy_regex::{regex, regex_captures};

fn main() {
    println!("Hello, world!");

    let string = "Hello!";
    for c in string.chars() {
        //        if 'H'.as_bytes() == c.as_bytes() {
        //            println!("TEST");
        //        }

        dbg!(c);
    }

    let s = include_str!("../../tt.tmp");

    let r = regex!("\r\n");
    dbg!(r.is_match(&s));

    for c in s.chars() {
        dbg!(c);
    }

    let s = "100_u64;";
    if let Some((_, cap, _, _, _, _)) =
        regex_captures!(r"^((\d+_?)+\.?(\d+_?)+([ufsize]+\d{0,3})?)(;|\s|$)", s)
    {
        println!("{cap}");
    }
    else if let Some((cap, _, _, _, _, _)) =
        regex_captures!(r"^((\d+_?)+\.?(\d+_?)+([ufsize]+\d{0,3})?)(;|\s|$)", s)
    {
    }
}
