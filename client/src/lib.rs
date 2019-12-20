mod utils;

extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate scripture_types;

#[macro_use]
extern crate lazy_static;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
// use std::collections::HashSet;
// use std::error::Error;

// extern crate web_sys;
// use web_sys::console;
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         console::log_1(&format!( $( $t )* ).into());
//     }
// }

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchPreferences {
  pub and: bool,
  #[serde(rename = "caseSensitive")]
  pub case_sensitive: bool,
  pub exact: bool,
  #[serde(rename = "includedSources")]
  pub included_sources: IncludedSources,
  #[serde(rename = "includedBooks")]
  pub included_books: IncludedBooks,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct IncludedSources {
    pub ot: bool,
    pub nt: bool,
    pub bom: bool,
    pub dc: bool,
    pub pogp: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IncludedBooks {
    // TODO: Represent these as `HashSet`s
    pub ot: Vec<String>,
    pub nt: Vec<String>,
    pub bom: Vec<String>,
    pub dc: (u64, u64),
    pub pogp: Vec<String>,
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


static STR_BOOK_OF_MORMON: &'static str = include_str!("../../data-bundler/data/book-of-mormon.json");
static STR_OLD_TESTAMENT: &'static str = include_str!("../../data-bundler/data/old-testament.json");
static STR_NEW_TESTAMENT: &'static str = include_str!("../../data-bundler/data/new-testament.json");
static STR_PEARL_OF_GREAT_PRICE: &'static str = include_str!("../../data-bundler/data/pearl-of-great-price.json");
static STR_DOCTRINE_AND_COVENANTS: &'static str = include_str!("../../data-bundler/data/doctrine-and-covenants.json");

// TODO: Figure out to do this one, at compile time.
lazy_static! {
    static ref BOOK_OF_MORMON: scripture_types::BookOfMormon = serde_json::from_str(&STR_BOOK_OF_MORMON).unwrap();
    static ref OLD_TESTAMENT: scripture_types::OldTestament = serde_json::from_str(&STR_OLD_TESTAMENT).unwrap();
    static ref NEW_TESTAMENT: scripture_types::NewTestament = serde_json::from_str(&STR_NEW_TESTAMENT).unwrap();
    static ref PEARL_OF_GREAT_PRICE: scripture_types::PearlOfGreatPrice = serde_json::from_str(&STR_PEARL_OF_GREAT_PRICE).unwrap();
    static ref DOCTRINE_AND_COVENANTS: scripture_types::DoctrineAndCovenants = serde_json::from_str(&STR_DOCTRINE_AND_COVENANTS).unwrap();
}


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

// enum AndOr {
//     And = 1,
//     Or = 0,
// }

fn format_verse(v: &scripture_types::Verse) -> String {
    format!("{}: {}", &v.reference, &v.text)
}

fn inclusive_contains(x: u64, bounds: (u64, u64)) -> bool {
    x >= bounds.0 && x <= bounds.1
}

#[wasm_bindgen]
pub fn full_match_search(search_term_raw: String, search_preferences_js: JsValue) -> JsValue {
    let search_preferences: SearchPreferences = search_preferences_js.into_serde().unwrap();
    let search_term = &search_term_raw.to_lowercase();
    let case_sensitive_match = |verse: &&scripture_types::Verse| verse.text.contains(&search_term_raw);
    let case_insensitive_match = |verse: &&scripture_types::Verse| verse.text.to_lowercase().contains(search_term);

    let verse_search: Box<dyn Fn(&&scripture_types::Verse) -> bool> = if search_preferences.case_sensitive {
        Box::new(case_sensitive_match)
    } else {
        Box::new(case_insensitive_match)
    };

    let mut results: Vec<String> = vec![];

    if search_preferences.included_sources.ot {
        let mut ot_results: Vec<String> = (&*OLD_TESTAMENT).books.iter()
            .filter(|book| search_preferences.included_books.ot.contains(&book.book))
            .flat_map(|book| &book.chapters)
            .flat_map(|chapter| &chapter.verses)
            .filter(&verse_search)
            .map(format_verse).collect();

        results.append(&mut ot_results);
    }

    if search_preferences.included_sources.nt {
        let mut nt_results: Vec<String> = (&*NEW_TESTAMENT).books.iter()
            .filter(|book| search_preferences.included_books.nt.contains(&book.book))
            .flat_map(|book| &book.chapters)
            .flat_map(|chapter| &chapter.verses)
            .filter(&verse_search)
            .map(format_verse).collect();
        results.append(&mut nt_results);
    }

    if search_preferences.included_sources.bom {
        let mut bom_results: Vec<String> = (&*BOOK_OF_MORMON).books.iter()
            .filter(|book| search_preferences.included_books.bom.contains(&book.book))
            .flat_map(|book| &book.chapters)
            .flat_map(|chapter| &chapter.verses)
            .filter(&verse_search)
            .map(format_verse).collect();
        results.append(&mut bom_results);
    }

    if search_preferences.included_sources.dc {
        let mut dc_results: Vec<String> = (&*DOCTRINE_AND_COVENANTS).sections.iter()
            .filter(|section| inclusive_contains(section.section, search_preferences.included_books.dc))
            .flat_map(|section| &section.verses)
            .filter(&verse_search)
            .map(format_verse).collect();
        results.append(&mut dc_results);
    }

    if search_preferences.included_sources.pogp {
        let mut pogp_results: Vec<String> = (&*PEARL_OF_GREAT_PRICE).books.iter()
            .filter(|book| search_preferences.included_books.pogp.contains(&book.book))
            .flat_map(|book| &book.chapters)
            .flat_map(|chapter| &chapter.verses)
            .filter(&verse_search)
            .map(format_verse).collect();
        results.append(&mut pogp_results);
    }

    JsValue::from_serde(&results).unwrap()
}
