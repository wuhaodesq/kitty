use kitty_3d::{Camera, Entity, Mesh, Pipeline, Scene};
use kitty_ai::{AiRuntime, EchoModel, InferenceRequest};
use kitty_compat::{BaselineChecker, SiteProfile};
use kitty_core::{AiSubsystem, Browser, BrowserConfig, RenderSubsystem, ScriptSubsystem};
use kitty_render::{DomNode, LayoutTree, Page};
use kitty_script::{ScriptRuntime, ScriptValue};
use kitty_webapp::{PageComponent, Route, WebApp};

struct AiAdapter {
    provider: String,
}

impl AiSubsystem for AiAdapter {
    fn provider_name(&self) -> &str {
        &self.provider
    }
}

struct RenderAdapter;

impl RenderSubsystem for RenderAdapter {
    fn backend_name(&self) -> &str {
        "kitty-render-layout-v0"
    }
}

struct ScriptAdapter;

impl ScriptSubsystem for ScriptAdapter {
    fn engine_name(&self) -> &str {
        "kitty-script-v0"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemoSummary {
    pub browser_name: String,
    pub ai_provider: String,
    pub ai_output: String,
    pub layout_boxes: usize,
    pub frame_entities: usize,
    pub frame_vertices: usize,
    pub compat_domain: String,
    pub compat_score: u8,
    pub webapp_home_component: String,
    pub script_mode: String,
}

pub fn run_demo() -> DemoSummary {
    let mut ai_runtime = AiRuntime::new("local");
    ai_runtime.register_model(EchoModel::new("echo-v1"));

    let ai_output = ai_runtime
        .infer("echo-v1", &InferenceRequest::new("hello"))
        .expect("ai inference should execute successfully");

    let browser = Browser::builder(BrowserConfig::default())
        .with_ai(AiAdapter {
            provider: ai_runtime.provider().to_string(),
        })
        .with_render(RenderAdapter)
        .with_script(ScriptAdapter)
        .build();

    let page = Page::new(
        "Kitty Home",
        DomNode::element("html")
            .with_child(DomNode::element("body").with_child(DomNode::text("h1", "Hello Kitty"))),
    );
    let layout = LayoutTree::from_dom(&page.root);

    let mut scene = Scene::default();
    scene.add_entity(Entity {
        id: 1,
        mesh: Mesh::triangle("demo-triangle"),
    });
    let frame = Pipeline::new("software-3d-v0").render(&scene, &Camera::default());

    let mut script = ScriptRuntime::new();
    let script_out = script
        .execute("set mode dev\nadd visits 1\nget mode")
        .expect("script should execute successfully");
    let script_mode = match script_out {
        Some(ScriptValue::Str(value)) => value,
        Some(ScriptValue::Number(value)) => value.to_string(),
        None => "<none>".to_string(),
    };

    let mut site = SiteProfile::new("example.com");
    site.requires_webgl2 = true;
    site.requires_webassembly = true;
    site.requires_service_worker = true;
    let compat = BaselineChecker::default().check(&site);

    let mut app = WebApp::new("kitty-demo");
    app.add_route(Route::new(
        "/",
        PageComponent::new("home", "<h1>Hello Kitty</h1>"),
    ));
    let home_component = app
        .resolve("/")
        .map(|r| r.component.name.as_str())
        .unwrap_or("missing")
        .to_string();

    let compat_score = compat.score();

    DemoSummary {
        browser_name: browser.config().name.clone(),
        ai_provider: browser.ai_provider().unwrap_or("unbound").to_string(),
        ai_output: ai_output.text,
        layout_boxes: layout.boxes.len(),
        frame_entities: frame.entities,
        frame_vertices: frame.vertices,
        compat_domain: compat.domain,
        compat_score,
        webapp_home_component: home_component,
        script_mode,
    }
}

pub fn print_summary(summary: &DemoSummary) {
    println!("Starting {}", summary.browser_name);
    println!("AI provider: {}", summary.ai_provider);
    println!("AI output: {}", summary.ai_output);
    println!("Layout boxes: {}", summary.layout_boxes);
    println!("3D frame entities: {}", summary.frame_entities);
    println!("3D frame vertices: {}", summary.frame_vertices);
    println!("Compat domain: {}", summary.compat_domain);
    println!("Compat score: {}", summary.compat_score);
    println!("WebApp route '/': {}", summary.webapp_home_component);
    println!("Script mode: {}", summary.script_mode);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_summary_has_expected_baseline_values() {
        let summary = run_demo();
        assert_eq!(summary.browser_name, "Kitty Browser");
        assert_eq!(summary.ai_provider, "local");
        assert_eq!(summary.ai_output, "echo:hello");
        assert_eq!(summary.layout_boxes, 3);
        assert_eq!(summary.frame_entities, 1);
        assert_eq!(summary.frame_vertices, 3);
        assert_eq!(summary.compat_domain, "example.com");
        assert_eq!(summary.compat_score, 4);
        assert_eq!(summary.webapp_home_component, "home");
        assert_eq!(summary.script_mode, "dev");
    }
}
