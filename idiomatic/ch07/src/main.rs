fn main() {
    println!("Hello, world!");
    let buf = Buffer::from([0, 1, 2, 3]);
    dbg!(&buf);
}

struct Buffer0 {
    buf: [u8; 256],
}

struct Buffer00<T> {
    buf: [T; 256],
}

#[derive(Debug)]
struct Buffer<T, const LENGTH: usize> {
    buf: [T; LENGTH],
}

// impl From<[u8; 256]> for Buffer<u8, 256> {
//     fn from(buf: [u8; 256]) -> Self {
//         Buffer { buf }
//     }
// }

impl<T, const LENGTH: usize> From<[T; LENGTH]> for Buffer<T, LENGTH> {
    fn from(buf: [T; LENGTH]) -> Self {
        Buffer { buf }
    }
}