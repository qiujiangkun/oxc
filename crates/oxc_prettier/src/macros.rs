//! Utility macros for constructing the IR

#[macro_export]
macro_rules! format {
    ($p:ident, $s:expr) => {{
        $s.format($p)
    }};
}

/// Wrap a static string (ss)
#[macro_export]
macro_rules! ss {
    ($s:expr) => {{
        Doc::Str($s)
    }};
}

#[macro_export]
macro_rules! string {
    ($p:ident, $s:expr) => {{
        $p.str($s)
    }};
}

#[macro_export]
macro_rules! indent {
    ($p:ident, $( $x:expr ),* $(,)?) => {
        {
            use $crate::doc::DocBuilder;
            let mut temp_vec = $p.vec();
            $(
                temp_vec.push($x);
            )*
            Doc::Indent(temp_vec)
        }
    };
}

#[macro_export]
macro_rules! indent_if_break {
    ($p:ident, $( $x:expr ),* $(,)?) => {
        {
            let mut temp_vec = $p.vec();
            $(
                temp_vec.push($x);
            )*
            Doc::IndentIfBreak(temp_vec)
        }
    };
}

#[macro_export]
macro_rules! line {
    () => {
        Doc::Line
    };
}

#[macro_export]
macro_rules! softline {
    () => {
        Doc::Softline
    };
}

#[macro_export]
macro_rules! hardline {
    () => {
        Doc::Hardline
    };
}

#[macro_export]
macro_rules! array {
    ($p:ident, $( $x:expr ),* $(,)?) => {
        {
            use $crate::doc::DocBuilder;
            let mut temp_vec = $p.vec();
            $(
                temp_vec.push($x);
            )*
            Doc::Array(temp_vec)
        }
    };
}

#[macro_export]
macro_rules! group {
    ($p:ident, $( $x:expr ),* $(,)?) => {
        {
            use $crate::doc::DocBuilder;
            let mut temp_vec = $p.vec();
            $(
                temp_vec.push($x);
            )*
            Doc::Group($crate::doc::Group::new(temp_vec, false))
        }
    };
}

#[macro_export]
macro_rules! if_break {
    ($p:ident, $s:expr) => {{
        use $crate::doc::DocBuilder;
        Doc::IfBreak($p.boxed(Doc::Str($s)))
    }};
}

#[macro_export]
macro_rules! wrap {
    ($p:ident, $self:expr, $kind:ident, $block:block) => {{
        let kind = AstKind::$kind($p.alloc($self));
        $p.enter_node(kind);
        let doc = $block;
        let doc = $p.wrap_parens(doc, kind);
        $p.leave_node();
        doc
    }};
}
