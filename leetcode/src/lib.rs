mod l453;
mod l454;
mod l455;
mod l456;
mod l457;
mod l458;
mod l459;
mod l460;
mod l461;
mod l462;
mod l1910;
mod l689;



pub fn num_square(n: i32) -> i32 {
    let n = n as usize;

    let mut results = vec![0; n + 1];
    for i in 1..=n {
        let mut minn = i32::MAX;
        let mut j = 1;
        while j * j <= i {
            minn = minn.min(results[i - j * j]);
            j += 1;
        }
        results[i] = minn + 1;
    }
    results[n]
}

#[test]
pub fn test_rob2() {
    assert_eq!(rob2(vec![1, 2, 3, 1]), 4);
}

pub fn rob2(nums: Vec<i32>) -> i32 {
    let length = nums.len();
    if length == 1 {
        return nums[0];
    } else if length == 2 {
        return i32::max(nums[0], nums[1]);
    }
    i32::max(
        rob2_range(&nums, 0, length - 2),
        rob2_range(&nums, 1, length - 1),
    )
}

pub fn rob2_range(
    nums: &Vec<i32>,
    start: usize,
    end: usize,
) -> i32 {
    let mut first = nums[start];
    let mut second = i32::max(nums[start + 1], first);

    let mut temp;
    for i in (start + 2)..=end {
        temp = second;
        second = i32::max(first + nums[i], second);
        first = temp;
    }
    println!("start: {}, result: {}", start, second);
    second
}

pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
    0
}

pub fn single_nmber(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    for &i in &nums {
        result ^= i
    }
    result
}

// https://leetcode-cn.com/problems/majority-element
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    nums[nums.len() / 2]
}
pub fn majority_element2(nums: Vec<i32>) -> i32 {
    let mut card_num = nums[0];
    let mut count = 1;
    for &i in &nums[1..] {
        if i == card_num {
            count += 1;
        } else {
            count -= 1;
            if count == 0 {
                card_num = i;
                count = 1
            }
        }
    }
    card_num
}
