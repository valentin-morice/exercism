pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    const STUDENTS: [&str; 12] = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    fn plant_name(c: char) -> &'static str {
        match c {
            'G' => "grass",
            'C' => "clover",
            'R' => "radishes",
            'V' => "violets",
            _ => unreachable!(),
        }
    }

    let i = STUDENTS.iter().position(|n| n == &student).unwrap();
    let mut result = Vec::new();

    for row in diagram.lines() {
        let chars: Vec<char> = row.chars().collect();
        let chunks: Vec<&[char]> = chars.chunks(2).collect();
        result.extend(chunks[i].iter().map(|&c| plant_name(c)));
    }

    result
}
