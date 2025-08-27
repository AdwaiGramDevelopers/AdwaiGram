use std::io::Write;
use std::fs::File;
use std::fs::create_dir_all;
use std::io::BufWriter;
use std::path::PathBuf;

pub fn build() {
    println!("cargo:rerun-if-changed=locales/");

    let locales_dir = PathBuf::from("locales").read_dir().unwrap();
    let out_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"),"/__locales_compiled"));

    for dir in locales_dir {
        let dir = dir.unwrap().path();
        if dir.is_dir() {
            let language = dir
                .components()
                .last()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap();
            create_dir_all(out_dir.join(language)).unwrap();

            let mut out_file =
                BufWriter::new(File::create(out_dir.join(language).join("AdwaiGram.ftl")).unwrap());

            for ftl in dir.read_dir().unwrap() {
                let ftl = ftl.unwrap().path();
                let contents = std::fs::read_to_string(&ftl).unwrap();
                writeln!(out_file, "{}", contents).unwrap();
            }
        };
    }
}
