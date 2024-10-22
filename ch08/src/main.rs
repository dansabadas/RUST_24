use std::collections::HashMap;

fn main() {
    let new_vec = (1..).take(10).collect::<Vec<i32>>();
    let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let new_vec2 = my_vec.into_iter().skip(3).take(4).collect::<Vec<i32>>();
    println!("{new_vec:?}");
    println!("{new_vec2:?}");

    let vector1 = vec![1, 2, 3];
    let mut vector2 = vec![10, 20, 30];
    for num in vector1.iter() {
        println!("Printing a &i32: {num}");
    }
    for num in vector1 {
        println!("Printing an i32: {num}");
    }
    for num in vector2.iter_mut() {
        *num *= 10;
        println!("num is now {num}");
    }
    println!("{vector2:?}");
    //println!("{vector1:?}");

    let vector1 = vec![1, 2, 3];
    let vector1_a = vector1
        .iter()
        .map(|x| x + 1)
        .collect::<Vec<i32>>();
    let vector1_b = vector1
        .into_iter()
        .map(|x| x * 10)
        .collect::<Vec<i32>>();
    let mut vector2 = vec![10, 20, 30];
    vector2.iter_mut().for_each(|x| *x +=100);
    println!("{:?}", vector1_a);
    println!("{:?}", vector1_b);
    println!("{:?}", vector2);

    let my_vec = vec!['a', 'b', '거', '柳'];
    let mut my_vec_iter = my_vec.iter();
    assert_eq!(my_vec_iter.next(), Some(&'a'));
    assert_eq!(my_vec_iter.next(), Some(&'b'));
    assert_eq!(my_vec_iter.next(), Some(&'거'));
    assert_eq!(my_vec_iter.next(), Some(&'柳'));
    assert_eq!(my_vec_iter.next(), None);
    assert_eq!(my_vec_iter.next(), None);

    let my_library = Library2::new("Calgary");
    println!("{my_library:?}");

    let mut my_library = Library::new("Calgary");
    my_library.add_book("The Doom of the Darksword");
    my_library.add_book("Demian - die Geschichte einer Jugend");
    my_library.add_book("구운몽");
    my_library.add_book("吾輩は猫である");
    for item in my_library.get_books() {
        println!("{item}");
    }
    let five_ones: Vec<i32> = GivesOne.into_iter().take(5).collect();
    println!("{five_ones:?}");

    let my_closure = |x: i32| println!("{x}");
    my_closure(5);
    my_closure(5+5);

    let number_one = 6;
    let number_two = 10;
    let my_closure = || println!("{}", number_one + number_two);
    my_closure();

    let number_one = 6;
    let number_two = 10;
    let my_closure = |x: i32| println!("{}", number_one + number_two + x);
    my_closure(5);

    let my_vec = vec![8, 9, 10];
    let fourth = my_vec.get(3).unwrap_or_else(|| {
        if let Some(val) = my_vec.get(2) {
            //println!("why here? {}", val);
            val
        } else {
            &0
        }
    });
    println!("{fourth}");

    let char_vec = vec!['z', 'y', 'x'];
    char_vec
        .iter()
        .enumerate()
        .for_each(|(index, c)| println!("Index {index} is: {c}"));

        let num_vec = vec![2, 4, 6];
        let double_vec: Vec<i32> = num_vec
            .iter()
            .map(|num| num * 2)
            .collect();
    println!("{:?}", double_vec);

    let some_keys = vec![0, 1, 2, 3, 4, 5];
    let some_values = vec!["zero", "one", "two", "three", "four", "five"];
    let number_word_hashmap = some_keys
        .into_iter()
        .zip(some_values.into_iter())
        .collect::<HashMap<_, _>>();

    println!(
        "The value at key 2 is: {}",
        number_word_hashmap.get(&2).unwrap()
    );

    let some_numbers = vec![0, 1, 2, 3, 4, 5];
    let some_words = vec!["zero", "one", "two", "three", "four", "five"];
    let number_word_hashmap: HashMap<_, _> = some_numbers
        .into_iter()
        .zip(some_words.into_iter())
        .collect();

    let keys = vec![0, 1, 2, 3, 4, 5].into_iter();
    let values = vec!["zero", "one", "two", "three", "four", "five"].into_iter();
    let number_word_hashmap: HashMap<i32, &str> = keys.zip(values).collect();
    println!(
        "The value at key 2 is: {}",
        number_word_hashmap.get(&2).unwrap()
    );

    let my_vec = vec![8, 9, 10];
        my_vec
        .iter()
        .for_each(|_| println!("We didn't use the variables at all"));
}

struct GivesOne;
impl Iterator for GivesOne {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        Some(1)
    }
}

#[derive(Debug)]
struct Library {
    name: String,
    books: BookCollection,
}

#[derive(Debug, Clone)]
struct BookCollection(Vec<String>);

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.0.push(book.to_string());
    }
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            books: BookCollection(Vec::new()),
        }
    }
    fn get_books(&self) -> BookCollection {
        self.books.clone()
    }
}

impl Iterator for BookCollection {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        match self.0.pop() {
            Some(book) => {
                println!("Accessing book: {book}");
                Some(book)
            }
            None => {
                println!("Out of books at the library!");
                None
            }
        }
    }
}

#[derive(Debug)]
struct Library2 {
    name: String,
    books: Vec<String>,
}

impl Library2 {
    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string());
    }
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            books: Vec::new(),
        }
    }
}