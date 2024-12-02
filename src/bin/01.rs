use std::collections::HashMap;
use std::fmt::format;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {

        let mut arr1: Vec<i32> = Vec::new();
        let mut arr2: Vec<i32> = Vec::new();

        // prepare two sorted arrays
        for line in reader.lines() {
            let unwrapped_line = line?;
            let row: Vec<_> = unwrapped_line.split_whitespace().collect();

            arr1.push(row.get(0).unwrap().parse()?);
            arr2.push(row.get(1).unwrap().parse()?);
        }
        
        // solve
        Ok(solve(arr1, arr2))
    }

    fn solve(mut arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        let mut answer = 0;
        // sort the arrays
        arr1.sort();
        arr2.sort();

        // work through the sums (take absolute value)
        for i in 0..arr1.len() {
            answer += (arr1[i] - arr2[i]).abs();
        }

        answer
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        // find each arr1 number's occurrence, ideally in another array

        let mut arr1: Vec<i32> = Vec::new();
        let mut arr2: Vec<i32> = Vec::new();
        let mut ans: i32 = 0;

        // prepare three sorted arrays
        for line in reader.lines() {
            let unwrapped_line = line?;
            let row: Vec<_> = unwrapped_line.split_whitespace().collect();

            arr1.push(row.get(0).unwrap().parse()?);
            arr2.push(row.get(1).unwrap().parse()?);
        }

        let arr1_occurences_in_arr2 = check_occurences(&arr1, &arr2);

        for mem in arr1  {
            if let Some(value) = arr1_occurences_in_arr2.get(&mem) {
                ans += mem * value;
            }
        }

        // then do the sum
        Ok(ans)
    }

    fn check_occurences(arr1: &Vec<i32>, arr2: &Vec<i32>) -> HashMap<i32, i32> {

        let mut ocurrences: HashMap<i32, i32> = HashMap::new();
        let mut arr2_counts: HashMap<i32, i32> = HashMap::new();

        // build frequency hash map of arr1
        for i in arr2 {
            *arr2_counts.entry(*i).or_insert(0) += 1;
        }

        // for every number of arr1
        // check in freq map of arr2
        // if arr2 include the number, get the value => that's how many times one arr1 show up
        // if arr2 doesn't include the number, the value is 0

        for j in arr1 {
            match arr2_counts.get(&j) {
                Some(count) => ocurrences.insert(*j, *count),
                None => ocurrences.insert(*j, 0),
            };
        }

        ocurrences
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
