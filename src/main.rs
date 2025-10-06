fn main() {
use std::io;

let mut input = String::new();
println!("Enter BF code: ");
match io::stdin().read_line(&mut input) {
    Ok( n) => {
        let mut arr: [u8; 30000] = [0; 30000];
        let mut pointer: usize = 0;
        input = input.to_string().replace("\n", "");
        let m = n - 2;
        println!("{m} bytes read");
        println!("{input}");

        let mut reading: usize = 0;
        while reading < m {
            let char = input.chars().nth(reading);
            match char {
                Some('>') => pointer += 1,
                Some('<') => pointer -= 1,
                Some('+') => arr[pointer] = arr[pointer].wrapping_add(1),
                Some('-') => arr[pointer] = arr[pointer].wrapping_sub(1),
                Some('.') => println!("{}", arr[pointer] as char),
                Some(',') => println!("{}", "no"),
                Some('[') => {
                    if arr[pointer] == 0 {
                        let mut leftwin: i32 = 1;
                        while leftwin > 0 {
                            reading += 1;
                            if input.chars().nth(reading) == Some('[') {leftwin += 1}
                            if input.chars().nth(reading) == Some(']') {leftwin -= 1}
                        }
                        reading -= 1
                    }
                },
                Some(']') => {
                    if arr[pointer] != 0 {
                        let mut rightwin: i32 = 1;
                        while rightwin > 0 {
                            reading -= 1;
                            if input.chars().nth(reading) == Some('[') {rightwin -= 1}
                            if input.chars().nth(reading) == Some(']') {rightwin += 1}
                        }
                        reading -= 1
                    }
                },
                _ => {}
            }
            reading += 1
        }
        
    }
    Err(error) => println!("error: {error}"),
}    

}