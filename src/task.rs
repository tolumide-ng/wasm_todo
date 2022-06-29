use std::fmt::Display;

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

impl Display for TaskWorth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Low => { write!(f, "1") },
            Self::Mid => { write!(f, "2") }
            Self::High => { write!(f, "3") },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Task {
    title: String,
    status: TaskStatus,
    pub(crate) worth: TaskWorth,
    pub(crate) id: Uuid,
}

impl Task {
    pub(crate) fn new(title: String, worth: TaskWorth) -> Self {
        Self { 
            title, 
            status: TaskStatus::Todo,
            worth,
            id: Uuid::new_v4(),
        }
    }

    pub(crate) fn start(&mut self) {
        if self.status != TaskStatus::Todo {
            return;
        }

        self.status = TaskStatus::Active;
    }

    pub(crate) fn finish(&mut self) {
        if self.status != TaskStatus::Active {
            return;
        }

        self.status = TaskStatus::Complete;
    }
}