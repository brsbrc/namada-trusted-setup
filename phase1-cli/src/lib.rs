// Documentation
#![doc = include_str!("../README.md")]

mod combine;
use std::path::PathBuf;

pub use combine::combine;

mod contribute;
pub use contribute::contribute;

pub mod keys;

mod new_challenge;
pub use new_challenge::new_challenge;

pub mod requests;

mod transform_pok_and_correctness;
pub use transform_pok_and_correctness::transform_pok_and_correctness;

mod transform_ratios;
pub use transform_ratios::transform_ratios;

use phase1_coordinator::{
    objects::round::LockedLocators,
    rest::{ContributorStatus, PostChunkRequest},
};

use reqwest::Url;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct CoordinatorUrl {
    #[structopt(
        help = "The ip address and port of the coordinator",
        required = true,
        default_value = "http://0.0.0.0:8080",
        env = "NAMADA_COORDINATOR_ADDRESS",
        parse(try_from_str)
    )]
    pub coordinator: Url,
}

#[derive(Debug, StructOpt)]
pub struct MnemonicPath {
    #[structopt(help = "The path to the mnemonic file", required = true, parse(try_from_str))]
    pub path: PathBuf,
}

#[derive(Debug, StructOpt)]
pub struct Contributors {
    #[structopt(
        help = "The path to the contributors.json file",
        required = true,
        parse(try_from_str),
        long
    )]
    pub path: PathBuf,
    #[structopt(help = "The amount of tokens to assign", required = true, long)]
    pub amount: u32,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "namada-mpc", about = "Namada CLI for trusted setup.")]
pub enum CeremonyOpt {
    #[structopt(about = "Contribute to the ceremony")]
    Contribute {
        #[structopt(flatten)]
        url: CoordinatorUrl,
        #[structopt(
            long,
            help = "Perform only the randomness computation step skipping all communication"
        )]
        offline: bool,
    },
    #[structopt(about = "Stop the coordinator and close the ceremony")]
    CloseCeremony(CoordinatorUrl),
    #[structopt(about = "Generate a Namada keypair from a mnemonic")]
    ExportKeypair(MnemonicPath),
    #[structopt(about = "Generate the list of addresses of the contributors")]
    GenerateAddresses(Contributors),
    #[structopt(about = "Get a list of all the contributions received")]
    GetContributions(CoordinatorUrl),
    #[cfg(debug_assertions)]
    #[structopt(about = "Verify the pending contributions")]
    VerifyContributions(CoordinatorUrl),
    #[cfg(debug_assertions)]
    #[structopt(about = "Update manually the coordinator")]
    UpdateCoordinator(CoordinatorUrl),
}
