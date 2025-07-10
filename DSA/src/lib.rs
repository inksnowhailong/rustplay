#[test]
fn test1() {
    let arr = [1, 2, 8, 10, 10, 12, 19];
    let x = 5;

    fn find_Ceiling(arr: &[i32], x: i32) -> i32 {
        let mut target_index: i32 = -1;
        let mut target_value = i32::MAX;
        for (index, item) in arr.iter().enumerate() {
            if *item >= x && *item < target_value {
                target_index = index as i32;
                target_value = *item;
            }
        }
        target_index
    }
    assert_eq!(find_Ceiling(&arr, x), 2);
    assert_eq!(find_Ceiling(&[1, 2, 8, 10, 10, 12, 19], 20), -1);
    assert_eq!(find_Ceiling(&[1, 1, 2, 8, 10, 10, 12, 19], 0), 0);
}

#[test]
fn test2() {
    /*
    给定一个仅包含非负整数的数组 arr[]，你的任务是找到一个连续的子数组（元素的连续序列），
    其和等于指定的目标值。你需要返回此子数组最左边和最右边元素的从 1 开始的索引。
    你需要找到第一个和等于目标值的子数组。
    注意：如果不存在这样的数组，则返回 [-1]。
    */
    fn find_subarray(arr: &[i32], target: i32) -> Vec<i32> {
       let mut left = 0;
       let mut sum = 0;
       for (index,item) in arr.iter().enumerate(){
        sum += item;
        while sum > target{
            sum-=arr[left];
            left+=1;
        }
        if sum == target {
            return vec![(left + 1) as i32, (index + 1) as i32];
        }
       }
       vec![-1]
    }
    assert_eq!(find_subarray(&[1, 2, 3, 7, 5],12), [2, 4]);
    assert_eq!(find_subarray(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10],15), [1, 5]);
    assert_eq!(find_subarray(& [5, 3, 4],2), [-1]);
}
