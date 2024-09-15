pub enum CmdlineCommands {
    Quit,
    Write,
}

impl CmdlineCommands {
    pub fn from_str(cmdline: &str) -> Option<Self> {
        match cmdline {
            "q" => Some(Self::Quit),
            "w" => Some(Self::Write),
            _ => None,
        }
    }
}
