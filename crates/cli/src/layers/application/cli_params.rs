#[derive(Clap)]
pub struct ExemplifyCliParams {
    #[clap(short)]
    pub source_directory: String,

    #[clap(short)]
    pub extensions: Vec<String>,

    #[clap(long)]
    pub print: bool,

    #[clap(long, default_value="##exemplify-start##")]
    pub start_token: String,

    #[clap(long, default_value="##exemplify-end##")]
    pub end_token: String,

    #[clap(short,about="Folder to generate example files into. If this parameter is not provided, examples are printed to stdout")]
    pub output_folder: Option<String>,
}
