extern crate rustc_serialize;
extern crate walkdir;
extern crate pulldown_cmark;
extern crate mustache;
extern crate yaml_rust;
extern crate core;

pub mod builders {
    use walkdir::DirEntry;
    use std::path::Path;
    use yaml_rust::Yaml;

    fn is_markdown(entry: &DirEntry) -> bool {
        entry.file_name()
            .to_str()
            .map(|s| s.ends_with(".md"))
            .unwrap_or(false)
    }

    fn is_special(entry: &DirEntry) -> bool {
        entry.file_name()
            .to_str()
            .map(|s| s.starts_with("_"))
            .unwrap_or(false)
    }

    fn is_git(entry: &DirEntry) -> bool {
        entry.file_name()
            .to_str()
            .map(|s| s.starts_with(".git"))
            .unwrap_or(false)
    }

    pub fn build_all(directory: &Path, config: &Yaml) -> Result<(), &'static str> {
        // get list of all files down tree from current directory

        markdown::build(directory, config);
        direct_copy::build(directory, config);

        Ok(())
    }

    pub mod markdown {
        use walkdir::WalkDir;
        use std::fs::{File, DirBuilder};
        use std::io::{Read};
        use std::path::{Path, PathBuf};
        use pulldown_cmark;
        use mustache;
        use yaml_rust::Yaml;
        use builders;

        pub fn build(directory: &Path, config: &Yaml) {
            let templates_dir = config["templates_dir"].as_str().unwrap_or("_templates");
            let template_name = config["template_name"].as_str().unwrap_or("default");

            let mut template_path = PathBuf::from(directory);
            template_path.push(templates_dir);
            template_path.push(template_name);
            template_path.push("page.mustache");

            let mut file = File::open(&template_path).expect("couldn't open template file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("couldn't read template file");

            let default_template = mustache::compile_str(&contents);

            let files = WalkDir::new(directory)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| !builders::is_git(&e))
                .filter(|e| !builders::is_special(&e))
                .filter(|e| builders::is_markdown(&e));

            for entry in files {
                // load file contents
                let path = entry.path();

                let mut file = File::open(&path).expect("couldn't open file");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("couldn't read file");

                // convert markdown to html
                let mut html = String::with_capacity(contents.len() * 3/2);
                let parsed = pulldown_cmark::Parser::new_ext(&contents, pulldown_cmark::Options::empty());
                pulldown_cmark::html::push_html(&mut html, parsed);

                // wrap post body with html page template
                let data_builder  = mustache::MapBuilder::new()
                    .insert_str("body", &html);

                let out_path_prefix = "./_site/";
                let mut out_path = PathBuf::from(out_path_prefix);
                out_path.push(path);
                out_path.set_extension("html");

                // ensure dir exists
                DirBuilder::new()
                    .recursive(true)
                    .create(out_path.parent().unwrap()).unwrap();

                let mut out_file = File::create(&out_path).expect("couldn't create out file");

                default_template.render_data(&mut out_file,&data_builder.build());
            }
        }

    }

    pub mod direct_copy {
        use walkdir::WalkDir;
        use std::fs;
        use std::fs::DirBuilder;
        use std::path::{Path, PathBuf};
        use yaml_rust::Yaml;

        pub fn build(directory: &Path, config: &Yaml) {
            let output_dir = config["output_dir"].as_str().unwrap_or("_site");
            let templates_dir = config["templates_dir"].as_str().unwrap_or("_templates");
            let template_name = config["template_name"].as_str().unwrap_or("default");

            let mut statics_directory = PathBuf::from(directory);
            statics_directory.push(templates_dir);
            statics_directory.push(template_name);
            statics_directory.push("static");

            let files = WalkDir::new(statics_directory.clone())
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file());

            for entry in files {
                // load file contents
                let in_path = entry.path();
                let rel_path = iter_after(in_path.components(), statics_directory.components())
                    .map(|c| c.as_path()).expect("static file not in statics_directory");

                let mut out_path = PathBuf::from(directory);
                out_path.push(output_dir);
                out_path.push(rel_path);

                // ensure dir exists
                DirBuilder::new()
                    .recursive(true)
                    .create(out_path.parent().unwrap()).expect("couldn't create directory");

                println!("{} -> {} ({})", in_path.to_str().unwrap(), out_path.to_str().unwrap(), rel_path.to_str().unwrap());

                fs::copy(in_path, out_path).expect("couldn't copy static file");
            }
        }

        // Iterate through `iter` while it matches `prefix`; return `None` if `prefix`
        // is not a prefix of `iter`, otherwise return `Some(iter_after_prefix)` giving
        // `iter` after having exhausted `prefix`.
        fn iter_after<A, I, J>(mut iter: I, mut prefix: J) -> Option<I>
            where I: Iterator<Item = A> + Clone,
                  J: Iterator<Item = A>,
                  A: PartialEq
        {
            loop {
                let mut iter_next = iter.clone();
                match (iter_next.next(), prefix.next()) {
                    (Some(x), Some(y)) => {
                        if x != y {
                            return None;
                        }
                    }
                    (Some(_), None) => return Some(iter),
                    (None, None) => return Some(iter),
                    (None, Some(_)) => return None,
                }
                iter = iter_next;
            }
        }
    }
}

pub mod config {
    use std::fs::File;
    use std::io;
    use std::io::Read;
    use std::path::Path;
    use yaml_rust::{Yaml, YamlLoader};

    pub fn read(folder: &Path) -> Result<Yaml, io::Error> {
        let mut file_path = folder.to_path_buf();
        file_path.push("Virgil.yaml");

        let mut file = try!(File::open(file_path));
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents));

        let mut documents = YamlLoader::load_from_str(&contents)
            .unwrap_or(vec![Yaml::Null]);
        Ok(documents.pop().unwrap_or(Yaml::Null))
    }
}

pub mod init {
    use walkdir::WalkDir;
    use std::fs::File;
    use std::path::Path;

    pub fn init_folder(folder: &Path) -> Result<(), &'static str> {
        if !folder.is_dir() {
            return Err("must init existing directory");
        }

        let files = WalkDir::new(folder)
            .into_iter().peekable();

        if files.count() > 1 {
            return Err("directory not empty, can't initialize site");
        }

        File::create("Virgil.yaml").expect("couldn't create blank config");
        Ok(())
    }
}
