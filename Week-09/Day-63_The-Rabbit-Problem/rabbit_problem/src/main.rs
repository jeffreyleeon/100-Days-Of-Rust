fn get_months_to_dominate(mut male_rabbits_vec: Vec<u64>, mut female_rabbits_vec: Vec<u64>, number_to_dominate: u64, current_month: u64) -> u64 {
    // Get number of female that is fertile
    let total_female: u64 = female_rabbits_vec.iter().sum();
    let fertiled_female = total_female - female_rabbits_vec[0] - female_rabbits_vec[1] - female_rabbits_vec[2] - female_rabbits_vec[3];

    // Get number of male and female born this month
    let new_male_number = fertiled_female * 5;
    let new_female_number = fertiled_female * 9;

    // Add to the vectors
    male_rabbits_vec.pop();
    male_rabbits_vec.insert(0, new_male_number);
    female_rabbits_vec.pop();
    female_rabbits_vec.insert(0, new_female_number);

    // Check if the number of rabbits is greater than the number to dominate
    let total_male_after_breed: u64 = male_rabbits_vec.iter().sum();
    let total_female_after_breed: u64 = female_rabbits_vec.iter().sum();
    println!("Total number is {} in month {}", total_male_after_breed + total_female_after_breed, current_month);
    if total_male_after_breed + total_female_after_breed >= number_to_dominate {
        return current_month;
    }
    get_months_to_dominate(male_rabbits_vec, female_rabbits_vec, number_to_dominate, current_month + 1)
}

fn rabbit_problem(number_of_male: u64, number_of_femail: u64, number_to_dominate: u64) -> u64 {
    let max_month = 96 as usize;
    let mut male_rabbits_vec = vec![0; max_month];
    let mut female_rabbits_vec = vec![0; max_month];
    male_rabbits_vec[2] = number_of_male;
    female_rabbits_vec[2] = number_of_femail;
    get_months_to_dominate(male_rabbits_vec, female_rabbits_vec, number_to_dominate, 1)
}

fn main() {
    let months = rabbit_problem(2, 4, 15000000000);
    println!("Months: {}", months);
}
