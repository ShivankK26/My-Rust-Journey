fn main() {
    // Creating a Box Pointer
    let mut pointer = Box::new(5);
    *pointer = 10; // Here, we are dereferencing the pointer

    println!("{}", pointer);

    // let my_trait_object: Box<dyn MakeNoise> = Box::new(Bird {
    //     name: "Tweety".to_string(),
    //     color: "yellow".to_string(),
    // });

    // my_trait_object.talk();
    let mut speakers: Vec<Box<dyn MakeNoise>> = Vec::new();
    speakers.push(Box::new(Bird {
        name: "bird1".to_string(),
        color: "yellow".to_string(),
    }));
    speakers.push(Box::new(Dog {
        name: "dog1".to_string(),
        breed: "german shelphard".to_string(),
    }));

    for speaker in speakers {
        speaker.talk();
    }
}

trait MakeNoise {
    fn talk(&self);
}

struct Bird {
    name: String,
    color: String,
}

struct Dog {
    name: String,
    breed: String,
}

impl MakeNoise for Bird {
    fn talk(&self) {
        println!("That's me, the birdy is talking!")
    }
}

impl MakeNoise for Dog {
    fn talk(&self) {
        println!("That's me, the doggy is talking!")
    }
}

fn invite_to_animal_talks(speaker: Box<dyn MakeNoise>) {
    println!("Ladies and gentlement, please welcome our next speaker: ");
    speaker.talk();
}

trait Calculate<T> {
    const PI: f64;

    fn calculate_area(&self, value: T) -> f64 {}
}
