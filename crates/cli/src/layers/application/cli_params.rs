#[derive(Clap)]
pub struct ExemplifyCliParams {
    #[clap(short)]
    pub source_directories: Vec<String>,

    #[clap(short)]
    pub extensions: Vec<String>,

    #[clap(short)]
    pub output_directory: Option<String>,

    #[clap(long)]
    pub print: bool,
}
