use crate::{indeed::IndeedProvider, providers::{JobScrapper, Regions}};

pub fn fetch_jobs() {
    let indeed_scrapper = IndeedProvider::new(Regions::EC);
    let content_result = indeed_scrapper.load_raw_content();

    match content_result {
        Ok(content) => {
            let titles = indeed_scrapper.find_job_titles(&content);
            print!("{:?}", titles);
        },
        Err(_) => println!("Error on fecth"),
    }

}