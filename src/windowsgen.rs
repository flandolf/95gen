// Export
//  - windowsgen::generate_windows
//  - windowsgen::generate_office
//  - windowsgen::generate_key_95

use rand::Rng;

pub fn generate_key_95() -> String {
    let day = rand::thread_rng().gen_range(1..366);
    let years_allowed = ["95", "96", "97", "98", "99", "00", "01", "02", "03"];
    let year = years_allowed[rand::thread_rng().gen_range(0..years_allowed.len())];
    println!("Year: {}", year);
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
    let day = key[0].get(0..3).unwrap().parse::<i32>().unwrap();
    let year = key[0].get(3..5).unwrap().parse::<i32>().unwrap();
    let algo = key[2];

    let years_allowed = [95, 96, 97, 98, 99, 00, 01, 02, 03];

    if !years_allowed.contains(&year) {
        println!("Year is not in the allowed range");
        return false;
    }

    println!("Day: {}\nYear: {}\nAlgo: {}", day, year, algo);

    if day <= 1 || day >= 365 {
        println!("Day is out of range");
        return false;
    }

    let mut sum = 0;
    for algo in algo.chars() {
        print!("{} +", algo);
        sum += algo.to_digit(10).unwrap();
    }
    println!(" = {}", sum);
    println!("{} / 7 = {}", sum, sum / 7);
    if sum % 7 != 0 {
        println!("Algo sum is not divisible by 7");
        return false;
    }

    return true;
}

pub fn verify_office95(key: &str) -> bool {
    let key: Vec<&str> = key.split("-").collect();
    let first_three_digits = key[0];
    let next_seven_digits = key[1];

    if first_three_digits.len() != 3 {
        println!("First three digits are not 3 characters long");
        return false;
    }

    if next_seven_digits.len() != 7 {
        println!("Next seven digits are not 7 characters long");
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
        println!("First three digits are not valid");
        return false;
    }

    if next_seven_digits % 21 != 0 {
        println!("Next seven digits are not divisible by 21");
        return false;
    }

    println!("Key is valid");
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

    // Generate a random 7-digit number divisible by 21
    let next_seven_digits: u32 = loop {
        let num = rng.gen_range(1000000..10000000);
        if num % 21 == 0 {
            break num;
        }
    };

    // Format the key
    let key = format!("{:03}-{}", first_three_digits, next_seven_digits);

    key
}

fn gen_algo_key() -> i32 {
    let mut random_number;
    let mut sum;
    loop {
        random_number = generate_six_digit_number();
        sum = sum_of_digits(random_number);

        if sum % 7 == 0 {
            return random_number;
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
        .map(|c| c.to_digit(10).unwrap() as i32) // Convert to i32
        .sum()
}
