
use sqlx::{Postgres, Pool};
use tokio_cron_scheduler::{Job, JobSchedulerError};

use crate::errors::BootError;

pub struct CronJob {}
impl CronJob {
    pub fn job(&self, p: Pool<Postgres>) -> Result<Job, BootError> {
        let job = Job::new_async("* * * * * * *", move |_, mut __| {
            let pool = p.clone();
            Box::pin(async move {
                let row: (i32,) = sqlx::query_as(
                    r#"
                        SELECT 1234;
                    "#
                ).fetch_one(&pool).await.unwrap();
                let u = usize::try_from(row.0).unwrap();
                println!("Hello! {:?}", u)
            })
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
