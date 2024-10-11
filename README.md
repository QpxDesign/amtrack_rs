### Amtrack: a Fully-Open Source Amtrak GetTrains & GetStations API and Package
This project aims to be a complete remake of piemadd's amtrak npm package, with exactly the same fields and structure. Piemadd's actual API is closed-source, and they only provide a wrapper package written in Javascript. Other rust packages that claim to provide a rust-interface to this data just use piemadd's closed-source API. This package and api is different: it fetches data directly from Amtrak, and gives you access to all raw and formatted headers. You are welcome to use the API in your projects. Please note that the API is behind a redis cache-layer, as to avoid overuse of Amtrak's API and to drastically improve speeds. The cache layer typically only adds 10-15 seconds of delay at worst. The package does not have this cache layer, and you are welcome to use it to quickly create and host your own API.

#### API
We offer three API Endpoints:
- `https://amtrak-api.marcmap.app/get-trains`, which fetches live train data from Amtrak and formattes it identically to `api.amtraker.com`
- `https://amtrak-api.marcmap.app/get-stations`, which gets station metadata from a static file and formats it identically to `api.amtraker.com` (we use a static file as Amtrak stations very rarely change, but we plan on adding updated station data soon)
- `https://amtrak-api.marcmap.app/get-trains-raw`, raw, decrypted live train data

#### Package
- Our package has two functions: `GetTrains()` and `GetStations()`, which are the same as our `get-trains` and `get-stations` endpoints (only without the cache layer). `GetTrains` returns `Option<GetTrainsResponse>`, and `GetStations` returns a stringified JSON file as: `Option<String>`.

Install With:
`cargo add amtrack_rs`

#### Usage Limits
None! If things get crazy, I might add some IP-based rate limiting, but I highly doubt that will happen. If you want to support this project, or my other transit-based projects, please consider sponsoring me, or just star this project! If you use it in an app or website, please put a link to my website somewhere. (i.e. "data provided by https://quinnpatwardhan.com/") Not at all a requirement, just if you're willing.
