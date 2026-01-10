use std::hint::black_box;

fn main() {
    for _ in 0..10_000_000 {
        black_box(fen_generator::random_fen());
    }
}

