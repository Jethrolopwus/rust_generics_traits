
struct Person <PetType, PetType2: Animal + Dangerous> where PetType:  Animal + NotDangerous{
    first_name: String,
    pet: PetType,
    pet2: PetType2,

}

trait Animal {
    fn make_sound(&self) ->();
     fn pet_name(&self) -> &str; 
}

trait NotDangerous {

}

trait Dangerous {
    
}
struct Dog {}

impl NotDangerous for Dog {

}
impl Animal for Dog {
    fn make_sound(&self) ->() {
        println!("The Dog Has Barked!")
    }
    fn pet_name(&self) -> &str {  
        "Buddy the Dog"
    }
  
}



struct Cat {}
impl NotDangerous for Cat {

}

impl Animal for Cat {
     fn make_sound(&self) ->() {
        println!("The Cat Meowed!!")
    }
   fn pet_name(&self) -> &str {  
        "Whiskers the Cat"
    }
}
struct Bear {}
impl Dangerous for Bear {

}
impl Animal for Bear {
     fn make_sound(&self) ->() {
        println!("The Bear Roared!")
    }
     fn pet_name(&self) -> &str {  
        "Grizzly the Bear"
    }
   
}

struct Tiger {}

impl Dangerous for Tiger {

}
impl Animal for Tiger {
     fn make_sound(&self) ->() {
        println!("The Tiger Roared Too!")
    }
      fn pet_name(&self) -> &str {  
        "Stripes the Tiger"
    }
}

impl<PetType: Animal + NotDangerous,PetType2: Animal + Dangerous> Person<PetType, PetType2> {
    pub fn introduce(&self) {
        println!("Hi! I'm {} and I have a pet named {}.", 
                 self.first_name, 
                 self.pet.pet_name());   
        print!("My pet says: ");
        self.pet.make_sound();
        self.pet2.make_sound();

    }
}

pub fn create_person() {
    let pet1 = Dog{};
    let _pet2 = Cat{};
    let _pet3 = Bear{};
    let pet4 = Tiger{};

    let p1 = Person{ first_name: String::from("Jethro"), pet: pet1, pet2: pet4};
     p1.introduce();
}