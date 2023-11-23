# Personal Bluesky Feeds

Personal Bluesky feeds published using [Atmosfeed](https://github.com/pojntfx/atmosfeed).

[![hydrun CI](https://github.com/pojntfx/bluesky-feeds/actions/workflows/hydrun.yaml/badge.svg)](https://github.com/pojntfx/bluesky-feeds/actions/workflows/hydrun.yaml)
[![Matrix](https://img.shields.io/matrix/atmosfeed:matrix.org)](https://matrix.to/#/#atmosfeed:matrix.org?via=matrix.org)

## Overview

These are my personal Bluesky feeds, which are hosted on [Atmosfeed](https://atmosfeed.p8.lu/) and published to Bluesky automatically with a [GitHub action](./.github/workflows/hydrun.yaml) and [Atmosfeed's web frontend](https://atmosfeed.p8.lu/). To learn more about Atmosfeed, Scale and the role of classifiers, check out the [Atmosfeed GitHub repo](https://github.com/pojntfx/atmosfeed).

The following feeds are available:

- [Everything](https://bsky.app/profile/did:plc:jr5pspcicy5er44le6gdklnr/feed/everything): A test feed with everything (written in Go, see: [classifiers/everything/main.go](./classifiers/everything/main.go))
- [Trending](https://bsky.app/profile/did:plc:jr5pspcicy5er44le6gdklnr/feed/trending): Currently trending skeets (written in Go, see: [classifiers/trending/main.go](./classifiers/trending/main.go))
- [German](https://bsky.app/profile/did:plc:jr5pspcicy5er44le6gdklnr/feed/german): All German posts on Bluesky (written in Go, see: [classifiers/german/main.go](./classifiers/german/main.go))
- [Questions](https://bsky.app/profile/did:plc:jr5pspcicy5er44le6gdklnr/feed/questions): All Questions on Bluesky (written in Rust, see: [classifiers/questions/lib.rs](./classifiers/questions/lib.rs))

## Contributing

To contribute, please use the [GitHub flow](https://guides.github.com/introduction/flow/) and follow our [Code of Conduct](./CODE_OF_CONDUCT.md).

To build and start a development version of the feed classifiers locally, run the following:

```shell
$ git clone https://github.com/pojntfx/bluesky-feeds.git
$ cd bluesky-feeds

$ scale function build --release -d classifiers/questions # Build a feed classifier
$ scale function export local/questions:latest out # Export the classifier
$ atmosfeed-client dev --feed-classifier out/local-questions-latest.scale # Connect to Bluesky and run the classifier locally
```

For instructions on how to push the classifiers without pushing to this repo, see [the GitHub action](./.github/workflows/hydrun.yaml).

Have any questions or need help? Chat with us [on Matrix](https://matrix.to/#/#atmosfeed:matrix.org?via=matrix.org)!

## License

Personal Bluesky Feeds (c) 2023 Felicitas Pojtinger and contributors

SPDX-License-Identifier: Apache-2.0
