use std::fmt::Display;
use std::sync::{Arc, Mutex};

use leptos::prelude::*;

use super::_builder::ToastBuilder;
use super::_data::{ToastData, ToastId, ToastLevel};

#[derive(Clone, Debug)]
pub struct ToasterContext {
    stats: Arc<Mutex<ToasterStats>>,
    pub queue: RwSignal<Vec<ToastData>>,
}

#[derive(Clone, Default, Debug)]
struct ToasterStats {
    visible: u32,
    total: u64,
}

impl ToasterContext {
    pub fn toast(&self, builder: ToastBuilder) {
        let mut stats = self.stats.lock().expect("Failed to lock stats");
        let toast = builder.build(stats.total + 1);

        let mut queue = self.queue.get_untracked();
        queue.push(toast);
        self.queue.set(queue);
        stats.visible += 1;
        stats.total += 1;
    }

    pub fn info<T>(&self, message: T)
    where
        T: Display,
    {
        self.toast(ToastBuilder::new(message).with_level(ToastLevel::Info));
    }

    pub fn success<T>(&self, message: T)
    where
        T: Display,
    {
        self.toast(ToastBuilder::new(message).with_level(ToastLevel::Success));
    }

    pub fn warn<T>(&self, message: T)
    where
        T: Display,
    {
        self.toast(ToastBuilder::new(message).with_level(ToastLevel::Warn));
    }

    pub fn error<T>(&self, message: T)
    where
        T: Display,
    {
        self.toast(ToastBuilder::new(message).with_level(ToastLevel::Error));
    }

    pub fn clear(&self) {
        for toast in &self.queue.get_untracked() {
            toast.clear_signal.set(true);
        }
    }

    /// Removes the toast corresponding with the supplied `ToastId`.
    pub fn remove(&self, toast_id: ToastId) {
        let index = self
            .queue
            .get_untracked()
            .iter()
            .enumerate()
            .find(|(_, toast)| toast.id == toast_id)
            .map(|(index, _)| index);

        if let Some(index) = index {
            let mut queue = self.queue.get_untracked();
            queue.remove(index);
            self.queue.set(queue);

            self.stats.lock().expect("Failed to lock stats").visible -= 1;
        }
    }
}

impl Default for ToasterContext {
    fn default() -> Self {
        ToasterContext {
            stats: Arc::new(Mutex::new(ToasterStats::default())),
            queue: RwSignal::new(Vec::new()),
        }
    }
}
