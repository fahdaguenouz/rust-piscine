use json::{JsonValue, object};
pub struct Food {
     pub name: String,
    pub calories: (String, String), 
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
 
}
fn round_trim(v: f64) -> f64 {
   
    let rounded = (v * 100.0).round() / 100.0;
    let s = format!("{:.2}", rounded);
    if s.ends_with("0") {
        s[..s.len()-1].parse().unwrap()
    } else {
        rounded
    }
}


pub fn calculate_macros(foods: &[Food]) -> JsonValue {
 let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;
    let mut fats = 0.0;
     for food in foods {
        let kcal_str = food.calories.1.trim_end_matches("kcal");
        let kcal: f64 = kcal_str.parse().unwrap_or(0.0);

        cals += kcal * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
    }
     object!{
        "cals" => round_trim(cals),
        "carbs" => round_trim(carbs),
        "proteins" => round_trim(proteins),
        "fats" => round_trim(fats)
    }
}