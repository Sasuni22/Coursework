macro_rules! square {
    ($x:expr) => {{
        let val = $x;
        val * val
    }};
}

fn main() {
    let mut k = 3;

    let result = square!({
        k += 1;
        k
    });

    println!("Result: {}", result);
    println!("Final value of k: {}", k);
}