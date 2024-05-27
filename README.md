# MySQL Migrator

MySQL Migrator is a Rust-based utility designed to migrate tables from one MySQL server to another. It's particularly useful when the tables are not precisely identical, allowing for selective migration of specific columns.

## Key Features

- **Selective Column Migration**: One of the main reasons this project was created is to cater to scenarios where you want to import only specific columns from the source. This is especially useful when certain columns are no longer in use in the newer version of the software. This feature is a key selling point of MySQL Migrator.

- **Configurable Table Renaming**: The utility allows for renaming tables during the migration process. This can be useful when merging data from multiple databases or when restructuring your database schema.

- **Easy Configuration**: MySQL Migrator uses a simple JSON configuration file to specify the source and destination databases, as well as the tables and columns to migrate.

## Getting Started

1. Ensure you have Rust installed on your machine. If not, you can download it from the official website.

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

## Config

- **source**: This section contains the details of the source database from which the tables will be migrated.
    - **host**: The hostname of the source database.
    - **port**: The port number of the source database.
    - **name**: The name of the source database.
    - **user**: The username to connect to the source database.
    - **pass**: The password to connect to the source database.
    - **tables**: An array of tables in the source database to be migrated. Each table is represented by an object that contains:
        - **name**: The name of the table in the source database.
        - **columns**: An array of column names to be migrated from the source table. If this field is not specified, all columns will be migrated.
        - **rename**: The new name for the table in the destination database. If this field is not specified, the original table name will be used.
- **destination**: This section contains the details of the destination database to which the tables will be migrated. The fields in this section have the same meaning as those in the source section.
    - **host**: The hostname of the destination database.
    - **port**: The port number of the destination database.
    - **name**: The name of the destination database.
    - **user**: The username to connect to the destination database.
    - **pass**: The password to connect to the destination database.
    - **tables**: An array of tables in the destination database to be migrated. Each table is represented by an object that contains:
        - **name**: The name of the table in the destination database. This should match the rename field of the corresponding table in the source database. If the rename field was not specified in the source database, this should match the original table name.


## Todo
- export multi-threaded
- refine the configuration file, currently the confuration file looks odd and it should be improved.
- importing with XML
- importing with JSON

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.
