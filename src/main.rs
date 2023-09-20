fn randchar() -> char {
    let r = rand::random::<u8>() % 36;
    if r < 10 {
        (b'0' + r) as char
    } else {
        (b'a' + r - 10) as char
    }
}

fn codegen() -> String {
    let mut code = String::with_capacity(12);
    for _ in 0..5 {
        code.push(randchar());
    }
    code.push('-');
    for _ in 0..5 {
        code.push(randchar());
    }
    code
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let count: usize;
    if args.len() < 2 {
        count = 1;
    } else if args[1] == "benchmark" {
        count = 10000000;
        let start = std::time::Instant::now();
        for _ in 0..count {
            codegen();
        }
        let end = std::time::Instant::now();
        println!(
            "generated {} codes in {} seconds",
            count,
            (end - start).as_secs_f64()
        );
        return;
    } else if args[1].parse::<f64>().is_ok() {
        count = args[1].parse().unwrap();
    } else {
        println!("Usage: {} [count]", args[0]);
        return;
    }
    for _ in 0..count {
        println!("bsky-social-{}", codegen());
    }
}
