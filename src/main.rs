use std::io;
use std::io::Read;
use std::io::Write;

fn main() {

    let delimiter: char = '\t';

    let out : HTML = HTML {};
    out.tag("head");
    out.tag("/head");
    out.tag("body");
    out.write  ("<table border=\"1\">");

    let mut prefix = "<tr><th>";
    let mut infix = "</th><th>";
    let mut suffix = "</th></tr>";

    let mut write_prefix = true;
    let mut character = [0];
    loop {
        match io::stdin().read(&mut character) {
            Ok(size) => {
                if size == 0 {
                    out.write(suffix);
                    break;
                }
                if write_prefix {
                    out.write(prefix);
                    write_prefix = false;
                }
                let ch = character [0] as char;
                if ch == delimiter {
                    out.write(infix);
                } else if ch == '\n' {
                    out.write(suffix);
                    prefix = "<tr><td>";
                    infix = "</td><td>";
                    suffix = "</td></tr>";
                    write_prefix = true;
                } else {
                    out.character (ch);
                }
            },
            Err(message) => { eprintln!("Error {}", message); break; }
        }
    }
    out.tag("/table");
    out.tag("/body");
}

struct HTML {

}

impl HTML {

    fn character (&self, ch : char) {
        self.write(ch.to_string().trim());  //FIXME
    }
    fn tag(&self, text: &str) {
        self.character ('<');
        self.write  (text);
        self.character ('>');
    }
    fn write (&self, text : &str)
    {
        match io::stdout ().write (text.to_string().as_bytes()) {
            Ok(_) => {},
            Err(message) => eprint!("Error writing {}", message)
        }
        io::stdout().flush().unwrap();
    }
}