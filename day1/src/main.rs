use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let fvec = read_file("day1-input.txt");
    println!("part1 = {}", part1(&fvec));
    println!("part2 = {}", part2(&fvec));
}

fn part1(fvec: &Vec<i32>) -> i32 {
    calc_freq(0, &fvec)
}

fn part2(fvec: &Vec<i32>) -> i32 {
    let mut res: i32 = 0;
    let mut done = false;
    let mut stash = HashSet::new();

    while !done {
        let (_done, _res) = calc_freq2(res, &fvec, &mut stash);
        done = _done;
        res = _res;
    }
    res
}

fn read_file(filename: &str) -> Vec<i32> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(why) => panic!("unable to open {}. {}", filename, why)
    };
    // let file = File::open(filename).unwrap();

    let mut numvec = Vec::new();
    let br = BufReader::new(file);
    for line in br.lines() {
        let sl = line.unwrap();
        let value = sl.parse::<i32>().unwrap();
        numvec.push(value);
    }
    numvec
}

fn calc_freq(init_freq: i32, fvec: &Vec<i32>) -> i32 {
    let mut res_freq = init_freq;
    for value in fvec {
        res_freq += value;
    }
    res_freq
}

fn calc_freq2(init_freq: i32, fvec: &Vec<i32>, stash: &mut HashSet<i32>) -> (bool, i32) {
    let mut res_freq = init_freq;
    for value in fvec {
        res_freq += value;
        if stash.contains(&res_freq) {
            return (true, res_freq);
        } else {
            stash.insert(res_freq);
        }
    }
    (false, res_freq)
}
