# kot

A command runner.

## Info (TODO)

### Dot

Identifiers with a ```.``` in front of them can have special rules.

- object
- args
- regex

- cmd
- return
- inject: Inject object's fields into current scope ```.inject .object { a: 11 }```.
          Or injects vars in scope into scope above ```.inject { let b = 12 }```.
          Requires config option ```unsafe_inject```.
- spawn: Only allows commands.
- parallel: Can return, use let, and run commands.
- try: ```let a = .try CMD``` == ```let a = CMD if a.code != 0 { .exit a.code }```

- triplet
- arch
- os
- family

- panic
- exit

#### Test file status (REMOVE)

- kotfile: broken
- kotfile2: broken
- kotfilelexer: broken
- kotfiledev: working
