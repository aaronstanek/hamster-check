#[derive(Debug)]
pub enum ArgumentCommand {
    Help,
    Init,
    About,
    Edit,
    Check,
    Update,
}

#[derive(Debug)]
pub struct Arguments {
    command: ArgumentCommand,
    target_paths: Vec<String>,
    add_paths: Vec<String>,
    remove_paths: Vec<String>,
    allow_hidden: Option<bool>,
    is_verbose: bool,
    is_forced: bool,
}

pub fn parse_arguments(raw_arguments: Vec<String>) -> Result<Arguments, String> {
    let mut arguments = Arguments {
        command: ArgumentCommand::Help,
        target_paths: vec![],
        add_paths: vec![],
        remove_paths: vec![],
        allow_hidden: None,
        is_verbose: false,
        is_forced: false,
    };

    let mut is_command_set = false;
    let mut is_adding_path = false;
    let mut is_removing_path = false;

    for argument_term in raw_arguments.into_iter().skip(1) {
        if !is_command_set {
            let command: Option<ArgumentCommand> = match argument_term.as_str() {
                "help" => Some(ArgumentCommand::Help),
                "init" => Some(ArgumentCommand::Init),
                "about" => Some(ArgumentCommand::About),
                "edit" => Some(ArgumentCommand::Edit),
                "check" => Some(ArgumentCommand::Check),
                "update" => Some(ArgumentCommand::Update),
                _ => None,
            };
            match command {
                Some(x) => {
                    arguments.command = x;
                    is_command_set = true;
                }
                None => {
                    return Err(format!("Unknown command: {}", argument_term));
                }
            }
            continue;
        }

        if argument_term.starts_with("-") {
            match argument_term.as_str() {
                "-a" => {
                    is_adding_path = true;
                    is_removing_path = false;
                }
                "-r" => {
                    is_adding_path = false;
                    is_removing_path = true;
                }
                "-h" => match arguments.allow_hidden {
                    Some(_) => {
                        return Err(String::from("Allow-hidden flag set multiple times"));
                    }
                    None => {
                        arguments.allow_hidden = Some(true);
                    }
                },
                "-d" => match arguments.allow_hidden {
                    Some(_) => {
                        return Err(String::from("Allow-hidden flag set multiple times"));
                    }
                    None => {
                        arguments.allow_hidden = Some(false);
                    }
                },
                "-v" => {
                    if arguments.is_verbose {
                        return Err(String::from("Verbose flag set multiple times"));
                    }
                    arguments.is_verbose = true;
                }
                "-f" => {
                    if arguments.is_forced {
                        return Err(String::from("Force flag set multiple times"));
                    }
                    arguments.is_forced = true;
                }
                _ => {
                    return Err(format!("Unknown flag: {}", argument_term));
                }
            }
            continue;
        }

        if is_adding_path {
            arguments.add_paths.push(argument_term);
        } else if is_removing_path {
            arguments.remove_paths.push(argument_term);
        } else {
            arguments.target_paths.push(argument_term);
        }
    }

    match arguments.command {
        ArgumentCommand::Help => {}
        ArgumentCommand::Init => {
            if arguments.target_paths.len() < 2 {
                return Err(String::from("init command must be followed by at least 2 paths. The first is the path to the config file to be created. The second is the path to the content to archive."));
            }
        }
        ArgumentCommand::About => {
            if arguments.target_paths.len() != 1 {
                return Err(String::from("about command must be followed by exactly one path. This path should correspond to a config file."));
            }
        }
        ArgumentCommand::Edit => {
            if arguments.target_paths.len() != 1 {
                return Err(String::from("edit command must be followed by exactly one path. This path should correspond to a config file."));
            }
            if arguments.add_paths.len() == 0
                && arguments.remove_paths.len() == 0
                && arguments.allow_hidden.is_none()
            {
                return Err(String::from("edit command must make an edit to a config file. Valid edits are adding a source path, removing a source path, and allowing/disallowing hidden files."));
            }
        }
        ArgumentCommand::Check => {
            if arguments.target_paths.len() != 1 {
                return Err(String::from("check command must be followed by exactly one path. This path should correspond to a config file."));
            }
        }
        ArgumentCommand::Update => {
            if arguments.target_paths.len() != 1 {
                return Err(String::from("update command must be followed by exactly one path. This path should correspond to a config file."));
            }
        }
    }

    return Ok(arguments);
}
