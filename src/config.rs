/// Defines the configuration for the application.

/*
 Copyright (c) 2026 RayZh

 Permission is hereby granted, free of charge, to any person obtaining a copy of
 this software and associated documentation files (the "Software"), to deal in
 the Software without restriction, including without limitation the rights to
 use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 the Software, and to permit persons to whom the Software is furnished to do so,
 subject to the following conditions:

 The above copyright notice and this permission notice shall be included in all
 copies or substantial portions of the Software.

 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

use serde::{Deserialize, Serialize};
use crate::typealias::Result;

#[derive(Debug, Clone, Default)]
#[derive(Serialize, Deserialize)]
pub struct Config {
    api_config: ApiConfig,
}

#[derive(Debug, Clone, Default)]
#[derive(Serialize, Deserialize)]
pub struct ApiConfig {
    pub hostname: String,
    pub port: u16,
    pub access_token: String,
}

impl Config {
    pub fn default() -> Self {
        Self {
            api_config: ApiConfig {
                hostname: "localhost".to_string(),
                port: 8080,
                access_token: "".to_string(),
            },
        }
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let config_content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }

    pub fn is_empty(&self) -> bool {
        self.api_config.access_token.is_empty()
    }
}