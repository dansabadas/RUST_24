use std::fmt;
use std::fmt::Debug;

struct Dog {
    name: String,
}

struct Parrot {
    name: String,
}

trait DogLike {
    fn bark(&self) {
        println!("Woof woof!");
    }
    fn run(&self) {
        println!("The dog is running!");
    }
}

impl DogLike for Dog {}
//impl DogLike for Parrot {}
impl DogLike for Parrot{
    fn run(&self) {
        println!("{} the parrot is running!", self.name);
    }
}

struct Animal {
    name: String,
}

trait DogLike2 {
    fn bark(&self);
    fn run(&self);
}

impl DogLike2 for Animal {
    fn bark(&self) {
        println!("{}, stop barking!!", self.name);
    }
    fn run(&self) {
        println!("{} is running!", self.name);
    }
}

#[derive(Debug)]
struct Cat {
    name: String,
    age: u8,
}

struct Position {
    longitude: f32,
    latitude: f32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.longitude, self.latitude)
    }
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is a cat who is {} years old", self.name, self.age)
    }
}

fn print_excitedly(input: String) {
    println!("{input}!!!!!");
}

#[derive(Debug)]
struct Monster {
    health: u32,
}

#[derive(Debug)]
struct Wizard {
    health: i32,
}

#[derive(Debug)]
struct Ranger {
    health: i32,
}

trait DisplayHealth {
    fn health(&self) -> i32;
}

trait FightClose: Debug {
    fn attack_with_sword(&self, opponent: &mut Monster) {
        opponent.health -= 10;
        println!(
            "Sword attack! Opponent's health: {}. You are now at: {:?}",
            opponent.health, self
        );
    }
    fn attack_with_hand(&self, opponent: &mut Monster) {
        opponent.health -= 2;
        println!(
            "Hand attack! Opponent's health: {}. You are now at: {:?}",
            opponent.health, self
        );
    }
}
impl FightClose for Wizard {}
impl FightClose for Ranger {}
trait FightFromDistance: Debug {
    fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
        if distance < 10 {
            opponent.health -= 10;
            println!(
                "Bow attack! Opponent's health: {}. You are now at: {:?}",
                opponent.health, self
            );
        }
    }

    fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
        if distance < 3 {
            opponent.health -= 4;
        }
        println!(
            "Rock attack! Opponent's health: {}. You are now at: {:?}",
            opponent.health, self
        );
    }
}

impl FightFromDistance for Ranger {}

trait MonsterBehavior2: Debug {
    fn take_damage(&mut self, damage: i32);
    fn display_self(&self) {
        println!("The monster is now: {self:?}");
    }
}
#[derive(Debug)]
struct Monster2 {
    health: i32,
}

#[derive(Debug)]
struct Wizard2 {
    health: i32,
}

#[derive(Debug)]
struct Ranger2 {
    health: i32,
}

impl MonsterBehavior2 for Monster2 {
    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }
}

trait FightFromDistance2: Debug {
    fn attack_with_bow<T: MonsterBehavior2>(&self, opponent: &mut T, distance: u32) {
        println!("You attack with your bow!");
        if distance < 10 {
            opponent.take_damage(10);
        } else {
            println!("Too far away!");
        }
        opponent.display_self();
    }
    fn attack_with_rock<T: MonsterBehavior2>(&self, opponent: &mut T, distance: u32) {
        println!("You attack with a rock!");
        if distance < 3 {
            opponent.take_damage(4);
        } else {
            println!("Too far away!");
        }
        opponent.display_self();
    }
}

impl FightFromDistance2 for Ranger2 {}

trait FightClose2 {
    fn attack_with_sword<T: MonsterBehavior2>(&self, opponent: &mut T) {
        println!("You attack with your sword!");
        opponent.take_damage(10);
        opponent.display_self();
    }

    fn attack_with_hand<T: MonsterBehavior2>(&self, opponent: &mut T) {
        println!("You attack with your hand!");
        opponent.take_damage(2);
        opponent.display_self();
    }
}
impl FightClose2 for Wizard2 {}
impl FightClose2 for Ranger2 {}

///
struct Monster3 {
    health: i32,
}

#[derive(Debug)]
struct Wizard3 {
    health: i32,
}

#[derive(Debug)]
struct Ranger3 {
    health: i32,
}

trait Magic3 {}
trait FightClose3 {}
trait FightFromDistance3 {}

impl FightClose3 for Ranger3 {}
impl FightClose3 for Wizard3 {}
impl FightFromDistance3 for Ranger3 {}
impl Magic3 for Wizard3 {}

fn attack_with_bow3<T>(pc: &T, opponent: &mut Monster3, distance: u32)
where
    T: FightFromDistance3 + Debug,
{
    if distance < 10 {
        opponent.health -= 10;
        println!(
            "Bow attack! Opponent's health: {}. You are now at: {pc:?}",
            opponent.health
        );
    }
}

fn attack_with_sword3<T>(pc: &T, opponent: &mut Monster3)
where
    T: FightClose3 + Debug,
{
    opponent.health -= 10;
    println!(
        "Sword attack! Opponent's health: {}. You are now at: {pc:?}",
        opponent.health
    );
}

fn fireball3<T>(pc: &T, opponent: &mut Monster3, distance: u32)
where
    T: Magic3 + Debug,
{
    if distance < 15 {
        opponent.health -= 20;
        println!(
            "A massive fireball! Opponent's health: {}. You are now at: {pc:?}",
            opponent.health
        );
    }
}

trait French {}
trait LawyerSkill {}
trait MedicalSkill {}

struct FrenchCitizen;
struct ExchangeStudentInFrance;
struct AmericanLawyer;
struct AmericanDoctor;
struct FrenchLawyer;
struct FrenchDoctor;
struct MrKnowsEverything;

impl French for FrenchCitizen {}
impl French for ExchangeStudentInFrance {}
impl French for FrenchLawyer {}
impl French for FrenchDoctor {}
impl French for MrKnowsEverything {}

impl LawyerSkill for AmericanLawyer {}
impl LawyerSkill for FrenchLawyer {}
impl LawyerSkill for MrKnowsEverything {}

impl MedicalSkill for AmericanDoctor {}
impl MedicalSkill for FrenchDoctor {}
impl MedicalSkill for MrKnowsEverything {}

fn speak_french<T: French>(speaker: T) {}
fn enter_court<T: LawyerSkill>(lawyer: T) {}
fn cure_patient<T: MedicalSkill>(doctor: T) {}
fn enter_french_court<T: LawyerSkill + French>(lawyer: T) {}
fn cure_french_patient<T: MedicalSkill + French>(doctor: T) {}
fn present_medical_case_in_french_court<T: MedicalSkill + French + LawyerSkill>(lawyer: T) {}

fn main() {
    let rover = Dog {
        name: "Rover".to_string(),
    };
    let brian = Parrot {
        name: "Brian".to_string(),
    };
    rover.bark();
    rover.run();
    brian.bark();
    brian.run();

    let rover = Animal {
        name: "Rover".to_string(),
    };
    rover.bark();
    rover.run();

    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };

    println!("Mr. Mantle is a {mr_mantle:?}");
    println!("Mr. Mantle is a {mr_mantle}");

    print_excitedly(mr_mantle.to_string());
    println!(
        "Mr. Mantle's String is {} letters long.",
        mr_mantle.to_string().chars().count()
    );

    let radagast = Wizard { health: 60 };
    let aragorn = Ranger { health: 80 };
    let mut uruk_hai = Monster { health: 40 };
    radagast.attack_with_sword(&mut uruk_hai);
    aragorn.attack_with_bow(&mut uruk_hai, 8);

    let radagast = Wizard2 { health: 60 };
    let aragorn = Ranger2 { health: 80 };
    let mut uruk_hai = Monster2 { health: 40 };
    radagast.attack_with_sword(&mut uruk_hai);
    aragorn.attack_with_bow(&mut uruk_hai, 8);

    let radagast = Wizard3 { health: 60 };
    let aragorn = Ranger3 { health: 80 };
    let mut uruk_hai = Monster3 { health: 40 };
    attack_with_sword3(&radagast, &mut uruk_hai);
    attack_with_bow3(&aragorn, &mut uruk_hai, 8);
    fireball3(&radagast, &mut uruk_hai, 8);

    speak_french(FrenchCitizen);
    speak_french(ExchangeStudentInFrance);
    speak_french(FrenchLawyer);
    speak_french(FrenchDoctor);
    speak_french(MrKnowsEverything);
    enter_court(AmericanLawyer);
    enter_court(FrenchLawyer);
    enter_court(MrKnowsEverything);
    cure_patient(AmericanDoctor);
    cure_patient(FrenchDoctor);
    cure_patient(MrKnowsEverything);
    enter_french_court(FrenchLawyer);
    enter_french_court(MrKnowsEverything);
    cure_french_patient(FrenchDoctor);
    cure_french_patient(MrKnowsEverything);
    present_medical_case_in_french_court(MrKnowsEverything);
    //present_medical_case_in_french_court(FrenchDoctor);
}
