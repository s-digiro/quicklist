# quicklist
Command line based To Do list manager

## Usage
usage: ql [--help, update, ls, <list name> [RepeatsDaily, ExistsDaily,
       [t, d <YYY-MM-DD>, m] [edit, add <lines...>, cut <line numbers...>, search <term>,
       delete, create]]

### Creating a list
ql <list name> create
Creates a new list as long as there isn't already one with that name

### Showing a list
ql <list>
Displays list in stdout

### Adding items to list
ql <list> [a, add] <item>
Adds item to end of list

### Removing item from list
ql <list> [x, cut] <line number>
Removes item at line number from list

### Search list
ql <list> [s, search] <term>
Finds all lines containing <term> in <list> and prints to stdout

### Delete a list
ql <list> delete
Deletes the list

### Open list in editor
ql <list> [e, edit]

## Configuration
There aren't many configuration options right now, but the editor used in ql <list> e can be changed by setting 'EDITOR' constant in config.rs.
By default it wil use neovim

## To Do:
* Complete different list types
	* RepeatsDaily
	* NewDaily
