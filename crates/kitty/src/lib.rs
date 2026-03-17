use kitty_3d::{Camera, Entity, FrameStats, Mesh, Pipeline, Scene};
use kitty_ai::{AiRuntime, AiRuntimeError, EchoModel, InferenceRequest};
use kitty_compat::{BaselineChecker, CompatibilityReport, SiteProfile};
use kitty_render::{DomNode, LayoutTree};
use kitty_script::{ScriptError, ScriptRuntime, ScriptValue};
use kitty_webapp::{PageComponent, Route, WebApp};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KittySdkError {
    MissingEchoModel,
}

pub struct KittySdk {
    ai: AiRuntime,
    script: ScriptRuntime,
    compat: BaselineChecker,
    pipeline: Pipeline,
}

impl Default for KittySdk {
    fn default() -> Self {
        Self::new()
    }
}

impl KittySdk {
    pub fn new() -> Self {
        Self {
            ai: AiRuntime::new("local"),
            script: ScriptRuntime::new(),
            compat: BaselineChecker::default(),
            pipeline: Pipeline::new("software-3d-v0"),
        }
    }

    pub fn register_echo_model(&mut self) {
        self.ai.register_model(EchoModel::new("echo-v1"));
    }

    pub fn infer_echo(&self, prompt: &str) -> Result<String, KittySdkError> {
        let output = self
            .ai
            .infer("echo-v1", &InferenceRequest::new(prompt))
            .map_err(|err| match err {
                AiRuntimeError::ModelNotFound(_) => KittySdkError::MissingEchoModel,
            })?;
        Ok(output.text)
    }

    pub fn run_script(&mut self, source: &str) -> Result<Option<ScriptValue>, ScriptError> {
        self.script.execute(source)
    }

    pub fn build_layout_box_count(&self) -> usize {
        let root = DomNode::element("html")
            .with_child(DomNode::element("body").with_child(DomNode::text("h1", "Kitty SDK")));
        let layout = LayoutTree::from_dom(&root);
        layout.boxes.len()
    }

    pub fn render_triangle_frame(&self) -> FrameStats {
        let mut scene = Scene::default();
        scene.add_entity(Entity {
            id: 1,
            mesh: Mesh::triangle("sdk-triangle"),
        });
        self.pipeline.render(&scene, &Camera::default())
    }

    pub fn check_site(&self, site: &SiteProfile) -> CompatibilityReport {
        self.compat.check(site)
    }

    pub fn create_webapp_home_component_name(&self) -> String {
        let mut app = WebApp::new("kitty-app");
        app.add_route(Route::new(
            "/",
            PageComponent::new("home", "<h1>Kitty SDK</h1>"),
        ));
        app.resolve("/")
            .map(|r| r.component.name.clone())
            .unwrap_or_else(|| "missing".to_string())
    }

    pub fn resolve_webapp_user_route(&self, path: &str) -> Option<String> {
        let mut app = WebApp::new("kitty-app");
        app.add_route(Route::new(
            "/users/:id",
            PageComponent::new("user-profile", "<h1>User</h1>"),
        ));

        let resolved = app.resolve_with_params(path)?;
        resolved.param("id").map(str::to_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sdk_can_run_baseline_flow() {
        let mut sdk = KittySdk::new();
        sdk.register_echo_model();

        let ai = sdk.infer_echo("hello").expect("ai infer should work");
        assert_eq!(ai, "echo:hello");

        let script = sdk
            .run_script("set mode sdk\nget mode")
            .expect("script should work");
        assert_eq!(script, Some(ScriptValue::Str("sdk".to_string())));

        assert_eq!(sdk.build_layout_box_count(), 3);

        let frame = sdk.render_triangle_frame();
        assert_eq!(frame.entities, 1);
        assert_eq!(frame.vertices, 3);

        let mut site = SiteProfile::new("sdk.dev");
        site.requires_webgl2 = true;
        site.requires_webassembly = true;
        site.requires_service_worker = true;
        let report = sdk.check_site(&site);
        assert_eq!(report.score(), 4);

        assert_eq!(sdk.create_webapp_home_component_name(), "home");
        assert_eq!(
            sdk.resolve_webapp_user_route("/users/42"),
            Some("42".to_string())
        );
        assert_eq!(
            sdk.resolve_webapp_user_route("/users/42?tab=profile"),
            Some("42".to_string())
        );
    }

    #[test]
    fn sdk_reports_missing_model() {
        let sdk = KittySdk::new();
        let err = sdk.infer_echo("hello").unwrap_err();
        assert_eq!(err, KittySdkError::MissingEchoModel);
    }
}
