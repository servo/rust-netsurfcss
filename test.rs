use wapcaplet::ll::lwc_string;
use hl::*;

// Based off of libcss's examples/example1.c
mod example1 {
    #[test]
    fn run() {
        let data = "h1 { color: red; }\
                    h4 { color: #321; }\
                    h4, h5 { color: #123456; }";

        let params: CssStylesheetParams = CssStylesheetParams {
            params_version: CssStylesheetParamsVersion1,
            level: CssLevel21,
            charset: ~"UTF-8",
            url: ~"foo",
            title: ~"foo",
            allow_quirks: false,
            inline_style: false,
            resolve: Some(resolve_url),
            import: None,
            color: None,
            font: None,
        };

        let sheet: CssResult<CssStylesheetRef> = css_stylesheet_create(&params);
        debug!("stylesheet: %?", sheet);
        assert sheet.is_ok();
        let sheet = result::unwrap(move sheet);
        let size: CssResult<uint> = sheet.size();
        assert size.is_ok();
        debug!("stylesheet size: %?", size);

        let res: CssResult<()> = sheet.append_data(str::to_bytes(data));
        debug!("%?", res);
        match res {
            Err(CssNeedData) => (),
            _ => fail
        }
        let res: CssResult<()> = sheet.data_done();
    }

    fn resolve_url(base: &str, rel: &lwc_string, abs: & &lwc_string) -> CssError {
        fail ~"resolving url";
        CssOk
    }
}
