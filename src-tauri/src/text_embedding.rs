pub mod text_embedding{
    use fastembed::{InitOptions, TextEmbedding};

    pub struct TextEmbeddingService{
        model: TextEmbedding
    }

    impl TextEmbeddingService{
        pub fn new(options: Option<InitOptions>) -> TextEmbeddingService{
            let model = TextEmbedding::try_new(options.unwrap_or(Default::default())).unwrap();
            Self { model }
        }

        pub fn embed(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>>{

            let result = self.model.embed(texts, None)?;
    
            Ok(result)
        }
    }
}