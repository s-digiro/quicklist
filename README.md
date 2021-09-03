# quicklist
Command line based To Do list manager

## Usage
usage: ql [--help, update, ls, <list name> [RepeatsDaily, ExistsDaily,
       [t, d <YYY-MM-DD>, m] [e, a <lines...>, x <line numbers...>, s <term>,
       delete]]

### Creating a list
ql <list name>
Creates a new list as long as there isn't already one with that name

### Checking a list
ql <list>
Displays list in stdout

### Adding items to list
ql <list> a <item>
Adds item to end of list

### Removing item from list
ql <list> x <line number>
Removes item at line number from list

### Search list
ql <list> s <term>
Finds all lines containing <term> in <list> and prints to stdout

### Delete a list
ql <list> delete
Deletes the list

### Open list in editor
ql <list> e

## Configuration
There aren't many configuration options right now, but the editor used in ql <list> e can be changed by setting 'EDITOR' constant in config.rs.
By default it wil use neovim

## To Do:
* Complete different list types
	* RepeatsDaily
	* NewDaily
