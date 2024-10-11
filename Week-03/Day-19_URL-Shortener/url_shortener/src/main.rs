use std::collections::HashMap;
use rand::Rng;

struct URLShortener {
    url_map: HashMap<String, String>,
}

impl URLShortener {
    fn new() -> Self {
        URLShortener {
            url_map: HashMap::new(),
        }
    }

    fn generate_short_code(&self, url: &str) -> String {
        let mut rng = rand::thread_rng();
        let chars: Vec<char> = url.chars().collect();
        (0..6)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect()
    }

    fn shorten_url(&mut self, url: &str) -> String {
        let short_code = self.generate_short_code(url);
        self.url_map.insert(short_code.clone(), url.to_string());
        short_code
    }

    fn get_original_url(&self, short_code: &str) -> Option<String> {
        self.url_map.get(short_code).cloned()
    }
}

fn main() {
    let mut shortener = URLShortener::new();
    let url = "https://www.google.com";
    let short_code = shortener.shorten_url(url);
    println!("Shortened URL: http://localhost:8080/{}", short_code);
    if let Some(original_url) = shortener.get_original_url(&short_code) {
        println!("Original URL: {}", original_url);
    }
}