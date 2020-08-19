use std::str::Chars;
pub struct CharIterator<'a> {
    chars: Chars<'a>,
    eof: bool,
}

impl<'a> CharIterator<'a> {
    pub fn new(src: &mut String) -> CharIterator {
        CharIterator {
            chars: src.chars(),
            eof: false,
        }
    }

    fn read_until(&mut self, a: &str) -> String {
        let mut value = String::new();
        let mut i: usize = 0;
        loop {
            match self.chars.next() {
                None => {
                    self.eof = true;
                    return value;
                }
                Some(c) => {
                    if c == a.chars().nth(i).unwrap() {
                        i += 1;
                        value.push(c);
                        if i == a.len() {
                            break;
                        }
                    } else {
                        i = 0;
                        value.push(c);
                    }
                }
            };
        }
        let take = value.chars().count() as i64 - a.chars().count() as i64;
        if take > 0 {
            value.chars().take(take as usize).collect()
        } else {
            String::new()
        }
    }

    fn read_n(&mut self, n: usize) -> String {
        let mut value = String::new();
        for _ in 1..n {
            match self.chars.next() {
                None => {
                    self.eof = true;
                    break;
                }
                Some(c) => value.push(c),
            }
        }
        value
    }

    fn read_char(&mut self) -> Option<char> {
        self.chars.next()
    }
}

fn minify(s: String) -> String {
    // println!("{}", s.replace("\n", ""));
    s.trim().replace("  ", "")
    // String::new()
}

pub fn parse() -> Result<(), ()> {
    let mut src = std::fs::read_to_string("test/index.ejs").expect("hge");
    let mut cs = CharIterator::new(&mut src);
    while !cs.eof {
        println!(
            "out!(\"{}\");",
            minify(cs.read_until("<%").replace("\"", "\\\""))
        );
        match cs.read_char() {
            None => break,
            Some(c) => match c {
                '=' | '-' => println!("out!(format!(\"{{}}\",{}));", minify(cs.read_until("%>"))),
                '#' => println!("/* {} */", minify(cs.read_until("%>"))),
                _ => println!("{}", minify(cs.read_until("%>"))),
            },
        }
    }
    println!(
        "out!(\"{}\");",
        minify(cs.read_until("%>").replace("\"", "\\\""))
    );

    Ok(())
}
