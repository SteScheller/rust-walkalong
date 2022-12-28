use std::collections::HashMap;
use std::io;

fn read_integers() -> Vec<i32> {
    let mut line = String::new();
    let mut integers: Vec<i32> = Vec::new();

    'input: loop {
        line.clear();
        println!("Please into a list of space seperated integers:");
        match io::stdin().read_line(&mut line) {
            Err(_) => {
                println!("Failed to read input");
                continue 'input;
            }
            _ => (),
        }

        integers.clear();
        for word in line.split_whitespace() {
            let number: i32 = match word.trim().parse() {
                Ok(num) => num,
                Err(_) => continue 'input,
            };
            integers.push(number);
        }

        break;
    }

    integers
}

fn print_metrics(mut integers: Vec<i32>) {
    {
        integers.sort_unstable();
        let median = integers[integers.len() / 2];
        println!("median: {median}");
    }

    {
        let mut int_counts = HashMap::new();
        let mut max_v = 0;
        for value in integers {
            let count = int_counts.entry(value).or_insert(0);
            *count += 1;
            if *count > max_v {
                max_v = *count;
            }
        }
        println!("mode(s):");
        for (k, v) in int_counts {
            if v == max_v {
                println!("\t{k} (count={v})");
            }
        }
    }
}

pub fn run() {
    let integers = read_integers();
    print_metrics(integers);
}
