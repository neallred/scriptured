extern crate rust_stemmers;
extern crate scripture_types;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use regex::Regex;
use rust_stemmers::{Algorithm, Stemmer};

mod utils;
mod preferences;

#[macro_use]
extern crate lazy_static;

use flate2::read::GzDecoder;
use scripture_types::{
    BookOfMormon, DoctrineAndCovenants, NewTestament, OldTestament, PathsIndex, PearlOfGreatPrice,
    VersePath, WordsIndex,
};
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::prelude::*;
use wasm_bindgen::prelude::*;

extern crate web_sys;
use web_sys::console;
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static BYTES_OLD_TESTAMENT: &'static [u8] =
    include_bytes!("../../data-bundler/data/old-testament.json.gz");
static BYTES_NEW_TESTAMENT: &'static [u8] =
    include_bytes!("../../data-bundler/data/new-testament.json.gz");
static BYTES_BOOK_OF_MORMON: &'static [u8] =
    include_bytes!("../../data-bundler/data/book-of-mormon.json.gz");
static BYTES_DOCTRINE_AND_COVENANTS: &'static [u8] =
    include_bytes!("../../data-bundler/data/doctrine-and-covenants.json.gz");
static BYTES_PEARL_OF_GREAT_PRICE: &'static [u8] =
    include_bytes!("../../data-bundler/data/pearl-of-great-price.json.gz");

static BYTES_WORDS_INDEX: &'static [u8] =
    include_bytes!("../../data-bundler/data/words-index.json.gz");
static BYTES_PATHS_INDEX: &'static [u8] =
    include_bytes!("../../data-bundler/data/paths-index.json.gz");
static BASE_URL: &'static str = "https://www.churchofjesuschrist.org/study/scriptures";

// TODO: Figure out to do this one, at compile time.
lazy_static! {
    static ref BOOK_OF_MORMON: BookOfMormon = parse_gzip(&BYTES_BOOK_OF_MORMON);
    static ref OLD_TESTAMENT: OldTestament = parse_gzip(&BYTES_OLD_TESTAMENT);
    static ref NEW_TESTAMENT: NewTestament = parse_gzip(&BYTES_NEW_TESTAMENT);
    static ref PEARL_OF_GREAT_PRICE: PearlOfGreatPrice = parse_gzip(&BYTES_PEARL_OF_GREAT_PRICE);
    static ref DOCTRINE_AND_COVENANTS: DoctrineAndCovenants =
        parse_gzip(&BYTES_DOCTRINE_AND_COVENANTS);
    static ref WORDS_INDEX: WordsIndex = parse_gzip(&BYTES_WORDS_INDEX);
    static ref PATHS_INDEX: PathsIndex = parse_gzip(&BYTES_PATHS_INDEX);
    static ref STEMMER: rust_stemmers::Stemmer = Stemmer::create(Algorithm::English);
    static ref RE_VERSE_CHARS: Regex = Regex::new(r"[^A-Za-z0-9\sæ\-]").unwrap();
}

fn make_link(verse_path: &scripture_types::VersePath) -> String {
    let url_slug = match verse_path {
        VersePath::PathOT(b, c, v) => {
            let coll = &(&*OLD_TESTAMENT);

            let book = &coll.books[*b];
            format!("{}/{}/{}.{}", coll.lds_slug, book.lds_slug, c + 1, v + 1)
        }
        VersePath::PathNT(b, c, v) => {
            let coll = &(&*NEW_TESTAMENT);

            let book = &coll.books[*b];
            format!("{}/{}/{}.{}", coll.lds_slug, book.lds_slug, c + 1, v + 1)
        }
        VersePath::PathBoM(b, c, v) => {
            let coll = &(&*BOOK_OF_MORMON);

            let book = &coll.books[*b];
            format!("{}/{}/{}.{}", coll.lds_slug, book.lds_slug, c + 1, v + 1)
        }
        VersePath::PathDC(s, v) => {
            let coll = &(&*DOCTRINE_AND_COVENANTS);

            format!("{}/{}.{}", coll.lds_slug, s + 1, v + 1)
        }
        VersePath::PathPOGP(b, c, v) => {
            let coll = &(&*PEARL_OF_GREAT_PRICE);

            let book = &coll.books[*b];
            format!(
                "{}/{}/{}.{}?lang=eng",
                coll.lds_slug,
                book.lds_slug,
                c + 1,
                v + 1
            )
        }
    };
    format!("{}/{}", BASE_URL, url_slug)
}

fn highlight_matches(text: &String, highlights: &Vec<(usize, usize)>) -> String {
    highlights
        .iter()
        .rev()
        .fold(text.to_string(), |mut acc, (from, to)| {
            let word_to_replace = &acc[*from..*to];
            acc.replace_range(
                from..to,
                &format!("<span class=\"match\">{}</span>", word_to_replace),
            );
            acc
        })
}

fn format_verse(
    p: &scripture_types::VersePath,
    v: &scripture_types::Verse,
    highlights: &Vec<(usize, usize)>,
) -> String {
    format!(
        "<span><a target=\"_blank\" rel=\"noopener noreferrer\" href=\"{}\">{}</a>: {}</span>",
        make_link(p),
        &v.reference,
        highlight_matches(&v.text, highlights),
    )
}

pub fn parse_gzip<T: serde::de::DeserializeOwned + serde::ser::Serialize>(gzipped: &[u8]) -> T {
    let mut d = GzDecoder::new(gzipped);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();

    let data: T = serde_json::from_str(&s).unwrap();
    data
}

#[wasm_bindgen]
pub fn bootstrap_searcher() {
    // Force the minimal amount of work to initialize all data structures
    // so that user searches are speedy.
    let empty_preferences = preferences::make_empty_preferences();
    full_match_search(
        String::from("BOOSTRAP SCRIPTURED SEARCHER"),
        JsValue::from_serde(&empty_preferences).unwrap(),
    );
    log!("words: {:?}", WORDS_INDEX.len());
    log!("paths: {:?}", PATHS_INDEX.len());
}

fn make_splittable(text: &String) -> String {
    let with_substitutions = text
        .replace("–", " ")
        .replace("—", " ")
        .replace("—", " ")
        .replace("'s", "")
        .to_lowercase();
    let splittable = RE_VERSE_CHARS.replace_all(&with_substitutions, "");
    splittable.to_string()
}

pub fn resolve_verse_path(
    path: &VersePath,
    _preferences: &preferences::SearchPreferences,
) -> &'static scripture_types::Verse {
    match path {
        VersePath::PathOT(b, c, v) => &(&*OLD_TESTAMENT).books[*b].chapters[*c].verses[*v],
        VersePath::PathNT(b, c, v) => &(&*NEW_TESTAMENT).books[*b].chapters[*c].verses[*v],
        VersePath::PathBoM(b, c, v) => &(&*BOOK_OF_MORMON).books[*b].chapters[*c].verses[*v],
        VersePath::PathDC(s, v) => &(&*DOCTRINE_AND_COVENANTS).sections[*s].verses[*v],
        VersePath::PathPOGP(b, c, v) => &(&*PEARL_OF_GREAT_PRICE).books[*b].chapters[*c].verses[*v],
    }
}

// #[wasm_bindgen]
// pub fn full_match_search_old(search_term_raw: String, search_preferences_js: JsValue) -> JsValue {
//     let search_preferences: SearchPreferences = search_preferences_js.into_serde().unwrap();
//     let search_term = &make_splittable(&search_term_raw.to_lowercase());
//     // let case_sensitive_match =
//     //     |verse: &&scripture_types::Verse| verse.text.contains(&search_term_raw);
//     // let case_insensitive_match =
//     //     |verse: &&scripture_types::Verse| verse.text.to_lowercase().contains(search_term);
//
//     // let verse_search: Box<dyn Fn(&&scripture_types::Verse) -> bool> =
//     //     if search_preferences.case_sensitive {
//     //         Box::new(case_sensitive_match)
//     //     } else {
//     //         Box::new(case_insensitive_match)
//     //     };
//
//     let all_verses: HashSet<u32> = (1..(&*PATHS_INDEX).len() as u32).collect();
//     // TODO: Use this for "or" searches
//     // let no_verses: HashSet<u32> = HashSet::new();
//
//     let index_results = search_term
//         .split_whitespace()
//         .fold(all_verses, |acc, word| {
//             let stemmed_word = STEMMER.stem(word);
//             // log!("searching: '{}' (stemmed as '{}')", word, stemmed_word);
//             let index_results = match (&*WORDS_INDEX).get(&stemmed_word.to_string()) {
//                 Some(x) => {
//                     // log!("acc: {:?}", acc.len());
//                     // log!("new: {:?}", x.len());
//                     let in_both: HashSet<_> = acc.intersection(x).map(|&x| x).collect();
//                     // log!("in_both: {:?}", in_both.len());
//                     in_both
//                 }
//                 None => {
//                     // log!("not found: {}", word);
//                     HashSet::new()
//                 }
//             };
//             index_results
//         });
//     let verses: Vec<String> = index_results
//         .iter()
//         .map(|x| (&*PATHS_INDEX).get(x).unwrap())
//         .map(|x| (x, resolve_verse_path(x, &search_preferences), search_term))
//         .map(format_verse)
//         .collect();
//     JsValue::from_serde(&verses).unwrap()
//
//     // TODO: Use this code to further filter potential matches
//     // let mut results: Vec<String> = vec![];
//     // if search_preferences.included_sources.ot {
//     //     let mut ot_results: Vec<String> = (&*OLD_TESTAMENT)
//     //         .books
//     //         .iter()
//     //         .filter(|book| search_preferences.included_books.ot.contains(&book.book))
//     //         .flat_map(|book| &book.chapters)
//     //         .flat_map(|chapter| &chapter.verses)
//     //         .filter(&verse_search)
//     //         .map(format_verse)
//     //         .collect();
//
//     //     results.append(&mut ot_results);
//     // }
//
//     // if search_preferences.included_sources.nt {
//     //     let mut nt_results: Vec<String> = (&*NEW_TESTAMENT)
//     //         .books
//     //         .iter()
//     //         .filter(|book| search_preferences.included_books.nt.contains(&book.book))
//     //         .flat_map(|book| &book.chapters)
//     //         .flat_map(|chapter| &chapter.verses)
//     //         .filter(&verse_search)
//     //         .map(format_verse)
//     //         .collect();
//     //     results.append(&mut nt_results);
//     // }
//
//     // if search_preferences.included_sources.bom {
//     //     let mut bom_results: Vec<String> = (&*BOOK_OF_MORMON)
//     //         .books
//     //         .iter()
//     //         .filter(|book| search_preferences.included_books.bom.contains(&book.book))
//     //         .flat_map(|book| &book.chapters)
//     //         .flat_map(|chapter| &chapter.verses)
//     //         .filter(&verse_search)
//     //         .map(format_verse)
//     //         .collect();
//     //     results.append(&mut bom_results);
//     // }
//
//     // if search_preferences.included_sources.dc {
//     //     let mut dc_results: Vec<String> = (&*DOCTRINE_AND_COVENANTS)
//     //         .sections
//     //         .iter()
//     //         .filter(|section| {
//     //             inclusive_contains(section.section, search_preferences.included_books.dc)
//     //         })
//     //         .flat_map(|section| &section.verses)
//     //         .filter(&verse_search)
//     //         .map(format_verse)
//     //         .collect();
//     //     results.append(&mut dc_results);
//     // }
//
//     // if search_preferences.included_sources.pogp {
//     //     let mut pogp_results: Vec<String> = (&*PEARL_OF_GREAT_PRICE)
//     //         .books
//     //         .iter()
//     //         .filter(|book| search_preferences.included_books.pogp.contains(&book.book))
//     //         .flat_map(|book| &book.chapters)
//     //         .flat_map(|chapter| &chapter.verses)
//     //         .filter(&verse_search)
//     //         .map(format_verse)
//     //         .collect();
//     //     results.append(&mut pogp_results);
//     // }
//
//     // JsValue::from_serde(&results).unwrap()
// }

fn check_collection_searchable(verse_path: &VersePath, preferences: &preferences::SearchPreferences) -> bool {
    let return_value = match verse_path {
        VersePath::PathOT(book, _, _) => (
            preferences.included_sources.ot &&
            preferences.included_books.ot.contains(&(&*OLD_TESTAMENT).books[*book].book)
        ),
        VersePath::PathNT(book, _, _) => (
            preferences.included_sources.nt &&
            preferences.included_books.nt.contains(&(&*NEW_TESTAMENT).books[*book].book)
        ),
        VersePath::PathBoM(book, _, _) => (
            preferences.included_sources.bom &&
            preferences.included_books.bom.contains(&(&*BOOK_OF_MORMON).books[*book].book)
        ),
        VersePath::PathDC(sec, _) => (
            preferences.included_sources.dc &&
            sec >= &(preferences.included_books.dc.0 as usize) &&
            sec <= &(preferences.included_books.dc.1 as usize)
        ),
        VersePath::PathPOGP(book, _, _) => {
            let title = &(&*PEARL_OF_GREAT_PRICE).books[*book].book;
            (
                preferences.included_sources.pogp &&
                preferences.included_books.pogp.contains(title)
            )
        },
    };

    log!("verse_path {:?}: {:?}", verse_path, return_value);
    return_value
}

pub type WordsIndexBorrowing = HashMap<String, &'static HashMap<u32, Vec<(usize, usize)>>>;
#[wasm_bindgen]
pub fn full_match_search(search_term_raw: String, search_preferences_js: JsValue) -> JsValue {
    let search_preferences: preferences::SearchPreferences = search_preferences_js.into_serde().unwrap();
    if !preferences::check_can_search(&search_term_raw, &search_preferences) {
        let no_results: Vec<String> = vec![];
        return JsValue::from_serde(&no_results).unwrap();
    }

    let search_term = &make_splittable(&search_term_raw.to_lowercase());

    let search_stems: HashSet<_> = search_term
        .split_whitespace()
        .map(|term| STEMMER.stem(term).to_string())
        .collect();
    let possible_matches: WordsIndexBorrowing =
        search_stems
            .iter()
            .fold(HashMap::new(), |mut matching_index, term| {
                if let Some(v) = (&*WORDS_INDEX).get(term) {
                    matching_index.insert(term.to_string(), v);
                }
                matching_index
            });

    let p_index = &*PATHS_INDEX;

    log!("search preferences:\n{:?}", search_preferences);
    let or_matches: HashSet<u32> = possible_matches
        .iter()
        .flat_map(|(_k, v)| v.keys())
        .map(|x| *x)
        .filter(|x| check_collection_searchable(p_index.get(x).unwrap(), &search_preferences))
        .collect();

    let and_matches: HashSet<u32> = possible_matches.iter().fold(or_matches, |acc, (_k, v)| {
        let current_matches: HashSet<u32> = v.keys().map(|x| *x).collect();
        let result: HashSet<u32> = acc.intersection(&current_matches).map(|x| *x).collect();
        result
    });

    let verses: Vec<String> = possible_matches
        .iter()
        .flat_map(|(_k, v)| v.iter().filter(|x| and_matches.contains(x.0)))
        .map(|(scripture_id, highlights)| {
            let verse_path = p_index.get(scripture_id).unwrap();
            (verse_path, highlights)
        })
        .fold(
            HashMap::new(),
            |mut acc: HashMap<&VersePath, Vec<(usize, usize)>>, (verse_path, highlights)| {
                let mut cloned = highlights.clone();
                acc.entry(verse_path)
                    .and_modify(|existing_highlights| {
                        existing_highlights.append(&mut cloned);
                        existing_highlights.sort();
                    })
                    .or_insert(cloned);
                acc
            },
        )
        .iter()
        .map(|(verse_path, highlights)| {
            let verse = resolve_verse_path(verse_path, &search_preferences);
            format_verse(verse_path, verse, highlights)
        })
        .collect();

    JsValue::from_serde(&verses).unwrap()
}
