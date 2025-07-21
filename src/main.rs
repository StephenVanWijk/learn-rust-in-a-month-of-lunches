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
    mapping_and_filtering_paragraph_911_1();
    mapping_and_filtering_paragraph_911_2();
    mapping_and_filtering_paragraph_911_3();
    mapping_andfiltering_paragraph_911_4();
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

fn mapping_andfiltering_paragraph_911_4(){
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