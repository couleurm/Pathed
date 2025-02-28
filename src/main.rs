#![windows_subsystem = "windows"] // https://stackoverflow.com/questions/29763647/how-to-make-a-program-that-does-not-display-the-console-window
use std::{env, fs::File, io::Read, process::Command};

fn cfg() -> Option<String> {
    let (bin_name, filepath) = {
        // path/to/pathed
        let exe_path = env::current_exe().expect("Pathed: Failed to get current executable path");

        // path/to/
        let exe_directory = exe_path
            .parent()
            .expect("Pathed: Failed to get parent directory");

        // path/to/pd.txt
        let pd_path = exe_directory.join("pd.txt");
        let pd_path_str = pd_path.to_string_lossy();

        // pd
        let exe_filename_without_extension = exe_path
            .file_stem()
            .expect("Failed to get filename without extension")
            .to_string_lossy();

        (
            exe_filename_without_extension.into_owned(),
            pd_path_str.into_owned(),
        )
    };

    let mut file = match File::open(&filepath) {
        Ok(file) => file,
        Err(e) => panic!("Error opening file: {}", e),
    };

    let metadata = match file.metadata() {
        Ok(metadata) => metadata,
        Err(e) => panic!("Error getting file metadata: {}", e),
    };
    if metadata.len() == 0 {
        panic!("Error: File is empty: {file:?}");
    }

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(e) => panic!("Error reading file: {}", e),
    };

    let mut round = 1;

    for mut cur in content.split('\n') {
        cur = cur.trim();
        round += 1;
        println!("HI {bin_name}");

        match cur {
            _comment
                if cur.starts_with('#')
                    || cur.starts_with('/')
                    // || cur.starts_with(';') // unused in smoothie, but Pathed uses this
                    || cur.starts_with(':')
                    || cur.is_empty() => {}

            base if cur.contains(";") => {
                let (trigger, value) = base
                    .split_once(';')
                    .expect("Pathed: Failed to split_once an url");

                // split all aliases into an array
                let mut aliases: Vec<&str> = trigger.split("|").collect();

                // turn Vec<&str> into Vec<String> and .triml() them
                aliases = aliases.iter().map(|s| s.to_owned().trim()).collect();

                // omit empty strings (e.g if you've put multiple ||||)
                aliases.retain(|&s| !s.is_empty());

                if aliases.len() == 0 {
                    panic!("Pathed: found empty aliases, does a line start with ||; somewhere?")
                }
                println!("{:?}", aliases);

                if aliases.contains(&bin_name.as_str()) {
                    return Some(value.trim().to_owned());
                }
            }
            _folder_shortcut if cur.contains("=") => {}
            _ => panic!("Pathed: Failed to parse {cur:?}, line {round}"),
        }
    }
    return None;
}

fn insufficient_args() {
    println!(
        r#"
Pathed {}

CLI arguments examples:

--urlhttps://youtube.com/?searchquery=
--sep+
rick roll

    "#,
        // env!("CARGO_PKG_VERSION")
        "0.1.0"
    );
    std::process::exit(0);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args.get(2).is_some() {
        if vec!["-h", "-help", "--help", "/?", "?"].contains(&args.get(2).unwrap().as_str()) {
            insufficient_args();
        }
    }
    if args.len() == 1 {
        // no arguments passed, open explorer in pd's directrory

        // path/to/pathed
        let exe_path = env::current_exe().expect("Pathed: Failed to get current executable path");

        // path/to/
        let exe_directory = exe_path
            .parent()
            .expect("Pathed: Failed to get parent directory");

        let mut cmd = Command::new("explorer");

        cmd.arg(exe_directory);

        match cmd.spawn() {
            Ok(_) => std::process::exit(0),
            Err(_) => std::process::exit(1),
        }
    }

    let mut separator = "+";
    let mut url: Option<&str> = None;
    let mut query: Vec<String> = vec![];

    let mut round = 1; // skip exe filename

    for argument in args[1..].iter().map(|s| s.trim()) {
        println!("Round: {}, Argument: {}", round, argument);

        if round <= 2 {
            if argument.starts_with("--sep") {
                separator = &argument["--sep".len()..];
                println!("Selecting separator from arguments: {:?}", separator);
            } else if argument.starts_with("--url") {
                url = Some(&argument["--url".len()..]);
                println!("Selecting url from arguments: {:?}", url);
            } else {
                println!("Adding early query {argument:?}");
                query.push(argument.to_string());
            }
        } else {
            println!("Adding query {argument:?}");
            query.push(argument.to_string());
        }
        round += 1;
    }

    let url: String = match url {
        Some(url) => url.to_string(),
        None => match cfg() {
            Some(url) => url,
            None => {
                println!("Insufficient arguments provided. Exiting program...");
                std::process::exit(1);
            }
        },
    };

    let query_joined: String = query.join(separator);

    let final_url: String = {
        if url.contains("{{}}") {
            // if the query needs to be at a specific place / multiple times within the code
            url.replace("{{}}", &query_joined)
        } else if url.contains("{0}") {
            let mut count = 0; // start at {0}

            let mut ret = url; // set return value

            loop {
                let formatter = "{".to_string() + &count.to_string() + "}";
                println!("{query:?}[{count}]: {}", query.get(count).is_none());
                if query.get(count).is_none() {
                    println!("break! we gut now!");
                    break;
                }
                if ret.contains(&formatter) {
                    ret = ret.replace(&formatter, &query[count]);
                    count += 1;
                } else {
                    break;
                }
            }

            ret
        } else {
            // no format specifiers found, concat normally
            url + query_joined.as_str()
        }
    };

    // println!("{final_url}");

    webbrowser::open(&final_url).expect("Failed to open URL");
}
