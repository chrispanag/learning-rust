use std::io::stdin;

fn print_vec(seq: &[u128]) {
    let mut s = String::new();
    for (i, el) in seq.iter().enumerate() {
        s.push_str(&el.to_string());

        // Don't add it in the last string
        if i != seq.len() - 1 {
            s.push_str(", ");
        }
    }

    println!("{s}");
}

fn main() {
    let mut s = String::new();

    println!("Input the number of elements:");
    stdin().read_line(&mut s).expect("Test");
    let nth: usize = s.trim().parse().expect("Test");

    let mut seq: Vec<u128> = Vec::new();
    seq.resize(nth, 0);
    seq[1] = 1;

    for i in 0..nth - 2 {
        seq[i + 2] = seq[i] + seq[i + 1]
    }

    print_vec(&seq);
}
