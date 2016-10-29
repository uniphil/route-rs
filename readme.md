# route-rs

Safely match and parse URL routes

[![Build Status](https://travis-ci.org/uniphil/route-rs.svg?branch=master)](https://travis-ci.org/uniphil/route-rs)
[![Crates.io Badge](https://img.shields.io/crates/v/route.svg)](https://crates.io/crates/route)


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
