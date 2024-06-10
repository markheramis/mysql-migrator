# MySQL Migrator

`mysql-migrator` is a Rust-based tool designed to facilitate the migration of MySQL databases. It enables exporting data from a source database and importing it into a destination database, with customizable configurations for tables, columns, and insert statements.

## Features

- Connection Configuration: Easily configure source and destination database connections.
- Table Configuration: Specify tables, columns, and conditions for export and import.
- Customizable Insert Statements: Options for extended inserts, complete inserts, and INSERT IGNORE statements.
- Export and Import Modes: Export data only or both export and import.
- Clean Export Directory: Option to clean previous exports before starting a new export.
- Interactive Prompts: Prompts for missing configuration details during runtime.

## Command-Line Arguments

### General Options

- `--connection-config`: Specify the connection configuration file.
- `--table-config`: Specify the table configuration file.
- `--export-path`: Set the path for export files.
- `--clean`: Clean previous exports before starting a new export.
- `--export-only`: Run in export-only mode, skipping the import process.

### Insert Options
- `--extended-insert`: Use extended insert statements.
- `--complete-insert`: Include column names in insert statements.
- `--insert-ignore`: Use INSERT IGNORE instead of INSERT.
- `--extended-insert-limit`: Limit the number of rows in extended insert statements (default: 50).

### Source Database Configuration
- `--source-host`: Source database host/hostname/IP address.
- `--source-port`: Source database port.
- `--source-database`: Source database name.
- `--source-username`: Source database username.
- `--source-password`: Source database password.

### Destination Database Configuration
- `--destination-host`: Destination database host/hostname/IP address.
- `--destination-port`: Destination database port.
- `--destination-database`: Destination database name.
- `--destination-username`: Destination database username.
- `--destination-password`: Destination database password.

## Usage

To use mysql-migrator, run the following command with the appropriate arguments:

```bash
cargo run -- --connection-config=connection.json --table-config=table.json --export-path=./exports --extended-insert --complete-insert --insert-ignore --export-only
```
Ensure that the connection and table configuration files are correctly specified and provide the necessary details for the migration.

## Example Configuration Files

#### example connection.json
```
{
    "source": {
        "hostname": "source_host",
        "port": 3306,
        "database": "source_db",
        "username": "source_user",
        "password": "source_pass"
    },
    "destination": {
        "hostname": "destination_host",
        "port": 3306,
        "database": "destination_db",
        "username": "destination_user",
        "password": "destination_pass"
    }
}
```
#### example table.json
```
[
    {
        "name": "table1",
        "columns": ["column1", "column2"],
        "table_rename": "new_table1",
        "condition": "column1 > 10",
        "overrides": [
        {
            "name": "column2",
            "value": "old_value",
            "set": {"column3": "new_value"}
        }
        ],
        "column_rename": {"old_column1": "new_column1"}
    },
    "table2"
]
```
This setup allows for flexible and efficient migration of MySQL databases, tailored to your specific needs.

## Getting Started

1. Ensure you have Rust installed on your machine. If not, you can download it from the official website
2. Clone this repository to your local machine.
    ```bash
    git clone https://github.com/markheramis/mysql-migrator.git
    ```
3. Navigate to the project directory and build the project.
    ```bash
    cd mysql-migrator
    cargo build --release
    ```
4. Create a `config.json` file in the project root directory, specifying your source and destination databases, as well as the tables and columns to migrate.

5. Run the utility.
    ```bash
    cargo run --release
    ```

## Todo
- export multi-threaded
- importing with XML
- importing with JSON

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.
