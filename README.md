# ng-i18n

_ng-i18n_ is a set of utilities for use with the Angular web framework's `@angular/localize` package.
Angular themselves said that they will not be addressing the issue of localization file changes themselves,
so I've decided to create a small utility that generates and updates different language i18n files automatically.

## How to install


### Using Cargo

This method required the Rust toolchain to be installed. If you do not already have it get it
[here](https://www.rust-lang.org/tools/install).

_ng-i18n_ is available for download through the cargo package manager.

```shell
cargo install ng-i18n
```

### Building from source
This method required the Rust toolchain to be installed. If you do not already have it get it 
[here](https://www.rust-lang.org/tools/install).

1. Clone the repo and move into it
    ```shell
    git clone https://github.com/local-interloper/ng-i18n && cd ng-i18n
    ```

2. Compile the project.
   ```shell
   cargo build --release
   ```

3. Move the binary somewhere within your `$PATH` or `%PATH%`.
   ```shell
   sudo cp target/release/ng-i18n /usr/local/bin
   ```

## How to use

Generate your i18n files by running the following command. Please note that they must be in JSON format.

```shell
ng extract-i18n --output-path i18n --format json
```

This should create `messages.json` under the `i18n` directory. Then we can create or update our i18n
files by running the following command.

```shell
ng-i18n update i18n/messages.json de it
```

The `de` and `it` arguments are here used as an example. The angular standard way of naming i18n files
is using 2 letters before the ".json" suffix (e.g. `messages.de.json`) so those are generated from these
arguments.

Running that last command should create or update `messages.de.json` and `messages.it.json`. In case they
already existed before running the command they will be updated with the missing keys from the source i18n
file.

## Contributing

Want to contribute? Feel free to make a pull request and spam me if I don't reply in a timely manner.
Contacts are on my profile.