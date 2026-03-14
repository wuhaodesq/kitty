#[derive(Debug, Clone)]
pub struct BrowserConfig {
    pub name: String,
    pub ai_enabled: bool,
    pub three_d_enabled: bool,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            name: "Kitty Browser".to_string(),
            ai_enabled: true,
            three_d_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineCapability {
    AiNative,
    ThreeDNative,
    MainstreamCompatibility,
}

#[derive(Debug)]
pub struct Browser {
    config: BrowserConfig,
}

impl Browser {
    pub fn new(config: BrowserConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &BrowserConfig {
        &self.config
    }

    pub fn capabilities(&self) -> [EngineCapability; 3] {
        [
            EngineCapability::AiNative,
            EngineCapability::ThreeDNative,
            EngineCapability::MainstreamCompatibility,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_ai_and_3d_enabled() {
        let config = BrowserConfig::default();
        assert!(config.ai_enabled);
        assert!(config.three_d_enabled);
    }
}
