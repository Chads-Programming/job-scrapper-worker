#[derive(Debug)]
pub enum CustomError {
    ScrapperError(String),
    ParserError(String),
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

pub struct JobQuery {
    pub search: Option<&'static str>,
}

pub trait JobScrapper {
    fn find_job_titles(&self, content: &str) -> Vec<JobTitle>;
    async fn load_raw_content(&self, query: &JobQuery) -> Result<String, CustomError>;
    fn get_domain(&self) -> String;
}

