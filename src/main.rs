use anyhow::Result;
use confique::Config;
use gguf_rs::get_gguf_container;

#[derive(Debug, Config)]
pub struct Configurations {
    /// A path that points to the model file
    pub model_filepath: String,
}

impl Configurations {
    pub fn get_model_filepath(&self) -> &str {
        &self.model_filepath
    }
}

/// /data/llama-cpp/Qwen3.8-27B-Q4_0.gguf
fn main() -> Result<()> {
    let configurations = Configurations::from_file("./configurations.toml")?;

    let mut container = get_gguf_container(configurations.get_model_filepath())?;
    let model = container.decode()?;

    println!("{}", model.model_family());

    Ok(())
}
