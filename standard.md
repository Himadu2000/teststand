24/05/20 - 5:00p.m.

For front end

- src
- - main.rs
- - pages
- -- mod.rs - router
- routes - more router components if required
- -- home
- --- mod.rs - controller or data for the views
- --- view.rs - views
- -- 404.rs
- -- nav - if required
- -- footer - if required
- -- complex - page
- --- mod.rs
- --- components
- ---- complex part managed by another team
- ----- mod.rs
- ----- data.rs - shared layer between controller and view
- ----- query.rs graphql query
- ----- test.rs - testing controller
- ----- process.rs - Additional processing for data
- ----- views
- ------ mod.rs
- ------ image.rs
- - lib - commonly used components which rarely change during website lifecycle or it will interfere with developers
- - client - graphql client
- - images - if required

Themes for example e-commerce platforms includes whole pages as a 7z file.

In themes to avoid code duplication use git repo as a dependency to include some pages from another theme

Use rust cynic graphql client

For back end

- src
- - main.rs
- - schema - controller or GraphQL schema
- routes - more GraphQL schema components if required
- - services
- -- home
- --- mod.rs - controller or GraphQL schema
- --- schemas - more GraphQL schema components if required
- --- models.rs - Database models
- -- complex - service
- --- mod.rs
- --- components
- ---- complex part managed by another team
- ----- mod.rs
- ----- data.rs - shared layer between controller and view
- ----- query.rs graphql query
- ----- test.rs - testing controller
- ----- process.rs - Additional processing for data
- ----- views
- ------ mod.rs
- ------ image.rs
- - lib - commonly used components which rarely change during website lifecycle or it will interfere with developers

Database models are also included in the micro service so each team don't conflict each other when changing the schema.
