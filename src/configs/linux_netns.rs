use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct LinuxNetNsConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for LinuxNetNsConfig<'a> {
    fn default() -> Self {
        LinuxNetNsConfig {
            format: "[[$symbol]($style) $ns ]($style bold)",
            symbol: "ﯱ",
            style: "blue",
            disabled: false,
        }
    }
}
