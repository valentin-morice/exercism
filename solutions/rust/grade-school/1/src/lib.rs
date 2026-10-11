use std::collections::HashMap;

pub struct School {
    grades: HashMap<u32, Vec<String>>,
}

impl School {
    #[must_use]
    pub fn new() -> School {
        School {
            grades: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self
            .grades
            .values()
            .any(|students| students.iter().any(|s| s == student))
        {
            return;
        }

        self.grades
            .entry(grade)
            .or_default()
            .push(String::from(student));
    }

    #[must_use]
    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.grades.keys().copied().collect();
        grades.sort_unstable();
        grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    #[must_use]
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students = self.grades.get(&grade).cloned().unwrap_or_default();
        students.sort();
        students
    }
}
