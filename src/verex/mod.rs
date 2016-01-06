extern crate regex;

use self::regex::Regex;
use self::regex::Error;

#[cfg(test)] pub mod test;

pub struct VerEx {
    string: String
}

impl VerEx {
    // constructors
    pub fn new() -> VerEx {
        VerEx {
            string: String::new(),
        }
    }

    pub fn from_string(string: String) -> VerEx {
        VerEx {
            string: string,
        }
    }

    pub fn from_str(string: &str) -> VerEx {
        VerEx::from_string(string.to_string())
    }

    // --------------------------------------------------
    // fundamental methods
    pub fn add(&mut self, value: &str) -> &mut VerEx{
        self.string.push_str(value);
        self
    }

    pub fn compile(&mut self) -> Result<Regex, Error> {
        Regex::new(self.string.as_ref())
    }

    pub fn raw(& self) -> &str {
        self.source()
    }

    pub fn regex(&mut self) -> Result<Regex, Error> {
        self.compile()
    }

    pub fn source(& self) -> &str {
        self.string.as_ref()
    }

    pub fn value(& self) -> &str {
        self.source()
    }

    // classes
    pub fn open_class(&mut self) -> &mut VerEx {
        self.add(r"[")
    }

    pub fn close_class(&mut self) -> &mut VerEx {
        self.add(r"]")
    }

    // groups
    pub fn open_group(&mut self) -> &mut VerEx {
        self.add(r"(?:")
    }

    pub fn open_capturing_group(&mut self) -> &mut VerEx {
        self.add(r"(")
    }

    pub fn close_group(&mut self) -> &mut VerEx {
        self.add(r")")
    }

    // --------------------------------------------------

    /// Any of the given characters
    pub fn any(&mut self, chars: &str) -> &mut VerEx {
        self.open_class()
            .add(chars)
            .close_class()
    }

    pub fn any_of(&mut self, chars: &str) -> &mut VerEx {
        self.any(chars)
    }

    /// Any character zero or more times
    pub fn anything(&mut self) -> &mut VerEx {
        self.add(r"(.*)")
    }

    /// Any character zero or more times except the provided characters
    pub fn anything_but(&mut self, value: &str) -> &mut VerEx {
        self.open_group()
            .open_class()
            .add(r"^")
            .add(value)
            .close_class()
            .add(r"*")
            .close_group()
    }

    pub fn br(&mut self) -> &mut VerEx {
        self.line_break()
    }

    /// Find a specific string and capture it
    pub fn capture(&mut self, value: &str) -> &mut VerEx {
        self.open_capturing_group()
            .add(value)
            .close_group()
    }

    pub fn end_of_line(&mut self) -> &mut VerEx {
        self.add(r"$")
    }

    /// Find a specific string
    pub fn find(&mut self, value: &str) -> &mut VerEx {
        self.open_group()
            .add(value)
            .close_group()
    }

    /// A line break!
    pub fn line_break(&mut self) -> &mut VerEx {
        self.open_group()
            .add(r"\n")
            .or(r"\r\n")
            .close_group()
    }

    /// Any string either one or zero times
    pub fn maybe(&mut self, value: &str) -> &mut VerEx {
        self.open_group()
            .add(value)
            .close_group()
            .add(r"?")
    }

    pub fn or(&mut self, value: &str) -> &mut VerEx {
        self.add(r"|");
        if value.is_empty() {
            return self;
        } else {
            return self.then(value);
        }
    }

    /// A range of characters e.g. [A-Z]
    /// Usage example: verex.range(vec![('a', 'z'),('A', 'Z')])
    pub fn range(&mut self, range: Vec<(char, char)>) -> &mut VerEx {
        let mut string = r"[".to_string();
        for tuple in range {
            let from = tuple.0;
            let to = tuple.1;
            string.push(from);
            string.push('-');
            string.push(to);
        }
        string.push(']');
        self.add(string.as_ref())
    }

    /// Replace a substring
    pub fn replace(&mut self, from: &str, to: &str) -> &mut VerEx {
        self.string = self.string.replace(from, to);
        self
    }

    /// Any character at least one time
    pub fn something(&mut self) -> &mut VerEx {
        self.add(r"(.+)")
    }

    /// Any character at least one time except for these characters
    pub fn something_but(&mut self, value: &str) -> &mut VerEx {
        self.open_group()
            .open_class()
            .add(r"^")
            .add(value)
            .close_class()
            .add(r"+")
            .close_group()
    }

    pub fn start_of_line(&mut self) -> &mut VerEx {
        self.add(r"^")
    }

    // A tab
    pub fn tab(&mut self) -> &mut VerEx {
        self.add(r"\t")
    }

    /// To use find "in the sentence" and make the chaining flow better
    pub fn then(&mut self, value: &str) -> &mut VerEx {
        self.find(value)
    }

    /// Any alphanumeric characters
    pub fn word(&mut self) -> &mut VerEx {
        self.find(r"\w+")
    }
}
