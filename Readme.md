<!-- PROJECT LOGO -->
<br />
<div align="center">
  <h2 align="center">Near Protocol | Pack of examples for Cross Contract Call feature</h2>

  <p align="center">
    The repository that contains real code examples of work with Cross Contract Calls in <a href="https://docs.rs/near-sdk/latest/near_sdk/">Near Protocol Rust SDK</a>
    <br />
    <br />
    <a href="https://github.com/nearuaguild"> Explore other projects</a>
    ·
    <a href="https://github.com/nearuaguild/near-rust-cross-contract-examples/issues">Report a bug</a>
  </p>
</div>

<!-- ABOUT THE PROJECT -->

## About The Project

The repository provides practical use cases that showcase Cross Contract Calls in action

This pack of examples covers everything from simple implementation to managing parallel and sequential requests with callbacks

### Examples

[![Basic][basic]](https://github.com/nearuaguild/near-rust-cross-contract-examples/tree/main/basic)

[![Sequence][sequence]](https://github.com/nearuaguild/near-rust-cross-contract-examples/tree/main/sequence)

[![Parallel][parallel]](https://github.com/nearuaguild/near-rust-cross-contract-examples/tree/main/parallel)

[![State rollback][state_rollback]](https://github.com/nearuaguild/near-rust-cross-contract-examples/tree/main/state_rollback)


### Built With

- [![Rust][rust]][rust-url]
- [near-sdk-rs (v4.0.0)](https://github.com/near/near-sdk-rs)

---

<!-- GETTING STARTED -->

## Getting Started

💡 _Before starting, please ensure that you have all the necessary installations completed_

- [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Cargo](https://github.com/rust-lang/cargo#compiling-from-source)
- [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git/)

### Installation

Follow these simple instructions to set up a local development environment

1. Clone the repo
  ```sh
   git clone https://github.com/nearuaguild/near-rust-cross-contract-examples.git
  ```
2. Open root folder 
  ```sh
  cd near-rust-cross-contract-examples
  ```
3. Move to a folder of the example you're interesting in (e.g. `state_rollback`)
  ```sh
  cd state_rollback
  ```

## Build

Each example has script that performs compiling of the project

Make sure you've added `wasm32-unknown-unknown` target for `rustc` compiler

If not, run this command
```sh
rustup target add wasm32-unknown-unknown
```

Finally, run `./build.sh` inside the example folder to compile to WASM

> Keep in mind that compiled WASM file would be located in `target/wasm32-unknown-unknown/release/{EXAMPLE_NAME}.wasm

## Usage

Please note that the provided examples are not intended to be run locally, and because of it there's no deployment guide

You can use them to improve your coding skills/understanding of Near Protocol architecture or directly incorporate them into your project

## Bonus+

Also, consider reading this [beautiful post](https://stackoverflow.com/questions/70452485/exceeded-prepaid-gas-common-solutions) that outlines common solutions for Exceeded Gas problems that can arise when working with Cross Contract Calls

---

## Developed by

![Guild cover][cover]

**Near Ukraine Guild** is a fast-growing guild based in Ukraine, aimed at providing high-quality educational content and assistance to grow a strong community of developers/entrepreneurs/enthusiasts within the Near Protocol ecosystem

## Community Validator Node

![Community Validator cover][validator]

Our validator has been active for a few months now, and the funds it generates are being put towards sponsoring community activities

Join us now to stake and earn 10% APY

**Click below to get started 👇**

<a href="https://bit.ly/43GSKhs" target="_blank">
<img src="https://img.shields.io/badge/stake-red?style=for-the-badge"  height="48" />
</a>

## Socials

[![Twitter][twitter]][twitter-url]
[![Youtube][youtube]][youtube-url]
[![Telegram Chat][telegram-chat]][telegram-chat-url]
[![Telegram Channel][telegram-channel]][telegram-channel-url]
[![Medium][medium]][medium-url]
[![Github][github]][github-url]

<!-- Images -->

[cover]: https://github.com/nearuaguild/.github/blob/main/images/cover.png
[validator]: https://github.com/nearuaguild/.github/blob/main/images/validator.png

<!-- Socials -->

[twitter]: https://img.shields.io/badge/news-1DA1F2?style=for-the-badge&logo=twitter&logoColor=white
[youtube]: https://img.shields.io/badge/broadcasting-282828?style=for-the-badge&logo=youtube&logoColor=ff0000
[medium]: https://img.shields.io/badge/articles-202020?style=for-the-badge&logo=medium&logoColor=ffffff
[telegram-chat]: https://img.shields.io/badge/chat-229ED9?style=for-the-badge&logo=telegram&logoColor=white
[telegram-channel]: https://img.shields.io/badge/channel-229ED9?style=for-the-badge&logo=telegram&logoColor=white
[github]: https://img.shields.io/badge/code-000000?style=for-the-badge&logo=github&logoColor=ffffff
[twitter-url]: https://twitter.com/nearuaguild
[youtube-url]: https://www.youtube.com/@nearprotocolukraineguild4064
[medium-url]: https://medium.com/near-protocol-ua
[telegram-chat-url]: https://t.me/nearprotocolua
[telegram-channel-url]: https://t.me/nearprotocoluachannel
[github-url]: https://github.com/nearuaguild

<!-- CTA -->

[stake]: https://img.shields.io/badge/stake-yellow?style=for-the-badge
[stake-url]: https://bit.ly/43GSKhs

<!-- LICENSE -->

## License

See `LICENSE.txt` for more information

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

<!-- Built with -->

[basic]: https://img.shields.io/badge/making_basic_request_with_callback-DEEBF7?style=for-the-badge
[parallel]: https://img.shields.io/badge/sending_parallel_requests_with_single_callback-5B9BD5?style=for-the-badge
[sequence]: https://img.shields.io/badge/making_sequence_of_requests_with_callback-5B9BD5?style=for-the-badge
[state_rollback]: https://img.shields.io/badge/transferring_funds_and_rolling_back_state_if_failed-1F4E79?style=for-the-badge

<!-- Built with -->

[rust]: https://img.shields.io/badge/rust-000000?style=for-the-badge&logo=rust&logoColor=white
[rust-url]: https://www.rust-lang.org/

[javascript]: https://img.shields.io/badge/javascript-000000?style=for-the-badge&logo=javascript&logoColor=F7E018
[javascript-url]: https://developer.mozilla.org/en-US/docs/Web/JavaScript
[assemblyscript]: https://img.shields.io/badge/assembly%20script-1B7ACE?style=for-the-badge&logo=assemblyscript&logoColor=white
[assemblyscript-url]: https://www.assemblyscript.org/
[littlelink]: https://img.shields.io/badge/LittleLink-1D84FF?style=for-the-badge
[littlelink-url]: https://littlelink.io/