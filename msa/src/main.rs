mod fasta;
mod utils;
mod adjacency_matrix;
mod gusfields;
mod alignment;

use clap::{Parser, Subcommand};
use fasta::read_fasta_file;
use utils::read_submatrix_file;
use crate::alignment::msa;

/// Multiple sequence alignment using minimum spanning trees
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Reference implementation for Gusfield's 2-approximation algorithm
    Ref {
        /// FASTA files
        #[clap(group = "input", value_parser)]
        records: String,
        /// Substitution matrix
        #[clap(short, long, value_parser)]
        submat: String,
        /// Should we maximize the cost ?
        #[clap(short, long, value_parser, default_value_t = false)]
        maximize: bool,
        /// Gap cost
        #[clap(short, long, value_parser, default_value_t = 1)]
        gap_cost: i32,
    },

    /// Gusfield's 2-approximation algorithm using Kruskal's algorithm
    Mst {
        // Some additional args can be added in the future,
        // specific to this subcommand (e.g. the order we choose).
        /// FASTA files
        #[clap(group = "input", value_parser)]
        records: String,
        /// Substitution matrix
        #[clap(short, long, value_parser)]
        submat: String,
        /// Should we maximize the cost ?
        #[clap(short, long, value_parser, default_value_t = false)]
        maximize: bool,
        /// Gap cost
        #[clap(short, long, value_parser, default_value_t = 1)]
        gap_cost: i32,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Ref {
            records,
            submat,
            maximize,
            gap_cost,
        } => {
            let sm_string = read_submatrix_file(submat);
            let fasta_string = read_fasta_file(records);

            let output = msa(
                &sm_string,
                *gap_cost,
                *maximize,
                &fasta_string,
                &"gusfield".to_string()).unwrap();

            let result = output.0.last().unwrap().last().unwrap();
            let score = output.1;

            for seq in result {
                // convert Vec<u8> to str
                let row = std::str::from_utf8(&seq).unwrap();
                println!("{}", row);
            }
            println!("Cost: {}", score);
        }
        Commands::Mst {
            records,
            submat,
            maximize,
            gap_cost,
        } => {
            let sm_string = read_submatrix_file(submat);
            let fasta_string = read_fasta_file(records);

            let output = msa(
                &sm_string,
                *gap_cost,
                *maximize,
                &fasta_string,
                &"kruskal".to_string()).unwrap();

            let result = output.0.last().unwrap().last().unwrap();
            let score = output.1;

            for seq in result {
                // convert Vec<u8> to str
                let row = std::str::from_utf8(&seq).unwrap();
                println!("{}", row);
            }
            println!("Cost: {}", score);
        }
    }
}
