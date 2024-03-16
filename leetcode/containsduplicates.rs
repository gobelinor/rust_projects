use std::collections::HashSet;

fn main() {

    let nums = vec![1, 2, 3, 1];
    let result = contains_duplicate(nums.clone());
    println!("{}", result);
    let result = contains_duplicate_version_triée(nums.clone());
    println!("{}", result);
    let result = contains_duplicate_version_triée_avec_iterateur(nums.clone());
    println!("{}", result);
}

fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for num in nums {
            if set.contains(&num) {
                return true;
            }
            set.insert(num);
        }
        false
}

fn contains_duplicate_version_triée(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        nums.sort();
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                return true;
            }
        }
        false
}


fn contains_duplicate_version_triée_avec_iterateur(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if nums[i] == nums[j] && i != j {
                    return true;
                }
            }
        }
        false        
}



