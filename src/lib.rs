macro_rules! seg {
    ( $s:ident, $p:ident, $end:ident, [_] ) => (
        $p = $end;
    );
    ( $s:ident, $p:ident, $end:ident, [ $n:ident ] ) => (
        let $n = &$s[$p..$end];
        $p = $end;
    );
    ( $s:ident, $p:ident, $end:ident, [ $n:ident : $t:ty ] ) => (
        let $n: $t;
        match $s[$p..$end].parse::<$t>() {
            Ok(v) =>
                $n = v,
            Err(_) =>
                break,
        }
        $p = $end;
    );
    ( $s:ident, $p:ident, $end:ident, $e:expr ) => (
        if &$s[$p..$end] == $e {
            $p = $end;
        } else {
            break
        }
    );
}

macro_rules! split {
    ( $s:ident, $p:ident, $term:ident, ( / $( $segment:tt )/ * ) ) => (
        $(
            $p += 1;  // advance past '/' sep
            if $p >= $s.len() {  // done so soon?
                break
            }
            let end = $s[$p..]
                .find("/")
                .map(|i| $p + i)
                .unwrap_or($s.len());
            seg!($s, $p, end, $segment);
        )*
    );
    ( $s:ident, $p:ident, $term:ident, ( / $( $segment:tt )/ * [ / $rest:ident .. ] ) ) => (
        $term = false;
        split!($s, $p, $term, ( / $( $segment )/ * ) );
        let $rest = &$s[$p..];
    );
}

macro_rules! route {
    ( $s:ident, $( $path:tt => $handle:expr ; )* ) => ({
        let mut p;
        $(
            loop {
                let mut terminate = true;
                p = 0;
                split!($s, p, terminate, $path);
                if terminate &&
                   !(p == $s.len() ||
                     p == $s.len() - 1 && &$s[p..] == "/") {
                    break
                }
                return $handle;
            }
        )*
    });
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
        let mut terminate = true;
        loop {
            split!(s, p, terminate, (/));
            ok = true;
            break
        }
        assert_eq!(ok, true);
    }
    {
        let s = "/uniphil";
        let mut p = 0;
        let mut ok = false;
        let mut terminate = true;
        loop {
            split!(s, p, terminate, (/[username]));
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
        let mut terminate = true;
        loop {
            split!(s, p, terminate, (/"abc"));
            ok = true;
            break
        }
        assert_eq!(ok, true);
    }
    {
        let s = "/abc/xyz";
        let mut p = 0;
        let mut ok = false;
        let mut terminate = true;
        loop {
            split!(s, p, terminate, (/"abc"/"xyz"));
            ok = true;
            break
        }
        assert_eq!(ok, true);
    }
    {
        let s = "/abc/xyz";
        let mut p = 0;
        let mut ok = false;
        let mut terminate = true;
        loop {
            split!(s, p, terminate, (/"abc"/"xy"));
            ok = true;
            break
        }
        assert_eq!(ok, false);
    }
    {
        let s = "/abc/xyz/qrs";
        let mut p = 0;
        let mut ok = false;
        let mut terminate = true;
        loop {
            split!(s, p, terminate, (/"abc"[/rest..]));
            ok = true;
            assert_eq!(rest, "/xyz/qrs");
            break;
        }
        assert_eq!(terminate, false);
        assert_eq!(ok, true);
    }
}

#[test]
fn test_route() {

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

    fn route(path: &str) -> Page {
        route!(path,
            (/)                         => Page::Home;
            (/"blog")                   => Page::BlogIndex;
            (/"blog"/[id: u32])         => Page::BlogPost(id);
            (/"blog"/[id: u32]/"edit")  => Page::BlogEdit(id);
            (/"blog"/[id: u32]/[_])     => Page::BlogEdit(id);  // ignored slug
            (/"u"/[handle])             => Page::User(handle);
            (/"me"[/rest..])            => Page::Account(rest);
        );
        Page::NotFound
    }

    assert_eq!(route("/"), Page::Home);
    assert_eq!(route("/blog"), Page::BlogIndex);
    assert_eq!(route("/blog/42"), Page::BlogPost(42));
    assert_eq!(route("/blog/42/edit"), Page::BlogEdit(42));
    assert_eq!(route("/u/uniphil"), Page::User("uniphil"));
    assert_eq!(route("/asdf"), Page::NotFound);
    assert_eq!(route("/blog/abc"), Page::NotFound);
    assert_eq!(route("/me/a/b/c/d/e/f/g"), Page::Account("/a/b/c/d/e/f/g"));
}
