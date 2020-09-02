# Making a release

1. Update all `Cargo.toml` to have the new version.
2. Update README, CHANGELOG (run `date --iso-8601` to get ISO-8601 date format).
3. Run `cargo build` and review `Cargo.lock` changes if all looks well, make a commit.
4. Package up your crate into a format that can be uploaded to https://crates.io
  ```bash
  cargo package
  ```
  Check if files in package are correct by `cargo package --list`.

5. Now upload the package
  ```bash
  cargo publish
  ```
6. Create tag and publish to remote

  ```bash
  VER_NUM=0.x.x
  git tag -as ${VER_NUM} -m ${VER_NUM}
  git push origin master
  git push origin ${VER_NUM}
  ```
