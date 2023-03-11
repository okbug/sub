fn main() {
    let mut arr = vec![7, 2, 8, 5, 10, 22, 87, 54, 32, 12, 53, 3, 9];
    println!("before sort");
    println!("{:?}", arr);
    sort(&mut arr);
    println!("sorted");
    println!("{:?}", arr);
}

fn sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j + 1] < arr[j] {
                arr.swap(j, j + 1);
            }
        }
    }
}

