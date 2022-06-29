use uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TaskStatus {
    Todo,
    Active,
    Complete,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TaskWorth {
    Low,
    Mid,
    High,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Task {
    title: String,
    status: TaskStatus,
    worth: TaskWorth,
    id: Uuid,
}

impl Task {
    pub fn new(title: String, worth: TaskWorth) -> Self {
        Self { 
            title, 
            status: TaskStatus::Todo,
            worth,
            id: Uuid::new_v4(),
        }
    }

    pub fn start(&mut self) {
        if self.status != TaskStatus::Todo {
            return;
        }

        self.status = TaskStatus::Active;
    }

    pub fn finish(&mut self) {
        if self.status != TaskStatus::Active {
            return;
        }

        self.status = TaskStatus::Complete;
    }
}