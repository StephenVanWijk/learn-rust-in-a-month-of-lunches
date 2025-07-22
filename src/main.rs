 use chrono::Local;

struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        }; //
        Self {
            name: name.to_string(),
            ceo,
        }
    }

    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone() //
    }
}

fn main() {
    // mapping_and_filtering_paragraph_911_1();
    // mapping_and_filtering_paragraph_911_2();
    // mapping_and_filtering_paragraph_911_3();
    // mapping_and_filtering_paragraph_911_4();
    // mapping_and_filtering_paragraph_911_5();
    mapping_and_filtering_paragraph_912_1();
    mapping_and_filtering_paragraph_912_3();
    // mapping_and_filtering_paragraph_913_1()
}

fn get_current_datetime() -> String{
    let now = Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn mapping_and_filtering_paragraph_911_1() {
    let months = vec![
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December",
    ];

    let filtered_months: Vec<&str> = months
        .into_iter()
        .filter(|month| month.len() < 5)
        .filter(|month| month.contains("u"))
        .collect();

    println!("{:?}", filtered_months);
}

fn mapping_and_filtering_paragraph_911_2() {
    let companies = vec![
        Company::new("TechCorp", "Alice Smith"),
        Company::new("InnovateX", ""),
        Company::new("FutureWorks", "Bob Johnson"),
    ];

    let all_the_ceos: Vec<String> = companies
        .iter()
        .filter_map(|company| company.get_ceo())
        .collect();

    println!("{:?}", all_the_ceos);
}

fn mapping_and_filtering_paragraph_911_3(){
    let user_input = vec![
        "8.9",
        "Nine point nine five",
        "8.0",
        "7.6",
        "eleventy-twelve",
    ];

    let successful_numbers = user_input
        .iter()
        .filter_map(|input| input.parse::<f32>().ok())
        .collect::<Vec<f32>>();

    println!("{:?}", successful_numbers);

    let x: Result<i32, &str> = Ok(-3);
    assert_eq!(x.is_ok(), true);

    let x: Result<i32, &str> = Err("Some error message");
    assert_eq!(x.is_ok(), false);
}

fn mapping_and_filtering_paragraph_911_4(){
    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Brendan McCracken"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];

    let results: Vec<Result<String, &str>> = company_vec
        .iter()
        .map(|company| company.get_ceo().ok_or("No CEO found"))
        .collect();

    for item in results {
        println!("{:?}", item);
    }
}

fn mapping_and_filtering_paragraph_911_5(){
    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Brendan McCracken"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];

    let results: Vec<Result<String, String>> = company_vec
        .iter()
        .map(|company| {
            company.get_ceo().ok_or_else(|| {
                let err_message = format!("No CEO found for {}",
                company.name);
                println!("{} at {}",
                err_message,
                get_current_datetime());
                err_message
            })
        })
        .collect();

    results
        .iter()
        .filter(|res| res.is_ok())
        .for_each(|res| println!("{:?}", res));
}   

fn in_char_vec(char_vec: &Vec<char>, check: char) {
    println!(
        "Is {check} inside? {}",
        char_vec.iter().any(|&char| char == check)
    );
}

fn mapping_and_filtering_paragraph_912_1(){
    let num_array = ["8", "9", "Hi", "9898989898"];
    let mut char_vec = vec![];

    for index in 0..5 {
        char_vec.push(
            num_array
                .get(index)
                .and_then(|number| number.parse::<u64>().ok())
                .and_then(|number| u64::try_from(number).ok()),
        );
    }

    // Use filter_map to get only the Some values
    let successful_values: Vec<u64> = char_vec
        .into_iter()
        .filter_map(|x| x)
        .collect();
    println!("{:?}", successful_values);
}

//20250722 Apparently you can substitute Opetion instances, Some(value) = true, None = false.
fn mapping_and_filtering_paragraph_912_3(){
    let try_1 = [Some("Okay!"), None, Some("Okay!"), Some("Okay!"), None];
    let try_2 = [None, Some("Okay!"), Some("Okay!"), Some("Okay!"),Some("Okay!")];
    let try_3 = [Some("Okay!"), Some("Okay!"), Some("Okay!"),Some("Okay!"), None];

    for i in 0..try_1.len() {
        println!("{:?}", try_1[i].and(try_2[i]).and(try_3[i]));
    }
}

fn mapping_and_filtering_paragraph_913_1(){
    let char_vec = ('A'..'頄').collect::<Vec<char>>();
    in_char_vec(&char_vec, 'i');
    in_char_vec(&char_vec, '삄');
    in_char_vec(&char_vec, '鏗');

    let smaller_vec = ('A'..'z').collect::<Vec<char>>();
    print!("{:?}", smaller_vec);
    println!(
        "All alphabetic? {}",
        smaller_vec.iter().all(|&x| x.is_alphabetic())
    );
    println!(
        "All less than the character 행? {}",
        smaller_vec.iter().all(|&x| x < '행')
    );
}