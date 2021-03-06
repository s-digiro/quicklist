pub const PROGRAM_NAME: &str = "quicklist";
pub const EDITOR: &str = "nvim";

pub const HELP_TEXT: &str = "usage: ql [--help, update, ls, <list name> [RepeatsDaily, ExistsDaily,
       [t, d <YYY-MM-DD>, m] [e, a <lines...>, x <line numbers...>, s <term>,
       delete]]

    ql | ql --help:
        Show this message
    ql [ls, list]:
        Display available lists
    ql update:
        Create any lists that should be created daily if they don't
        already exist

    ql <list that does not exist> create:
        Create a list with the name <list that does not exist> that does not
        repeat daily
    ql <list that does not exist> create RepeatsDaily:
        Create a list with name <list that does not exist> that will repeat
        based on a template daily
    ql <list that does not exist> create ExistsDaily:
        Create a list with name <list that does not exist> that will be created
        each day, but will not have any contents

    ql <list>:
        Show contents of <list>, for today if it repeats
    ql <list> [e, edit]:
        Open the list in an editor
    ql <list> [a, add] [vals]...:
        Adds each <val> to <list> as a new line
    ql <list> [x, cut] [linenums]...:
        Deletes each line number [linenum] from <list>
    ql <list> [s, search] <term>:
        Shows the line containing <term> if it exists in <list>
    ql <list> delete:
        Deletes the list and all instances of it

    ql <list> t:
        Shows the list for tomorrow's date, only works on repeating lists
    ql <list> t [e, edit]:
        Open tomorrow's list in an editor
    ql <list> t [a, add] [vals]...:
        Adds each <val> to tomorrow's <list> as a new line
    ql <list> t [x, cut] [linenums]...:
        Deletes each line number [linenum] from tomorrow's <list>
    ql <list> t [s, search] <term>:
        Shows the line containing <term> if it exists in tomorrow's <list>

    ql <list> d <YYYY-MM-DD>:
        Shows the instance of <list> for the date <YYYY-MM-DD>
    ql <list> d <YYYY-MM-DD> [e, edit]:
        Open the instance of <list> for the date <YYYY-MM-DD> in an editor
    ql <list> d <YYYY-MM-DD> [a, add] [vals]...:
        Adds each <val> to instance of <list> for the date <YYYY-MM-DD>> as a
        new line
    ql <list> d <YYYY-MM-DD> [x, cut] [linenums]...:
        Deletes each line number [linenum] from instance of <list> for the date
        <YYYY-MM-DD>>
    ql <list> d <YYYY-MM-DD> [s, seach] <term>:
        Shows the line containing <term> if it exists in instance of <list> for
        the date <YYYY-MM-DD>>

usage: ql [--help, update, ls, <list name> [RepeatsDaily, ExistsDaily,
       [t, d <YYY-MM-DD>, m] [e, a <lines...>, x <line numbers...>, s <term>,
       delete, create]]";
pub const INVALID_TEXT: &str = "Malformed Command";
