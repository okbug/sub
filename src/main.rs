fn main() {
    let mut arr = vec![1, 2, 3, 5, 2];
    println!("before sort");
    print_arr(&arr);
    println!("sorted");
    sort(&mut arr);
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
    print_arr(&arr);
}

fn print_arr(arr: &Vec<i32>) {
    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }
}

