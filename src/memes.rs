pub struct XorShift32 {
    state: u64,
}

impl XorShift32 {
    pub fn new(seed: u64) -> Self {
        XorShift32 { state: seed }
    }

    pub fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    pub fn gen_range(&mut self, n: usize) -> usize {
        self.next() as usize % n // returns 1..=N
    }
}

pub const MEMES: [(&'static str, &'static str); 2] = [
    ("me when the stonks go brr", "bottom text."),
    ("helo", "henlo"),
];
