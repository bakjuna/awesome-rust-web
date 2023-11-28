use tokio_cron_scheduler::{Job, JobSchedulerError};

use crate::errors::BootError;

pub struct CronJob {}
impl CronJob {
    pub fn job(&self) -> Result<Job, BootError> {
        let job = Job::new("* * * * * * *", |_, __| {
            println!("Job is running")
        });
        Self::return_job(job)
    }

    fn return_job(job: Result<Job, JobSchedulerError>) -> Result<Job, BootError> {
        match job {
            Ok(value) => Ok(value),
            Err(err) => Self::cron_job_error_handler::<Job>(err),
        }
    }

    fn cron_job_error_handler<T>(err: JobSchedulerError) -> Result<T, BootError> {
        match err {
            tokio_cron_scheduler::JobSchedulerError::StartScheduler => {
                Err(BootError::CronJobInit)
            }
            _ => Err(BootError::CronJobRun),
        }
    }
}
