use regex::Regex;

pub fn extract_link_and_date(input: &str) -> Option<String> {
    let re = Regex::new(r#"<a href="([^"]+)">([^<]+)</a>, (\d{2}:\d{2})"#).unwrap();
    
    if let Some(captures) = re.captures(input) {
        let url = &captures[1]; 
        let date = &captures[2]; 
        let time = &captures[3];
        
        Some(format!("{} {}", date, time))
    } else {
        None
    }
}