macro_rules! seg {
    // ( $s: ident, $p: ident, ( * ) ) => ("star");
    // ( $s: ident, $p: ident, | ( * ) ) => ("last star");
    ( $s: ident, $p: ident, ( $t:ty ) ) => (
        $p += 1;  // leading '/'
        if $p >= $s.len() {
            break;
        }
        let seg_end = $s[$p..]
            .find("/")
            .map(|i| $p + i)
            .unwrap_or($s.len());
        if $s[$p..seg_end].parse::<$t>().is_ok() {
            $p = seg_end;
        } else {
            break;
        }
    );
    // ( $s: ident, $p: ident, | ( $t:ty ) ) => ("last type");
    // ( $s: ident, $p: ident, ( $n:ident : * ) ) => ("named star");
    // ( $s: ident, $p: ident, | ( $n:ident : * ) ) => ("named last star");
    ( $s: ident, $p: ident, ( $n:ident : $t:ty ) ) => (
        $p += 1;  // leading '/'
        if $p >= $s.len() {
            break;
        }
        let seg_end = $s[$p..]
            .find("/")
            .map(|i| $p + i)
            .unwrap_or($s.len());
        let parsed = $s[$p..seg_end].parse::<$t>();
        if parsed.is_err() {
            break;
        }
        let $n = parsed.unwrap();
        $p = seg_end;
    );
    // ( $s: ident, $p: ident, | ( $n:ident : $t:ty ) ) => ("named last type");
    ( $s: ident, $p: ident, $e:expr ) => (
        $p += 1;  // leading '/'
        if $p >= $s.len() {
            break;
        }
        let seg_end = $s[$p..]
            .find("/")
            .map(|i| $p + i)
            .unwrap_or($s.len());
        if &$s[$p..seg_end] == $e {
            $p = seg_end;
        } else {
            break;
        }
    );
    // ( $s: ident, $p: ident, | $e:tt ) => (
    //     $p += 1;
    //     if $p >= $s.len() {
    //         break;
    //     }
    //     if &$s[$p..] != $e {
    //         break;
    //     }
    // );
}
#[test]
fn seg_test() {
    {
        let mut ok = false;
        let mut p = 0;
        let s = "/asdf";
        loop {
            seg!(s, p, "asdf");
            ok = true;
            break;
        }
        assert_eq!(ok, true, "segment matched");
        assert_eq!(p, 5);
        loop {
            seg!(s, p, "xyz");
            unreachable!();
        }
    }

    {
        let mut ok = false;
        let mut ok2 = false;
        let mut p = 0;
        let s = "/abc/def";
        loop {
            seg!(s, p, "abc");
            ok = true;
            break;
        }
        assert_eq!(ok, true, "should match first segment");
        loop {
            seg!(s, p, "def");
            ok2 = true;
            break;
        }
        assert_eq!(ok2, true, "should match second segment");
        assert_eq!(p, 8);
    }

    {
        let mut ok = false;
        let mut p = 0;
        let s = "/asdf";
        loop {
            seg!(s, p, "fdsa");
            ok = true;
            break;
        }
        assert_eq!(ok, false, "should not match segment");
        assert_eq!(p, 1);
    }
}
// fn seg() {
//     let s = "";
//     let mut p = 0;
//     "||| tags";
//     seg!(s, p, "a");
//     seg!(s, p, |"b");
//     // "||| eaters";
//     // seg!(s, p, (u32));
//     // seg!(s, p, |(u32));
//     // seg!(s, p, (*));
//     // seg!(s, p, |(*));
//     // "||| extractors";
//     // seg!(s, p, (z: u32));
//     // seg!(s, p, |(z: u32));
//     // seg!(s, p, (z: *));
//     // seg!(s, p, |(z: *));
// }

macro_rules! split {
    // ( $s:ident, $p:ident, / ) => (
    //     if $s == "" {
    //         "yay"
    //     }
    // );

    ( $s:ident, $p:ident, ( $( $segment:tt ), * ) ) => (
        $( seg!($s, $p, $segment); )*
    );

    // ( $s:ident, $p:ident, $( / $segment:tt )* | $rest:tt ) => (
    //     $( seg!($s, $p, $segment); )* seg!($s, $p, |$rest)
    // );
}
#[test]
fn test_split() {
    {
        let s = "/abc";
        let mut p = 0;
        let mut ok = false;
        loop {
            split!(s, p, ("abc"));
            ok = true;
        }
        assert_eq!(ok, true);
    }
    {
        let s = "/abc/xyz";
        let mut p = 0;
        let mut ok = false;
        loop {
            split!(s, p, ("abc", "xyz"));
            ok = true;
        }
        assert_eq!(ok, true);
    }
    {
        let s = "/abc/xyz";
        let mut p = 0;
        let mut ok = false;
        loop {
            split!(s, p, ("abc", "xy"));
            ok = true;
        }
        assert_eq!(ok, false);
    }
}
// fn split() {
//     let s = "";
//     let mut p = 0;
//     split!(s, p, /);
//     split!(s, p, /"a");
//     split!(s, p, /"a"/"b");
//     // split!(s, p, /"a"/(u32));
//     // split!(s, p, /"a"/(id: u32));
//     // split!(s, p, /"a"|(path: *));
// }

macro_rules! route {
    ( $s:ident, $( $path:tt => $handle:expr ; )* ) => ({
        let mut p;
        $(
            loop {
                p = 0;
                split!($s, p, $path);
                if p != $s.len() {
                    break;
                }
                return $handle;
            }
        )*
    });
}


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
        ("home") => Page::Home;
        ("blog") => Page::BlogIndex;
        ("blog", (id: u32)) => Page::BlogPost(id);
        ("blog", (id: u32), "edit") => Page::BlogEdit(id);
        ("u", (handle: String)) => Page::User(handle);
    );
    Page::NotFound
}

#[test]
fn test_route() {
    assert_eq!(route("/home"), Page::Home);
    assert_eq!(route("/blog"), Page::BlogIndex);
    assert_eq!(route("/blog/42"), Page::BlogPost(42));
    assert_eq!(route("/blog/42/edit"), Page::BlogEdit(42));
    assert_eq!(route("/u/uniphil"), Page::User("uniphil".to_string()));
    assert_eq!(route("/asdf"), Page::NotFound);
    assert_eq!(route("/blog/abc"), Page::NotFound);
}
