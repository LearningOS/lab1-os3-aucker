//! Types related to task management

use super::TaskContext;

#[derive(Clone, Copy)]
// take control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    // add whatever you need in Lab1
}

#[derive(Copy, Clone, PartialEq)]
// Task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}