pub mod atrio;
pub mod giochi;
pub mod utils;
use atrio::atrio;

fn main() {
    let credito_iniziale:i32 = 1000;

    atrio(credito_iniziale);
}