### Amtrack: a Fully-Open Source Amtrak GetTrains & GetStations API and Package
This project aims to be a complete remake of piemadd's amtrak npm package, with exactly the same fields and structure. 
Piemadd's actual API is closed-source, and they only provide a wrapper package written in Javascript. Other rust packages
that claim to provide a rust-interface to this data just use piemadd's closed-source API. This package and api is different: it 
fetches data directly from Amtrak, and gives you access to all raw and formatted headers. You are welcome to use the API in your projects. 
Please note that the API is behind a redis cache-layer, as to avoid overuse of Amtrak's API and to drastically improve speeds. The cache layer
typically only adds 10-15 seconds of delay at worst. The package does not have this cache layer, and you are welcome to use it to quickly create
and host your own API. 
