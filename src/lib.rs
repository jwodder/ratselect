mod app;
use crate::app::App;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Form<T> {
    selectors: Vec<(T, Selector)>,
}

impl<T> Form<T> {
    pub fn new() -> Self {
        Form {
            selectors: Vec::new(),
        }
    }

    pub fn add<S: Into<Selector>>(&mut self, key: T, selector: S) {
        self.selectors.push((key, selector.into()));
    }

    pub fn run(self) -> std::io::Result<Option<Vec<(T, Selection)>>> {
        let app = App::from(self);
        let terminal = ratatui::try_init()?;
        let r = app.run(terminal);
        ratatui::restore();
        r
    }
}

impl<T> Default for Form<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Selector {
    Radio(RadioSelector),
    Multi(MultiSelector),
}

impl Selector {
    fn title(&self) -> &str {
        match self {
            Selector::Radio(s) => &s.title,
            Selector::Multi(s) => &s.title,
        }
    }

    fn default_selection(&self) -> Selection {
        match self {
            Selector::Radio(s) => s.default_selection(),
            Selector::Multi(s) => s.default_selection(),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Selector::Radio(s) => s.is_empty(),
            Selector::Multi(s) => s.is_empty(),
        }
    }
}

impl From<RadioSelector> for Selector {
    fn from(value: RadioSelector) -> Selector {
        Selector::Radio(value)
    }
}

impl From<MultiSelector> for Selector {
    fn from(value: MultiSelector) -> Selector {
        Selector::Multi(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RadioSelector {
    title: String,
    options: Vec<String>,
    default: usize,
}

impl RadioSelector {
    pub fn new<S, I>(title: S, options: I) -> RadioSelector
    where
        S: Into<String>,
        I: IntoIterator<Item: Into<String>>,
    {
        RadioSelector {
            title: title.into(),
            options: options.into_iter().map(Into::into).collect(),
            default: 0,
        }
    }

    pub fn with_default(mut self, mut default: usize) -> RadioSelector {
        if default >= self.options.len() {
            default = 0;
        }
        self.default = default;
        self
    }

    fn default_selection(&self) -> Selection {
        Selection::Radio(self.default)
    }

    fn into_checked_options(self) -> impl Iterator<Item = (bool, String)> {
        self.options
            .into_iter()
            .enumerate()
            .map(move |(i, s)| (i == self.default, s))
    }

    fn is_empty(&self) -> bool {
        self.options.is_empty()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultiSelector {
    title: String,
    options: Vec<String>,
    defaults: BTreeSet<usize>,
}

impl MultiSelector {
    pub fn new<S, I>(title: S, options: I) -> MultiSelector
    where
        S: Into<String>,
        I: IntoIterator<Item: Into<String>>,
    {
        MultiSelector {
            title: title.into(),
            options: options.into_iter().map(Into::into).collect(),
            defaults: BTreeSet::new(),
        }
    }

    pub fn with_defaults<I: IntoIterator<Item = usize>>(mut self, defaults: I) -> MultiSelector {
        self.defaults = defaults
            .into_iter()
            .filter(|&i| i < self.options.len())
            .collect();
        self
    }

    fn default_selection(&self) -> Selection {
        Selection::Multi(self.defaults.clone())
    }

    fn into_checked_options(self) -> impl Iterator<Item = (bool, String)> {
        self.options
            .into_iter()
            .enumerate()
            .map(move |(i, s)| (self.defaults.contains(&i), s))
    }

    fn is_empty(&self) -> bool {
        self.options.is_empty()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Selection {
    Radio(usize),
    Multi(BTreeSet<usize>),
}
