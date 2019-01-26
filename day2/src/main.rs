use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let lvec = read_file("day2-input.txt");
    println!("part1 = {}", part1(&lvec));
    println!("part2 = {}", part2(&lvec));
}

fn part1(lvec: &Vec<Vec<u8>>) -> i32 {
    calc(lvec, 2) * calc(lvec, 3)
}

fn part2(lvec: &Vec<Vec<u8>>) -> String {
    let mut myvec = lvec.clone();
    myvec.sort();
    let end = myvec.len() - 1;
    for i in 0..end {
        let line1 = &myvec[i];
        let line2 = &myvec[i+1];
        match check_one_diff(line1, line2) {
            (true, res) => return String::from_utf8(res.to_vec()).unwrap(),
            (false, _) => 1
        };
    }
    panic!("NOT FOUND");
}

fn read_file(filename: &str) -> Vec<Vec<u8>> {
    let file = File::open(filename).unwrap();
    let mut linevec = Vec::new();
    let br = BufReader::new(file);
    for line in br.lines() {
        let sl = line.unwrap();
        linevec.push(sl.into_bytes());
    }
    linevec
}

fn calc(linevec: &Vec<Vec<u8>>, count: i32) -> i32 {
    let mut res = 0;
    for bytes in linevec {
        if check_exact_count(bytes, count) {
            res += 1;
        }
    }
    res
}

fn check_exact_count(bytes: &Vec<u8>, count: i32) -> bool {
    for byte in bytes {
        let mut cnt = 0;
        for cb in bytes {
            if cb == byte {
                cnt += 1;
            }
        }
        if cnt == count {
            return true;
        }
    }
    false
}

fn check_one_diff(line1: &Vec<u8>, line2: &Vec<u8>) -> (bool, Vec<u8>) {
    let mut cnt = 0;
    let mut res = Vec::new();
    for i in 0..line1.len() {
        if line1[i] != line2[i] {
            cnt += 1;
        } else {
            res.push(line1[i]);
        }
    }
    (cnt == 1, res)
}
