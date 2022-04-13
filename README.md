# ganjoor-pretty

Go to the page for any _ghazal_ or _qaṣīdah_ on [Ganjoor](https://ganjoor.net/), and replace `.net` with `.t6e.dev` to see a pretty-printed version of the poem.

For example, <https://ganjoor.net/hafez/ghazal/sh250> becomes <https://ganjoor.t6e.dev/hafez/ghazal/sh250>.

The developers at Ganjoor were gracious enough to add this functionality to the site itself. You can activate it with a button labeled _numā-yi chāpī_. I'm continuing to maintain my service so that I can improve it over time.

## Technical notes

This is basically a little Rust application, built with [Actix Web](https://actix.rs/). It takes a Ganjoor URL, fetches and parses the relevant contents of the page (using the [isahc](https://github.com/sagebind/isahc) and [scraper](https://github.com/causal-agent/scraper) crates), and constructs a new HTML document with [Pandoc](https://github.com/jgm/pandoc)---which is a required external dependency. Note that we're asking Pandoc to bundle in `head.html` and `script.html`, which in turn reference `styles.css` and `pretty.js`. So those files also need to be present. The application listens on localhost port 5779 (chosen at random) and can be put on a server behind a reverse proxy, with a CDN for caching, etc. Pretty straightforward.
