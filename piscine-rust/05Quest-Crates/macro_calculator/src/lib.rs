pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        // Extract the numerical kcal value from the string
        let kcal_str = food.calories.1.replace("kcal", "");
        let kcal: f64 = kcal_str.parse().unwrap_or(0.0);

        // Multiply the macros by the number of portions and add to totals
        total_cals += kcal * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
    }

    // Helper closure to handle rounding up to 2 decimal points
    let round = |val: f64| (val * 100.0).round() / 100.0;

    // Use the json::object! macro to construct the JSON layout
    json::object! {
        "cals": round(total_cals),
        "carbs": round(total_carbs),
        "proteins": round(total_proteins),
        "fats": round(total_fats)
    }
}