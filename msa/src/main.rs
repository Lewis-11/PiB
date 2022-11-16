mod fasta;
mod utils;
mod adjacency_matrix;
mod gusfields;
mod alignment;

use clap::{Parser, Subcommand};
use alignment::gusfield_msa;
use fasta::read_fasta_file;
use utils::{read_submatrix_file,map_seq_name_to_id};

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

    /// Gusfield's 2-approximation algorithm using minimum spanning trees
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
            println!(
                "we should process the 'ref' subcommand with parameters: {:?},{:?},{:?}",
                records, submat, maximize
            );
            let sm = read_submatrix_file(submat);
            let records = read_fasta_file(records);
            println!("Sequences to align:");
            for record in &records {
                println!("{}", record);
            }
            let seq_id_map = map_seq_name_to_id(&records);
            let alignment = gusfield_msa(&records, &sm, &seq_id_map, *gap_cost, *maximize
            ).expect("gusfields alignment failed");
            println!("\n{}", alignment);

        }
        Commands::Mst {
            records,
            submat,
            maximize,
            gap_cost,
        } => {
            println!(
                "we should process the 'mst' subcommand with parameters: {:?},{:?},{:?},{:?}",
                records, submat, maximize, gap_cost
            );
        }
    }
}
