fn main() {
    let response = reqwest::blocking::get(
        "https://bednovel.com/bednovel/the-beginning-after-the-end-73592",
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);

    let mut chapter_links = Vec::new();

    for element in document.select(&scraper::Selector::parse("a.con").unwrap()) {
        let link = element.value().attr("href").unwrap();
        chapter_links.push(link);
    }

    for link in chapter_links {
        if std::path::Path::new(&format!("chapter_{}.txt", link
            .split("/")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .split("ch")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .to_string())).exists()
        {
            continue
        }
        else {
            let chapter_response = reqwest::blocking::get(&format!("https://bednovel.com{}", link))
            .unwrap()
            .text()
            .unwrap();

        let chapter_document = scraper::Html::parse_document(&chapter_response);

        let mut chapter_text = String::new();

        for element in chapter_document.select(&scraper::Selector::parse("p").unwrap()) {
            chapter_text.push_str(&format!("{}\n\n", element.inner_html()));
        }

        if chapter_text.contains("Use arrow keys (or A / D) to PREV/NEXT chapter") {
            chapter_text = chapter_text.replace("Use arrow keys (or A / D) to PREV/NEXT chapter", "");
        }
    
        let chapter_number = link
            .split("/")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .split("ch")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .to_string();

        std::fs::write(
            format!("chapters/chapter_{}.txt", chapter_number),
            chapter_text,
        )
        .unwrap();

        }

    }

    println!("Done!");
}