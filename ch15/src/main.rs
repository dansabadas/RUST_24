use std::ops::{Deref, DerefMut};

fn main() {
    let default_i8: i8 = Default::default();
    let default_str: String = Default::default();
    let default_bool: bool = Default::default();
    println!("'{default_i8}', '{default_str}', '{default_bool}'");

    let character_2 = Character::new("Billy".to_string(), 15, 170, 70, true);

    let character_1 = Character::default();
    println!(
        "The character {:?} is {:?} years old.",
        character_1.name, character_1.age
    );

    println!(
        "The character {:?} is {:?} years old.",
        character_2.name, character_2.age
    );

    let only_height = Size {
        height: 1.0,
        ..Default::default()
    };
    println!("{only_height:?}");

    let character_3 = Character::default().height(180).weight(60).name("Bobby");
    println!("{character_3:?}");

    let character_with_smurf = Character::default()
        .name("Lol I am Smurf!!").build();
    let character_too_tall = Character::default()
        .height(400).build();
    let character_too_heavy = Character::default()
        .weight(500).build();
    let okay_character = Character::default()
        .name("Billybrobby")
        .height(180)
        .weight(100)
        .build();

    let character_vec = vec![
        character_with_smurf,
        character_too_tall,
        character_too_heavy,
        okay_character,
    ];

    for character in character_vec {
        match character {
            Ok(character) => println!("{character:?}\n"),
            Err(err_info) => println!("{err_info}\n"),
        }
    }

    let default_character = Character2::default();
    do_something_with_character(&default_character);
    let second_character = CharacterBuilder::new("Bobby".to_string(), 27)
        .try_build()
        .unwrap();
    do_something_with_character(&second_character);
    let bad_character = CharacterBuilder::new("Smurfysmurf".to_string(), 40)
        .try_build();

    println!("{bad_character:?}");
    // do_something_with_character(&bad_character);

    let value = 7;
    let reference = &7;
    let reference2 = &value;
    println!("{}", value == *reference);
    println!("{}", value == *reference2);

    let boxed_number = Box::new(20);
    println!("This works fine: {}", *boxed_number);
    let my_number = HoldsANumber(20);
    //println!("This fails though: {}", *my_number + 20);
    println!("This fails though: {}", my_number.0 + 20);

    let x = DerefExample { value: 'a' };
    assert_eq!('a', *x);

    let my_number = HoldsANumber(20);
    println!("{:?}", *my_number + 20);

    let my_number = HoldsANumber(20);
    println!("{:?}", my_number.checked_sub(10));
    println!("{:?}", my_number.checked_sub(100));
    my_number.prints_the_number_times_two();
    //*my_number = 30;

    let mut my_number = HoldsANumber(20);
    *my_number = 30;
    println!("{:?}", my_number.checked_sub(100));
    my_number.prints_the_number_times_two();

    let billy = Character3::new("Billy".to_string(), 9, 12, 7, 10);

    let mut billy = Character3::new("Billy".to_string(), 9, 12, 7, 10);
    let mut brandy = Character3::new("Brandy".to_string(), 10, 8, 9, 10);
    *billy -= 10;
    *brandy += 1;
    let mut hit_points_vec = vec![];
    hit_points_vec.push(*billy);
    hit_points_vec.push(*brandy);
}

struct Character3 {
    name: String,
    strength: u8,
    dexterity: u8,
    intelligence: u8,
    hit_points: i8,
}

impl Character3 {
    fn new(
        name: String,
        strength: u8,
        dexterity: u8,
        intelligence: u8,
        hit_points: i8,
    ) -> Self {
        Self {
            name,
            strength,
            dexterity,
            intelligence,
            hit_points,
        }
    }
}

impl Deref for Character3 {
    type Target = i8;
    fn deref(&self) -> &Self::Target {
        &self.hit_points
    }
}

impl DerefMut for Character3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.hit_points
    }
}

struct DerefExample<T> {
    value: T
}

impl<T> Deref for DerefExample<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(Debug)]
struct HoldsANumber(u8);
impl HoldsANumber {
    fn prints_the_number_times_two(&self) {
        println!("{}", self.0 * 2);
    }
}

impl Deref for HoldsANumber {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HoldsANumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct Character2 {
    name: String,
    age: u8,
}

impl Default for Character2 {
    fn default() -> Self {
        Self {
            name: "Billy".to_string(),
            age: 15,
        }
    }
}

#[derive(Debug)]
pub struct CharacterBuilder {
    pub name: String,
    pub age: u8,
}

impl CharacterBuilder {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
    fn try_build(self) -> Result<Character2, &'static str> {
        if !self.name.to_lowercase().contains("smurf") {
            Ok(Character2 {
                name: self.name,
                age: self.age,
            })
        } else {
            Err("Can't make a character with the word 'smurf' inside it!")
        }
    }
}

fn do_something_with_character(character: &Character2) {}

#[derive(Debug,Default)]
struct Size {
    height: f64,
    length: f64,
    width: f64,
}

#[derive(Debug)]
struct Character {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: LifeState,
    can_use: bool,
}

#[derive(Debug)]
enum LifeState {
    Alive,
    Dead,
    NeverAlive,
    Uncertain
}

impl Character {
    fn new(name: String, age: u8, height: u32, weight: u32, alive: bool) -> Self {
        Self {
            name,
            age,
            height,
            weight,
            lifestate: if alive {
                LifeState::Alive
            } else {
                LifeState::Dead
            },
            can_use: true,
        }
    }

    fn height(mut self, height: u32) -> Self {
        self.height = height;
        self.can_use = false;
        self
    }

    fn weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self.can_use = false;
        self
    }

    fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self.can_use = false;
        self
    }

    fn build(mut self) -> Result<Character, String> {
        if self.height < 200
            && self.weight < 300
            && !self.name.to_lowercase().contains("smurf")
        {
            self.can_use = true;
            Ok(self)
        } else {
            Err("Could not create character. Characters must have:
            1) Height below 200
            2) Weight below 300
            3) A name that is not Smurf (that is a bad word)"
            .to_string())
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Billy".to_string(),
            age: 15,
            height: 170,
            weight: 70,
            lifestate: LifeState::Alive,
            can_use: true,
        }
    }
}