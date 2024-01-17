## Releasing guide

* Update version in Cargo.toml
* Update CHANGELOG

```
git commit -am 'v0.2.0'
```

Push and check CI passes.

Create and push a tag

```
git tag -a 'v0.2.0' -m 'v0.2.0'
git push upstream refs/tags/v0.2.0
```

Checkout in a clean tree and publish

```
cd [clean tree]
git pull upstream
cargo publish
```
