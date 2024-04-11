macro_rules! group {
    ($($command_name:ident,)*) => {
        use slashook::commands::Command as SlashookCommand;

        $(mod $command_name;)*

        pub fn get_commands() -> Vec<SlashookCommand> {
            vec![$($command_name::get_command(),)*]
        }
    }
}

pub(crate) use group;

macro_rules! yes_no {
    ($condition:expr $(, $yes:expr, $no:expr)?$(,)?) => {
        {
            let _yes = "Yes";
            $(let _yes = $yes;)?

            let _no = "No";
            $(let _no = $no;)?

            match $condition {
                true => _yes,
                false => _no,
            }
        }
    };
}

pub(crate) use yes_no;
