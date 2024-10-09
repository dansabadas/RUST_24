struct FileDirectory;
struct ColorRgb(u8, u8, u8);

struct SizeAndColor {
    size: u32,
    color: ColorRgb,
}

fn main() {
    let my_color = ColorRgb(50, 0, 50);
    println!("The second part of the color is: {}", my_color.1);
}
//cargo new hello_world
//cargo build
//cargo run
//https://code.visualstudio.com/docs/languages/rust