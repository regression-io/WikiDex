use std::error::Error;

pub(crate) trait SearchService {
    type E: Error;
    async fn search(&mut self, query: &Vec<f32>, neighbors: usize) -> Result<Vec<i64>, Self::E>;
}
