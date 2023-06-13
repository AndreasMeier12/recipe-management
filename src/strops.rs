use regex::Regex;

pub fn extract_domain(a: String) -> String {
    let start = Regex::new(r"^https?://(www.)?").unwrap();
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
        assert_eq!("theguardian.com", extract_domain(asdf));
        let fdsa = "https://www.theguardian.com/food/2022/aug/24/nigel-slater-midweek-dinner-recipe-baked-peaches-cinnamon-cream-sauce";
        assert_eq!("theguardian.com", extract_domain(fdsa.to_string()));
    }

    #[test]
    fn nyt(){
        let url = "https://cooking.nytimes.com/recipes/1017018-shoofly-pie".to_string();
        assert_eq!("cooking.nytimes.com", extract_domain(url))

    }
}