use core::num;

pub fn q02a(nums: &[i32]) -> Vec<i32>{
    let mut res = vec![-0;nums.len()];
    q02a_helper(&nums[..], &mut res,0);
    res
}

fn q02a_helper(nums:&[i32],res:&mut Vec<i32>,num_idx:usize){
    if nums.len() <= 1 {
        res[num_idx] = -1;
        return;
    }
    let next = num_idx+1;
    q02a_helper(&nums[1..], res, next);
    res[num_idx] = -1;
    if nums[1] > nums[0] { // 如果后面第一值大于当前值，直接取后面的值
        res[num_idx] = nums[1];
    }else{
        // 从后面的结果集中取大于当前值的第一个值
        for i in &res[(next)..] {
           if *i > nums[0] {
               res[num_idx] = *i;
               break;
           } 
        }
    }

}


// O(n^2), 最坏情况下，数组依次递减 
fn q02(nums: &[i32]) -> Vec<i32>{
    let mut res = vec![-1;nums.len()];
    // 从倒数第二个反向遍历
    for num_idx in (0..(nums.len()-1)).rev() {
        let current = nums[num_idx]; // 数组当前值
        let mut m = -1;
        if current < nums[num_idx+1] {
            m = nums[num_idx+1];
        }else{
            for n in &res[(num_idx+1)..] {
                if *n > current {
                    m = *n;
                }
            }
        }
        res[num_idx]=m;
    }
    res
}


#[test]
fn test_q02() {
    let v = vec![15,2,8,13];
    dbg!(q02(&v)); // [-1,8,13,-1];
}

#[test]
fn test_q02a() {
    let v = vec![15,2,8,13];
    dbg!(q02a(&v)); // [-1,8,13,-1];
}
