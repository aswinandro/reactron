pub struct Theme {
    pub background: &'static str,
    pub accent_primary: &'static str,
    pub accent_secondary: &'static str,
    pub text_primary: &'static str,
    pub text_muted: &'static str,
    pub font_label: &'static str,
    pub font_button: &'static str,
}

pub const REACTRON_THEME: Theme = Theme {
    background: "#080b13",
    accent_primary: "#ff2d2d",
    accent_secondary: "#27ffd8",
    text_primary: "#d8e3ff",
    text_muted: "#9eb4ff",
    font_label: "14px Consolas",
    font_button: "600 22px Consolas",
};

