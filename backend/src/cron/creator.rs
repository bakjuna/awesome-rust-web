use std::sync::Arc;

use tokio_cron_scheduler::JobScheduler;

use crate::errors::BootError;
use crate::cron::handler::CronJob;


pub async fn create_cron_jobs() -> Result<JobScheduler, BootError> {
    let sched = JobScheduler::new().await;
    if sched.is_err() {
        return Err(BootError::CronJobInit);
    }
    let sched = sched.unwrap();

    let cron_job = Arc::new(CronJob {} );
    let res = sched
        .add(cron_job.job().unwrap())
        .await;
    if res.is_err() {
        return Err(BootError::CronJobRun);
    }
    Ok(sched)
}