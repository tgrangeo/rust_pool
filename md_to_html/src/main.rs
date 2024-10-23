use std::fs::File;
use std::io::{Read, Write};
use regex::Regex;


fn replace_headers(str: String) -> String{
    let regex_headers: Regex = Regex::new(r"(?m)^(#{1,6})\s+(.+)$").unwrap();

    let html_output = regex_headers.replace_all(&str, |caps: &regex::Captures| {
        let level = caps[1].len();
        let content = &caps[2];
        format!("<h{}>{}</h{}>", level, content, level)
    });
    html_output.to_string()
}

fn replace(str: String, regex: String, html: String) -> String{
    let regex_headers: Regex = Regex::new(&regex).unwrap();

    let html_output = regex_headers.replace_all(&str, |caps: &regex::Captures| {
        let content = &caps[2];
        format!("<{}>{}</{}>", html, content,html)
    });
    html_output.to_string()
}

fn replace_link(str: String) -> String{
    let regex_link = Regex::new(r"\[(.+?)\]\((https?://[^\s]+)\)").unwrap();

    // Replace all Markdown links with HTML <a> tags
    let html_output = regex_link.replace_all(&str, |caps: &regex::Captures| {
        let link_text = &caps[1]; // Captured link text
        let url = &caps[2];       // Captured URL
        format!("<a href=\"{}\">{}</a>", url, link_text)
    });
    html_output.to_string()
}

fn replace_list(input: String, prefix :String) -> String {
    let lines = input.split('\n');
    let mut result = String::new();
    let mut inside_list = false;

    for s in lines {
        if s.trim_start().starts_with(&prefix) {
            if !inside_list {
                result.push_str("<ul>\n");
                inside_list = true;
            }
            let list_item = s.trim_start().trim_start_matches("- ").trim();
            result.push_str(&format!("<li>{}</li>\n", list_item));
        } else {
            if inside_list {
                result.push_str("</ul>\n");
                inside_list = false;
            }
            result.push_str(&replace_p(s));
            result.push('\n');
        }
    }
    if inside_list {
        result.push_str("</ul>\n");
    }
    result
}

fn replace_p(str:&str)->String{
    if str.chars().next().map_or(false, |c| c.is_alphanumeric()) {
        format!("<p>{}</p>", str)
    } else {
        format!("{}", str)
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("./input.md")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut res = replace_headers(contents);
    res = replace_link(res);
    res = replace(res,r"(?m)^(^>\s*(.+)$)$".to_string(),"blockquote".to_string());
    res = replace(res,r"(?m)^(`([^`]+)`)$".to_string(),"code".to_string());
    res = replace(res,r"(?m)(\*\*(.+?)\*\*|__(.+?)__)".to_string(),"b".to_string());
    res = replace(res,r"(?m)(\*(.+?)\*|_(.+?)_)".to_string(),"i".to_string());
    res = replace_list(res, "-".to_string());

    let mut output_file = File::create("./result.html")?;
    output_file.write_all(res.as_bytes())?;
    Ok(())
}
