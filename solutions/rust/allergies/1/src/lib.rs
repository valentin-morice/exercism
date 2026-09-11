#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs = 0,
    Peanuts = 1,
    Shellfish = 2,
    Strawberries = 3,
    Tomatoes = 4,
    Chocolate = 5,
    Pollen = 6,
    Cats = 7,
}

pub struct Allergies {
    score: u32,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & (1 << *allergen as u32) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        use Allergen::*;
        [Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats]
            .into_iter()
            .filter(|a| self.is_allergic_to(a))
            .collect()
    }
}
