pub const HELP_INFO: &str = "
todo_app.rs (by lazzydisa)

Usage:
    todo help|h                 # print this information
    todo list|l                 # print all todo notes
    todo new|n <id:'note'>      # create a new note
    todo delete|d <id>          # delete an existing note
    todo serch|s <id>           # print a todo by its id

Note:
    you need to use different id's for notes, for example:
    todo new \"important:'to do someting!'\"

    also you can use \"\" instead of '', for example:
    todo new \"new:\\\"someting else\\\"\"

    or:
    todo new 'new:\"someting else\"'
";

pub const ERROR: &str = "Unknown command! For help and usage info use `todo help|h`";
