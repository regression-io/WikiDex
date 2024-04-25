use parse_wiki_text::ConfigurationSource;

pub(crate) const WIKIPEDIA_CONFIGURATION: &ConfigurationSource = &ConfigurationSource {
    category_namespaces: &["category"],
    extension_tags: &[
        "categorytree",
        "ce",
        "charinsert",
        "chem",
        "gallery",
        "graph",
        "hiero",
        "imagemap",
        "indicator",
        "inputbox",
        "langconvert",
        "mapframe",
        "maplink",
        "math",
        "nowiki",
        "phonos",
        "poem",
        "pre",
        "ref",
        "references",
        "score",
        "section",
        "source",
        "syntaxhighlight",
        "templatedata",
        "templatestyles",
        "timeline",
    ],
    file_namespaces: &["file", "image"],
    link_trail: "abcdefghijklmnopqrstuvwxyz",
    magic_words: &[
        "archivedtalk",
        "disambig",
        "expected_unconnected_page",
        "expectunusedcategory",
        "forcetoc",
        "hiddencat",
        "index",
        "newsectionlink",
        "nocc",
        "nocontentconvert",
        "noeditsection",
        "nogallery",
        "noglobal",
        "noindex",
        "nonewsectionlink",
        "notalk",
        "notc",
        "notitleconvert",
        "notoc",
        "staticredirect",
        "toc",
    ],
    protocols: &[
        "//",
        "bitcoin:",
        "ftp://",
        "ftps://",
        "geo:",
        "git://",
        "gopher://",
        "http://",
        "https://",
        "irc://",
        "ircs://",
        "magnet:",
        "mailto:",
        "matrix:",
        "mms://",
        "news:",
        "nntp://",
        "redis://",
        "sftp://",
        "sip:",
        "sips:",
        "sms:",
        "ssh://",
        "svn://",
        "tel:",
        "telnet://",
        "urn:",
        "worldwind://",
        "xmpp:",
    ],
    redirect_magic_words: &["redirect"],
};
