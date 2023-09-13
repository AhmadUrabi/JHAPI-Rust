# JHAPI-Rust
A rust-rocket based API Scaffolding for connecting and fetching data from an Oracle 12c database. 
## Features
* Full User Management
* Token based Authentication
* Permission Management
* Optional Query Parameters
* Various Data Structs
## Configuring for your project
The version in this repository is a slightly modified version of a production API currently in use by a company, and thus designed around their need.
When modifying this project for personal use, you can edit the structs in each `structs.rs` file to match either the database schema or query parameters.
Some functions, like `src/product_data/mod.rs/get_product` have some hardwired logic for the query, and you may have to edit that to match your needs.
## Future Work
This project is not meant for production use or as a proper API, it's simply to give a general idea of how I implemented my API for my usecase, and needs to be modified and tested for your own usecase. Future updates and commits to this repository will be to make it more extensible and customizable, eventually hoping to become a sort-of a standalone ready API.
