# Frequently Asked Questions

## Regular Expressions

### What regular expression syntax is supported?

The `recurse` executable uses the Rust `regex` crate for execution of regular expressions.  The following statement from the `regex` crate documentation summarizes the regular expression syntax support:

> Its syntax is similar to Perl-style regular expressions, but lacks a few features like look around and backreferences.

Detailed `regex` crate documentation is available at https://docs.rs/regex.

### Can I use Unicode characters in my regular expression?

Yes, you can include Unicode characters on the command line in your regular expression pattern.

### Where can I find more detailed information about the Unicode support in regular expression patterns?

Please see https://docs.rs/regex/#unicode and https://github.com/rust-lang/regex/blob/master/UNICODE.md.
