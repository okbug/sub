fn main() {
    let mut arr = vec![7, 2, 8, 5, 10, 22, 87, 54, 32, 12, 53, 3, 9];
    println!("before sort");
    println!("{:?}", arr);
    sort(&mut arr);
    println!("sorted");
    println!("{:?}", arr);
    let mut arr_f = vec![1.1, 2.3, 5.5, 5.2, 5.1];
    println!("before sort f64 array");
    println!("{:?}", arr_f);
    sort(&mut arr_f);
    println!("after sort f64 array");
    println!("{:?}", arr_f);
}

fn sort<T: std::cmp::PartialOrd>(arr: &mut Vec<T>) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j + 1] < arr[j] {
                arr.swap(j, j + 1);
            }
        }
    }
}

