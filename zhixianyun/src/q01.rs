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
    machines
}

// 根据上一解跑出的规律, 结果以4为周期循环
// 可能直接按月份取结果
fn q01a(_months:Vec<i32>, month:usize) -> Vec<i32> {
    let v = vec![
        [6,7,5,8],
        [7,8,6,5],
        [8,5,7,6],
        [5,6,8,7],
    ];
    v[month%4].into()
}

#[test]
fn test_q01() {
    let _v = q01(vec![10, 7, 5, 4], 5 * 12);
    let mut max = _v[0];
    let mut max_index = 0;
    let max = _v.iter().enumerate().skip(1).fold(max, |acc,(idx,val)| {
        if *val > acc {
            max_index = idx;
            return *val;
        }
        acc
    });
    println!("最大值是{}, 流水线{}, 总结果: {:?}", max, ['a','b','c','d'][max_index],_v );
}
