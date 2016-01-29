Virgil
======

[![Build Status](https://travis-ci.org/azdle/virgil.svg?branch=master)](https://travis-ci.org/azdle/virgil)
[![Crates.io Link](http://meritbadge.herokuapp.com/virgil)](https://crates.io/crates/virgil)

Yet another static site generator, this time written in rust.

Status
------

**Virgil is incomplete and you likely shouldn't use it.**


Getting Started
---------------
The easiest way to use Virgil is to install it through cargo.

```
$ cargo install virgil
```

Then create a new directory and initialize a Virgil site.

```
$ mkdir my-site
$ cd my-site
$ virgil init
```

Next you'll need to create some templates and markdown files. Virgil converts
any markdown file (except anything in a `_*` directory) into an HTML file and
recreates the same structure of files under a `_site` directory.

You'll then need a mustache template, by default this should be under
`_templates/default`. You'll need a `post.mustache` with a `{{{body}}}` in it,
this is where the markdown will be rendered to. Anything in the directory
`_templates/default/static` will be copied to `_site` as-is.

Once your site is setup, you can generate your site.

```
$ virgil
# or
$ virgil build
```
