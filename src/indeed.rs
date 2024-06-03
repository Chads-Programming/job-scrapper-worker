use std::fs;

use crate::providers::{self, CustomError, JobScrapper, JobTitle, Regions};

#[derive(Debug)]
pub struct JobProviderInfo {
    pub name: &'static str,
    pub base_domain_url: &'static str,
    pub region: Regions,
}

pub struct IndeedProvider {
    pub provider_info: JobProviderInfo,
}

impl IndeedProvider {
    pub fn new(region: Regions) -> Self {
        Self {
            provider_info: JobProviderInfo {
                name: "Indeed",
                base_domain_url: "htto",
                region,
            },
        }
    }
}

impl JobScrapper for IndeedProvider {
    fn find_job_titles(&self, content: &str) -> Vec<providers::JobTitle> {
        let document = scraper::Html::parse_document(content);

        let html_jobtitle_selector = scraper::Selector::parse(".job_seen_beacon").unwrap();
        let html_jobtitles = document.select(&html_jobtitle_selector);
        let mut job_titles: Vec<JobTitle> = vec![];

        for html_jobtitle in html_jobtitles {
            let title_result = html_jobtitle
                .select(&scraper::Selector::parse("h2.jobTitle span").unwrap())
                .next()
                .map(|span| span.text().collect::<String>());

            let description_text_result = html_jobtitle.select(&scraper::Selector::parse(".job_seen_beacon div[role=\"presentation\"] .underShelfFooter .heading6 div").unwrap())
                .next()
                .map(|div| div.text().collect::<String>());

            let description = match description_text_result {
                Some(inner_text) => inner_text,
                None => {
                    let description_list: Vec<String> = html_jobtitle
                        .select(
                            &scraper::Selector::parse(
                                ".job_seen_beacon div[role=\"presentation\"] ul",
                            )
                            .unwrap(),
                        )
                        .map(|list_item_html| list_item_html.text().collect::<String>())
                        .collect();

                    description_list.join("\n")
                }
            };

            let salary_info = html_jobtitle
                .select(&scraper::Selector::parse(".salary-snippet-container > div").unwrap())
                .next()
                .map(|div| div.text().collect::<String>());

            let url_result = html_jobtitle
                .select(&scraper::Selector::parse(".job_seen_beacon a").unwrap())
                .next()
                .and_then(|a| a.value().attr("href"))
                .map(str::to_owned);

            if let (Some(title), Some(url)) = (title_result, url_result) {
                job_titles.push(JobTitle {
                    name: title,
                    description,
                    url,
                    salary_info,
                })
            }
        }

        job_titles
    }

    fn load_raw_content(&self) -> Result<String, CustomError> {
        let read_result = fs::read_to_string("./examples/indeed.html");

        match read_result {
            Ok(content) => Ok(content),
            Err(_) => Err(CustomError::FetchError("Error feching data".to_string())),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{indeed::IndeedProvider, providers::{JobScrapper, JobTitle, Regions}};
    use std::fs;
    use assert_matches::assert_matches;
    
    #[test]
    fn should_list_10_job_titles(){
        let raw_content = fs::read_to_string("./examples/indeed.html").unwrap();
        let indeed_scrapper = IndeedProvider::new(Regions::EC);
        
        let job_titles = indeed_scrapper.find_job_titles(&raw_content);
        
        assert_eq!(job_titles.len(), 10);
    }

    #[test]
    fn should_return_with_expected_format(){
        let raw_content = fs::read_to_string("./examples/indeed.html").unwrap();
        let indeed_scrapper = IndeedProvider::new(Regions::EC);
        
        let mut job_titles = indeed_scrapper.find_job_titles(&raw_content);
        
        let _test_job_title = JobTitle {
            name: "Senior React Developer".to_string(),
            description: " \n \"Revolutionizing our customer's growth by creating scalable and sustainable technology solutions.\".\n Skills: +5 years of experience in SSR (Server-side rendering), React 18, Tailwind CSS..\n".to_string(),
            url: "/rc/clk?jk=f78c499b35095c96&from=hp&tk=1hvci83ko20hu001&bb=Y5BbYwEvmICV5hiB-6Kap8a5a1siCRW3nqxtqvkatux5NlKe4XUOIsH2iskyswfunKfouVMfP7u8gDyiZ-L8GO9rdYrsJF5c6QcT-N-W9IBPAvPceMc6xbZaPvyHD9pY&xkcb=SoBj67M3Assvogw39B0BbzkdCdPP".to_string(),
            salary_info: Some("$2.060 - $3.090 por mes".to_string()),
        };

        let first_jobtitle = job_titles.pop().unwrap();

        assert_matches!(first_jobtitle, _test_job_title);
    }
}