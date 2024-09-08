#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub(crate) badge: bool,
    pub(crate) icon: bool,
    pub(crate) text: bool,
    pub(crate) time: bool,
    pub(crate) target: bool,
    pub(crate) file: bool,
    pub(crate) line: bool,
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

    pub fn time(mut self, time: bool) -> Self {
        self.0.time = time;
        self
    }

    pub fn icon(mut self, icon: bool) -> Self {
        self.0.icon = icon;
        self
    }

    pub fn text(mut self, text: bool) -> Self {
        self.0.text = text;
        self
    }

    pub fn target(mut self, target: bool) -> Self {
        self.0.target = target;
        self
    }

    pub fn file(mut self, file: bool) -> Self {
        self.0.file = file;
        self
    }

    pub fn line(mut self, line: bool) -> Self {
        self.0.line = line;
        self
    }

    pub fn finish(self) -> Result<Config, String> {
        check_conflict(&self.0)?;
        Ok(self.0)
    }
}

fn check_conflict(config: &Config) -> Result<(), String> {
    let mut conflict_count = 0;

    if config.badge {
        conflict_count += 1;
    }
    if config.icon {
        conflict_count += 1;
    }

    if config.text {
        conflict_count += 1;
    }

    if conflict_count > 1 {
        Err("Conflict: More than one of badge, icon, or text is enabled.".into())
    } else {
        Ok(())
    }
}


impl Default for ConfigBuilder {
    fn default() -> Self {
        ConfigBuilder::new()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config { badge: false, icon: false, text: false, time: true, target: false, file: false, line: false }
    }
}
