use scraper::{selectable::Selectable, Html, Selector};



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start_url = "https://svenska.se/saol/?id=0000004&pz=5";

    let response = reqwest::get(start_url).await?;

    let text = response.text().await?;
    
    parse(&text);

    Ok(())
}

fn parse(text: &str) {
    let doc = Html::parse_document(text);

    let lemma_selector = Selector::parse(".lemma").unwrap();
    let lemmas = doc.select(&lemma_selector);

    for lemma in lemmas {
        let word_class_selector = Selector::parse(".ordklass").unwrap();
        let x = lemma.select(&word_class_selector);
        for x in x {
            println!("{}", x.text().collect::<String>());
        }

        let conjugation_selector = Selector::parse(".ordform .bform").unwrap();
        let x = lemma.select(&conjugation_selector);
        for x in x {
            println!("{}", x.text().collect::<String>());
        }
    }
}

pub struct BaseLemma {
    word: String,
    wordclass: WordClass
}

pub enum WordClass {
    Noun,
    ProperNoun,
    Adjective,
    Verb,
    Adverb,
    Preposition,
}
