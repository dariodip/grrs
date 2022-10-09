<a name="readme-top"></a>

[![Contributors][contributors-shield]][contributors-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/dariodip/grrs">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">grrs</h3>

  <p align="center">
`grrs` (pronunced "grass) is a `grep` clone. We can give a string and a path and it’ll print only the lines that contain the given string.
    <br />
    <br />
    <br />
    <a href="https://github.com/dariodip/grrs/issues">Report Bug</a>
    ·
    <a href="https://github.com/dariodip/grrs/issues">Request Feature</a>
  </p>
</div>



<!-- ABOUT THE PROJECT -->
## About The Project

`grrs` has been created by following the book [Command Line Applications in Rust](https://rust-cli.github.io/book/index.html).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With

* [![Rust][Rust]][Rust-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## Git hooks configuration
Execute this command to setup git hooks:
```
git config --local core.hooksPath ./hooks
```

## Getting started

### Usage

A typical invocation of `grss` will look like this:

```bash
$ cat text.txt
	foo: 1
	bar: 2
	foobar: 3

$ grrs foobar text.txt
	foobar: 3
```


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/othneildrew/Best-README-Template.svg?style=for-the-badge
[contributors-url]: https://dariodip.github.io/
[issues-shield]: https://img.shields.io/github/issues/othneildrew/Best-README-Template.svg?style=for-the-badge
[issues-url]: https://github.com/dariodip/grrs/issues
[license-shield]: https://img.shields.io/github/license/othneildrew/Best-README-Template.svg?style=for-the-badge
[license-url]: https://github.com/dariodip/grrs/blob/master/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://www.linkedin.com/in/dario-di-pasquale
[Rust]: https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org
