use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let result = day1_p2();
    println!("{result} are larger than the previous");
}

fn day1_p1() -> i32 {
    let file = File::open("inputs/day1_p1.txt").expect("File not found");
    let mut measurements = BufReader::new(file).lines();

    let mut prev = measurements
        .next()
        .unwrap()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let mut count = 0;

    for measure in measurements {
        if let Ok(measure) = measure {
            let measure_int = measure.parse::<i32>().unwrap();
            if is_deeper(&prev, &measure_int) {
                count += 1;
            }
            prev = measure_int
        }
    }

    return count;
}

fn day1_p2() -> i32 {
    let file = File::open("inputs/day1_p1.txt").expect("File not found");
    let mut measurements = BufReader::new(file).lines();

    let mut first = measurements
        .next()
        .unwrap()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let mut second = measurements
        .next()
        .unwrap()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let mut third = measurements
        .next()
        .unwrap()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let mut prev_sum = first + second + third;

    let mut count = 0;

    for measure in measurements {
        if let Ok(new_value) = measure {
            first = second;
            second = third;
            third = new_value.parse::<i32>().unwrap();
            let new_sum = first + second + third;

            if is_deeper(&prev_sum, &new_sum) {
                count += 1;
            }
            prev_sum = new_sum;
        }
    }

    return count;
}

fn is_deeper(prev: &i32, measure: &i32) -> bool {
    if prev < measure {
        println!("{measure} (increased)");
        return true;
    } else if prev == measure {
        println!("{measure} (equal)");
        return false;
    } else {
        println!("{measure} (decreased)");
        return false;
    }
}
