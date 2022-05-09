use alloc::collections::VecDeque;
use alloc::string::{String, ToString};
use hashbrown::HashMap;
use lazy_static::lazy_static;
use log::info;
use spin::Mutex;

#[cfg(target_arch = "riscv64")]
use crate::arch::riscv64;

#[cfg(target_arch = "aarch64")]
use crate::arch::aarch64;

lazy_static! {
    pub static ref TASK_MANAGER: Mutex<TaskManager> = Mutex::new(TaskManager::new());
}

pub trait ArchTaskManager {
    unsafe fn context_switch(&mut self, from: TaskId, to: TaskId);
}

pub type TaskId = usize;

pub enum TaskState {
    Running,
    Ready,
    Stop,
}

pub struct Task {
    id: TaskId,
    name: String,
    state: TaskState,
}

impl Task {
    pub fn new(name: &str, id: TaskId) -> Self {
        Self {
            id,
            name: name.to_string(),
            state: TaskState::Stop,
        }
    }

    pub fn update_state(&mut self, state: TaskState) {
        self.state = state;
    }
}

pub struct TaskManager {
    tasks: HashMap<TaskId, Task>,
    ready_queue: VecDeque<TaskId>,
    task_id: TaskId,
    running: TaskId,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            ready_queue: VecDeque::new(),
            task_id: 0,
            running: 0,
        }
    }

    pub fn next_task_id(&mut self) -> TaskId {
        let result = self.task_id;
        self.task_id += 1;
        result
    }

    pub fn init(&mut self) {
        info!("Initialize Task Manager");
        self.task_id = 0;
        self.running = self.create_task("kernel");
        self.tasks
            .get_mut(&self.running)
            .unwrap()
            .update_state(TaskState::Running);
    }

    pub unsafe fn schedule(&mut self) {
        if self.ready_queue.len() == 0 {
            return;
        }
        assert!(1 <= self.ready_queue.len());
        let next_running = self.ready_queue.pop_front().unwrap();
        let current_running = self.running;
        assert!(self.tasks.contains_key(&next_running));
        assert!(self.tasks.contains_key(&current_running));

        self.tasks
            .get_mut(&next_running)
            .unwrap()
            .update_state(TaskState::Running);
        self.tasks
            .get_mut(&current_running)
            .unwrap()
            .update_state(TaskState::Ready);

        self.ready_queue.push_back(current_running);
        self.running = next_running;
        // TASK_MANAGER must be unlocked because of the risk of deadlock.
        TASK_MANAGER.force_unlock();
        // Do context switch
        #[cfg(target_arch = "riscv64")]
        riscv64::task::ARCH_TASK_MANAGER
            .lock()
            .context_switch(current_running, next_running);
    }

    pub fn ready_task(&mut self, id: TaskId) {
        if !self.tasks.contains_key(&id) {
            panic!("Unknown Task ID: {}", id);
        }
        assert!(self.tasks.contains_key(&id));
        self.tasks
            .get_mut(&id)
            .unwrap()
            .update_state(TaskState::Ready);
        self.ready_queue.push_back(id);
    }

    pub fn create_task(&mut self, name: &str) -> TaskId {
        let task_id = self.next_task_id();
        let task = Task::new(name, task_id);
        self.tasks.insert(task_id, task);
        assert!(self.tasks.contains_key(&task_id));

        #[cfg(target_arch = "riscv64")]
        riscv64::task::ARCH_TASK_MANAGER
            .lock()
            .create_arch_task(task_id);

        task_id
    }
}