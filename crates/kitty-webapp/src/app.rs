#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageComponent {
    pub name: String,
    pub template: String,
}

impl PageComponent {
    pub fn new(name: impl Into<String>, template: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            template: template.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route {
    pub path: String,
    pub component: PageComponent,
}

impl Route {
    pub fn new(path: impl Into<String>, component: PageComponent) -> Self {
        Self {
            path: path.into(),
            component,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WebApp {
    pub name: String,
    pub routes: Vec<Route>,
}

impl WebApp {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            routes: Vec::new(),
        }
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn resolve(&self, path: &str) -> Option<&Route> {
        self.routes.iter().find(|r| r.path == path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_register_and_resolve_routes() {
        let mut app = WebApp::new("kitty-demo");
        app.add_route(Route::new(
            "/",
            PageComponent::new("home", "<h1>Kitty</h1>"),
        ));

        let route = app.resolve("/").expect("route should exist");
        assert_eq!(route.component.name, "home");
        assert_eq!(route.component.template, "<h1>Kitty</h1>");
    }
}
