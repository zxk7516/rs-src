## 第⼀题
方法一，蛮力法, 根据上个月的数值，计算下个月的，先把所有生产线+1，再把最大的产能 -4
```rust
fn q01(mut machines: Vec<i32>, month: i32) -> Vec<i32> {
    for _m in 0..month {
        // 按月份处理,每过一个月份生成新的产能数据
        let mut max = i32::MIN; // 初始化最大产能,比较用
        let mut max_idx = 0; // 最大产能的index
        for (idx, m) in machines.iter_mut().enumerate() {
            // 将每个流水线的产能+1，同时找到最大的产能
            if *m >= max {
                max = *m;
                max_idx = idx;
            }
            *m += 1;
        }
        machines[max_idx as usize] -= 4; // 将最大产能减4, 因为之前多加了1，减3改变减4

    }
    return machines;
}
```
方法一，观察蛮力法每轮的结果，可能发现每四个月，产能会重置
```rust
fn q01_b -> Vec<i32> {
    let v = vec![
        [6,7,5,8],
        [7,8,6,5],
        [8,5,7,6],
        [5,6,8,7],
    ];
    return v[month%4].into();
}
```


## 第二题
O(n*n)
```rust
fn q02(nums: &[i32]) -> Vec<i32>{
    let mut res = vec![-1;nums.len()];
    // 从倒数第二个反向遍历
    for num_idx in (0..(nums.len()-1)).rev() {
        let current = nums[num_idx]; // 数组当前值
        let mut m = -1;
        if current < nums[num_idx+1] { // 比较后面的第一个值 
            m = nums[num_idx+1];
        }else{
            // 比较后面的结果集
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
```


## 第三题
将字符串切开再旋转相当于，将字段串从中间断开，再把头和尾相连

```rust
fn q03(s: &str) -> usize {
    // 将字符串首尾拼接
    let s2 = s.repeat(2);
    
    let mut s_iter = s2.chars();
    let mut last_char = s_iter.next().unwrap();
    let mut size = 1;
    let mut sum = 0;
    for c in s_iter {
        if c != last_char {
            size += 1;
        } else {
            if size > sum {
                sum = size;
            }
            size = 1;
        }
        last_char = c;
    }
    sum
}
```


## 第四题
```php

```