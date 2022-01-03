use clap::{AppSettings, Clap};
use path_genie::{paths::PathStore, ui::UI};
use std::error::Error;
use std::{env::current_dir, path::PathBuf};

#[derive(Debug, Clap)]
#[clap(name = "options")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long, default_value("config.cfg"))]
    config_file: PathBuf,

    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Clap)]
enum SubCommands {
    #[clap()]
    Add {
        #[clap(required = false)]
        paths: Vec<PathBuf>,
    },
    #[clap()]
    List,

    #[clap()]
    TestUI,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    // println!("options: {:?}", opts);
    match opts.subcommand {
        SubCommands::TestUI => {
            println!("testui");
            let engine = get_engine();
            let ui = UI::new(engine.list_sorted_paths());
            let selected_path = ui.init();
            println!("{:?}", selected_path);
        }
        SubCommands::Add { paths } => {
            let mut store = get_engine();
            if paths.len() > 0 {
                // add all specified paths
                for path in paths {
                    store.add_path_buf(path);
                }
            } else {
                // add current path
                store.add_path(current_dir()?.to_str().unwrap().to_owned());
            }
            store.persist();
        }
        SubCommands::List => {
            let store = get_engine();
            for path in store.list_sorted_paths() {
                println!("=> {}", path);
            }
        }
        _ => println!("Not supported"),
    };
    Ok(())
}

fn get_engine() -> PathStore {
    let mut store = PathStore::new();
    store.config_file = Some("config.yaml".to_owned());
    store.load_from_config();
    store
}
