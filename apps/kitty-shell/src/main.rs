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

fn main() {
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
        .unwrap_or("missing");

    println!("Starting {}", browser.config().name);
    println!("Capabilities: {:?}", browser.capabilities());
    println!("AI provider: {}", browser.ai_provider().unwrap_or("unbound"));
    println!("AI output: {}", ai_output.text);
    println!("Render backend: {}", browser.render_backend().unwrap_or("unbound"));
    println!("Script engine: {}", browser.script_engine().unwrap_or("unbound"));
    println!("Render page: {}", page.title);
    println!("Layout boxes: {}", layout.boxes.len());
    println!("3D frame entities: {}", frame.entities);
    println!("3D frame vertices: {}", frame.vertices);
    println!("Compat domain: {}", compat.domain);
    println!("Compat score: {}", compat.score());
    println!("WebApp route '/': {}", home_component);
    match script_out {
        Some(ScriptValue::Str(value)) => println!("Script mode: {}", value),
        Some(ScriptValue::Number(value)) => println!("Script output number: {}", value),
        None => println!("Script output: <none>"),
    }
}
