# Personal Bluesky Feeds

Personal Bluesky feeds published using [Atmosfeed](https://github.com/pojntfx/atmosfeed).

## Overview

🚧 This project is a work-in-progress! Instructions will be added as soon as it is usable. 🚧

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
