use std::str::FromStr;

#[derive(Clap, Clone)]
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

    #[clap(long)]
    pub output_format: Option<OutputFormat>,

    #[clap(short,about="Folder to generate example files into. If this parameter is not provided, examples are printed to stdout")]
    pub output_folder: Option<String>,
}

#[derive(Clone)]
pub enum OutputFormat {
    Asciidoctor
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "asciidoc" => Ok(OutputFormat::Asciidoctor),
            _ => Err("invalid output format".into())
        }
    }
}
