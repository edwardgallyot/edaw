use self::load_task::SampleLoadTask;

mod load_task;

pub struct SampleLoader {
    load_tasks: Vec<SampleLoadTask>,
}

impl SampleLoader {
    pub fn new() -> SampleLoader {
        let load_tasks = vec![];
        SampleLoader {
            load_tasks,
        }
    }
}

