# doc see [website](https://rbatis.github.io/rbatis.io/#/)

# 1 It is recommended to clone this project using Intellij Clion/VSCode



### These examples provide a comprehensive overview of rbatis's features and functionalities.

* crud.rs: Demonstrates basic CRUD (Create, Read, Update, Delete) operations.
* crud_json.rs: Shows storing and querying JSON data.
* crud_sql.rs: Illustrates the usage of ids: &[&str] parameter, which directly converts to SQL statements.
* log.rs: Demonstrates how to set up a logging interceptor in rbatis.
* macro_proc_htmlsql.rs: Shows the usage of htmlsql for processing HTML-like SQL statements.
* macro_proc_htmlsql_custom_func.rs: Demonstrates the functionality of custom functions in htmlsql.
* macro_proc_htmlsql_file.rs: Shows how to load html_sql statements from a file.
* macro_proc_htmlsql_page.rs: Illustrates pagination using htmlsql.
* macro_proc_pysql: Demonstrates the usage of pysql.
* macro_proc_rawsql: Shows how to use raw SQL statements in rbatis.
* plugin_intercept: Demonstrates the usage of interceptors in rbatis.
* plugin_table_sync: Illustrates automatically synchronizing struct definitions to table structures.
* pool_custom: Shows how to customize connection pool parameters.
* table_extend.rs: Demonstrates a struct inheriting members from another struct.
* transaction: Shows how to use transactions in rbatis.
