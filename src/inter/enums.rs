use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum Usage {
    ChatGPT,
    Api,
    Dashboard,
    Configuration,
    OAuth,
    Quit,
}

impl Usage {
    // could be generated by macro
    pub const USAGE_VARS: &'static [Usage] = &[
        Self::Api,
        Self::ChatGPT,
        Self::OAuth,
        Self::Dashboard,
        Self::Configuration,
        Self::Quit,
    ];
}

impl Display for Usage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Usage::Api => write!(f, "{self:?}           - Turbo API interactive conversation"),
            Usage::ChatGPT => write!(f, "{self:?}       - ChatGPT API interactive conversation"),
            Usage::Dashboard => write!(f, "{self:?}     - Dashboard Manager"),
            Usage::OAuth => write!(f, "{self:?}         - OAuth authorization"),
            Usage::Configuration => write!(f, "{self:?} - Configuration settings"),
            Usage::Quit => write!(f, "{self:?}"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OAuth {
    AccessToken,
    RefreshToken,
    RevokeToken,
    Quit,
}

impl OAuth {
    // could be generated by macro
    pub const OAUTH_VARS: &'static [OAuth] = &[
        Self::AccessToken,
        Self::RefreshToken,
        Self::RevokeToken,
        Self::Quit,
    ];
}

impl Display for OAuth {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            OAuth::AccessToken => write!(f, "{self:?}  - Login to get AccessToken"),
            OAuth::RefreshToken => write!(f, "{self:?} - Refresh to get AccessToken"),
            OAuth::RevokeToken => write!(f, "{self:?}  - Revoke AccessToken"),
            OAuth::Quit => write!(f, "{self:?}"),
        }
    }
}
