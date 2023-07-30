use std::env;
use std::fs;


fn main() {
    // relative to current execution folder?
    let file_path = "inputs/day1.txt";
    println!("In file {}", file_path);

    let calories_list = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut calories_by_elf: Vec<i32> = Vec::new();
    let mut calories_for_elf: usize = 0;
    for line in calories_list.split('\n') {
        if calories_by_elf.len() == calories_for_elf {
            calories_by_elf.push(0);
        }

        if line.is_empty() {
            calories_for_elf += 1;
        } else {
            calories_by_elf[calories_for_elf] += line.parse::<i32>().unwrap();
        };
    }

    println!("Calories by elf:");
    for (elf, total_calories_carried) in calories_by_elf.iter().enumerate() {
        println!("\tElf {} carries {} calories.", elf + 1, total_calories_carried);
    }

    let mut elf_carrying_most_calories: usize = 0;
    let mut most_calories: &i32 = &0;
    for (elf, total_calories_carried) in calories_by_elf.iter().enumerate() {
        if *most_calories < *total_calories_carried {
            elf_carrying_most_calories = elf;
            most_calories = total_calories_carried;
        }
    }
    println!("Elf {} is carrying the most calories: {}", elf_carrying_most_calories, *most_calories);

    println!("Combined sum of three elves carrying the most calories: {}", top_three(&calories_by_elf).iter().sum::<i32>());
    // println!("With text:\n{contents}");
}

fn top_three(calories_by_elf: &Vec<i32>) -> Vec<i32> {
    let mut result= Vec::with_capacity(3);

    if calories_by_elf.len() <= 3 {
        result = calories_by_elf.to_vec();
        result.sort_unstable_by(|a, b| b.cmp(a));
    } else {
        result = vec![0; 3];
        for &calories in calories_by_elf.iter() {
            if calories > result[0] {
                result[2] = result[1];
                result[1] = result[0];
                result[0] = calories;
            } else if calories > result[1] {
                result[2] = result[1];
                result[1] = calories;
            } else if calories > result[2] {
                result[2] = calories;
            }
        }
    }
    result
}