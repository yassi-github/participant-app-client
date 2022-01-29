# participant-app-client

Client side of an application to record participants in a SLP.

[Server side](https://github.com/higuruchi/participant-app)

[Router side](https://github.com/yassi-github/participant-app-router)

# 🚀 Getting Started

1. Download binary archive

    Download excutable binary archived file from [here](https://github.com/yassi-github/participant-app-client/releases/latest).

1. Extract

    Zip file includes:

    - `participant-app-client`
    - `participant-app-client.conf`

1. Modify config file

    Rewrite the configuration file according to your information.

    Example of `participant-app-client.conf`:
    ```yaml
    # User settings
    user:
        id: '19T999'
        name: "名無"

    # Server information
    server:
        # Server IP address and Port number
        destination: "127.0.0.1:1323"
    ```

1. Regist your information

    Before use this service, you must regist your information to API server.

    <details>
    <summary>Linux</summary>
    
    at shell terminal, run:

    ```sh
    ./participant-app-client regist
    ```

    </details>
    <details>
    <summary>Windows</summary>

    at Command Prompt, run:

    ```cmd
    participant-app-client regist
    ```

    </details>

🎉 Now, you are ready to use !!

# 🚀 Usage

Get today's participants list as "議事録" format.

Example:

```sh
$ ./participant-app-client get
○ B4 金太郎
○ B3 銀太郎
○ B2 銅太郎、鉄太郎
○ B1 一郎、二郎、三郎、四郎、五郎
```

Get the past participants, use `--year`, `--month`, `--day` options.

Example:

```sh
$ ./participant-app-client get --year 2021 --month 12 --day 24
○ B3 三太苦郎栖
○ B2 都中井
```

To show help, use `help` subcommand or `--help` flag.

```sh
$ ./participant-app-client help
$ ./participant-app-client regist --help
$ ./participant-app-client get --help
$ ./participant-app-client change-macaddr --help
```
