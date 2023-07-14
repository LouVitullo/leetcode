fn main() {
    let ar : [i64; 5] = [ 100000000000001, 10000000002, 100000000004, 1000000003, 1000000005 ];
    let sum = big_sum(&ar);
    println!("Sum: {}", sum);
}

fn big_sum(ar: &[i64]) -> i64{
    let mut sum : i64 = 0;
    for i in 0..ar.len() {
        sum += ar[i];
    }

    sum
}
