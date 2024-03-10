use rand::Rng;

pub fn generate_key_95() -> String {
    let day = rand::thread_rng().gen_range(1..366);
    let years_allowed = ["95", "96", "97", "98", "99", "00", "01", "02", "03"];
    let year = years_allowed[rand::thread_rng().gen_range(0..years_allowed.len())];
    let algodigits = gen_algo_key();
    let randomend = rand::thread_rng().gen_range(10000..99999);
    if day < 10 {
        format!("00{}{}-OEM-0{}-{}", day, year, algodigits, randomend)
    } else if day < 100 {
        format!("0{}{}-OEM-0{}-{}", day, year, algodigits, randomend)
    } else {
        format!("{}{}-OEM-0{}-{}", day, year, algodigits, randomend)
    }
}

pub fn verify_win95(key: &str) -> bool {
    let key: Vec<&str> = key.split("-").collect();
    let day: i32 = key[0].get(0..3).unwrap().parse::<i32>().unwrap();
    let year: i32 = key[0].get(3..5).unwrap().parse::<i32>().unwrap();
    let algo: &str = key[2];

    let years_allowed = [95, 96, 97, 98, 99, 00, 01, 02, 03];

    if !years_allowed.contains(&year) {
        return false;
    }

    if day < 0 || day > 366 {
        return false;
    }

    let mut sum = 0;
    for algo in algo.chars() {
        sum += algo.to_digit(10).unwrap();
    }
    if sum % 7 != 0 {
        return false;
    }

    return true;
}

pub fn verify_office95(key: &str) -> bool {
    let key: Vec<&str> = key.split("-").collect();
    let first_three_digits = key[0];
    let next_seven_digits = key[1];

    if first_three_digits.len() != 3 {
        return false;
    }

    if next_seven_digits.len() != 7 {
        return false;
    }

    let first_three_digits: i32 = first_three_digits.parse().unwrap();
    let next_seven_digits: i32 = next_seven_digits.parse().unwrap();

    if first_three_digits == 333
        || first_three_digits == 444
        || first_three_digits == 555
        || first_three_digits == 666
        || first_three_digits == 777
        || first_three_digits == 888
        || first_three_digits == 999
    {
        return false;
    }

    if sum_of_digits(next_seven_digits) % 7 != 0 {
        return false;
    }

    return true;
}

pub fn generate_key_office95() -> String {
    let mut rng = rand::thread_rng();

    let first_three_digits: u16 = loop {
        let num = rng.gen_range(100..1000);
        if num != 333
            && num != 444
            && num != 555
            && num != 666
            && num != 777
            && num != 888
            && num != 999
        {
            break num;
        }
    };

    let next_seven_digits: i32 = loop {
        let num = rng.gen_range(1000000..10000000);
        if sum_of_digits(num) % 7 == 0 {
            break num;
        }
    };

    let key: String = format!("{:03}-{}", first_three_digits, next_seven_digits);

    return key;
}

fn gen_algo_key() -> i32 {
    let mut random_number;
    let mut sum;
    loop {
        random_number = generate_six_digit_number();
        sum = sum_of_digits(random_number);

        if sum % 7 == 0 {
            // check if last digit is 8, 9 or 0
            let last_digit = random_number % 10;
            if last_digit == 8 || last_digit == 9 || last_digit == 0 {
                // regen
                continue;
            } else {
                return random_number;
            }
        }
    }
}



fn generate_six_digit_number() -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(100_000..1_000_000);
}

fn sum_of_digits(number: i32) -> i32 {
    number
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .sum()
}
