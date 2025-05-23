use headless_chrome::Browser;
use regex::Regex;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::Write;
use std::{io::stdin, time::Duration};

fn write_to_file(
    root_url: String,
    url_list: Vec<String>,
    names_list: Vec<String>,
) -> std::io::Result<()> {
    let mut gofile_id = "";
    let re = Regex::new(r"/d/([a-zA-Z0-9]+)").unwrap();
    if let Some(captures) = re.captures(root_url.as_str()) {
        if let Some(id) = captures.get(1) {
            gofile_id = id.as_str();
        }
    }

    let filename = gofile_id.to_string() + ".txt";
    let mut file = File::create_new(filename).expect("same file already exists");

    for (url, name) in url_list.iter().zip(names_list.iter()) {
        file.write_fmt(format_args!("{}{}\n", url, name)).unwrap();
    }

    Ok(())
}

fn main() {
    let browser = Browser::default().unwrap();
    let tab = browser.new_tab().unwrap();
    println!("Enter the gofile url: ");
    let mut root_url = String::new();
    stdin().read_line(&mut root_url).unwrap();
    tab.navigate_to(&root_url).unwrap();
    tab.wait_until_navigated().unwrap();

    std::thread::sleep(Duration::from_secs(3));

    let content = tab.get_content().unwrap();

    let document = Html::parse_document(&content);
    let selector = Selector::parse("img").unwrap();
    let text_selector = Selector::parse("a.item_open").unwrap();

    let mut url_list: Vec<String> = vec![];

    for img in document.select(&selector) {
        if let Some(alt) = img.value().attr("alt") {
            if alt == "Thumbnail" {
                let src = img.value().attr("src").unwrap_or("no src");

                let re = Regex::new(r"\bthumb\w*").unwrap();
                let result = re.replace(src, "");
                url_list.push(result.to_string());
            }
        }
    }

    let mut names_list: Vec<String> = vec![];

    for element in document.select(&text_selector) {
        let text = element.text().collect::<String>().trim().to_string();
        names_list.push(text.to_string());
    }

    let _ = write_to_file(root_url, url_list, names_list);
    println!("Done!");
}
