// Problem 1: fix the 'describe_animal' function signature and also fix the calls to it in main
// Note: The essential requirement for a trait object is that it must be behind a pointer. 
// In course, we used a box smart pointer (Box<dyn SomeTrait>) but we can use any other type 
// of pointer, including a simple reference (e.g. &dyn SomeTrait). 
// In this problem, we will use the simple reference syntax, i.e., &dyn SomeTrait.  

trait Animal {
    fn name(&self) -> &str;
    fn make_sound(&self);
}

struct Lion {
    name: String,
}

impl Animal for Lion {
    fn name(&self) -> &str {
        &self.name
    }

    fn make_sound(&self) {
        println!("Roar!");
    }
}

struct Penguin {
    name: String,
}

impl Animal for Penguin {
    fn name(&self) -> &str {
        &self.name
    }

    fn make_sound(&self) {
        println!("Honk!");
    }
}

fn describe_animal(animal: &dyn Animal) { // This line needs a fix -- changed &Animal to &dyn Animal
    println!("The {} says:", animal.name());
    animal.make_sound();
}

fn main() {
    let lion = Lion {
        name: "Simba".to_string(),
    };
    let penguin = Penguin {
        name: "Happy Feet".to_string(),
    };

    // The calls to function needs fixes
    describe_animal(&lion);  // Made this a pointer
    describe_animal(&penguin);  // Made this a pointer
}
