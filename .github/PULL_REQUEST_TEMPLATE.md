fixes #ISSUE_ID

:tada: Hi and welcome! Please read the text below and remove it - Thank you! :tada:

No worries if anything in these lists is unclear. Just submit the PR and ask away! :+1:

--------------------------
### Things to check before submitting a PR

- [ ] the tests are passing locally with `cargo test`
- [ ] cookbook renders correctly in `mdbook serve -o`
- [ ] commits are squashed into one and rebased to latest master
- [ ] PR contains correct "fixes #ISSUE_ID" clause to autoclose the issue on PR merge
    -  if issue does not exist consider creating it or remove the clause
- [ ] non rendered items are in sorted order (links, reference, identifiers, Cargo.toml)
- [ ] links to docs.rs have wildcard version `https://docs.rs/tar/*/tar/struct.Entry.html`
- [ ] example has standard [error handling](https://rust-lang-nursery.github.io/rust-cookbook/about.html#a-note-about-error-handling)
- [ ] code identifiers in description are in hyperlinked backticks
```markdown
[`Entry::unpack`]: https://docs.rs/tar/*/tar/struct.Entry.html#method.unpack
```

### Things to do after submitting PR
- [ ] check if CI is happy with your PR

Thank you for reading, you may now delete this text! Thank you! :smile:
