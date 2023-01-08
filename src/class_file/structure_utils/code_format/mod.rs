mod attribute;
mod constant_pool;
mod root;

fn padding(str: String, n: usize) -> String {
    str.lines().map(|x| format!("{}{}", " ".repeat(n), x)).collect::<Vec<String>>().join("\n")
}
