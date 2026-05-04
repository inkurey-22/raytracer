use omni_light::OmniLight;

#[derive(Debug, Clone)]
pub enum Light {
    OmniLight(OmniLight),
}

impl std::fmt::Display for Light {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Light::OmniLight(omni_light) => write!(f, "{}", omni_light),
        }
    }
}
