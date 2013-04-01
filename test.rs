// Based off of libcss's examples/example1.c
mod example1 {

    use CssResult;
    use CssProperty;
    use types::*;
    use hint::*;
    use select::*;
    use util::VoidPtrLike;
    use wapcaplet::LwcString;

    struct MyDomNode {
        name: @LwcString
    }

    impl VoidPtrLike for MyDomNode {
        fn from_void_ptr(node: *libc::c_void) -> MyDomNode {
            assert!(node.is_not_null());
            MyDomNode {
                name: unsafe {
                    let box = cast::reinterpret_cast(&node);
                    cast::bump_box_refcount(box);
                    box
                }
            }
        }

        fn to_void_ptr(&self) -> *libc::c_void {
            unsafe { cast::reinterpret_cast(&self.name) }
        }
    }

    #[test]
    fn run() {
        let data = "h1 { color: red; }\
                    h4 { color: #321; }\
                    h4, h5 { color: #123456; }";

        let resolve: CssUrlResolutionFn = |a,b| resolve_url(a, b);

        let params: CssStylesheetParams = CssStylesheetParams {
            params_version: CssStylesheetParamsVersion1,
            level: CssLevel21,
            charset: ~"UTF-8",
            url: ~"foo",
            title: ~"foo",
            allow_quirks: false,
            inline_style: false,
            resolve: Some(resolve),
            import: None,
            color: None,
            font: None,
        };

        let mut sheet: CssStylesheet = css_stylesheet_create(&params);
        debug!("stylesheet: %?", sheet);
        debug!("stylesheet size: %?", sheet.size());

        sheet.append_data(str::to_bytes(data));
        sheet.data_done();
        debug!("stylesheet size: %?", sheet.size());


        let mut select_ctx: CssSelectCtx = css_select_ctx_create();
        assert!(select_ctx.count_sheets() == 0);
        select_ctx.append_sheet(sheet, CSS_ORIGIN_AUTHOR, CSS_MEDIA_ALL);
        debug!("count sheets: %?", select_ctx.count_sheets());
        assert!(select_ctx.count_sheets() == 1);

        for uint::range(1, 7) |hh| {
            let element = fmt!("h%u", hh);
            let element_name: @LwcString = @from_rust_string(element);
            let node = MyDomNode { name: element_name };
            let select_handler = SelectHandler { bogus: () };
            let style: CssSelectResults = select_ctx.select_style(&node,
                                                                  CSS_MEDIA_SCREEN,
                                                                  None,
                                                                  &select_handler);

            let computed: CssComputedStyle = style.computed_style(CssPseudoElementNone);

            match computed.color() {
                CssColorInherit => {
                    debug!("color of h%u is 'inherit'", hh);
                },
                CssColorColor(color) => {
                    debug!("color of h%u is %x", hh, color.to_ll() as uint);
                }
            }
        }
    }

    fn resolve_url(_base: &str, _rel: &LwcString) -> CssResult<LwcString> {
        fail!(~"resolving url");
    }

    struct SelectHandler {
        bogus: ()
    }

    impl CssSelectHandler<MyDomNode> for SelectHandler {
        fn node_name(&self, node: &MyDomNode) -> CssQName {
            debug!("HL node_name!");
            debug!("SS %?", (*node.name).to_str());

            CssQName {
                ns: None,
                name: node.name.clone()
            }
        }

        fn node_id(&self, _node: &MyDomNode) -> Option<LwcString> { None }

        fn named_parent_node(&self, _node: &MyDomNode, _qname: &CssQName) -> Option<MyDomNode> {
            None
        }

        fn parent_node(&self, _node: &MyDomNode) -> Option<MyDomNode> {
            None
        }

        fn node_has_id(&self, _node: &MyDomNode, _name: LwcString) -> bool { false }

        fn named_ancestor_node(&self, _node: &MyDomNode, _qname: &CssQName) -> Option<MyDomNode> {
            None
        }

        fn node_is_root(&self, _node: &MyDomNode) -> bool { false }

        fn node_is_link(&self, _node: &MyDomNode) -> bool { false }

        fn ua_default_for_property(&self, property: CssProperty) -> CssHint {
            match property {
                _ => CssHintDefault
            }
        }
    }
}

#[test]
fn test_arc() {
    use std::arc::ARC;
    use stylesheet::*;
    use types::CssLevel21;

    let resolve: CssUrlResolutionFn = |a,b| resolve_url(a, b);
    let params: CssStylesheetParams = CssStylesheetParams {
        params_version: CssStylesheetParamsVersion1,
        level: CssLevel21,
        charset: ~"UTF-8",
        url: ~"foo",
        title: ~"foo",
        allow_quirks: false,
        inline_style: false,
        resolve: Some(resolve),
        import: None,
        color: None,
        font: None,
    };

    let sheet: CssStylesheet = css_stylesheet_create(&params);
    let _arc = ARC(sheet);

    fn resolve_url(_base: &str, _rel: &LwcString) -> CssResult<LwcString> {
        fail!(~"resolving url");
    }
}
