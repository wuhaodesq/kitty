#[derive(Debug, Clone)]
pub struct AiRuntime {
    provider: String,
}

impl AiRuntime {
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
        }
    }

    pub fn provider(&self) -> &str {
        &self.provider
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_keeps_provider_name() {
        let runtime = AiRuntime::new("local");
        assert_eq!(runtime.provider(), "local");
    }
}
