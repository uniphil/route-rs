# route-rs

[![Build Status](https://travis-ci.org/uniphil/route-rs.svg?branch=master)](https://travis-ci.org/uniphil/route-rs)
[![Crates.io Badge](https://img.shields.io/crates/v/route.svg)](https://crates.io/crates/route)

This crate is my attempt at a safe helper for mapping URL routes to handlers for rust web applications.

There are several routing libraries already available, but all of the ones I have been able to find share a common problem: path patterns are defined as strings, and URL parameters get parsed out at runtime and stashed in some kind of `Map` that handlers have to `.get()` and `.unwrap()` to access. I want to extract parameters without unwrapping, and I want rust's type system to ensure that I'm not making mistakes!


## Setup

in Cargo.toml:

```toml
[dependencies]
route = "0.2.0"
```

`route` just exports a the macro `route!`, so you need to `#[macro use]` it:

```rust
#[macro use]
extern crate route;
```


## Usage

Suppose you have some HTTP request/response server setup like

```rust

// imaginary request/response structs provided by the framework:

struct Request<'a> {
    path: &'a str,
}

type Response = String;


// application handlers that we need to route:
// Note that some handlers take extra parameters that we hope to fill from the path!

fn home(req: &Request) -> Response {
    "home".to_string()
}

fn blog_post(req: &Request, id: u32) -> Response {
    format!("blog: {}", id)
}

fn account(req: &Request, subpath: &str) -> Response {
    format!("account -- subpath: {}", subpath)
}
```

Then you could set up a routing handler like:

```rust
fn handle_route(req: &Request) -> Response {
    route!(req.path, {
        (/)                 => home(req);
        (/"blog"/[id: u32]) => blog_post(req, id);
        (/"me"[/rest..])    => account(req, rest);
    });
    Response::from("not found")
}
```

And you're set!

```rust
assert_eq!(&handle_route(&Request { path: "/" }), "home");
assert_eq!(&handle_route(&Request { path: "/blog/42" }), "blog: 42");
assert_eq!(&handle_route(&Request { path: "/me/a/b/c" }), "account -- subpath: /a/b/c");
assert_eq!(&handle_route(&Request { path: "/foo" }), "not found");
```
