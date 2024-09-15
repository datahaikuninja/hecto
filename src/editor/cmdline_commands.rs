pub enum CmdlineCommands {
    Quit,
    Write,
    Saveas(String),
}

impl CmdlineCommands {
    pub fn parse_cmdline(cmdline: &Vec<String>) -> Option<Self> {
        match cmdline[0].as_str() {
            "q" => Some(Self::Quit),
            "w" => Some(Self::Write),
            "saveas" => Some(Self::Saveas(cmdline[1].clone())),
            _ => None,
        }
    }
}
