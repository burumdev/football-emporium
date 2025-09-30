# ⚽ Football Emporium ⚽

Search through football match results from 2010 to 2026 with a web interface. Based on [openfootball](https://github.com/openfootball) json data. Powered by Rust and Axum on the backend and VueJs on the frontend.
Match results can be filtered cumulatively and paginated from the VueJs app on the frontend.

# Installing

* The app requires rust build system for the backend and Node + npm build system for the frontend.
* Make a copy of `.env.example` as `.env`. Contents can stay the same for development environment as there're no secrets in `.env` because we use an internal in-memory database built from scratch with the json file data.

## License

[MIT](https://choosealicense.com/licenses/mit/)
