use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InferenceRequest {
    pub prompt: String,
}

impl InferenceRequest {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InferenceResponse {
    pub text: String,
}

pub trait Model: Send + Sync {
    fn name(&self) -> &str;
    fn infer(&self, request: &InferenceRequest) -> InferenceResponse;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiRuntimeError {
    ModelNotFound(String),
}

#[derive(Debug, Clone)]
pub struct EchoModel {
    name: String,
}

impl EchoModel {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl Model for EchoModel {
    fn name(&self) -> &str {
        &self.name
    }

    fn infer(&self, request: &InferenceRequest) -> InferenceResponse {
        InferenceResponse {
            text: format!("echo:{}", request.prompt),
        }
    }
}

pub struct AiRuntime {
    provider: String,
    models: HashMap<String, Box<dyn Model>>,
}

impl AiRuntime {
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            models: HashMap::new(),
        }
    }

    pub fn provider(&self) -> &str {
        &self.provider
    }

    pub fn register_model(&mut self, model: impl Model + 'static) {
        self.models.insert(model.name().to_string(), Box::new(model));
    }

    pub fn has_model(&self, name: &str) -> bool {
        self.models.contains_key(name)
    }

    pub fn infer(
        &self,
        model_name: &str,
        request: &InferenceRequest,
    ) -> Result<InferenceResponse, AiRuntimeError> {
        let model = self
            .models
            .get(model_name)
            .ok_or_else(|| AiRuntimeError::ModelNotFound(model_name.to_string()))?;
        Ok(model.infer(request))
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

    #[test]
    fn runtime_can_register_and_infer_model() {
        let mut runtime = AiRuntime::new("local");
        runtime.register_model(EchoModel::new("echo-v1"));

        let out = runtime
            .infer("echo-v1", &InferenceRequest::new("kitty"))
            .unwrap();
        assert_eq!(out.text, "echo:kitty");
    }

    #[test]
    fn runtime_returns_error_when_model_missing() {
        let runtime = AiRuntime::new("local");
        let err = runtime
            .infer("missing", &InferenceRequest::new("kitty"))
            .unwrap_err();
        assert_eq!(err, AiRuntimeError::ModelNotFound("missing".to_string()));
    }
}
