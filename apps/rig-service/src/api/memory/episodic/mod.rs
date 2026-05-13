#[derive(Debug, Clone)]
pub(crate) struct EpisodicMemory {
    pub(crate) pid: String,
    pub(crate) role: String,
    pub(crate) content: String,
    pub(crate) similarity: f32,
    pub(crate) created_at: chrono::NaiveDateTime,
}
