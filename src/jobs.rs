use crate::{indeed::IndeedProvider, providers::{CustomError, JobQuery, JobScrapper, JobTitle, Regions}};

pub async fn fetch_jobs() -> Result<Vec<JobTitle>, CustomError> {
    let job_query = JobQuery {
        search: Some("developer+remote"),
    };

    let indeed_scrapper = IndeedProvider::new(Regions::EC);
    let content_result: Result<String, crate::providers::CustomError> = indeed_scrapper.load_raw_content(&job_query).await;

    match content_result {
        Ok(content) => {
            let titles = indeed_scrapper.find_job_titles(&content);
            Ok(titles)
        },
        Err(err) => {            
            Err(CustomError::ScrapperError(format!("Error on fecth, {:?}", err)))
        },
    }
}