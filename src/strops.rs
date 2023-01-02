use regex::Regex;

pub fn extract_domain(a: String) -> String {
    let start = Regex::new(r"^https?://www.").unwrap();
    let end = Regex::new("/.*$").unwrap();
    let b = start.replace(a.as_str(), "");
    let temp = b.to_string();
    let c = end.replace(temp.as_str(), "");
    return c.to_string();
}


#[cfg(test)]
mod tests {
    use crate::strops::extract_domain;

    #[test]
    fn guardian() {
        let asdf = "https://www.theguardian.com/international".to_string();
        assert_eq!("theguardian.com", extract_domain(asdf))
    }
}