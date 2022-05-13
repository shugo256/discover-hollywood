# Discover Hollywood

Movie finder application based on the MovieLens dataset.

Both the frontend and the backend of the app is implemented in Rust using the following crates:
* [Actix-Web](https://actix.rs): Efficient modern web framework for rust.
* [Yew](https://yew.rs): Wasm framework for building client web apps, inspired by React.js and JSX.

## Run
1. Add a `.env` file to the root of this project which contains your tbdb api key as `TMDB_V3_API_KEY`.
    ```
    TMDB_V3_API_KEY={YOUR_KEY}
    ```
    **NOTE**: Building the project without `TMDB_V3_API_KEY` will result in a compilation error.

1. Start the docker service.
    ```bash
    docker-compose up --build app
    ```
    **NOTE**: The building process can take ~10 minutes.\

1. Go to http://localhost:8080 and starat discovering!

## Documentation
The project contains a [cargo doc](https://doc.rust-lang.org/cargo/commands/cargo-doc.html).
After starting the server, you can see the documents at http://localhost:8080/docs/discover_hollywood/index.html.

API documentation of the backend server will be at the each handler methods' page which can be found [here](http://localhost:8080/docs/discover_hollywood_server/handlers/index.html).

## Project Structure
The project consists of 5 crates (= rust packages) and each of them is documented.

### `discover-hollywood` (module path: /)
[local document link](http://localhost:8080/docs/discover_hollywood/index.html)

Entrypoint module to compose other crates and start the application.

### `discover-hollywood-core` (module path: /core)
[local document link](http://localhost:8080/docs/discover_hollywood_core/index.html)

Domain models, usecase logics, and DB schema definition of the application.
By writing the frontend and backend in the same language, these core logics can be shared and reused on both sides through this package.

### `discover-hollywood-dataset` (module path: /dataset)
[local document link](http://localhost:8080/docs/discover_hollywood_dataset/index.html)

Movielens dataset loader.

### `discover-hollywood-client` (module path: /client)
local document link

Client WebAssembly application.

### `discover-hollywood-server` (module path: /server)
[local document link](http://localhost:8080/docs/discover_hollywood_server/index.html)

Backend server application.

## TODOs
* Movie recommendation (Item to item recommendation using collaborative filtering).
* Genre based search.
* Add tests.
* Improve presentation of the client app.
