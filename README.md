<div id="readme-top"></div>

<h1 align="center">
  <br />
    <a href="https://xodium.org/">
      <img src="https://gist.githubusercontent.com/illyrius666/a38f03b4fbe9b43faa2c5623137c1250/raw/3a1410e77807097bcfbcf963822b41fadd495d9f/xodium.svg" alt="Xodium Logo" width="200">
    </a>
  <br /><br />
  xbim
  <br /><br />
</h1>

<h4 align="center">Xodium Building Information Modeling</h4><br />

<div align="center">

[![Contributors][contributors_shield_url]][contributors_url]
[![Issues][issues_shield_url]][issues_url]
[![Roadmap][roadmap_shield_url]][roadmap_url]<br />
[![Deps.rs][deps_shield_url]][deps_url]
[![LINUX ONLY][linux_only_shield_url]][linux_only_url]

</div>

## Table of Contents

- [About The Project](#about-the-project)
- [Usage](#usage)
- [Built With](#built-with)
- [Code of Conduct][code_of_conduct_url]
- [Contributing][contributing_url]
- [License][license_url]

## About The Project

xbim is a project aimed to provide a complete solution for working with BIM models. It is written in Rust, which
provides a high level of performance and safety. The project is still in its early stages, but it is already capable of
reading and writing IFC files.

## Usage

1. Download the latest version of xbim from the [release][release_latest] page.
2. Place it in a directory of your choice.
3. Run the executable. It will return an error that it cannot connect to the database. This is expected, as the
   database is not yet set up in the config.toml which will generate on first time run.
4. Replace the default values in the config with yours.
5. Rerun the executable and voila!

## Built With

<div align="center">

[![Built With][built_with_shield_url]][built_with_url]

</div>

<p align="right"><a href="#readme-top">▲</a></p>

[built_with_shield_url]: https://skillicons.dev/icons?i=linux,rust,github,githubactions

[built_with_url]: https://skillicons.dev

[code_of_conduct_url]: https://github.com/XodiumSoftware/xbim?tab=coc-ov-file

[contributing_url]: https://github.com/XodiumSoftware/xbim/blob/main/CONTRIBUTING.md

[contributors_shield_url]: https://img.shields.io/github/contributors/XodiumSoftware/xbim?style=for-the-badge&color=blue

[contributors_url]: https://github.com/XodiumSoftware/xbim/graphs/contributors

[deps_shield_url]: https://deps.rs/repo/github/XodiumSoftware/xbim/status.svg?style=for-the-badge

[deps_url]: https://deps.rs/repo/github/XodiumSoftware/xbim

[issues_shield_url]: https://img.shields.io/github/issues/XodiumSoftware/xbim?style=for-the-badge&color=yellow

[issues_url]: https://github.com/XodiumSoftware/xbim/issues

[license_url]: https://github.com/XodiumSoftware/xbim?tab=AGPL-3.0-1-ov-file

[linux_only_shield_url]: https://img.shields.io/badge/OS-Linux%20Only-red.svg?style=for-the-badge

[linux_only_url]: https://www.linux.org/

[release_latest]: https://github.com/XodiumSoftware/xbim/releases/latest

[roadmap_shield_url]: https://img.shields.io/badge/Roadmap-Click%20Me!-purple.svg?style=for-the-badge

[roadmap_url]: https://github.com/orgs/XodiumSoftware/projects/4
