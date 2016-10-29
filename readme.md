# route-rs

[![Build Status](https://travis-ci.org/uniphil/route-rs.svg?branch=master)](https://travis-ci.org/uniphil/route-rs)
[![Crates.io Badge](https://img.shields.io/crates/v/route.svg)](https://crates.io/crates/route)

This crate is my attempt at a safe helper for mapping URL routes to handlers for rust web applications.

There are several routing libraries already available, but all of the ones I have been able to find share a common problem: path patterns are defined as strings, and URL parameters get parsed out at runtime and stashed in some kind of `Map` that handlers have to `.get()` and `.unwrap()` to access. I want to extract parameters without unwrapping, and I want rust's type system to ensure that I'm not making mistakes!

The current form is a macro, `route_fn!`, which creates a function mapping a `path: &str` to a member of an `enum` that you provide:

```rust
#[macro use]
extern crate route;

#[derive(Debug, PartialEq, Eq)]
enum Page<'a> {
    Home,
    BlogIndex,
    BlogPost(u32),
    BlogEdit(u32),
    User(&'a str),
    Account(&'a str),
    NotFound,
}

route_fn!(route -> Page {
    (/)                         => Page::Home,
    (/"blog")                   => Page::BlogIndex,
    (/"blog"/[id: u32])         => Page::BlogPost(id),
    (/"blog"/[id: u32]/"edit")  => Page::BlogEdit(id),
    (/"blog"/[id: u32]/[_])     => Page::BlogEdit(id),  // ignored slug
    (/"u"/[handle])             => Page::User(handle),
    (/"me"[/rest..])            => Page::Account(rest),
}, Page::NotFound);
```

You can now use the function `Fn(&str) -> Page` called 'route' created by the macro to match paths:

```rust
#[test]
fn test_route() {
    assert_eq!(route("/"), Page::Home);
    assert_eq!(route("/blog"), Page::BlogIndex);
    assert_eq!(route("/blog/42"), Page::BlogPost(42));
    assert_eq!(route("/blog/42/edit"), Page::BlogEdit(42));
    assert_eq!(route("/u/uniphil"), Page::User("uniphil"));
    assert_eq!(route("/asdf"), Page::NotFound);
    assert_eq!(route("/blog/abc"), Page::NotFound);
    assert_eq!(route("/me/a/b/c/d/e/f/g"), Page::Account("/a/b/c/d/e/f/g"));
}
```

`route()` will return a member of `Page`, so if you want to map it to, say, an Iron handler:

```rust
fn home_handler() -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello world!")))
}

fn blog_post_handler(id: u32) -> IronResult<Response> {
    Ok(Response::with((status::Ok, format!("This is blog post #{}", id))))
}

fn route_handler(req: &mut Request) -> IronResult<Response> {
    let path = format!("/{}", req.url.path().join("/"));
    match route(&path) {
        Page::Home => home_handler(),
        Page::BlogPost(id) => blog_post_handler(id),
        ...
    }
}

fn main() {
    Iron::new(route_handler).http("localhost:3000").unwrap();
}
