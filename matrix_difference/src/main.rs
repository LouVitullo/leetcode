fn main() {
    let mut matrix: Vec<Vec<i32>> = Vec::with_capacity(3);
    
    matrix.push(vec![1, 2, 3]);
    matrix.push(vec![4, 5, 6]);
    matrix.push(vec![44, 8, 9]);
    
    let diff = diagonal_difference(&matrix);
        
    println!("diff: {diff}");
}

fn diagonal_difference(arr: &Vec<Vec<i32>>) -> i32 {
    let (mut d1, mut d2) : (i32, i32) = (0,0);
    for i in 0..arr.len(){
        d1 += arr[i][i];
        d2 += arr[i][(arr.len()-1) - i];
    }
    (d1 - d2).abs()
}
