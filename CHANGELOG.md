## 2018-04-18, Version 0.3.0
### Commits
- [[`650df0bf4d`](https://github.com/yoshuawuyts/human-panic/commits/650df0bf4de62239e9592b2185ebbd4875640864)] (cargo-release) version 0.3.0 (Yoshua Wuyts)
- [[`a3ec0ddb97`](https://github.com/yoshuawuyts/human-panic/commits/a3ec0ddb97eb5940ab56785eb54eed52ead3a82b)] Fixing wrong data in certain crash dump fields (#15)

Fixing a bug in the log dumps

* Carrying over metadata from first macro call to properly
include metadata in crash dumps.
* Making Method derive Copy (Katharina)
- [[`ed11055e06`](https://github.com/yoshuawuyts/human-panic/commits/ed11055e0602c3c8d223ed8354058fefb9ac47ec)] Merge pull request #16 from killercup/docs

Ensure no docs are missin (Pascal Hertleif)
- [[`4540d77276`](https://github.com/yoshuawuyts/human-panic/commits/4540d77276eafbfb57c922f57f1aa04cd5cb1cd5)] Fix typos (#14)

* Correct typo embarrasing
* Fix typos (Andy Slack)
- [[`9e972ef654`](https://github.com/yoshuawuyts/human-panic/commits/9e972ef654df70047f73df51befa3ba2bcb2e5c5)] Ensure no docs are missing (Pascal Hertleif)
- [[`b82ac5c35a`](https://github.com/yoshuawuyts/human-panic/commits/b82ac5c35a9e5772a54033a084d1cc784ffd6510)] Merge pull request #11 from skade/update-readme

Update README with current interfac (Pascal Hertleif)
- [[`21c5417580`](https://github.com/yoshuawuyts/human-panic/commits/21c5417580e6bf4cbe330715b5cc4ae39e4f5d8e)] Update README with current interface (Florian Gilcher)
- [[`d86232967d`](https://github.com/yoshuawuyts/human-panic/commits/d86232967d3bf9dc868a4cd68bab2e1004b6d2dc)] Merge pull request #10 from killercup/rollup

Rollup (Pascal Hertleif)
- [[`80046e1488`](https://github.com/yoshuawuyts/human-panic/commits/80046e148860e0bcde3d5a8e9c1a56bf5f32a37c)] Use more generic way to get a Path (Pascal Hertleif)
- [[`dc05d332a0`](https://github.com/yoshuawuyts/human-panic/commits/dc05d332a0527812fc239f4622289fb593aac936)] Merge skade-join-properly into rollup (Pascal Hertleif)
- [[`2e0127c830`](https://github.com/yoshuawuyts/human-panic/commits/2e0127c8303d7ea5c46e9aacf83e2fa0fdbbbd83)] Merge yoshuawuyts-fix-example into rollup (Pascal Hertleif)
- [[`fc16cb8ac2`](https://github.com/yoshuawuyts/human-panic/commits/fc16cb8ac2b692450d689d5650fe82405f35f492)] Update Cargo.lock (Pascal Hertleif)
- [[`e53059ff3c`](https://github.com/yoshuawuyts/human-panic/commits/e53059ff3cc5e36bee7dc6e29a6605881122aac3)] rustfmt (Pascal Hertleif)
- [[`a51285bb10`](https://github.com/yoshuawuyts/human-panic/commits/a51285bb1044c0ef6e5a8c94f5149549315eef53)] Properly handle file paths using Path and PathBuf (Florian Gilcher)
- [[`82ebdccb5a`](https://github.com/yoshuawuyts/human-panic/commits/82ebdccb5a22baf369b12c888af7a0b9cd1d0ee8)] make clippy pass for real this time (Yoshua Wuyts)
- [[`2297066f50`](https://github.com/yoshuawuyts/human-panic/commits/2297066f504f98a62c1ddde357aad81a0ed147e4)] please clippy (Yoshua Wuyts)
- [[`b1ec2b5b7b`](https://github.com/yoshuawuyts/human-panic/commits/b1ec2b5b7bb5b679ed8287712272f1a7ba3387c8)] fix examples (Yoshua Wuyts)
- [[`369ca4e526`](https://github.com/yoshuawuyts/human-panic/commits/369ca4e526b911b8455e1759e7609edf1a606e34)] Bumpding version, adding author (Katharina)
- [[`31e1d9ada2`](https://github.com/yoshuawuyts/human-panic/commits/31e1d9ada2b3e0563cac37d32ec952552e129281)] Cleaning up warnings for the big rebase (Katharina Sabel)
- [[`3ffa055d57`](https://github.com/yoshuawuyts/human-panic/commits/3ffa055d576c8e572ddcde2744d5aef514b11fa5)] Attempting to fix the `err` example using the failures crate (Katharina Sabel)
- [[`5214754bc0`](https://github.com/yoshuawuyts/human-panic/commits/5214754bc093a389a7d44c5fd1e9d6d38df1ea86)] Adding a bit of padding in the log (Katharina Sabel)
- [[`031b2b846b`](https://github.com/yoshuawuyts/human-panic/commits/031b2b846b73e6fefa783ffee83c9c5ef6464c3a)] Merging advaned report functionality. (Katharina Sabel)
- [[`7a2e923075`](https://github.com/yoshuawuyts/human-panic/commits/7a2e9230751abd3a152d3240a8b4c75891ae8e41)] Merging #4 by yoshuawuyts

This commit adds the ability to generate reports based on an application
crash. It hooks into the existing panic handler and also exposes
the report generation feature via a `pub fn` (Katharina Sabel)
- [[`7dc354b88e`](https://github.com/yoshuawuyts/human-panic/commits/7dc354b88e0fc0cfc9f10e6477444f6b73d0afb3)] Preparing for cherrypick (Katharina Sabel)
- [[`5002578d8f`](https://github.com/yoshuawuyts/human-panic/commits/5002578d8f5b495d4f37fb68510dd5e5fa624cc6)] Cleaning up merge artefact (Katharina Sabel)
- [[`bd4526a315`](https://github.com/yoshuawuyts/human-panic/commits/bd4526a3156aacce08ffce4fbd2339a2bcb2cf84)] Changing the core functionality of the crate

Instead of having to wrap around every panic, this now uses `set_hook` once
to register the callback to print a pretty status message. This also has the
added benefit of pulling in env! calls because the main setup was made
into a macro.

Optionally the two core functions (print_help and dump_log) can now also be used
without the core macro, because they are `pub fn` (Katharina Sabel)
- [[`b90ea3ba1c`](https://github.com/yoshuawuyts/human-panic/commits/b90ea3ba1cda64f65928f36429da67523e78dcfb)] Stable and slim (#1)

* Make it compile on stable

Also adds a nightly feature flag that will automatically be picked up by
docs.rs to build nice docs.

* Make clippy a CI-only dependency

You can run `cargo clippy` locally to get the same effect. I've also
taken the liberty to nail down the rustfmt version to use, so we can
update it explicitly. (This is the same CI setup that assert_cli uses.)

* Get rid of all dependencies for now

Improves compile times :trollface:

* Use termcolor for colored output

This should make it compatible with windows consoles.

* Set up some kind of error handling for the hook

* rustfmt

* Bump clippy

and choose a nightly that actually exists.

* Make clippy happy (Pascal Hertleif)
- [[`c04ae22d1e`](https://github.com/yoshuawuyts/human-panic/commits/c04ae22d1ef3289d028f9ff5aaefd8a44b5c293c)] update readme output (Yoshua Wuyts)
- [[`4a35c860fd`](https://github.com/yoshuawuyts/human-panic/commits/4a35c860fd00835a36184be8068d777d4fa02519)] upgrade desc (Yoshua Wuyts)
- [[`ccaf3bce86`](https://github.com/yoshuawuyts/human-panic/commits/ccaf3bce8666879c89ce0222f7f8d1f306bde074)] init (Yoshua Wuyts)
- [[`a7135d1e8c`](https://github.com/yoshuawuyts/human-panic/commits/a7135d1e8c87409e3f553a761e2b2caa24d849c9)] catch (Yoshua Wuyts)
- [[`0129328ce4`](https://github.com/yoshuawuyts/human-panic/commits/0129328ce4472190366ac7835aed003d68aeb088)] . (Yoshua Wuyts)

### Stats
```diff
 .editorconfig                           |  25 +--
 .gitignore                              |   3 +-
 .travis.yml                             |   8 +-
 Cargo.lock                              | 391 +++++++++++++++++++++++++++++++++-
 Cargo.toml                              |  11 +-
 README.md                               |  51 +----
 examples/panic.rs                       |   9 +-
 src/lib.rs                              | 130 ++---------
 src/report.rs                           |  22 +--
 tests/custom-panic/Cargo.toml           |  10 +-
 tests/custom-panic/src/main.rs          |  14 +-
 tests/custom-panic/tests/integration.rs |  16 +-
 tests/single-panic/Cargo.toml           |  10 +-
 tests/single-panic/src/main.rs          |   9 +-
 tests/single-panic/tests/integration.rs |  14 +-
 15 files changed, 442 insertions(+), 281 deletions(-)
```


