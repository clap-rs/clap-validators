# How to Contribute

Contributions are always welcome! Please use the following guidelines when contributing to `clap-validators`

1. Fork `clap-validators`
2. Clone your fork (`git clone https://github.com/$YOUR_USERNAME/clap-validators && cd clap-validators`)
3. Create new branch (`git checkout -b new-branch`)
4. Make your changes, and commit (`git commit -am "your message"`)
 * I use a [conventional](https://github.com/ajoslin/conventional-changelog/blob/a5505865ff3dd710cf757f50530e73ef0ca641da/conventions/angular.md) changelog format so I can update my changelog using [clog](https://github.com/clog-tool/clog-cli)
 * In addition to the conventions defined above, I also use `imp`, `wip`, `examples`.
 * Format your commit subject line using the following format: `TYPE(COMPONENT): MESSAGE` where `TYPE` is one of the following:
    - `feat` - A new feature
    - `imp` - An improvement to an existing feature
    - `perf` - A performance improvement
    - `docs` - Changes to documentation only
    - `tests` - Changes to the testing framework or tests only
    - `fix` - A bug fix
    - `refactor` - Code functionality doesn't change, but underlying structure may
    - `style` - Stylistic changes only, no functionality changes
    - `wip` - A work in progress commit (Should typically be `git rebase`'ed away)
    - `chore` - Catch all or things that have to do with the build system, etc
    - `examples` - Changes to existing example, or a new example
 * The `COMPONENT` is optional, and may be a single file, directory, or logical component. Can be omitted if commit applies globally
5. Run the tests (`cargo test --features yaml && make -C clap-tests test`)
6. `git rebase` into concise commits and remove `--fixup`s (`git rebase -i HEAD~NUM` where `NUM` is number of commits back)
7. Push your changes back to your fork (`git push origin $your-branch`)
8. Create a pull request! (You can also create the pull request first, and we'll merge when ready. This a good way to discuss proposed changes.)

Another really great way to help is if you find an interesting, or helpful way in which to use `clap`. You can either add it to the [examples/](examples) directory, or file an issue and tell me. I'm all about giving credit where credit is due :)

## Goals

There are a few goals of `clap-validators` that I'd like to maintain throughout contributions.

* Remain backwards compatible when possible
  - If backwards compatibility *must* be broken, use deprecation warnings if at all possible before removing legacy code
  - This does not apply for security concerns
* Validate values quickly
  - Validating of arguments values shouldn't slow down usage of the main program
* Try to be cognizant of memory usage
* `panic!` on *developer* error, exit gracefully on *end-user* error