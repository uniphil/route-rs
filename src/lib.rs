#[macro_export]
macro_rules! seg {
    ( $s:expr, $p:ident, $segment:tt ) => (
        $p += 1;  // advance past '/' sep
        if $p >= $s.len() {  // done so soon?
            break
        }
        let end = $s[$p..]
            .find("/")
            .map(|i| $p + i)
            .unwrap_or($s.len());
        seg!($s, $p, end, $segment);
    );
    ( $s:expr, $p:ident, $end:ident, [_] ) => (
        $p = $end;
    );
    ( $s:expr, $p:ident, $end:ident, [ $n:ident ] ) => (
        let $n = &$s[$p..$end];
        $p = $end;
    );
    ( $s:expr, $p:ident, $end:ident, [ $n:ident : $t:ty ] ) => (
        let $n: $t;
        match $s[$p..$end].parse::<$t>() {
            Ok(v) =>
                $n = v,
            Err(_) =>
                break,
        }
        $p = $end;
    );
    ( $s:expr, $p:ident, $end:ident, $e:expr ) => (
        if &$s[$p..$end] == $e {
            $p = $end;
        } else {
            break
        }
    );
}

#[macro_export]
macro_rules! split {
    ( $s:expr, $p:ident, ( / $( $segment:tt )/ * ) ) => (
        $( seg!($s, $p, $segment); )*
        if !($p == $s.len() ||
             $p == $s.len() - 1 && &$s[$p..] == "/") {
            break
        }
    );
    ( $s:expr, $p:ident, ( / $( $segment:tt )/ * [ / $rest:ident .. ] ) ) => (
        $( seg!($s, $p, $segment); )*
        let $rest = &$s[$p..];
    );
}

#[macro_export]
macro_rules! route {
    ( $path:expr , {
        $( $m:tt => $handle:expr ; )*
    } ) => (
        $(loop {
            let mut p = 0;
            split!($path, p, $m);
            return $handle;
        })*
    );
    ( $path:expr , {
        $( $m:tt => $handle:expr ); *    // missing trailing comma
    } ) => (
        route_fn!($path, {
            $( $m => $handle , )*
        })
    );
}


#[test]
fn test_seg_macro() {
    {
        let mut ok = false;
        let mut p = 1;
        let end = 5;
        let s = "/asdf";
        loop {
            seg!(s, p, end, [hello]);
            ok = true;
            assert_eq!(hello, "asdf");
            break;
        }
        assert_eq!(ok, true);
    }

    {
        let mut ok = false;
        let mut p = 1;
        let end = 5;
        let s = "/asdf";
        loop {
            seg!(s, p, end, "asdf");
            ok = true;
            break;
        }
        assert_eq!(ok, true, "segment matched");
        assert_eq!(p, 5);
    }

    {
        let mut ok = false;
        let mut p = 1;
        let end = 5;
        let s = "/asdf";
        loop {
            seg!(s, p, end, "fdsa");
            ok = true;
            break;
        }
        assert_eq!(ok, false, "should not match segment");
        assert_eq!(p, 1);
    }
}

#[test]
fn test_split_macro() {
    {
        let s = "/";
        let mut p = 0;
        let mut ok = false;
        loop {
            split!(s, p, (/));
            ok = true;
            break
        }
        assert_eq!(ok, true);
    }
    {
        let s = "/uniphil";
        let mut p = 0;
        let mut ok = false;
        loop {
            split!(s, p, (/[username]));
            ok = true;
            assert_eq!(username, "uniphil");
            break
        }
        assert_eq!(ok, true);
    }
    {
        let s = "/abc";
        let mut p = 0;
        let mut ok = false;
        loop {
            split!(s, p, (/"abc"));
            ok = true;
            break
        }
        assert_eq!(ok, true);
    }
    {
        let s = "/abc/xyz";
        let mut p = 0;
        let mut ok = false;
        loop {
            split!(s, p, (/"abc"/"xyz"));
            ok = true;
            break
        }
        assert_eq!(ok, true);
    }
    {
        let s = "/abc/xyz";
        let mut p = 0;
        let mut ok = false;
        loop {
            split!(s, p, (/"abc"/"xy"));
            ok = true;
            break
        }
        assert_eq!(ok, false);
    }
    {
        let s = "/abc/xyz/qrs";
        let mut p = 0;
        let mut ok = false;
        loop {
            split!(s, p, (/"abc"[/rest..]));
            ok = true;
            assert_eq!(rest, "/xyz/qrs");
            break;
        }
        assert_eq!(ok, true);
    }
}

#[test]
fn test_route() {

    struct Request<'a> {
        path: &'a str,
    }

    type Response = String;

    fn home(req: &Request) -> Response {
        "home".to_string()
    }

    fn blog_post(req: &Request, id: u32) -> Response {
        format!("blog: {}", id)
    }

    fn account(req: &Request, subpath: &str) -> Response {
        format!("account -- subpath: {}", subpath)
    }

    fn handle_route(req: &Request) -> Response {
        route!(req.path, {
            (/)                 => home(req);
            (/"blog"/[id: u32]) => blog_post(req, id);
            (/"me"[/rest..])    => account(req, rest);
        });
        Response::from("not found")
    }

    assert_eq!(&handle_route(&Request { path: "/" }), "home");
    assert_eq!(&handle_route(&Request { path: "/blog/42" }), "blog: 42");
    assert_eq!(&handle_route(&Request { path: "/me/a/b/c" }), "account -- subpath: /a/b/c");
    assert_eq!(&handle_route(&Request { path: "/foo" }), "not found");
}
