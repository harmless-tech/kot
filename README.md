# kot

Currently experimental and is nowhere near a running state!

A command runner.

TODO: How to handle .cmd. It should probably be special for speed.

TODO: Move types and errors to their own module?

TODO: Kot lib should be able to compile to wasm. So many interpreter should move to binary or be locked behind a feature.

## Info (TODO)

### Slash

Identifiers with a ```/``` in front of them can have special rules.

- object
- args
- regex

- cmd
- return
- ???inject???: Inject object's fields into current scope ```.inject .object { a: 11 }```.
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
- kotfile3: working
