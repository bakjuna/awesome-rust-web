use crate::{database::ConnectionPool, env::Env, errors::BootError};
use shaku::{Component, Interface};
use std::{ops::Deref, sync::Arc};
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

pub trait CronJobInterface: Interface {
    fn initialize(self: Arc<Self>);
    fn cronjob_1(self: Arc<Self>) -> Result<Job, JobSchedulerError>;
    fn cronjob_2(self: Arc<Self>) -> Result<Job, JobSchedulerError>;
}
fn create_scheduler() -> Arc<JobScheduler> {
    let r = tokio::spawn(async { return JobScheduler::new().await });
    let sched = futures::executor::block_on(r).unwrap();
    if sched.is_err() {
        panic!("{}", BootError::CronJobInit);
    }
    let sched = sched.unwrap();
    Arc::new(sched)
}
#[derive(Component)]
#[shaku(interface = CronJobInterface)]
pub struct CronJobComponent {
    #[shaku(default=create_scheduler())]
    sched: Arc<JobScheduler>,
    #[shaku(inject)]
    db: Arc<dyn ConnectionPool>,
    #[shaku(inject)]
    env: Arc<dyn Env>,
}

impl CronJobInterface for CronJobComponent {
    fn initialize(self: Arc<Self>) {
        let scheduler = Arc::clone(&self.sched);
        let cronjobs = Arc::clone(&self);
        let cronjob_1 = cronjobs
            .clone()
            .cronjob_1()
            .expect("Failed to create cronjob_1");
        let cronjob_2 = cronjobs
            .clone()
            .cronjob_2()
            .expect("Failed to create cronjob_2");
        futures::executor::block_on(async {
            scheduler.add(cronjob_1).await.unwrap();
            scheduler.add(cronjob_2).await.unwrap();
            scheduler.start().await.unwrap();
        });
    }
    fn cronjob_1(self: Arc<Self>) -> Result<Job, JobSchedulerError> {
        Job::new_async("* * * * * * *", move |_, mut __| {
            let pool = Arc::clone(&self.db.get().0);
            let env = self.env.get();
            Box::pin(async move {
                let row: (i32,) = sqlx::query_as(
                    r#"
                        SELECT 1234;
                    "#,
                )
                .fetch_one(pool.deref())
                .await
                .unwrap();
                // println!("1: db testing {:?}", row.0);
                // println!("1: env testing {:?}", env.server.address);
            })
        })
    }
    fn cronjob_2(self: Arc<Self>) -> Result<Job, JobSchedulerError> {
        Job::new_async("*/2 * * * * * *", move |_, mut __| {
            let pool = Arc::clone(&self.db.get().0);
            let env = self.env.get();
            Box::pin(async move {
                let row: (i32,) = sqlx::query_as(
                    r#"
                        SELECT 1234;
                    "#,
                )
                .fetch_one(pool.deref())
                .await
                .unwrap();
                // println!("Hello cronjob 2! {:?}", row.0);
                // println!("1: env testing {:?}", env.server.address);
            })
        })
    }
}
