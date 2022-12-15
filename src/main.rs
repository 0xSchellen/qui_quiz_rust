use ethabi::token::Token;
use ethers::utils::keccak256;
use std::process::exit;
use std::time::SystemTime;

fn main() {
    generate_and_check_hash();
}

pub fn generate_and_check_hash() {
    // let pangram: &'static str = "abcdefghijklmnopqrstuvWxyzR";
    let pangram: &'static str = "abcdefghijklmnopqrstuvwxyzrR ";
    println!("Pangram: {}", pangram);
    let chars: Vec<char> = pangram.chars().collect();

    let chars_size = 28;

    let mut now = SystemTime::now();
    let mut items_count = 1;

    // for i0 in 0..=chars_size {
    for i0 in 0..=chars_size {
        // Create an empty and growable `String`
        let mut data_vec = String::new();

        data_vec.push(chars[i0]);

        for i1 in 0..=chars_size {
            data_vec.push(chars[i1]);

            for i2 in 0..=chars_size {
                data_vec.push(chars[i2]);

                for i3 in 0..=chars_size {
                    data_vec.push(chars[i3]);

                    for i4 in 0..=chars_size {
                        data_vec.push(chars[i4]);

                        for i5 in 0..=chars_size {
                            data_vec.push(chars[i5]);

                            for i6 in 0..=chars_size {
                                data_vec.push(chars[i6]);

                                for i7 in 0..=chars_size {
                                    data_vec.push(chars[i7]);

                                    for i8 in 0..=chars_size {
                                        data_vec.push(chars[i8]);

                                        // Evaluate Results
                                        // let data = &data_vec; //generate_str();
                                        // let result = calculate_hash(&data);

                                        let result = calculate_hash(&data_vec);                                       

                                        items_count += 1;

                                        if items_count % 100_000_000 == 0 {
                                            let total_time = now.elapsed().unwrap().as_millis()/1000;
                                            println!("Data : {:?} - Time {} s", &data_vec, total_time);
                                            // Reset timer
                                            now = SystemTime::now();
                                        }

                                        // "a barbeR "
                                        if result == "f152950bed091c9854229d3eecb07fae4c84127704751a692c8409543dc02bd3" {
                                            println!("Achou! : Data : {:?} - Result : {:?}", &data_vec, result);
                                            exit(0);
                                        }
                                        
                                        // letteR W 
                                        if result == "f152950bed091c9854229d3eecb07fae4c84127704751a692c8409543dc02bd3" {
                                            println!("Achou! : Data : {:?} - Result : {:?}", &data_vec, result);
                                            exit(0);
                                        }
                                        data_vec.pop();
                                    }
                                    data_vec.pop();
                                }
                                data_vec.pop();
                            }
                            data_vec.pop();
                        }
                        data_vec.pop();
                    }
                    data_vec.pop();
                }
                data_vec.pop();
            }
            data_vec.pop();
        }
    }
}


pub fn calculate_hash(data: &str) -> String {
    let token = Token::String(String::from(data));
    let resp_hash = keccak256(ethers::abi::encode(&[token]));
    let resp_hex = hex::encode(&resp_hash);
    resp_hex
}
