pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let question = message.ends_with('?');
    let has_letters = message.chars().any(char::is_alphabetic);
    let yell = has_letters
        && message
            .chars()
            .filter(|c| c.is_alphabetic())
            .all(char::is_uppercase);

    match (yell, question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        (false, false) => "Whatever.",
    }
}
