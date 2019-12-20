extern crate scripture_types;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::path::Path;

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";

fn ensure_data_source(test_path: &std::path::PathBuf) {
    if Path::new(&test_path).exists() {
        println!("Data source exists, skipping install.");
    } else {
        println!("Data source not found, installing...");
        std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("{} i", NPM))
            .status()
            .expect("Unable to install data source");
    }
}

pub fn read_file(filepath: &str) -> String {
    let file = File::open(filepath)
        .expect("could not open file");
    let mut buffered_reader = BufReader::new(file);
    let mut contents = String::new();
    let _number_of_bytes: usize = match buffered_reader.read_to_string(&mut contents) {
        Ok(number_of_bytes) => number_of_bytes,
        Err(_err) => 0
    };

    contents
}

pub fn copy_minified<T: serde::de::DeserializeOwned + serde::ser::Serialize>(
    src_folder: &std::path::PathBuf,
    dest_folder: &std::path::PathBuf,
    file_name: &str,
) -> () {
    let mut src = src_folder.clone();
    src.push(file_name); 

    let unparsed: String = read_file(&src.into_os_string().into_string().unwrap());
    let parsed: T = serde_json::from_str(&unparsed).unwrap();

    let mut dest = dest_folder.clone();
    dest.push(file_name);
    let mut f = BufWriter::new(File::create(dest).unwrap());
    serde_json::to_writer(&mut f, &parsed).unwrap();
}

fn main() {
    let mut project_root = std::env::current_exe().expect("Unable to find");
    project_root.pop();
    project_root.pop();
    project_root.pop();

    let mut data_dep_folder = project_root.clone();
    data_dep_folder.push("node_modules");
    data_dep_folder.push("@bencrowder");
    data_dep_folder.push("scriptures-json");

    let mut test_path = data_dep_folder.clone();
    test_path.push("old-testament.json"); 
    ensure_data_source(&test_path);

    let mut data_folder = project_root.clone();
    data_folder.push("data-bundler");
    data_folder.push("data");

    // TODO: is there a way to have a list of concrete values and types,
    // and interate on each of them?
    // AFAIK, you can't mix term and type levels like that in Haskell.
    // let sources = vec![
    //     ("old-testament.json", scripture_types::OldTestament),
    // ];
    copy_minified::<scripture_types::OldTestament>(
        &data_dep_folder,
        &data_folder,
        "old-testament.json",
    );

    copy_minified::<scripture_types::NewTestament>(
        &data_dep_folder,
        &data_folder,
        "new-testament.json",
    );

    copy_minified::<scripture_types::BookOfMormon>(
        &data_dep_folder,
        &data_folder,
        "book-of-mormon.json",
    );

    copy_minified::<scripture_types::DoctrineAndCovenants>(
        &data_dep_folder,
        &data_folder,
        "doctrine-and-covenants.json",
    );

    copy_minified::<scripture_types::PearlOfGreatPrice>(
        &data_dep_folder,
        &data_folder,
        "pearl-of-great-price.json",
    );
}
