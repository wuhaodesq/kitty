#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureSupport {
    Supported,
    Partial,
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiteProfile {
    pub domain: String,
    pub requires_webgl2: bool,
    pub requires_webassembly: bool,
    pub requires_service_worker: bool,
}

impl SiteProfile {
    pub fn new(domain: impl Into<String>) -> Self {
        Self {
            domain: domain.into(),
            requires_webgl2: false,
            requires_webassembly: false,
            requires_service_worker: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityReport {
    pub domain: String,
    pub webgl2: FeatureSupport,
    pub webassembly: FeatureSupport,
    pub service_worker: FeatureSupport,
}

impl CompatibilityReport {
    pub fn score(&self) -> u8 {
        fn points(s: FeatureSupport) -> u8 {
            match s {
                FeatureSupport::Supported => 2,
                FeatureSupport::Partial => 1,
                FeatureSupport::Missing => 0,
            }
        }

        points(self.webgl2) + points(self.webassembly) + points(self.service_worker)
    }
}

#[derive(Debug, Clone)]
pub struct BaselineChecker {
    pub webgl2: FeatureSupport,
    pub webassembly: FeatureSupport,
    pub service_worker: FeatureSupport,
}

impl Default for BaselineChecker {
    fn default() -> Self {
        Self {
            webgl2: FeatureSupport::Partial,
            webassembly: FeatureSupport::Supported,
            service_worker: FeatureSupport::Partial,
        }
    }
}

impl BaselineChecker {
    pub fn check(&self, site: &SiteProfile) -> CompatibilityReport {
        CompatibilityReport {
            domain: site.domain.clone(),
            webgl2: if site.requires_webgl2 {
                self.webgl2
            } else {
                FeatureSupport::Supported
            },
            webassembly: if site.requires_webassembly {
                self.webassembly
            } else {
                FeatureSupport::Supported
            },
            service_worker: if site.requires_service_worker {
                self.service_worker
            } else {
                FeatureSupport::Supported
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_checker_returns_expected_support() {
        let checker = BaselineChecker::default();
        let mut site = SiteProfile::new("example.com");
        site.requires_webgl2 = true;
        site.requires_webassembly = true;
        site.requires_service_worker = true;

        let report = checker.check(&site);
        assert_eq!(report.webassembly, FeatureSupport::Supported);
        assert_eq!(report.webgl2, FeatureSupport::Partial);
        assert_eq!(report.service_worker, FeatureSupport::Partial);
        assert_eq!(report.score(), 4);
    }
}
