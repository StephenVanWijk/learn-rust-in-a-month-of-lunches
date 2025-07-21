struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        };
        Self {
            name: name.to_string(),
            ceo,
        }
    }

    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

fn main() {
    mapping_and_filtering_paragraph_911_1();
    mapping_and_filtering_paragraph_911_2();

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
    assert_eq!(x.is_ok(), false);

    let x: Result<i32, &str> = Err("Some error message");
    assert_eq!(x.is_ok(), false);
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