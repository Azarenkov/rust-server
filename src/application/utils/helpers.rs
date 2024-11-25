use regex::Regex;

pub fn extract_link_and_date(html: &str) -> Option<String> {
    let re = Regex::new(r#"<a class="dimmed" href="[^"]+">([^<]+)</a>, ([^<]+)</span>"#).unwrap();
    if let Some(captures) = re.captures(html) {
        let date = captures.get(1)?.as_str().to_string();
        let time = captures.get(2)?.as_str().to_string();
        Some(format!("{} {}", date, time))
    } else {
        None
    }
}