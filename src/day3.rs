use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn day3_p1() -> i32 {
    let file = File::open("inputs/day3_p1.txt").expect("File not found");
    let mut lines = BufReader::new(file).lines().flatten();

    let bits = lines.next().unwrap();
    let len_bits = bits.chars().count();
    let mut v = vec![(0, 0); len_bits];
    process_bits(bits, &mut v);

    for line in lines {
        process_bits(line, &mut v)
    }

    let (gamma, epsilon) = calculate_gamma_epsilon(&v);

    return gamma * epsilon;
}

fn process_bits(bits: String, v: &mut [(i32, i32)]) {
    for (i, bit) in bits.chars().enumerate() {
        if bit == '0' {
            v[i].0 += 1
        } else {
            v[i].1 += 1
        }
    }
}

fn calculate_gamma_epsilon(v: &[(i32, i32)]) -> (i32, i32) {
    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    for tup in v {
        if tup.0 > tup.1 {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }

    return (
        i32::from_str_radix(&gamma, 2).unwrap(),
        i32::from_str_radix(&epsilon, 2).unwrap(),
    );
}
