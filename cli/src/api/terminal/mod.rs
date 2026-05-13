mod error;

pub use error::TerminalError;

use ratatui::{
    Frame, TerminalOptions, Viewport,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph},
};
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

const FRAME_DURATION: Duration = Duration::from_millis(16); // ~60fps

#[derive(Clone, PartialEq)]
enum TaskStatus {
    Pending,
    Running,
    Success(Option<String>),
    Failed(Option<String>),
}

#[derive(Clone)]
struct Task {
    name: String,
    status: TaskStatus,
}

/// Updates that can be queued for the render loop to process
enum StateUpdate {
    AddTask {
        name: String,
    },
    UpdateTask {
        name: String,
    },
    FinishTask {
        name: String,
        message: Option<String>,
        is_success: bool,
    },
    AddMessage {
        message: String,
    },
    SetFinalMessage {
        message: String,
    },
}

struct TerminalState {
    title: String,
    tasks: HashMap<String, usize>,
    task_list: Vec<Task>,
    messages: Vec<String>,
    max_messages: usize,
    final_message: Option<String>,
}

pub(crate) struct Terminal {
    #[allow(dead_code)]
    state: Arc<Mutex<TerminalState>>,
    queue: Arc<Mutex<VecDeque<StateUpdate>>>,
    stop_signal: Arc<AtomicBool>,
    render_handle: Option<JoinHandle<()>>,
}

impl Terminal {
    /// Create a terminal with auto-calculated viewport height.
    ///
    /// `task_count` - number of tasks that will be added
    /// `max_messages` - maximum number of log messages to display (default: 5)
    pub(crate) fn new(title: impl AsRef<str>, task_count: usize, max_messages: usize) -> Self {
        // Calculate height: title(1) + separator(1) + tasks + separator(1) + messages + separator(1) + final(1)
        let height = 2 + task_count + 1 + max_messages + 1 + 1;
        Self::with_options(title, height as u16, max_messages)
    }

    pub(crate) fn with_options(title: impl AsRef<str>, height: u16, max_messages: usize) -> Self {
        let state = Arc::new(Mutex::new(TerminalState {
            title: title.as_ref().to_string(),
            tasks: HashMap::new(),
            task_list: Vec::new(),
            messages: Vec::new(),
            max_messages,
            final_message: None,
        }));

        let queue = Arc::new(Mutex::new(VecDeque::new()));
        let stop_signal = Arc::new(AtomicBool::new(false));

        // Clone Arcs for the render thread
        let state_clone = Arc::clone(&state);
        let queue_clone = Arc::clone(&queue);
        let stop_clone = Arc::clone(&stop_signal);

        // Spawn the background render loop
        let render_handle = thread::spawn(move || {
            // Initialize ratatui in the render thread (it owns the terminal)
            let mut inner = ratatui::init_with_options(TerminalOptions {
                viewport: Viewport::Inline(height),
            });

            loop {
                thread::sleep(FRAME_DURATION);

                if stop_clone.load(Ordering::SeqCst) {
                    break;
                }

                // Process one update from the queue
                let update = queue_clone.lock().unwrap().pop_front();

                if let Some(update) = update {
                    let mut state = state_clone.lock().unwrap();
                    apply_update(&mut state, update);

                    // Draw after applying update
                    let title = state.title.clone();
                    let tasks = state.task_list.clone();
                    let messages = state.messages.clone();
                    let final_message = state.final_message.clone();
                    drop(state);

                    inner
                        .draw(|frame| {
                            render(frame, &title, &tasks, &messages, final_message.as_deref());
                        })
                        .ok();
                }
            }

            // Final cleanup - restore terminal
            ratatui::restore();
        });

        Self {
            state,
            queue,
            stop_signal,
            render_handle: Some(render_handle),
        }
    }

    pub(crate) fn add_task(&self, name: impl AsRef<str>) {
        self.queue.lock().unwrap().push_back(StateUpdate::AddTask {
            name: name.as_ref().to_string(),
        });
    }

    pub(crate) fn update_task(&self, name: impl AsRef<str>) {
        self.queue.lock().unwrap().push_back(StateUpdate::UpdateTask {
            name: name.as_ref().to_string(),
        });
    }

    pub(crate) fn finish_task(&self, name: impl AsRef<str>, message: Option<&str>, is_success: bool) {
        self.queue.lock().unwrap().push_back(StateUpdate::FinishTask {
            name: name.as_ref().to_string(),
            message: message.map(String::from),
            is_success,
        });
    }

    pub(crate) fn add_message(&self, message: &str) -> Result<(), TerminalError> {
        self.queue.lock().unwrap().push_back(StateUpdate::AddMessage {
            message: message.to_string(),
        });
        Ok(())
    }

    pub(crate) fn finish(mut self, message: Option<&str>) -> Result<(), TerminalError> {
        // Queue the final message so it gets rendered
        if let Some(msg) = message {
            self.queue.lock().unwrap().push_back(StateUpdate::SetFinalMessage {
                message: msg.to_string(),
            });
        }

        // Wait for queue to drain (including the final message)
        loop {
            if self.queue.lock().unwrap().is_empty() {
                break;
            }
            thread::sleep(Duration::from_millis(5));
        }

        // Signal render loop to stop
        self.stop_signal.store(true, Ordering::SeqCst);

        // Wait for render thread to finish
        if let Some(handle) = self.render_handle.take() {
            handle.join().ok();
        }

        // Prevent Drop from running (render thread already cleaned up)
        std::mem::forget(self);
        Ok(())
    }
}

/// Apply a state update to the terminal state
fn apply_update(state: &mut TerminalState, update: StateUpdate) {
    match update {
        StateUpdate::AddTask { name } => {
            let idx = state.task_list.len();
            state.tasks.insert(name.clone(), idx);
            state.task_list.push(Task {
                name,
                status: TaskStatus::Pending,
            });
        }
        StateUpdate::UpdateTask { name } => {
            if let Some(&idx) = state.tasks.get(&name)
                && let Some(task) = state.task_list.get_mut(idx)
            {
                task.status = TaskStatus::Running;
            }
        }
        StateUpdate::FinishTask {
            name,
            message,
            is_success,
        } => {
            if let Some(&idx) = state.tasks.get(&name)
                && let Some(task) = state.task_list.get_mut(idx)
            {
                task.status = if is_success {
                    TaskStatus::Success(message)
                } else {
                    TaskStatus::Failed(message)
                };
            }
        }
        StateUpdate::AddMessage { message } => {
            state.messages.push(message);
            if state.messages.len() > state.max_messages {
                state.messages.remove(0);
            }
        }
        StateUpdate::SetFinalMessage { message } => {
            state.final_message = Some(message);
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        // Signal render thread to stop if it's still running
        self.stop_signal.store(true, Ordering::SeqCst);

        // Wait for render thread (it handles ratatui::restore())
        if let Some(handle) = self.render_handle.take() {
            handle.join().ok();
        }
    }
}

/// Creates a labeled separator line: "── label ────────────────"
fn separator_line(label: &str, width: usize) -> Line<'static> {
    let label_with_spaces = format!(" {} ", label);
    let prefix = "──";
    let remaining = width.saturating_sub(prefix.len() + label_with_spaces.len());
    let suffix = "─".repeat(remaining);

    Line::from(vec![
        Span::styled(prefix, Style::default().fg(Color::DarkGray)),
        Span::styled(label_with_spaces, Style::default().fg(Color::DarkGray)),
        Span::styled(suffix, Style::default().fg(Color::DarkGray)),
    ])
}

fn render(frame: &mut Frame, title: &str, tasks: &[Task], messages: &[String], final_message: Option<&str>) {
    let area = frame.area();
    let width = area.width as usize;

    let has_messages = !messages.is_empty();
    let has_final = final_message.is_some();
    let title_height = 2u16; // title + separator
    let tasks_height = tasks.len() as u16;
    let messages_height = if has_messages { messages.len() as u16 + 1 } else { 0 }; // +1 for separator
    let final_height = if has_final { 2 } else { 0 }; // separator + message

    let chunks = Layout::vertical([
        Constraint::Length(title_height),
        Constraint::Length(tasks_height),
        Constraint::Length(messages_height),
        Constraint::Length(final_height),
        Constraint::Min(0),
    ])
    .split(area);

    // Render title
    let title_lines = vec![
        Line::from(Span::styled(
            title,
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        )),
        separator_line("tasks", width),
    ];
    frame.render_widget(Paragraph::new(title_lines), chunks[0]);

    // Render tasks
    let task_items: Vec<ListItem> = tasks
        .iter()
        .map(|task| {
            let (icon, icon_style, msg) = match &task.status {
                TaskStatus::Pending => ("○", Style::default().fg(Color::DarkGray), None),
                TaskStatus::Running => ("◐", Style::default().fg(Color::Yellow), None),
                TaskStatus::Success(m) => ("✓", Style::default().fg(Color::Green), m.as_ref()),
                TaskStatus::Failed(m) => ("✗", Style::default().fg(Color::Red), m.as_ref()),
            };

            let mut spans = vec![Span::styled(format!("{} ", icon), icon_style), Span::raw(&task.name)];

            if let Some(m) = msg {
                spans.push(Span::styled(format!(" - {}", m), Style::default().fg(Color::DarkGray)));
            }

            ListItem::new(Line::from(spans))
        })
        .collect();

    frame.render_widget(List::new(task_items), chunks[1]);

    // Render messages
    if has_messages {
        let message_lines: Vec<Line> = std::iter::once(separator_line("logs", width))
            .chain(
                messages
                    .iter()
                    .map(|m| Line::from(Span::styled(m.as_str(), Style::default().fg(Color::Cyan)))),
            )
            .collect();

        frame.render_widget(Paragraph::new(message_lines), chunks[2]);
    }

    // Render final message with separator
    if let Some(msg) = final_message {
        let final_lines = vec![
            separator_line("status", width),
            Line::from(Span::styled(msg, Style::default().fg(Color::Green))),
        ];
        frame.render_widget(Paragraph::new(final_lines), chunks[3]);
    }
}
