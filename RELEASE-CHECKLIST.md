# Release Checklist

If you're attempting to create a release for `atspi` or any of its subcrates, please make sure to follow the steps below.
Throughout the document, `MAJ`, `MIN` and `PATCH` should be replaced with the major, minor and patch version of the library respectively.

* Make a branch named `vMAJ.MIN.PATCH-release`
* Make any needed changes to internal and external version numbers
* Make a PR to `https://github.com/odilia-app/atspi/` with this branch
* Make sure all tests pass
* Attempt to do a `cargo publish` with the `--dry-run` flag to test for potential issues.
* Tag latest commit with `git tag vMAJ.MIN.PATCH -m "Update message"`
* Push tags `git push --tags`
* Publish to `crates.io`: `cargo publish` for each subcrate first, in this order:
	1. `atspi-common`
	2. `atspi-proxies`
	3. `atspi-connections`
	4. `atspi`
* Merge the changes of the branch back into `main`

Creating a separate branch for each release allows us to backport emergency fixes if required for a patch release.

