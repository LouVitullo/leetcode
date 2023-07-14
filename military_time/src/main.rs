fn main() {
    let s : &str = "12:05:45PM";
    let military_time = convert_to_military(s);
    println!("{military_time}");
}

fn convert_to_military(s: &str) -> String {
    let mut military_time : String = s[..8].to_string();
    let hour_int : i32 = get_hour_as_int(&military_time);
    
    if let Some(ninth_char) = s.chars().nth(8){
        if ninth_char == 'A' {
            if hour_int == 12 {
                let hour_string : String = "00".to_string();
                military_time.replace_range(..hour_string.len(), &hour_string);
                return military_time;
            } else {
                return s[..8].to_string();
            }
        }
    }

    let hour_string : String;
    
    if hour_int == 12 {
        hour_string = hour_int.to_string();
    } else {
        hour_string = (hour_int + 12).to_string();
    }

    military_time.replace_range(..hour_string.len(), &hour_string);

    military_time.to_string()
}

fn get_hour_as_int(time: &String) -> i32{
    let mut hour : i32 = 0;
    if let Some(hours) = time.get(0..2) {
        if let Ok(number) = hours.parse::<i32>() {
            hour = number;
        }
    }
    hour
}

