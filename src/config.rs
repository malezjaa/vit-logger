#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub(crate) badge: bool,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ConfigBuilder(pub(crate) Config);

impl ConfigBuilder {
    pub fn new() -> Self {
        ConfigBuilder(Config::default())
    }

    pub fn badge(mut self, badge: bool) -> Self {
        self.0.badge = badge;
        self
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        ConfigBuilder::new()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config { badge: false }
    }
}
