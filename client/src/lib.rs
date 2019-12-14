mod utils;

extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use std::error::Error;



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


// TODO: Figure out how to get a static struct in the desired format at compile time.
// This does not work:
// static BOM_2: &'static str = serde_json::to_string(serde_json::from_str(&include_str!("../client/node_modules/@bencrowder/scriptures-json/book-of-mormon.json"))?)?;

// TODO: Figure out how to make the compiler run an npm install if the json files do not exist at
// compile time

//    let npm = Command::new(NPM)
//        .arg("install")
//        .status();
//
//    println!("npm {:?}", npm);

static BOOK_OF_MORMON: &'static str = include_str!("../../node_modules/@bencrowder/scriptures-json/book-of-mormon.json");
static OLD_TESTAMENT: &'static str = include_str!("../../node_modules/@bencrowder/scriptures-json/old-testament.json");
static NEW_TESTAMENT: &'static str = include_str!("../../node_modules/@bencrowder/scriptures-json/new-testament.json");
static PEARL_OF_GREAT_PRICE: &'static str = include_str!("../../node_modules/@bencrowder/scriptures-json/pearl-of-great-price.json");
static DOCTRINE_AND_COVENANTS: &'static str = include_str!("../../node_modules/@bencrowder/scriptures-json/doctrine-and-covenants.json");

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[derive(Serialize, Deserialize, Debug)]
struct Verse {
    heading: Option<String>,
    pilcrow: Option<bool>,
    reference: String,
    subheading: Option<String>,
    text: String,
    verse: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Chapter {
    chapter: u64,
    heading: Option<String>,
    note: Option<String>,
    reference: String,
    verses: Vec<Verse>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Facsimile {
    explanations: Vec<String>,
    image_url: String,
    lds_slug: String,
    note: Option<String>,
    number: u64,
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    book: String,
    chapters: Vec<Chapter>,
    facsimiles: Option<Vec<Facsimile>>,
    full_subtitle: Option<String>,
    full_title: String,
    heading: Option<String>,
    lds_slug: String,
    note: Option<String>,
}

// structs
#[derive(Serialize, Deserialize, Debug)]
struct Section {
    section: u64,
    reference: String,
    verses: Vec<Verse>,
    signature: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookOfMormon {
    books: Vec<Book>,
    last_modified: String,
    lds_slug: String,
    subtitle: String,
    testimonies: Vec<Testimony>,
    title: String,
    title_page: TitlePage,
    version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct TitlePage {
    subtitle: String,
    text: Vec<String>,
    title: String,
    translated_by: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Testimony {
    text: String,
    title: String,
    witnesses: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DoctrineAndCovenants {
    last_modified: String,
    lds_slug: String,
    sections: Vec<Section>,
    subsubtitle: String,
    subtitle: String,
    title: String,
    version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct NewTestamentTitlePage {
    subtitle: String,
    text: String,
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTestament {
    books: Vec<Book>,
    last_modified: String,
    lds_slug: String,
    title: String,
    title_page: NewTestamentTitlePage,
    version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OldTestament {
    books: Vec<Book>,
    last_modified: String,
    lds_slug: String,
    the_end: String,
    title: String,
    version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PearlOfGreatPrice {
    books: Vec<Book>,
    last_modified: String,
    subtitle: String,
    title: String,
    version: u64,
}

pub fn get_bom() -> Result<BookOfMormon, Box<dyn Error>> {
    Ok(serde_json::from_str(&BOOK_OF_MORMON)?)
}

pub fn get_ot() -> Result<OldTestament, Box<dyn Error>> {
    Ok(serde_json::from_str(&OLD_TESTAMENT)?)
}

pub fn get_nt() -> Result<NewTestament, Box<dyn Error>> {
    Ok(serde_json::from_str(&NEW_TESTAMENT)?)
}

pub fn get_pogp() -> Result<PearlOfGreatPrice, Box<dyn Error>> {
    Ok(serde_json::from_str(&PEARL_OF_GREAT_PRICE)?)
}

pub fn get_dc() -> Result<DoctrineAndCovenants, Box<dyn Error>> {
    Ok(serde_json::from_str(&DOCTRINE_AND_COVENANTS)?)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scriptures {
    ot: OldTestament,
    nt: NewTestament,
    bom: BookOfMormon,
    dc: DoctrineAndCovenants,
    pogp: PearlOfGreatPrice,
}

pub fn get_scriptures () -> Result<Scriptures, Box<dyn Error>> {
    Ok(
        Scriptures {
            ot: get_ot()?,
            nt: get_nt()?,
            bom: get_bom()?,
            dc: get_dc()?,
            pogp: get_pogp()?,
        }
    )
}

// Once struct generation happens at compile time, this won't be needed
#[wasm_bindgen]
pub fn check_parsing() {
    let bom_result = get_bom();
    match bom_result {
        Ok(_) => log!("got Book of Mormon"),
        Err(e) => log!("{:?}", e),
    }

    let ot_result = get_ot();
    match ot_result {
        Ok(_) => log!("got Old Testament"),
        Err(e) => log!("{:?}", e),
    }

    let nt_result = get_nt();
    match nt_result {
        Ok(_) => log!("got New Testatment"),
        Err(e) => log!("{:?}", e),
    }

    let pogp_result = get_pogp();
    match pogp_result {
        Ok(_) => log!("got Pearl of Great Price"),
        Err(e) => log!("{:?}", e),
    }

    let dc_result = get_dc();
    match dc_result {
        Ok(_) => log!("got Doctrine and Covenants"),
        Err(e) => log!("{:?}", e),
    }
}

#[wasm_bindgen]
pub fn full_match_search(search_term: String) -> JsValue {
    
    let final_result = match get_scriptures() {
        Ok(_) => vec![search_term],
        Err(err) => vec![format!("Error getting scriptures {:?}", err)],
    };

    JsValue::from_serde(&final_result).unwrap()
}
