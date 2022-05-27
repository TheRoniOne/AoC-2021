fn main() {
    let result = day1_p1();
    println!("{result} are larger than the previous");
}

fn day1_p1() -> i32 {
    let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    let mut prev = measurements[0];

    let mut count = 0;

    for measure in measurements {
        if is_deeper(&prev, &measure) {
            count += 1;
        }
        prev = measure
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
