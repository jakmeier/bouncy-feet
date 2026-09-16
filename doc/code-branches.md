# Where Bouncy Feet Code Lives

## Repositories

- Main repository: https://code.jakobmeier.ch/bouncy-feet/bouncy-feet
- Shallow GitHub mirror: https://github.com/jakmeier/bouncy-feet

Pull requests go towards https://code.jakobmeier.ch/bouncy-feet/bouncy-feet

## Branches

- `bf`
    - Code automatically deployed to the live Bouncy Feet app on [app.bouncy-feet.ch].
    - Keep PR frequency against this branch low. Only open a PR for a fully tested features that are ready for a release.
    - Feature development history is squashed into a single commit per PR, with an entry to [CHANGELOG.md](../CHANGELOG.md).
    - This branch is mirrored to GitHub, where it is named `main`.
- `bf-next`
    - Code automatically deployed to the testing environment on [dev.bouncy-feet.ch].
    - Integration branch for changes under development in features branches, on top of `bf`.
    - After a release, reset `bf-next` to `bf` and cherry-pick changes from the previous `bf-next` that were not released yet.
    - Releases to `bf` are NOT a fast-forward of `bf-next`.
- Feature branches (`feature/*`)
    - Most changes should create feature branches and create a pull request against `bf-next` first.
    - Keep an open PR against `bf-next` during development to run CI on every push.
    - Once a feature is ready to be released, create a PR from the feature branch against `bf`.
    - Avoid deleting or ovrwriting feature branches, as this is the only source of the development history that was squashed on `bf`.
    - After merging a feature branch, if development on the feature continues, create a new branch with a version suffix like `*_v2`.
- Small temporary branches
    - Non-feature related changes can use temporary branches but still need to intgrate with `bf-next` before they get released to `bf`.