# ⚽ Football Emporium ⚽

Search through football match results from 2010 to 2026 with a web interface.
* Based on [openfootball](https://github.com/openfootball) json data.
* Powered by Rust and Axum on the backend and Vue.js on the frontend.
* Match results can be filtered cumulatively and paginated from the Vue.js app on the frontend.

![Football Emporium](https://github.com/burumdev/football-search-axum-vuejs/blob/main/screenshot.jpg)

# Installing

* The app requires Rust build system for the backend and Node.js + npm build system for the frontend.
* The Rust build script `build.rs` will automatically build the frontend along with the backend but for the first time, navigate to `ui` directory and run:
  * `npm install`
* Make a copy of `.env.example` as `.env`. Contents can stay the same for development environment as there're no secrets in `.env` because we use an internal in-memory database built from scratch with the json file data.

## License

[MIT](https://choosealicense.com/licenses/mit/)
