use uuid::Uuid;

use crate::{task::{Task, TaskWorth}};
use crate::errors::{TodoErrors, Result};


pub struct Todos {
    tasks: Vec<Task>,
    max_active: usize,
}

impl Todos {
    pub fn new(max_active: usize) -> Self {
        Todos { tasks: vec![], max_active }
    }

    pub fn get(&self) -> &Vec<Task> {
        return &self.tasks
    }

    pub fn update_max_active(&mut self, max_active: usize) -> Result<()> {
        if &self.calculate_active() > &max_active {
            return Err(TodoErrors::LowerMaxValue)
        }

        Ok(())
    }

    pub fn add(&mut self, title: String, point: usize) {
        let mut worth = TaskWorth::High;

        match point {
            1 => { worth = TaskWorth::Low },
            2 => { worth = TaskWorth::Mid }
            _ => {}
        }

        let task = Task::new(title, worth);
        self.tasks.push(task);
    }

    pub fn remove(&mut self, id: Uuid) {
        let task_index = self.tasks.iter().position(|x| x.id == id);

        if let Some(index) = task_index {
            self.tasks.remove(index);
        }
    }

    pub fn start(&mut self, id: Uuid) -> Result<()> {
        let task_index = self.tasks.iter().position(|x| x.id == id);

        if let Some(index) = task_index {
            let task_worth = self.tasks[index].worth.to_string().parse::<usize>().unwrap();
            
            if task_worth + self.calculate_active() > self.max_active {
                return Err(TodoErrors::MaxReached(String::from("Adding this task would exceed your maximum active tasks, complete one of your active tasks or increase or max active tasks")))
            }

            self.tasks[index].start();
        }

        Ok(())
    }

    pub fn finish(&mut self, id: Uuid) {
        let task_index = self.tasks.iter().position(|x| x.id == id);

        if let Some(index) = task_index {
            self.tasks[index].finish();
        }
    }

    pub fn calculate_active(&self) -> usize {
        let mut total = 0;

        self.tasks.iter().for_each(|x| {
            total += x.worth.to_string().parse::<usize>().unwrap();
        });

        total
    }
}