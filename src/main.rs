extern crate rustc_serialize;
extern crate docopt;
extern crate walkdir;
extern crate pulldown_cmark;
extern crate mustache;
extern crate yaml_rust;
extern crate virgil;

const USAGE: &'static str = "
Virgil - a rusty static site generator.

Usage:
  virgil init [-v] [-p <path>]
  virgil post [-v]  [-p <path>] <file>
  virgil page [-v]  [-p <path>] <file>
  virgil [build] [-v]  [-p <path>]
  virgil serve [-v]  [-p <path>]

Options:
  -p <path>      Run in directory <path>
  -v             Display debug info
";

const INIT_WELCOME_MESSAGE: &'static str = "\
New virgil site initialized. Create some markdown files then run `virgil` to \
build your site.\
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_file: String,
    cmd_init: bool,
    cmd_build: bool,
    cmd_serve: bool,
    cmd_post: bool,
    cmd_page: bool,
    flag_p: Option<String>,
    flag_v: bool
}

fn main() {
    use docopt::Docopt;
    use std::path::Path;
    use std::process;

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_v {
        println!("{:?}", args);
    }

    let path_string = args.flag_p.unwrap_or("./".to_string());
    let path = Path::new(&path_string);

    if args.cmd_init {
        match virgil::init::init_folder(path) {
            Ok(_) =>  println!("{}", INIT_WELCOME_MESSAGE),
            Err(msg) => println!("{}", msg)
        }
    } else {
        let config_maybe = virgil::config::read(path);

        if config_maybe.is_err() {
            println!("This isn't a Virgil site, did you mean `virgil init`?");
            process::exit(-1);
        }

        let config = config_maybe.unwrap();

        if args.cmd_serve {
            println!("This command is not yet implemented.");
        } else if args.cmd_post {
            println!("This command is not yet implemented.");
        } else if args.cmd_page {
            println!("This command is not yet implemented.");
        } else /*if args.cmd_build*/ {
            virgil::builders::build_all(path, &config).unwrap();
        }
    }
}
