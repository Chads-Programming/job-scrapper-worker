mod jobs;
mod providers;
mod indeed;
fn main(){
    jobs::fetch_jobs();
}