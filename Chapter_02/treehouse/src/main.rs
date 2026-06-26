use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string()
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}
fn what_is_your_name() -> String
{
    let mut your_name = String::new();


    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    your_name
        .trim()
        .to_lowercase()

    // shitty short hand for 
    // return your_name;
}

fn main() {

    let mut allow_them_in = false;
    let visitor_list = [
        Visitor::new("Bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("Steve", "Hi Steve.  Your milk is in the fridge."),
        Visitor::new("Fred", "Wow, who invited Fred?")
    ];
    
    println!("HEY! WHATS YOUR NAME?!?");

    let name = what_is_your_name();

    // use iterator, find, and match to do the logic
    let known_visitor = visitor_list.iter().find(|visitor|visitor.name == name);

    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You are not on the visitor list.  Please leave.")
    }


    // alternatively loop over the visitor list using traditional for loop and logic.
    for allowed_visitor in visitor_list
    {
        if allowed_visitor.name == name {
            allow_them_in = true;
        }
    }
    
    if allow_them_in {
        println!("Welcome to the Treehouse, {name}");
    }
    else
    {
        println!("NOT WELCOME, {name} ! ");
    }
}
