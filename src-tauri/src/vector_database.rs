pub mod vector_database{


    use oasysdb::{collection::{self, Collection, Config, Record, SearchResult}, database::Database, distance::Distance, err::Error, metadata::Metadata, prelude, vector::{Vector, VectorID}};


    pub struct VectorDatabaseService{
        db: Database,
        collection_config: Config
    }


    impl VectorDatabaseService  {
        pub fn new(path: &str) -> VectorDatabaseService{
            let db = Database::new(path).unwrap();

            let mut config = Config::default();

            config.distance = Distance::Cosine;

            Self {db, collection_config: config}
        }

        fn get_collection(&self, collection_name: &str) -> Collection {
            return self.db.get_collection(collection_name)
            .unwrap_or(Collection::new(&self.collection_config));
        }


        pub fn add_vector(&mut self, collection_name: &str, vector: Vec<f32>, metadata: Option<&Metadata>) -> Result<VectorID, Error>{

            let mut collection = self.get_collection(collection_name);

            let db_vector = Vector::from(vector);

            let default_metadata = "";

            let rec = Record::new(&db_vector, metadata.unwrap_or(&default_metadata.into()));

            return match collection.insert(&rec) {
                Ok(vector_id) => {
                    self.db.save_collection(collection_name, &collection).unwrap();
                    return Ok(vector_id);
                }
                Err(err) => Err(err),
            }

        }

        pub fn query(&self, collection_name: &str, query_vector: Vec<f32>, count: usize) -> Result<Vec<SearchResult>, Error>{
            let db_query_vector = Vector::from(query_vector);

            let collection = self.get_collection(collection_name);

            collection.search(&db_query_vector, count)
        }

    }


}