extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use phf;

use serde::{Deserialize, Serialize};
use fnv::FnvHashMap;
use primitive_types::U256;

type VerseId = u16;
type BigHighlightId = u16;

pub type WordsIndex = FnvHashMap<String, FnvHashMap<VerseId, Vec<(usize, usize)>>>;
pub type PathsIndex = FnvHashMap<VerseId, VersePath>;
pub type VersePathsIndex = FnvHashMap<VersePath, VerseId>;
pub type PhfPathsIndex = phf::Map<VerseId, VersePath>;
pub type PhfWordsIndex = phf::Map<&'static str, phf::Map<VerseId, (HighlightIs, HighlightLs)>>;
pub type BigHighlightIs = phf::Map<BigHighlightId, U256>;
pub type BigHighlightLs = phf::Map<BigHighlightId, u128>;

pub fn paths_to_verse_paths_index(paths: &PhfPathsIndex) -> VersePathsIndex {
    paths
        .entries()
        .fold(
            FnvHashMap::default(),
            |mut acc, (k, v)| {
                acc.insert(v.clone(), *k);
                acc
            }
        )
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone)]
pub enum VersePath {
    PathBoM(u8, u8, u16),
    PathOT(u8, u8, u16),
    PathNT(u8, u8, u16),
    PathPOGP(u8, u8, u16),
    PathDC(u8, u16), // section verse
}

#[derive(Serialize, Deserialize)]
pub struct Verse {
    pub heading: Option<String>,
    pub pilcrow: Option<bool>,
    pub reference: String,
    pub subheading: Option<String>,
    pub text: String,
    pub verse: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Chapter {
    pub chapter: u8,
    pub heading: Option<String>,
    pub note: Option<String>,
    pub reference: String,
    pub verses: Vec<Verse>,
}

#[derive(Serialize, Deserialize)]
pub struct Facsimile {
    explanations: Vec<String>,
    image_url: String,
    lds_slug: String,
    note: Option<String>,
    number: u64,
    title: String,
}

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub book: String,
    pub chapters: Vec<Chapter>,
    pub facsimiles: Option<Vec<Facsimile>>,
    pub full_subtitle: Option<String>,
    pub full_title: String,
    pub heading: Option<String>,
    pub lds_slug: String,
    pub note: Option<String>,
}

// structs
#[derive(Serialize, Deserialize)]
pub struct Section {
    pub section: u8,
    pub reference: String,
    pub verses: Vec<Verse>,
    pub signature: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BookOfMormon {
    pub books: Vec<Book>,
    pub last_modified: String,
    pub lds_slug: String,
    pub subtitle: String,
    pub testimonies: Vec<Testimony>,
    pub title: String,
    pub title_page: TitlePage,
    pub version: u8,
}

#[derive(Serialize, Deserialize)]
pub struct TitlePage {
    pub subtitle: String,
    pub text: Vec<String>,
    pub title: String,
    pub translated_by: String,
}

#[derive(Serialize, Deserialize)]
pub struct Testimony {
    text: String,
    title: String,
    witnesses: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DoctrineAndCovenants {
    pub last_modified: String,
    pub lds_slug: String,
    pub sections: Vec<Section>,
    pub subsubtitle: String,
    pub subtitle: String,
    pub title: String,
    pub version: u8,
}

#[derive(Serialize, Deserialize)]
pub struct NewTestamentTitlePage {
    subtitle: String,
    text: String,
    title: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewTestament {
    pub books: Vec<Book>,
    pub last_modified: String,
    pub lds_slug: String,
    pub title: String,
    pub title_page: NewTestamentTitlePage,
    pub version: u8,
}

#[derive(Serialize, Deserialize)]
pub struct OldTestament {
    pub books: Vec<Book>,
    pub last_modified: String,
    pub lds_slug: String,
    pub the_end: String,
    pub title: String,
    pub version: u8,
}

#[derive(Serialize, Deserialize)]
pub struct PearlOfGreatPrice {
    pub books: Vec<Book>,
    pub last_modified: String,
    pub lds_slug: String,
    pub subtitle: String,
    pub title: String,
    pub version: u8,
}

// Optimal storage tradeoff is to store Is1 and Is2 as u32,
// and all else as U256 double lookup entries
#[derive(Debug)]
pub enum HighlightIs {
    Is1(u16),
    Is2(u32),
    Is3(BigHighlightId),
    Is4(BigHighlightId),
    Is5(BigHighlightId),
    Is6(BigHighlightId),
    Is7(BigHighlightId),
    Is8(BigHighlightId),
    Is9(BigHighlightId),
    Is10(BigHighlightId),
    Is11(BigHighlightId),
    Is12(BigHighlightId),
    Is13(BigHighlightId),
    Is14(BigHighlightId),
    Is15(BigHighlightId),
    Is16(BigHighlightId),
    Is17(BigHighlightId),
    Is18(BigHighlightId),
    Is19(BigHighlightId),
    Is20(BigHighlightId),
    Is21(BigHighlightId),
    Is22(BigHighlightId),
}

// optimal storage tradeoff is to store Ls1-3 as u16,
// and all else as u128 double lookup entries
#[derive(Debug)]
pub enum HighlightLs {
    Ls1(u8),
    Ls2(u16),
    Ls3(u16),
    Ls4(BigHighlightId),
    Ls5(BigHighlightId),
    Ls6(BigHighlightId),
    Ls7(BigHighlightId),
    Ls8(BigHighlightId),
    Ls9(BigHighlightId),
    Ls10(BigHighlightId),
    Ls11(BigHighlightId),
    Ls12(BigHighlightId),
    Ls13(BigHighlightId),
    Ls14(BigHighlightId),
    Ls15(BigHighlightId),
    Ls16(BigHighlightId),
    Ls17(BigHighlightId),
    Ls18(BigHighlightId),
    Ls19(BigHighlightId),
    Ls20(BigHighlightId),
    Ls21(BigHighlightId),
    Ls22(BigHighlightId),
}

type ISize = u16;
type LSize = u8;

fn unpack_highlight_is_num<T:
    std::cmp::PartialOrd +
    std::ops::Sub<Output = T> +
    std::ops::Shl<Output = T> +
    std::ops::Shr<Output = T> +
    Copy +
    std::fmt::Debug +
    std::convert::TryFrom<ISize> +
    std::convert::TryInto<ISize> +
>(packed: T) -> Vec<ISize> 
where <T as std::convert::TryInto<ISize>>::Error : std::fmt::Debug,
      <T as std::convert::TryFrom<ISize>>::Error : std::fmt::Debug,
{
    let mut tmp: T = packed.clone();
    let zero: T = tmp - tmp;
    let shift_by: T = T::try_from(11).unwrap();
    let mut result: Vec<ISize> = vec![];

    while tmp > zero {
        let diff: T = (tmp >> shift_by) << shift_by;
        let sub = tmp - diff;
        result.push(sub.try_into().unwrap() - 1);
        tmp = diff >> shift_by;
    }
    result.iter().map(|x| *x).rev().collect()
}

fn pack_is_helper<T:
    std::cmp::PartialOrd +
    std::ops::Add<Output = T> +
    std::ops::Shl<Output = T> +
    std::ops::Shr<Output = T> +
    Copy +
    std::fmt::Debug +
    std::convert::TryFrom<ISize> +
    std::convert::TryInto<ISize> +
>(lengths: &Vec<ISize>) -> T
where <T as std::convert::TryInto<ISize>>::Error : std::fmt::Debug,
      <T as std::convert::TryFrom<ISize>>::Error : std::fmt::Debug,
{
    let zero: T = T::try_from(0).unwrap();
    let mut result: T = T::try_from(0).unwrap();
    let shift_by: T = T::try_from(11).unwrap();
    for i in lengths {
        if result > zero {
            result = result << shift_by;
        }
        result = result + T::try_from(*i + 1).unwrap();
    }
    result
}

fn pack_is_arr(indices: &Vec<ISize>) -> [u64;4] {
    // offset all by one.
    let mut num: U256 = U256::from(0);
    for i in indices {
        if !num.is_zero() {
            num = num << 11;
        }
        num = num + U256::from(*i + 1);
    }

    let mut num_arr: [u64;4] = [0,0,0,0];

    let new_num = num >> 64 << 64;
    num_arr[0] = (num - new_num).as_u64();
    num = new_num;

    let new_num = num >> 64 << 64;
    num_arr[1] = (num - new_num).as_u64();
    num = new_num;

    let new_num = num >> 64 << 64;
    num_arr[2] = (num - new_num).as_u64();
    num = new_num;

    let new_num = num >> 64 << 64;
    num_arr[3] = (num - new_num).as_u64();

    num_arr
}

pub fn pack_is(is: &Vec<ISize>) -> HighlightIs {
    match is.len() {
        0 => HighlightIs::Is1(pack_is_helper(is)),
        1 => HighlightIs::Is1(pack_is_helper(is)),
        2 => HighlightIs::Is2(pack_is_helper(is)),
        3 => HighlightIs::Is3(pack_is_helper(is)),
        4 => HighlightIs::Is4(pack_is_helper(is)),
        5 => HighlightIs::Is5(pack_is_helper(is)),
        6 => HighlightIs::Is6(pack_is_helper(is)),
        7 => HighlightIs::Is7(pack_is_helper(is)),
        8 => HighlightIs::Is8(pack_is_helper(is)),
        9 => HighlightIs::Is9(pack_is_helper(is)),
        10 => HighlightIs::Is10(pack_is_helper(is)),
        11 => HighlightIs::Is11(pack_is_helper(is)),
        12 => HighlightIs::Is12(pack_is_helper(is)),
        13 => HighlightIs::Is13(pack_is_helper(is)),
        14 => HighlightIs::Is14(pack_is_helper(is)),
        15 => HighlightIs::Is15(pack_is_helper(is)),
        16 => HighlightIs::Is16(pack_is_helper(is)),
        17 => HighlightIs::Is17(pack_is_helper(is)),
        18 => HighlightIs::Is18(pack_is_helper(is)),
        19 => HighlightIs::Is19(pack_is_helper(is)),
        20 => HighlightIs::Is20(pack_is_helper(is)),
        21 => HighlightIs::Is21(pack_is_helper(is)),
        22 => HighlightIs::Is22(pack_is_helper(is)),
        _ => HighlightIs::Is22(pack_is_helper(&is.iter().take(22).map(|x| *x).collect())),
    }
}

pub fn pack_is_str(is: &Vec<ISize>) -> String {
    match is.len() {
        0 => format!("{:?}", HighlightIs::Is1(pack_is_helper(is))),
        1 => format!("{:?}", HighlightIs::Is1(pack_is_helper(is))),
        2 => format!("{:?}", HighlightIs::Is2(pack_is_helper(is))),
        3 => format!("{:?}", HighlightIs::Is3(pack_is_helper(is))),
        4 => format!("{:?}", HighlightIs::Is4(pack_is_helper(is))),
        5 => format!("{:?}", HighlightIs::Is5(pack_is_helper(is))),
        6 => format!("{:?}", HighlightIs::Is6(pack_is_helper(is))),
        7 => format!("{:?}", HighlightIs::Is7(pack_is_helper(is))),
        8 => format!("{:?}", HighlightIs::Is8(pack_is_helper(is))),
        9 => format!("{:?}", HighlightIs::Is9(pack_is_helper(is))),
        10 => format!("{:?}", HighlightIs::Is10(pack_is_helper(is))),
        11 => format!("{:?}", HighlightIs::Is11(pack_is_helper(is))),
        12 => format!("Is12(U256 {{ 0: {:?} }})", pack_is_arr(is)),
        13 => format!("Is13(U256 {{ 0: {:?} }})", pack_is_arr(is)),
        14 => format!("Is14(U256 {{ 0: {:?} }})", pack_is_arr(is)),
        15 => format!("Is15(U256 {{ 0: {:?} }})", pack_is_arr(is)),
        16 => format!("Is16(U256 {{ 0: {:?} }})", pack_is_arr(is)),
        17 => format!("Is17(U256 {{ 0: {:?} }})", pack_is_arr(is)),
        18 => format!("Is18(U256 {{ 0: {:?} }})", pack_is_arr(is)),
        19 => format!("Is19(U256 {{ 0: {:?} }})", pack_is_arr(is)),
        20 => format!("Is20(U256 {{ 0: {:?} }})", pack_is_arr(is)),
        21 => format!("Is21(U256 {{ 0: {:?} }})", pack_is_arr(is)),
        22 => format!("Is22(U256 {{ 0: {:?} }})", pack_is_arr(is)),
        _ => format!("Is22(U256 {{ 0: {:?} }})", pack_is_arr(&is.iter().take(22).map(|x| *x).collect())),
    }
}

// need a
//   String for first index storage
//   String for big storage
//   num id to link the two.
//   should ALWAYS be a U256 in big storage mode.
//   first index is Is##({:?} 
//   if mutate num id OUTSIDE this helper, then it keeps things simpler.
pub fn pack_is_str_big(is: &Vec<ISize>, big_id: u16) -> (String, String) {
    match is.len() {
        0 => panic!("Is vec should never be empty"),
        1 => panic!("Should not pack Is1 as big"),
        2 => panic!("Should not pack Is2 as big"),
        3 => (format!("Is3({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(is))),
        4 => (format!("Is4({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(is))),
        5 => (format!("Is5({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(is))),
        6 => (format!("Is6({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(is))),
        7 => (format!("Is7({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(is))),
        8 => (format!("Is8({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(is))),
        9 => (format!("Is9({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(is))),
        10 => (format!("Is10({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(is))),
        12 => (format!("Is12({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(is))),
        13 => (format!("Is13({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(is))),
        14 => (format!("Is14({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(is))),
        15 => (format!("Is15({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(is))),
        16 => (format!("Is16({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(is))),
        17 => (format!("Is17({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(is))),
        18 => (format!("Is18({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(is))),
        19 => (format!("Is19({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(is))),
        20 => (format!("Is20({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(is))),
        21 => (format!("Is21({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(is))),
        22 => (format!("Is22({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(is))),
        _ => (format!("Is22({})", big_id), format!("U256 {{ 0: {:?} }}", pack_is_arr(&is.iter().take(22).map(|x| *x).collect()))),
    }
}

pub fn unpack_highlight_is(highlight_is: &HighlightIs, bigs: &'static BigHighlightIs) -> Vec<ISize> {
    match highlight_is {
        HighlightIs::Is1(x) => unpack_highlight_is_num(*x),
        HighlightIs::Is2(x) => unpack_highlight_is_num(*x),
        HighlightIs::Is3(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
        HighlightIs::Is4(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
        HighlightIs::Is5(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
        HighlightIs::Is6(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
        HighlightIs::Is7(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
        HighlightIs::Is8(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
        HighlightIs::Is9(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
        HighlightIs::Is10(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
        HighlightIs::Is11(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
        HighlightIs::Is12(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
        HighlightIs::Is13(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
        HighlightIs::Is14(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
        HighlightIs::Is15(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
        HighlightIs::Is16(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
        HighlightIs::Is17(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
        HighlightIs::Is18(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
        HighlightIs::Is19(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
        HighlightIs::Is20(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
        HighlightIs::Is21(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
        HighlightIs::Is22(x) => unpack_highlight_is_num(*bigs.get(x).unwrap()),
    }
}

fn unpack_highlight_ls_num<T:
    std::cmp::PartialOrd +
    std::ops::Sub<Output = T> +
    std::ops::Shl<Output = T> +
    std::ops::Shr<Output = T> +
    Copy +
    std::fmt::Debug +
    std::convert::TryFrom<LSize> +
    std::convert::TryInto<LSize> +
>(packed: T) -> Vec<LSize> 
where <T as std::convert::TryInto<LSize>>::Error : std::fmt::Debug,
      <T as std::convert::TryFrom<LSize>>::Error : std::fmt::Debug,
{
    let mut tmp: T = packed.clone();
    let zero: T = tmp - tmp;
    let shift_by: T = T::try_from(5).unwrap();
    let mut result: Vec<LSize> = vec![];

    while tmp > zero {
        let diff: T = (tmp >> shift_by) << shift_by;
        let sub = tmp - diff;
        result.push(sub.try_into().unwrap());
        tmp = diff >> shift_by;
    }
    result.iter().map(|x| *x).rev().collect()
}


fn pack_ls_helper<T:
    std::cmp::PartialOrd +
    std::ops::Add<Output = T> +
    std::ops::Shl<Output = T> +
    std::ops::Shr<Output = T> +
    Copy +
    std::fmt::Debug +
    std::convert::TryFrom<LSize> +
    std::convert::TryInto<LSize> +
>(lengths: &Vec<LSize>) -> T
where <T as std::convert::TryInto<LSize>>::Error : std::fmt::Debug,
      <T as std::convert::TryFrom<LSize>>::Error : std::fmt::Debug,
{
    let zero: T = T::try_from(0).unwrap();
    let mut result: T = T::try_from(0).unwrap();
    let shift_by: T = T::try_from(5).unwrap();
    for i in lengths {
        if result > zero {
            result = result << shift_by;
        }
        result = result + T::try_from(*i).unwrap();
    }
    result
}

pub fn pack_ls(ls: &Vec<LSize>) -> HighlightLs {
    match ls.len() {
        0 => HighlightLs::Ls1(pack_ls_helper(ls)),
        1 => HighlightLs::Ls1(pack_ls_helper(ls)),
        2 => HighlightLs::Ls2(pack_ls_helper(ls)),
        3 => HighlightLs::Ls3(pack_ls_helper(ls)),
        4 => HighlightLs::Ls4(pack_ls_helper(ls)),
        5 => HighlightLs::Ls5(pack_ls_helper(ls)),
        6 => HighlightLs::Ls6(pack_ls_helper(ls)),
        7 => HighlightLs::Ls7(pack_ls_helper(ls)),
        8 => HighlightLs::Ls8(pack_ls_helper(ls)),
        9 => HighlightLs::Ls9(pack_ls_helper(ls)),
        10 => HighlightLs::Ls10(pack_ls_helper(ls)),
        11 => HighlightLs::Ls11(pack_ls_helper(ls)),
        12 => HighlightLs::Ls12(pack_ls_helper(ls)),
        13 => HighlightLs::Ls13(pack_ls_helper(ls)),
        14 => HighlightLs::Ls14(pack_ls_helper(ls)),
        15 => HighlightLs::Ls15(pack_ls_helper(ls)),
        16 => HighlightLs::Ls16(pack_ls_helper(ls)),
        17 => HighlightLs::Ls17(pack_ls_helper(ls)),
        18 => HighlightLs::Ls18(pack_ls_helper(ls)),
        19 => HighlightLs::Ls19(pack_ls_helper(ls)),
        20 => HighlightLs::Ls20(pack_ls_helper(ls)),
        21 => HighlightLs::Ls21(pack_ls_helper(ls)),
        22 => HighlightLs::Ls22(pack_ls_helper(ls)),
        _ => HighlightLs::Ls22(pack_ls_helper(&ls.iter().take(22).map(|x| *x).collect())),
    }
}

pub fn pack_ls_big(ls: &Vec<LSize>, big_id: u16) -> (HighlightLs, u16) {
    match ls.len() {
        0 => panic!("Ls highlights should never be empty"),
        1 => panic!("Should not pack Ls1 as big"),
        2 => panic!("Should not pack Ls2 as big"),
        3 => panic!("Should not pack Ls3 as big"),
        4 => (HighlightLs::Ls4(big_id), pack_ls_helper(ls)),
        5 => (HighlightLs::Ls5(big_id), pack_ls_helper(ls)),
        6 => (HighlightLs::Ls6(big_id), pack_ls_helper(ls)),
        7 => (HighlightLs::Ls7(big_id), pack_ls_helper(ls)),
        8 => (HighlightLs::Ls8(big_id), pack_ls_helper(ls)),
        9 => (HighlightLs::Ls9(big_id), pack_ls_helper(ls)),
        10 => (HighlightLs::Ls10(big_id), pack_ls_helper(ls)),
        11 => (HighlightLs::Ls11(big_id), pack_ls_helper(ls)),
        12 => (HighlightLs::Ls12(big_id), pack_ls_helper(ls)),
        13 => (HighlightLs::Ls13(big_id), pack_ls_helper(ls)),
        14 => (HighlightLs::Ls14(big_id), pack_ls_helper(ls)),
        15 => (HighlightLs::Ls15(big_id), pack_ls_helper(ls)),
        16 => (HighlightLs::Ls16(big_id), pack_ls_helper(ls)),
        17 => (HighlightLs::Ls17(big_id), pack_ls_helper(ls)),
        18 => (HighlightLs::Ls18(big_id), pack_ls_helper(ls)),
        19 => (HighlightLs::Ls19(big_id), pack_ls_helper(ls)),
        20 => (HighlightLs::Ls20(big_id), pack_ls_helper(ls)),
        21 => (HighlightLs::Ls21(big_id), pack_ls_helper(ls)),
        22 => (HighlightLs::Ls22(big_id), pack_ls_helper(ls)),
        _ => (HighlightLs::Ls22(big_id), pack_ls_helper(&ls.iter().take(22).map(|x| *x).collect())),
    }
}

pub fn unpack_highlight_ls(highlight_ls: &HighlightLs, bigs: &'static BigHighlightLs) -> Vec<LSize> {
    match highlight_ls {
        HighlightLs::Ls1(x) => unpack_highlight_ls_num(*x),
        HighlightLs::Ls2(x) => unpack_highlight_ls_num(*x),
        HighlightLs::Ls3(x) => unpack_highlight_ls_num(*x),
        HighlightLs::Ls4(x) => unpack_highlight_ls_num(*bigs.get(x).unwrap()),
        HighlightLs::Ls5(x) => unpack_highlight_ls_num(*bigs.get(x).unwrap()),
        HighlightLs::Ls6(x) => unpack_highlight_ls_num(*bigs.get(x).unwrap()),
        HighlightLs::Ls7(x) => unpack_highlight_ls_num(*bigs.get(x).unwrap()),
        HighlightLs::Ls8(x) => unpack_highlight_ls_num(*bigs.get(x).unwrap()),
        HighlightLs::Ls9(x) => unpack_highlight_ls_num(*bigs.get(x).unwrap()),
        HighlightLs::Ls10(x) => unpack_highlight_ls_num(*bigs.get(x).unwrap()),
        HighlightLs::Ls11(x) => unpack_highlight_ls_num(*bigs.get(x).unwrap()),
        HighlightLs::Ls12(x) => unpack_highlight_ls_num(*bigs.get(x).unwrap()),
        HighlightLs::Ls13(x) => unpack_highlight_ls_num(*bigs.get(x).unwrap()),
        HighlightLs::Ls14(x) => unpack_highlight_ls_num(*bigs.get(x).unwrap()),
        HighlightLs::Ls15(x) => unpack_highlight_ls_num(*bigs.get(x).unwrap()),
        HighlightLs::Ls16(x) => unpack_highlight_ls_num(*bigs.get(x).unwrap()),
        HighlightLs::Ls17(x) => unpack_highlight_ls_num(*bigs.get(x).unwrap()),
        HighlightLs::Ls18(x) => unpack_highlight_ls_num(*bigs.get(x).unwrap()),
        HighlightLs::Ls19(x) => unpack_highlight_ls_num(*bigs.get(x).unwrap()),
        HighlightLs::Ls20(x) => unpack_highlight_ls_num(*bigs.get(x).unwrap()),
        HighlightLs::Ls21(x) => unpack_highlight_ls_num(*bigs.get(x).unwrap()),
        HighlightLs::Ls22(x) => unpack_highlight_ls_num(*bigs.get(x).unwrap()),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     fn test_round_trip(lengths: Vec<u8>) {
//         assert_eq!(lengths, unpack_highlight_ls(pack_ls(&lengths)));
//     }
// 
//     fn test_round_trip_indices(indices: Vec<u16>) {
//         assert_eq!(indices, unpack_highlight_is(pack_is(&indices)));
//     }
// 
//     #[test]
//     fn packs_and_unpacks_highlight_lengths_less_than_32_up_to_22_elements() {
//         test_round_trip(vec![]);
//         test_round_trip((1..22).collect());
//         test_round_trip((10..31).collect());
//     }
// 
//     #[test]
//     fn packs_and_unpacks_highlight_indices_less_than_2048_up_to_22_elements() {
//         test_round_trip_indices(vec![]);
//         test_round_trip_indices(vec![1234,2046,0,1,2,3,4,5,88]);
//         test_round_trip_indices((0..21).collect());
//         test_round_trip_indices((2025..2046).collect());
//     }
// }
