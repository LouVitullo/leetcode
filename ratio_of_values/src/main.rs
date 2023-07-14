fn main() {
    let arr : [i32; 5] = [-1, 1, 0, 0, -1];
    create_sign_ratios(&arr);
}

fn create_sign_ratios(arr: &[i32]){
    let (mut pos, mut neg, mut zero) : (i32, i32, i32) = (0, 0, 0);

    for i in 0..arr.len(){
        if arr[i] > 0{
            pos += 1;
        } else if arr[i] < 0 {
            neg += 1;
        } else {
            zero += 1
        }
    }

    println!("{}", pos as f32/arr.len() as f32);
    println!("{}", neg as f32/arr.len() as f32);
    println!("{}", zero as f32/arr.len() as f32);
}

