use bitflags::bitflags;

bitflags! {
    /// Html renderer configuration options.
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        pub struct HtmlRenderConfig: u64 {
        /// skip preformatted HTML blocks
        const HTML_SKIP_HTML = 1 << 0;
        /// skip embedded <style> elements
        const HTML_SKIP_STYLE = 1 << 1;
        /// skip embedded images
        const HTML_SKIP_IMAGES = 1 << 2;
        /// skip all links
        const HTML_SKIP_LINKS = 1 << 3;
        /// only link to trusted protocols
        const HTML_SAFELINK = 1 << 4;
        /// only link with rel="nofollow"
        const HTML_NOFOLLOW_LINKS = 1 << 5;
        /// only link with rel="noreferrer"
        const HTML_NOREFERRER_LINKS = 1 << 6;
        /// only link with rel="noopener"
        const HTML_NOOPENER_LINKS = 1 << 7;
        /// add a blank target
        const HTML_HREF_TARGET_BLANK = 1 << 8;
        /// generate a table of contents
        const HTML_TOC = 1 << 9;
        /// skip the main contents (for a standalone table of contents)
        const HTML_OMIT_CONTENTS = 1 << 10;
        /// generate a complete HTML page
        const HTML_COMPLETE_PAGE = 1 << 11;
        /// generate XHTML output instead of HTML
        const HTML_USE_XHTML = 1 << 12;
        /// enable smart punctuation substitutions
        const HTML_USE_SMARTYPANTS = 1 << 13;
        /// enable smart fractions (with HTML_USE_SMARTYPANTS)
        const HTML_SMARTYPANTS_FRACTIONS = 1 << 14;
        /// enable smart dashes (with HTML_USE_SMARTYPANTS)
        const HTML_SMARTYPANTS_DASHES = 1 << 15;
        /// enable LaTeX-style dashes (with HTML_USE_SMARTYPANTS and HTML_SMARTYPANTS_DASHES)
        const HTML_SMARTYPANTS_LATEX_DASHES = 1 << 16;
        /// enable angled double quotes (with HTML_USE_SMARTYPANTS) for double quotes rendering
        const HTML_SMARTYPANTS_ANGLED_QUOTES = 1 << 17;
        /// enable "French guillemets" (with HTML_USE_SMARTYPANTS)
        const HTML_SMARTYPANTS_QUOTES_NBSP = 1 << 18;
        /// generate a link at the end of a footnote to return to the source
        const HTML_FOOTNOTE_RETURN_LINKS = 1 << 19;
    }
}
