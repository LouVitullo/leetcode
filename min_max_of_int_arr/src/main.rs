fn main() {
    let arr: [i32; 5] = [ 1, 3, 5, 7, 9];
    min_max_sum(&arr);
}

fn min_max_sum(arr: &[i32]){
    let (mut min, mut max) : (i64, i64) = (0, 0);

    for i in 0..arr.len(){
        let mut value: i64 = 0;
        
        for j in 0..arr.len(){
            value += arr[j] as i64; 
        }
        
        value-= arr[i] as i64;

        if i==0{
            min = value;
            max = value;
        }

        if value > max {
            max = value;
        }
        if value < min {
            min = value;
        }
    }
    println!("{min} {max}");
}
