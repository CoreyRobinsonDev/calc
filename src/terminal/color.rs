pub fn black(text: impl Into<String>) -> String {
    let text: String = text.into();
    let text = text.as_str();

    return "\x1b[30m".to_string() + text + "\x1b[0m";
}

pub fn red(text: impl Into<String>) -> String {
    let text: String = text.into();
    let text = text.as_str();

    return "\x1b[31m".to_string() + text + "\x1b[0m";
}

pub fn green(text: impl Into<String>) -> String {
    let text: String = text.into();
    let text = text.as_str();

    return "\x1b[32m".to_string() + text + "\x1b[0m";
}

pub fn yellow(text: impl Into<String>) -> String {
    let text: String = text.into();
    let text = text.as_str();

    return "\x1b[33m".to_string() + text + "\x1b[0m";
}

pub fn blue(text: impl Into<String>) -> String {
    let text: String = text.into();
    let text = text.as_str();

    return "\x1b[34m".to_string() + text + "\x1b[0m";
}

pub fn purple(text: impl Into<String>) -> String {
    let text: String = text.into();
    let text = text.as_str();

    return "\x1b[35m".to_string() + text + "\x1b[0m";
}

pub fn cyan(text: impl Into<String>) -> String {
    let text: String = text.into();
    let text = text.as_str();

    return "\x1b[36m".to_string() + text + "\x1b[0m";
}

pub fn white(text: impl Into<String>) -> String {
    let text: String = text.into();
    let text = text.as_str();

    return "\x1b[37m".to_string() + text + "\x1b[0m";
}

pub fn bold(text: impl Into<String>) -> String {
    let text: String = text.into();
    let text = text.as_str();

    return "\x1b[1m".to_string() + text + "\x1b[0m";
}

pub fn underline(text: impl Into<String>) -> String {
    let text: String = text.into();
    let text = text.as_str();

    return "\x1b[4m".to_string() + text + "\x1b[0m";
}

