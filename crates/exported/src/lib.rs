pub trait Exported {
    fn export() -> serde_json::Value;
}
