pub trait AiSubsystem: Send + Sync {
    fn provider_name(&self) -> &str;
}

pub trait RenderSubsystem: Send + Sync {
    fn backend_name(&self) -> &str;
}

pub trait ScriptSubsystem: Send + Sync {
    fn engine_name(&self) -> &str;
}
