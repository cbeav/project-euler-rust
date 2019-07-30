use std::cmp;
use std::env;
use primes;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Problem number required as argument.");
    }

    match args[1].as_ref() {
        "1" => problem1(),
        "2" => problem2(),
        "3" => problem3(),
        "4" => problem4(),
        "5" => problem5(),
        "6" => problem6(),
        "7" => problem7(),
        p@_ => println!("No solution implemented for problem #{}.", p)
    }
}

fn problem1() {
    println!("{}", (1 .. 1000).filter(|n| n % 3 == 0 || n % 5 == 0).fold(0, |a, b| a + b))
}

fn problem2() {
    let mut curr = 1;
    let mut next = 1;
    let mut sum  = 0;

    while curr < 4000000 {
        let together = curr + next;

        if curr % 2 == 0 {
            sum += curr
        }

        curr = next;
        next = together;
    }

    println!("{}", sum)
}

fn problem3() {
    println!("{}", primes::factors(600851475143).iter().max().unwrap_or(&0))
}

fn problem4() {
    let mut max = 0;

    for x in 100..1000 {
        for y in 100..1000 {
            let p = x * y;

            if p.to_string() == p.to_string().chars().rev().collect::<String>() {
                max = cmp::max(max, p)
            }
        }
    }

    println!("{}", max);
}

fn problem5() {
    println!("{}", 2 * 2 * 5 * 19 * 3 * 3 * 17 * 2 * 2 * 7 * 13 * 11);
}

fn problem6() {
    let sum_of_squares = (1i32..101).map(|n| n * n).fold(0, |a, b| a + b);
    let square_of_sums = (1i32..101).fold(0, |a, b| a + b).pow(2);

    println!("{}", square_of_sums - sum_of_squares)
}

fn problem7() {
    let mut pset = primes::PrimeSet::new();
    for n in pset.iter().skip(10_000).take(1) {
        println!("{}", n);
    }
}
