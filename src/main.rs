use std::io;
use std::io::Read;
use std::io::Write;

///
///  A program that does nothing more than convert a TSV file into an HTML table.
///  The first line of the TSV file is treated as the header.
///
fn main() {

    let out : HTMLWriter = HTMLWriter {};
    out.element("head", "");
    out.tag("body");
    out.element("style", "table {  border-collapse: collapse; } td,th { border: 1px solid #999; }");
    out.tag("table");

    let mut prefix = "<tr><th>";
    let mut infix  = "</th><th>";
    let mut suffix = "</th></tr>";

    let mut is_start_of_line = true;
    let mut character = [0];
    loop {
        match io::stdin().read(&mut character) {
            Ok(size) => {
                if size == 0 {
                    out.write(suffix);
                    break;
                }
                if is_start_of_line {
                    out.write(prefix);
                    is_start_of_line = false;
                }
                let ch = character [0] as char;
		match ch {
                    '\t' => out.write(infix),
                    '\n' => {
                       	out.write(suffix);
                    	prefix = "<tr><td>";
                    	infix  = "</td><td>";
                    	suffix = "</td></tr>";
                    	is_start_of_line = true;
		    },
                    _ => out.character (ch)
                }
            },
            Err(message) => { eprintln!("Error {}", message); break; }
        }
    }
    out.end_tag("table");
    out.end_tag("body");
}

///
/// This class provides some helper methods for writing HTML.
///
struct HTMLWriter {}

impl HTMLWriter {

    fn character (&self, ch : char) {
        self.write(ch.to_string().trim());  //FIXME
    }
    fn tag(&self, tag: &str) {
        self.character ('<');
        self.write  (tag);
        self.character ('>');
    }
    fn end_tag(&self, tag: &str) {
        self.write ("</");
        self.write  (tag);
        self.character ('>');
    }
    fn element(&self, tag : &str, text : &str) {
        self.tag     (tag);
        self.write   (text);
        self.end_tag (tag);
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
