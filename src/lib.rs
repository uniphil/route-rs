macro_rules! seg {
    ( $s:ident, $p:ident, $end:ident, ( $t:ty ) ) => (
        if $s[$p..$end].parse::<$t>().is_ok() {
            $p = $end;
        } else {
            break
        }
    );
    ( $s:ident, $p:ident, $end:ident, ( $n:ident : $t:ty ) ) => (
        let parsed = $s[$p..$end].parse::<$t>();
        if parsed.is_err() {
            break
        }
        let $n = parsed.unwrap();
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

#[test]
fn seg_test() {
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

macro_rules! split {
    ( $s:ident, $p:ident, ( / $( $segment:tt )/ * ) ) => (
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
}

#[test]
fn test_split() {
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
}

macro_rules! route {
    ( $s:ident, $( $path:tt => $handle:expr ; )* ) => ({
        let mut p;
        $(
            loop {
                p = 0;
                split!($s, p, $path);
                if !(p == $s.len() ||
                     p == $s.len() - 1 && &$s[p..] == "/") {
                    break
                }
                return $handle;
            }
        )*
    });
}


#[test]
fn test_route() {

    #[derive(Debug, PartialEq, Eq)]
    enum Page {
        Home,
        BlogIndex,
        BlogPost(u32),
        BlogEdit(u32),
        User(String),
        NotFound,
    }

    fn route<'a>(path: &'a str) -> Page {
        route!(path,
            (/) => Page::Home;
            (/"blog") => Page::BlogIndex;
            (/"blog"/(id: u32)) => Page::BlogPost(id);
            (/"blog"/(id: u32)/"edit") => Page::BlogEdit(id);
            (/"u"/(handle: String)) => Page::User(handle);
        );
        Page::NotFound
    }

    assert_eq!(route("/"), Page::Home);
    assert_eq!(route("/blog"), Page::BlogIndex);
    assert_eq!(route("/blog/42"), Page::BlogPost(42));
    assert_eq!(route("/blog/42/edit"), Page::BlogEdit(42));
    assert_eq!(route("/u/uniphil"), Page::User("uniphil".to_string()));
    assert_eq!(route("/asdf"), Page::NotFound);
    assert_eq!(route("/blog/abc"), Page::NotFound);
}
