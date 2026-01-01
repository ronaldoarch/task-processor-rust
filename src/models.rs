use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub duration_ms: u64,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

impl Task {
    pub fn new(name: String, duration_ms: u64, priority: TaskPriority) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            status: TaskStatus::Pending,
            priority,
            duration_ms,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            error_message: None,
        }
    }

    pub fn mark_as_processing(&mut self) {
        self.status = TaskStatus::Processing;
        self.started_at = Some(Utc::now());
    }

    pub fn mark_as_completed(&mut self) {
        self.status = TaskStatus::Completed;
        self.completed_at = Some(Utc::now());
    }

    pub fn mark_as_failed(&mut self, error: String) {
        self.status = TaskStatus::Failed;
        self.completed_at = Some(Utc::now());
        self.error_message = Some(error);
    }

    pub fn mark_as_cancelled(&mut self) {
        self.status = TaskStatus::Cancelled;
        self.completed_at = Some(Utc::now());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub duration_ms: u64,
    pub priority: TaskPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStats {
    pub total_tasks: u64,
    pub pending: u64,
    pub processing: u64,
    pub completed: u64,
    pub failed: u64,
    pub cancelled: u64,
    pub average_processing_time_ms: f64,
}

#[derive(Debug)]
pub struct Stats {
    total_tasks: AtomicU64,
    pending: AtomicU64,
    processing: AtomicU64,
    completed: AtomicU64,
    failed: AtomicU64,
    cancelled: AtomicU64,
    total_processing_time_ms: AtomicU64,
    completed_count: AtomicU64,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            total_tasks: AtomicU64::new(0),
            pending: AtomicU64::new(0),
            processing: AtomicU64::new(0),
            completed: AtomicU64::new(0),
            failed: AtomicU64::new(0),
            cancelled: AtomicU64::new(0),
            total_processing_time_ms: AtomicU64::new(0),
            completed_count: AtomicU64::new(0),
        }
    }

    pub fn increment_total(&self) {
        self.total_tasks.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_pending(&self) {
        self.pending.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_processing(&self) {
        self.processing.fetch_add(1, Ordering::Relaxed);
        self.pending.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn increment_completed(&self, processing_time_ms: u64) {
        self.completed.fetch_add(1, Ordering::Relaxed);
        self.processing.fetch_sub(1, Ordering::Relaxed);
        self.total_processing_time_ms
            .fetch_add(processing_time_ms, Ordering::Relaxed);
        self.completed_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_failed(&self) {
        self.failed.fetch_add(1, Ordering::Relaxed);
        self.processing.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn increment_cancelled(&self) {
        self.cancelled.fetch_add(1, Ordering::Relaxed);
        self.processing.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn decrement_pending(&self) {
        self.pending.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn get_stats(&self) -> TaskStats {
        let completed_count = self.completed_count.load(Ordering::Relaxed);
        let total_time = self.total_processing_time_ms.load(Ordering::Relaxed);
        let avg_time = if completed_count > 0 {
            total_time as f64 / completed_count as f64
        } else {
            0.0
        };

        TaskStats {
            total_tasks: self.total_tasks.load(Ordering::Relaxed),
            pending: self.pending.load(Ordering::Relaxed),
            processing: self.processing.load(Ordering::Relaxed),
            completed: self.completed.load(Ordering::Relaxed),
            failed: self.failed.load(Ordering::Relaxed),
            cancelled: self.cancelled.load(Ordering::Relaxed),
            average_processing_time_ms: avg_time,
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

