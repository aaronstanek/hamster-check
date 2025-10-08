#[derive(Debug)]
pub enum ArgumentCommand {
    Help,
    Init,
    About,
    Edit,
    Check,
    Update,
    Report,
}

#[derive(Debug)]
pub struct Arguments {
    command: ArgumentCommand,
    target_path: Option<String>,
    allow_hidden: Option<bool>,
    is_verbose: bool,
    is_forced: bool,
}

pub fn parse_arguments(raw_arguments: Vec<String>) -> Result<Arguments, String> {
    let mut arguments = Arguments {
        command: ArgumentCommand::Help,
        target_path: None,
        allow_hidden: None,
        is_verbose: false,
        is_forced: false,
    };

    let mut is_argument_term_possibly_a_flag = true;
    let mut is_command_set = false;

    for argument_term in raw_arguments.into_iter().skip(1) {
        if is_argument_term_possibly_a_flag && argument_term.starts_with("-") {
            match argument_term.as_str() {
                "--" => {
                    is_argument_term_possibly_a_flag = false;
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

        if !is_command_set {
            let command: Option<ArgumentCommand> = match argument_term.as_str() {
                "help" => Some(ArgumentCommand::Help),
                "init" => Some(ArgumentCommand::Init),
                "about" => Some(ArgumentCommand::About),
                "edit" => Some(ArgumentCommand::Edit),
                "check" => Some(ArgumentCommand::Check),
                "update" => Some(ArgumentCommand::Update),
                "report" => Some(ArgumentCommand::Report),
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

        if arguments.target_path.is_none() {
            arguments.target_path = Some(argument_term);
            continue;
        }

        return Err(format!("Unexpected parameter: {}", argument_term));
    }

    match arguments.command {
        ArgumentCommand::Help => {}
        ArgumentCommand::Init => {
            if arguments.is_forced {
                return Err(String::from("The init command cannot accept the -f flag."));
            }
        }
        ArgumentCommand::About => {
            if arguments.allow_hidden.is_some() || arguments.is_forced {
                return Err(String::from(
                    "The about command cannot accept any of the following flags: -h -d -f",
                ));
            }
        }
        ArgumentCommand::Edit => {
            if arguments.is_forced {
                return Err(String::from("The edit command cannot accept the -f flag"));
            }
        }
        ArgumentCommand::Check => {
            if arguments.allow_hidden.is_some() || arguments.is_forced {
                return Err(String::from(
                    "The check command cannot accept any of the following flags: -h -d -f",
                ));
            }
        }
        ArgumentCommand::Update => {
            if arguments.allow_hidden.is_some() {
                return Err(String::from(
                    "The update command cannot accept the -h or -d flags.",
                ));
            }
        }
        ArgumentCommand::Report => {
            if arguments.allow_hidden.is_some() || arguments.is_forced {
                return Err(String::from(
                    "The report command cannot accept any of the following flags: -h -d -f",
                ));
            }
        }
    }

    return Ok(arguments);
}
