fixes #ISSUE_ID

:tada: Hi and welcome! Please read the text below and remove it - Thank you! :smile: :tada:


--------------------------
### Things to check before submitting a PR

- [ ] the tests are passing locally with `cargo test`
- [ ] cookbook renders correctly in `mdbook serve -o`
- [ ] commits are squashed into one and rebased to latest master
- [ ] PR contains correct "fixes #ISSUE_ID" clause to autoclose the issue once PR is merged
- [ ] non rendered items are in sorted order (links, reference, identifiers, Cargo.toml)
- [ ] links to docs.rs have wildcard version 
- [ ] code identifiers in description are in hyperlinked backticks 
```markdown
[`Entry::unpack`]: https://docs.rs/tar/*/tar/struct.Entry.html#method.unpack
```

### Things to do after submitting PR
- [ ] check if CI is happy with your PR
- [ ] drop a comment on https://github.com/rust-lang-nursery/rust-cookbook/issues/209 if you consent to repository being relicensed with [CC0 license](https://creativecommons.org/choose/zero/) :+1: 
