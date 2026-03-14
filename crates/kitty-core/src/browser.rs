use crate::subsystem::{AiSubsystem, RenderSubsystem, ScriptSubsystem};

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

pub struct Browser {
    config: BrowserConfig,
    ai: Option<Box<dyn AiSubsystem>>,
    render: Option<Box<dyn RenderSubsystem>>,
    script: Option<Box<dyn ScriptSubsystem>>,
}

impl Browser {
    pub fn new(config: BrowserConfig) -> Self {
        Self {
            config,
            ai: None,
            render: None,
            script: None,
        }
    }

    pub fn builder(config: BrowserConfig) -> BrowserBuilder {
        BrowserBuilder::new(config)
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

    pub fn ai_provider(&self) -> Option<&str> {
        self.ai.as_ref().map(|ai| ai.provider_name())
    }

    pub fn render_backend(&self) -> Option<&str> {
        self.render.as_ref().map(|render| render.backend_name())
    }

    pub fn script_engine(&self) -> Option<&str> {
        self.script.as_ref().map(|script| script.engine_name())
    }
}

pub struct BrowserBuilder {
    config: BrowserConfig,
    ai: Option<Box<dyn AiSubsystem>>,
    render: Option<Box<dyn RenderSubsystem>>,
    script: Option<Box<dyn ScriptSubsystem>>,
}

impl BrowserBuilder {
    pub fn new(config: BrowserConfig) -> Self {
        Self {
            config,
            ai: None,
            render: None,
            script: None,
        }
    }

    pub fn with_ai(mut self, ai: impl AiSubsystem + 'static) -> Self {
        self.ai = Some(Box::new(ai));
        self
    }

    pub fn with_render(mut self, render: impl RenderSubsystem + 'static) -> Self {
        self.render = Some(Box::new(render));
        self
    }

    pub fn with_script(mut self, script: impl ScriptSubsystem + 'static) -> Self {
        self.script = Some(Box::new(script));
        self
    }

    pub fn build(self) -> Browser {
        Browser {
            config: self.config,
            ai: self.ai,
            render: self.render,
            script: self.script,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subsystem::{AiSubsystem, RenderSubsystem, ScriptSubsystem};

    #[test]
    fn default_config_has_ai_and_3d_enabled() {
        let config = BrowserConfig::default();
        assert!(config.ai_enabled);
        assert!(config.three_d_enabled);
    }

    #[test]
    fn builder_registers_subsystems() {
        struct Ai;
        impl AiSubsystem for Ai {
            fn provider_name(&self) -> &str {
                "local"
            }
        }

        struct Render;
        impl RenderSubsystem for Render {
            fn backend_name(&self) -> &str {
                "software"
            }
        }

        struct Script;
        impl ScriptSubsystem for Script {
            fn engine_name(&self) -> &str {
                "kitty-script"
            }
        }

        let browser = Browser::builder(BrowserConfig::default())
            .with_ai(Ai)
            .with_render(Render)
            .with_script(Script)
            .build();

        assert_eq!(browser.ai_provider(), Some("local"));
        assert_eq!(browser.render_backend(), Some("software"));
        assert_eq!(browser.script_engine(), Some("kitty-script"));
    }
}
