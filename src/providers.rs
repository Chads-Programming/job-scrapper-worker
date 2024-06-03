#[derive(Debug)]
pub enum CustomError {
    ScrapperError(String),
    FetchError(String)
}

#[derive(Debug)]
pub enum Regions {
    AR,
    EC,
}

#[derive(Debug)]
pub struct JobTitle {
    pub name: String,
    pub description: String,
    pub url: String,
    pub salary_info: Option<String>,
}
pub trait JobScrapper {
    fn find_job_titles(&self, content: &str) -> Vec<JobTitle>;
    fn load_raw_content(&self) -> Result<String, CustomError>;
}
