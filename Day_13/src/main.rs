fn main() {
    let dog = Dog {
        name: "Rudolf".to_string(),
    };

    dog.speak();

    let cow = Cow {
        name: "Guoy".to_string(),
    };

    cow.speak();

    let original_string = String::from("This is original");
    let copy_string = original_string.display();
    println!("{}", copy_string);

    animal_speaks(&cow);
    animal_speaks(&dog);

    let cat = Cat;
    cat.make_sound();
    cat.walk();
    cat.sleep();
}

trait Speak {
    fn speak(&self);
}

struct Dog {
    name: String,
}

struct Cow {
    name: String,
}

impl Speak for Dog {
    fn speak(&self) {
        println!("{} says: Woof!", self.name);
    }
}

impl Speak for Cow {
    fn speak(&self) {
        println!("{} says: Moo!", self.name);
    }
}

trait Display {
    fn display(&self) -> String;
}

impl Display for String {
    fn display(&self) -> String {
        self.clone()
    }
}

fn animal_speaks<T: Speak>(animal: &T) {
    animal.speak();
}

trait Animal {
    fn make_sound(&self);
    fn sleep(&self) {
        println!("Animals sleep.....");
    }
}

trait Mammal: Animal {
    // This is called as inheritance, wherein we're inheriting the Animal Trait.
    fn walk(&self);
}

trait Bird: Animal {
    fn fly(&self);
}

struct Cat;

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meeooowwww");
    }
}

impl Mammal for Cat {
    fn walk(&self) {
        println!("The cat is walking");
    }
}
