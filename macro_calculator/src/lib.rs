pub struct Food {
     name: String,
    calories: (String, String),
    fats:f64,
    carbs: f64,
    proteins: Vf64
    nbr_of_portions: f64
 
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {

}