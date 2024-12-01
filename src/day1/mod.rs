use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Error;
use std::io::ErrorKind;
use std::vec::Vec;
use std::fs::read_to_string;
use std::fs::File;
use std::path::Path;
use std::iter::zip;

pub fn part1(filepath: &Path) -> i64{
    let contents: String = read_to_string(filepath).expect("File not found.");
    let lines = contents.lines();
    let mut vec1: Vec<i64> = Vec::new();
    let mut vec2: Vec<i64> = Vec::new();
    for l in lines{
        let n1: i64 = l[0..5].trim().parse().unwrap();
        let n2: i64 = l[8..13].trim().parse().unwrap();
        vec1.push(n1);
        vec2.push(n2);
    }
    vec1.sort();
    vec2.sort();
    let combos = zip(vec1, vec2);
    let total_dist = combos.map(|(a, b)| i64::abs(a-b)).sum();
    return total_dist;
}

pub fn part2(filepath: &Path) -> Result<i64, Error> {
    let file: File = File::open(filepath)?;
    let reader: BufReader<File> = BufReader::new(file);
    let (mut vec1, mut vec2): (Vec<i64>, Vec<i64>) = reader.lines().map(
        |line| {
            let nums: Vec<i64> = line?
            .split_whitespace()
            .map(
                |s| s.parse::<i64>()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e)))
            .collect::<Result<Vec<_>, _>>()?;
            Ok::<(i64, i64), Error>((nums[0], nums[1]))
        }
    ).collect::<Result<(Vec<i64>, Vec<i64>), Error>>()?;
    vec1.sort();
    vec2.sort();

    let mut occurrences: HashMap<i64, i64> = HashMap::new();
    for i in vec2{
        *occurrences.entry(i).or_insert(0) += 1;
    }
    let similarities = vec1.iter().map(|i| i*occurrences.get(i).unwrap_or(&0)).sum();
    return Ok(similarities);
}