use std::collections::HashSet;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedRoute<'a> {
    pub route: &'a Route,
    pub params: Vec<(String, String)>,
}

impl<'a> ResolvedRoute<'a> {
    pub fn param(&self, name: &str) -> Option<&str> {
        self.params
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MatchResult {
    params: Vec<(String, String)>,
    static_segments: usize,
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

    pub fn resolve_with_params(&self, request_path: &str) -> Option<ResolvedRoute<'_>> {
        let normalized = request_path.split('?').next().unwrap_or_default();
        let request_segments = split_path(normalized);

        let mut best: Option<ResolvedRoute<'_>> = None;
        let mut best_static_segments = 0;

        for route in &self.routes {
            let route_segments = split_path(&route.path);
            let Some(result) = match_route(&route_segments, &request_segments) else {
                continue;
            };

            if best.is_none() || result.static_segments > best_static_segments {
                best_static_segments = result.static_segments;
                best = Some(ResolvedRoute {
                    route,
                    params: result.params,
                });
            }
        }

        best
    }
}

fn split_path(path: &str) -> Vec<&str> {
    let trimmed = path.trim_matches('/');
    if trimmed.is_empty() {
        return Vec::new();
    }
    trimmed
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect()
}

fn match_route(route_segments: &[&str], request_segments: &[&str]) -> Option<MatchResult> {
    if route_segments.len() != request_segments.len() {
        return None;
    }

    let mut params = Vec::new();
    let mut param_names = HashSet::new();
    let mut static_segments = 0;

    for (route_segment, request_segment) in route_segments.iter().zip(request_segments.iter()) {
        if let Some(param_name) = route_segment.strip_prefix(':') {
            if param_name.is_empty()
                || request_segment.is_empty()
                || !param_names.insert(param_name.to_string())
            {
                return None;
            }

            params.push((param_name.to_string(), request_segment.to_string()));
            continue;
        }

        if route_segment != request_segment {
            return None;
        }

        static_segments += 1;
    }

    Some(MatchResult {
        params,
        static_segments,
    })
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

    #[test]
    fn resolve_with_params_supports_dynamic_segments() {
        let mut app = WebApp::new("kitty-demo");
        app.add_route(Route::new(
            "/users/:id/posts/:post_id",
            PageComponent::new("post-detail", "<h1>Post</h1>"),
        ));

        let resolved = app
            .resolve_with_params("/users/42/posts/99")
            .expect("dynamic route should resolve");

        assert_eq!(resolved.route.component.name, "post-detail");
        assert_eq!(resolved.param("id"), Some("42"));
        assert_eq!(resolved.param("post_id"), Some("99"));
    }

    #[test]
    fn resolve_with_params_ignores_query_and_handles_root() {
        let mut app = WebApp::new("kitty-demo");
        app.add_route(Route::new(
            "/",
            PageComponent::new("home", "<h1>Kitty</h1>"),
        ));

        let resolved = app
            .resolve_with_params("/?tab=overview")
            .expect("root route should resolve");
        assert_eq!(resolved.route.component.name, "home");
        assert!(resolved.params.is_empty());
    }

    #[test]
    fn static_route_is_preferred_over_dynamic_route() {
        let mut app = WebApp::new("kitty-demo");
        app.add_route(Route::new(
            "/users/:id",
            PageComponent::new("user-profile", "<h1>User</h1>"),
        ));
        app.add_route(Route::new(
            "/users/me",
            PageComponent::new("current-user", "<h1>Me</h1>"),
        ));

        let resolved = app
            .resolve_with_params("/users/me")
            .expect("route should resolve");

        assert_eq!(resolved.route.component.name, "current-user");
        assert!(resolved.params.is_empty());
    }

    #[test]
    fn resolve_with_params_normalizes_multiple_slashes() {
        let mut app = WebApp::new("kitty-demo");
        app.add_route(Route::new(
            "/users/:id",
            PageComponent::new("user-profile", "<h1>User</h1>"),
        ));

        let resolved = app
            .resolve_with_params("//users///42//")
            .expect("route should resolve with normalized slashes");
        assert_eq!(resolved.param("id"), Some("42"));
    }

    #[test]
    fn invalid_route_with_duplicate_param_names_is_ignored() {
        let mut app = WebApp::new("kitty-demo");
        app.add_route(Route::new(
            "/:id/:id",
            PageComponent::new("invalid", "<h1>Invalid</h1>"),
        ));

        assert!(app.resolve_with_params("/a/b").is_none());
    }
}
