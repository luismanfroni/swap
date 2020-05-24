<h1 align="center">SWAP</h1>
<div align="center">
    🛡📦🗄🗃📬
</div>
<div align="center">
    <h2><b>Secured With AppArmor Packages</b></h2>
</div>
<div align="center">
    Is an Package Manager to introduce packages with AppArmor profiles easily to the end user.
</div>

<br />

<div align="center">
    <a href="https://github.com/luismanfroni/swap/releases">
        <img alt="GitHub milestone"
            src="https://img.shields.io/github/milestones/progress-percent/luismanfroni/swap/1?style=for-the-badge&color=%2323B023"
            />
    </a>
</div>

<br />

<!-- PROJECT SHIELDS -->
<div align="center">
    <a href="https://github.com/luismanfroni/swap/blob/master/LICENSE">
        <img src="https://img.shields.io/github/license/luismanfroni/swap.svg?style=for-the-badge" />
    </a>
    <a href="https://github.com/luismanfroni/swap/stargazers">
        <img src="https://img.shields.io/github/stars/luismanfroni/swap.svg?style=for-the-badge" />
    </a>
    <a href="https://github.com/luismanfroni/swap/issues?q=is%3Aopen+is%3Aissue+label%3Aenhancement">
        <img src="https://img.shields.io/github/issues-raw/luismanfroni/swap/enhancement?color=%23003388&label=features&style=for-the-badge">
    </a>
    <a href="https://github.com/luismanfroni/swap/issues?q=is%3Aopen+is%3Aissue+label%3Abug">
        <img src="https://img.shields.io/github/issues-raw/luismanfroni/swap/bug?color=%23D83333&label=bugs&style=for-the-badge">
    </a>
</div>


<br />

<!-- PROJECT LOGO -->

<p align="center">
    <a href="https://github.com/luismanfroni/swap/releases">
        <img src="https://img.shields.io/github/downloads/luismanfroni/swap/master/total.svg?style=for-the-badge" />
    </a>
    <br />
  <p align="center">
    <a href="https://github.com/luismanfroni/swap/releases">View Last Releases</a>
    ·
    <a href="https://github.com/luismanfroni/swap/issues">Report Bug</a>
    ·
    <a href="https://github.com/luismanfroni/swap/issues">Request Feature</a>
  </p>
</p>



<!-- TABLE OF CONTENTS -->
## Table of Contents

* [About the Project](#about-the-project)
  * [Built With](#built-with)
* [Roadmap](#roadmap)
* [License](#license)
* [Contact](#contact)



<!-- ABOUT THE PROJECT -->
## About The Project

There are quite many packages and binaries out there, one quite will not be available in the desired version on your favorite package manager. This idea came when i had to download an app binary, it wasn't being distributed in any package managers, also i didn't quite trust it, so i made a profile to confine it with AppArmor, that comes on Ubuntu.

Here's why this isn't the same as "any other package manager":
* Centralized, so there aren't many repositories to add, to get your desired package.
* No virtualization/container like Snap or Flatpaks.
* Secured by AppArmor profile defined on the package.

The only external dependeny is AppArmor, without, would be the same thing as executing the binary unsafely.

## Built With
Using [Rust Language](https://www.rust-lang.org/).
Libaries:
* [Manager](https://github.com/luismanfroni/swap/tree/master/manager)
    * [brotli](https://github.com/dropbox/rust-brotli)
    * [tar](https://github.com/alexcrichton/tar-rs)

* [CLI](https://github.com/luismanfroni/swap/tree/master/cli)
    * [Clap](https://github.com/clap-rs/clap)



<!-- GETTING STARTED --
## Getting Started

### Prerequisites

### Installation

<!-- USAGE EXAMPLES --
## Usage-->


<!-- ROADMAP -->
## Roadmap

See the [open issues](https://github.com/luismanfroni/swap/issues) for a list of proposed features (and known issues).


<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.



<!-- CONTACT -->
## Contact

Luis Felipe Manfroni - [@siul_manfroni](https://twitter.com/siul_manfroni) - luis.manfroni@catolicasc.org.br

Project Link: [https://github.com/luismanfroni/swap](https://github.com/luismanfroni/swap)



<!-- README Template: https://github.com/othneildrew/Best-README-Template -->