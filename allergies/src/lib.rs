pub struct Allergies {
    score: u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}
impl Allergen {
    pub fn all() -> Vec<Allergen> {
        return vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats
        ];
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {score}
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        // 5&4 = 4 5&2 = 0 
        return self.score & *allergen as u32 > 0;
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        return Allergen::all().into_iter().filter(|x| self.is_allergic_to(x)).collect::<Vec<_>>();
    }
}
