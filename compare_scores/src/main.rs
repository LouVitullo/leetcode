fn main() {
    let a : [i32; 3] = [ 1, 2, 3 ];
    let b : [i32; 3] = [ 3, 2 ,1 ];

    let result = compare(&a, &b);

    for i in 0..result.len() {
        println!("result: {}", result[i]);
    }
}

fn compare(a: &[i32], b: &[i32]) -> Vec<i32> {
    let (mut res_a, mut res_b) : (i32, i32) = (0, 0);
    for i in 0..a.len() {
        if a[i] > b[i] {
            res_a += 1
        } else if a[i] < b[i] {
            res_b += 1
        }
    }

    [res_a, res_b].to_vec()
}
