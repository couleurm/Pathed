
use clap::Parser;

/// Simple program to quickly open a website, with queries optionally!
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Website to search on
   #[clap(short, long, value_parser)]
   website: String,

   /// What you wish to search (optional)
   #[clap(short, long, value_parser, multiple_values=true, multiple=true, default_value = " ")]
   query: Vec<String>,

   /// Character to join spaces with
   #[clap(short, long, value_parser, default_value_t = '+')]
   separator: char,
}

fn main(){

    if cfg!(target_os = "windows"){

        use winapi::um::{wincon::GetConsoleWindow, winuser::{SW_HIDE, ShowWindow}};

        unsafe {
            let window = GetConsoleWindow();
            if !window.is_null() {
                        // Hide the terminal window
                        ShowWindow(window, SW_HIDE);
                    }
        }
    }

    let args = Args::parse();

    let mut web: &str = &args.website;
    match web {
        // Scoop, extremely opinionated >:)
        "scoop" =>          web = "https://scoop.sh/#/apps?q=",
        "extras" =>         web = "https://github.com/ScoopInstaller/Extras/search?q=",
        "main" =>           web = "https://github.com/ScoopInstaller/main/search?q=",
        "utils" =>          web = "https://github.com/couleur-tweak-tips/utils/search?q=",

        "pys" =>            web = "https://duckduckgo.com/?q=Python+",
        "ps" =>             web = "https://duckduckgo.com/?q=PowerShell+",
        "rs" =>             web = "https://duckduckgo.com/?q=Rust+",
        "crates" =>         web = "https://crates.io/crates/",

        "tw" =>             web = "https://twitter.com/search?q=",
        "twu" =>            web = "https://twitter.com/",

        "tele" =>           web = "https://t.me/",

        "aur" =>            web = "https://aur.archlinux.org/packages?K=",
        "archpkg" =>        web = "https://archlinux.org/packages/?q=",
        "wiki" =>           web = "https://wiki.archlinux.org/index.php?search=", // the one and only
        "wikipedia" =>      web = "https://en.wikipedia.org/w/index.php?search=", // less based
        "drive" =>          web = "https://drive.google.com/drive/search?q=",

        "gh" =>             web = "https://github.com/search?q=", // GitHub search
        "repo" =>           web = "https://github.com/", // GitHub repository/organization/user
        "gist" =>           web = "https://gist.github.com/search?q=", // GitHub gists

        "gm0" =>            web = "https://mail.google.com/mail/u/0/#search/",
        "gm1" =>            web = "https://mail.google.com/mail/u/1/#search/",
        "gm2" =>            web = "https://mail.google.com/mail/u/2/#search/",
        "gm3" =>            web = "https://mail.google.com/mail/u/3/#search/",

        "amazon" =>         web = "https://www.amazon.com/s?k=",
        "amazonfr" =>       web = "https://www.amazon.fr/s?k=",
        "amazonit" =>       web = "https://www.amazon.it/s?k=",
        "amazonca" =>       web = "https://www.amazon.ca/s?k=",
        "amazonuk" =>       web = "https://www.amazon.co.uk/s?k=",

        "chocolatey" =>     web = "https://community.chocolatey.org/packages?q=",
        "namemc" =>         web = "https://mine.ly/",
        "pwshgallery" =>    web = "https://www.powershellgallery.com/packages?q=",
        "duckduckgo" =>     web = "https://duckduckgo.com/?q=",
        "genius" =>         web = "https://genius.com/search?q=",
        "google" =>         web = "https://www.google.com/search?q=",
        "you" =>            web = "https://you.com/search?q=",
        "emojipedia" =>     web = "https://emojipedia.org/search/?q=",
        "stackoverflow" =>  web = "https://stackoverflow.com/search?q=",
        "discordid" =>      web = "https://discord.id/?prefill=",

        "ys" =>             web = "https://www.youtube.com/results?search_query=",

        "yl" => { // YouTube link
            web = "https://www.youtube.com/watch?v=";
            args.query = args.query
                            .replace("https://","")
                            .replace("http://","")
                            .replace("www.youtube.com/", "")
                            .replace("shorts/", "")
        },
        _ => {},
        "dc" => { // Discord server invite
            web = "https://discordapp.com/invite/";
            args.query = args.query
                            .replace("https://discordapp.com/invite/","")
                            .replace("https://discord.gg/invite/","")

        },

        "meriam-webster" => {
            web = "https://www.merriam-webster.com/dictionary/";
            args.separator = ' ';
        },
    };

    let url: String = format!("{web}{query}", query = args.query.join(&args.separator.to_string()));

    webbrowser::open(&url)
    .expect("Failed to open URL");

}