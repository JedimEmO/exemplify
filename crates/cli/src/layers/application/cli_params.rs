#[derive(Clap)]
pub struct ExemplifyCliParams {
    #[clap(short)]
    pub source_directory: String,

    #[clap(short)]
    pub extensions: Vec<String>,

    #[clap(short)]
    pub output_directory: Option<String>,

    #[clap(long)]
    pub print: bool,

    #[clap(long, default_value="##exemplify-start##")]
    pub start_token: String,

    #[clap(long, default_value="##exemplify-end##")]
    pub end_token: String
}
