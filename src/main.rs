mod app;
mod cli;
mod commands;
mod config;
mod domain;
mod generator;
mod ports;
pub mod model;

fn main() {
    cli::run();
}
