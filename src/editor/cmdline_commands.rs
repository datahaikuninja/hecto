pub enum CmdlineCommands {
    Quit,
    Write,
    Saveas(String),
}

impl CmdlineCommands {
    pub fn parse_cmdline(cmdline: &Vec<String>) -> Result<Self, String> {
        match cmdline[0].as_str() {
            "q" => Ok(Self::Quit),
            "w" => Ok(Self::Write),
            "saveas" => {
                if cmdline.len() >= 2 {
                    Ok(Self::Saveas(cmdline[1].clone()))
                } else {
                    Err("No filename provided for `saveas` command.".to_string())
                }
            }
            _ => Err(format!("No such command: {}", cmdline[0])),
        }
    }
}
