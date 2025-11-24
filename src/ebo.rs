pub struct Ebo {
    pub name: String,
    pub ingredients: Vec<String>,
}

impl Ebo {
    pub fn new(name: &str, ingredients: Vec<&str>) -> Self {
        Ebo {
            name: name.to_string(),
            ingredients: ingredients.iter().map(|s| s.to_string()).collect(),
        }
    }
    
    pub fn offer(&self) {
        println!("Offering: {}", self.name);
        for ingredient in &self.ingredients {
            println!("  - {}", ingredient);
        }
    }
}
