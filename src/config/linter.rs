use ahash::HashMap;
use config::ConfigBuilder;
use config::builder::BuilderState;
use serde::Deserialize;
use serde::Serialize;
use toml::value::Value;

use crate::config::ConfigurationEntry;
use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum LinterLevel {
    #[serde(alias = "off")]
    Off,
    #[serde(alias = "help")]
    Help,
    #[serde(alias = "note")]
    Note,
    #[serde(alias = "warning")]
    Warning,
    #[serde(alias = "error")]
    Error,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LinterConfiguration {
    pub default_plugins: Option<bool>,
    pub plugins: Vec<String>,
    pub rules: Vec<LinterRuleConfiguration>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LinterRuleConfiguration {
    pub name: String,
    pub level: Option<LinterLevel>,
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub options: HashMap<String, Value>,
}

impl ConfigurationEntry for LinterConfiguration {
    fn configure<St: BuilderState>(self, builder: ConfigBuilder<St>) -> Result<ConfigBuilder<St>, Error> {
        use ::config::Value;
        use ::config::ValueKind;

        let builder = builder
            .set_default("linter.default_plugins", Value::new(None, ValueKind::Nil))?
            .set_default("linter.plugins", Value::new(None, ValueKind::Array(vec![])))?
            .set_default("linter.rules", Value::new(None, ValueKind::Array(vec![])))?;

        Ok(builder)
    }
}
