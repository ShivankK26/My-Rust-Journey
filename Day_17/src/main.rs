static something: &str = "Hello something";
struct Ancestor<'a>(&'a str);

fn main() {
    let name = "Elon Musk";
    let person = Person { name: &name };

    let s: &'static str = "I am immortal!";
    let parent: &'static str = "I am here for forever and ever, until the program stops!";

    let kid: &'static str = {
        let short_lifetime_str = String::from("I am passing by");
        &short_lifetime_str
    };

    let name1 = "Grandpa".to_string();
    let person: Ancestor<'static> = Ancestor(&name);
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        return s1;
    } else {
        return s2;
    }
}

struct Sentence<'a> {
    content: &'a str,
}

impl<'a> Sentence<'a> {
    fn yell(&self) -> &str {
        return "Do not code until 3AM...";
    }
}

fn no_no_function<'a>(x: &'a str, y: &'a str) -> &'a str {
    let some_string = String::from(x);
    &some_string
}

// Practice
struct Person {
    name: &'a str,
}

struct LongLived<'a>(&'a str);

struct ShortLived<'a> {
    name: &'a str,
    long: LongLived<'a>,
}

fn problematic_function() -> &str {
    let some_string = String::from("some");
    &some_string;
}
