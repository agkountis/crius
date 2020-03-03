use legion::schedule::{Builder, Runnable, Schedulable, Schedule};
use legion::world::World;
use std::borrow::BorrowMut;

pub struct ExecutionSchedule {
    schedule: Schedule,
}

pub struct ExecutionScheduleBuilder {
    schedule_builder: Builder,
}

impl ExecutionScheduleBuilder {
    pub fn new() -> Self {
        Self {
            schedule_builder: Schedule::builder(),
        }
    }

    pub fn with_system<S>(mut self, system: S) -> Self
    where
        S: Into<Box<dyn Schedulable + 'static>>,
    {
        self.schedule_builder = self.schedule_builder.add_system(system);
        self
    }

    pub fn with_thread_local<S>(mut self, system: S) -> Self
    where
        S: Into<Box<dyn Runnable + 'static>>,
    {
        self.schedule_builder = self.schedule_builder.add_thread_local(system);
        self
    }

    pub fn with_thread_local_fn<F>(mut self, function: F) -> Self
    where
        F: FnMut(&mut World) + 'static,
    {
        self.schedule_builder = self.schedule_builder.add_thread_local_fn(function);
        self
    }

    pub fn barrier(mut self) -> Self {
        self.schedule_builder = self.schedule_builder.flush();
        self
    }

    pub fn build(mut self) -> Schedule {
        self.schedule_builder.build()
    }
}
