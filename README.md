# ganjoor-pretty

Go to the page for any _ghazal_ or _qaṣīdah_ on [Ganjoor](https://ganjoor.net/),
and replace `.net` with `.fly.dev` to see a pretty-printed version of the poem.

For example, <https://ganjoor.net/hafez/ghazal/sh407> becomes
<https://ganjoor.fly.dev/hafez/ghazal/sh407>.

The developers at Ganjoor were gracious enough to add this functionality to the
site itself. You can activate it with a button labeled _numā-yi chāpī_. I'm
continuing to maintain my service so that I can improve it over time.

_NB: This is the successor to an earlier mini-project,
[ghazal-typesetting](https://github.com/theodore-s-beers/ghazal-typesetting),
which may still be of interest to some._

## Technical notes

This is basically a little Rust application, built with
[Actix Web](https://actix.rs/). It takes a Ganjoor URL, fetches the relevant
poem through their API (using the
[reqwest](https://github.com/seanmonstar/reqwest) crate), and constructs a new
HTML document with [Pandoc](https://github.com/jgm/pandoc)—which is a required
external dependency. Note that we're asking Pandoc to bundle in `head.html`,
which in turn references `styles.css` and `pretty.js`. So those files also need
to be present. The application listens on localhost port 8080 and could be put
on a server behind a reverse proxy, with a CDN for caching, etc. Pretty
straightforward.
