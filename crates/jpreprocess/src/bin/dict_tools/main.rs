use std::{error::Error, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};
use jpreprocess::SystemDictionaryConfig;
use jpreprocess_dictionary_builder::{
    ipadic_builder::IpadicBuilder,
    serializer::{JPreprocessSerializer, LinderaSerializer},
};
use lindera_core::dictionary_builder::DictionaryBuilder;
use lindera_dictionary::{load_user_dictionary, UserDictionaryConfig};

use crate::dict_query::{Query, QueryDict};

mod dict_query;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Inspect {
        /// The Word id to display
        #[arg(short, long)]
        word_id: Option<u32>,

        input: PathBuf,
    },
    Build {
        /// User dictionary
        #[arg(short, long)]
        user: bool,
        /// The serlializer to be used
        #[arg(value_enum)]
        serlializer: Serlializer,

        input: PathBuf,
        /// The directory(system dictionary) or file(user dictionary) to put the dictionary.
        /// For user dictionary, the parent directory of the output file should not exist.
        output: PathBuf,
    },
}

#[derive(Clone, ValueEnum, Debug)]
enum Serlializer {
    /// Build lindera dictionary
    Lindera,
    /// Build jpreprocess dictionary
    Jpreprocess,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Inspect { word_id, input } => {
            let is_system_dict = input.is_dir()
                && input.join("dict.wordsidx").exists()
                && input.join("dict.words").exists();
            let is_user_bin_dict =
                input.is_file() && matches!(input.extension(),Some(s) if s.to_str()==Some("bin"));

            if is_system_dict || is_user_bin_dict {
                let dict = if is_system_dict {
                    println!("Lindera/JPreprocess system dictionary.");
                    let dict = SystemDictionaryConfig::File(input).load()?;
                    QueryDict::System(dict)
                } else {
                    println!("Lindera/JPreprocess user dictionary.");
                    let dict = load_user_dictionary(UserDictionaryConfig {
                        path: input,
                        kind: None,
                    })?;
                    QueryDict::User(dict)
                };

                if let Some(metadata) = dict.metadata() {
                    println!("Dictionary metadata: {}", metadata);
                } else {
                    println!("No metadata found. Assuming lindera dictionary.")
                }

                if let Some(word_id) = word_id {
                    let mode = dict.mode();

                    let query = Query { word_id, dict };

                    let word = mode.debug_get_word(&query);
                    println!("{}", word);
                }
            }
        }
        Commands::Build {
            user,
            serlializer,
            input,
            output,
        } => {
            let builder = IpadicBuilder::new(match serlializer {
                Serlializer::Lindera => Box::new(LinderaSerializer),
                Serlializer::Jpreprocess => Box::new(JPreprocessSerializer),
            });

            if user {
                println!("Building user dictionary...");
                builder.build_user_dictionary(&input, &output)?;
                println!("done.");
            } else {
                println!("Building system dictionary...");
                builder.build_dictionary(&input, &output)?;
                println!("done.");
            }
        }
    }

    Ok(())
}