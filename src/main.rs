fn main() {
    if !run() {
        std::process::exit(1)
    }
}

fn run() -> bool {
    let file_path = std::env::args().skip(1).next();
    if file_path.is_none() {
        return false;
    }

    let file_path = file_path.unwrap();
    let content_old = std::fs::read_to_string(&file_path);

    if content_old.is_err() {
        return false;
    }

    let content_old = content_old.unwrap();
    if content_old.is_empty() {
        return true;
    }

    let lines: Vec<&str> = content_old.lines().collect();
    let mut content_new = lines.join("\r\n");

    content_new.push_str("\r\n");

    if content_old.len() == content_new.len() {
        return true;
    }

    if std::fs::write(file_path, content_new).is_err() {
        false
    } else {
        true
    }
}
