# Contributing Guide

For each ticket/feature/code-change, work should be done on distinct branches.
Unless the change is trivial, or other extenuating circumstances dictate, work
**should never be directly pushed to the main or stage branches.** In addition
to whatever contribution guidelines are outlined here, also follow any that are
outlined within the workspaces of the repository itself – e.g.
`/client/CONTRIBUTING.md` for client specific guidelines.

## Submitting Your Work

In order to make the review and merge process as efficient and simple as
possible, we want to abide by the following guidelines:

- Each feature/ticket/code-change should be checked into main as a **single
  commit** that encapsulates the entire change – whether or not to make >1
  commits during the dev process is up to each individual.
- The main branch should have a clear and _concise_ history that will be easy to
  search through and [git bisect](https://git-scm.com/docs/git-bisect) in the
  event of bugs or regressions.
- The reviewer should not need to comb through multiple commits to get the full
  scope of a change during the review process.

The easiest way to ensure all of the above, we will use a
[rebase merge](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/about-merge-methods-on-github#rebasing-and-merging-your-commits)
workflow when merging PRs.

**Example Workflow**

1. You create a branch for your task and check it out
2. You do your work
3. If you have only one commit, move to 5
4. If you have more than one commit, do the following
   1. Once your work is done, you need to squash all commits down to one. There
      are various ways to do this – but the easiest way is to use
      `git rebase -i`
      ([interactive rebase](https://thoughtbot.com/blog/git-interactive-rebase-squash-amend-rewriting-history#interactive-rebase))
      and mark all but the first to be _squashed_ and leave the first to be
      _picked_. See the link for an illustrated example.
   2. It is likely that the commit message needs to be adjusted at this point,
      use `git commit --amend` to do so, ensuring that the commit message
      follows our [guidelines](about:blank#commit-messages).
   3. Alternatively, when merging your PR you can have GitHub perform the squash
      for you once the PR is ready to be integrated. This means you can continue
      to add commits to your branch until it is truly done. Just make sure the
      squashed commit follows our commit guidelines!
5. At this point, so long as the commit message follows our
   [guidelines](about:blank#commit-messages), you are free to push to the
   upstream branch and create a PR to the **staging** branch.
6. Add relevant reviewers to the PR and once they approve the change and/or any
   comments are resolved then you can merge the PR.

<aside>
🗒️ The GitHub repo settings will be configured such that **only** rebase merges are allowed during part 6 above.

</aside>

## Commit Messages

Commit messages should follow the style defined by the
[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0-beta.4/)
spec. You are _strongly_ recommended to read at least the summary provided for
the spec on the webpage, but a casual description is below.

If following the below seems too convoluted or over-the-top, you can simply use
the tool [Commitizen](https://github.com/commitizen/cz-cli), which will make the
commit message process much easier and more straightforward. The repository will
already be set up for this use case – simply use the command `cz` in your
terminal within the repo and (as long as you have staged changes) you’re off to
the races.

The commit message should be structured as follows:

---

```java
<type>[optional scope]: <title/description>

[recommended body]

[as-required footer]
```

---

**Type**

The _type_ of the commit will define what the scope and implications of the
change are. Namely, it will be (most usually) one of the following:

- `fix:` used to indicate a bug- or error-fix that does not introduce any new
  features
- `feat:` used to indicate a new feature is being introduced
- various other semantic types such as `chore:`, `build:`, `ci:`, `docs:`,
  `style:`, `refactor:`, `perf:`, `test:`, and others.
- changes that introduce breaking changes **MUST** have _types_ followed by a
  `!`
  - e.g. `chore!: introduce new type and prop definitions`

**Scope (optional)**

The _scope_ of the commit will define what area of the code is changing, usually
used when the commit message could plausibly refer to various different systems.
An example would be:

`fix(renderer): change encoding back to sRGB` vs
`fix(map_textures): change encoding back to sRGB`

Also useful in cases where a mono-repo is used:
`feat(client): add new mage class definitions` vs
`feat(server): add new mage class definitions`

**Description**

Fairly self-explanatory, the description should be a brief summary of the entire
change. In almost all circumstances the **title of the Jira ticket should be the
description**. On the rare occasion that this does not make sense, use your best
judgment. For legibility we usually want the summary line (type + scope +
description) to be less than 90 characters. In the two examples above, the
descriptions are `change encoding back to sRGB` and
`change encoding back to sRGB`, respectively.

**Body (recommended)**

For trivial changes whose summary already describes the change entirely, this is
not strictly needed. This means that the _vast majority_ of changes **do** need
this. The body is there to ultimately do two things:

- fully define the problem that the commit addresses or, more importantly, _why
  the change is needed_
  - e.g. _characters are currently not utilizing a sprinting animation while
    they sprint, characters should have an animation that is distinct from their
    walking animation when they are sprinting_
- How your commit solves the problem, and any other contextual information that
  may be useful for someone reviewing the change or simply browsing our commit
  history.

Each point above should be separated into **two distinct paragraphs**.

Whether or not a body needs both of the above or something a bit more unique
will be ultimately up to the author, and is slightly subjective. Use your best
judgement here. The important bit here is that the body defines _why we are
changing/doing X_ – if it is not already clear.

**Footer**

The footer is used to denote what issues/tickets are being resolved by the
change. In addition, the footer will also indicate whether or not the change is
a breaking one.

To denote what ticket corresponds to the change, ensure this is in the footer:

`~~JIRA: <ticket-id>` e.g. `JIRA: FG-69`~~ (DEPRECATED: USING LINEAR OVER JIRA
NOW)

`resolves <ticket-id>` e.g. `resolves FG-69`

Note that this is **required** except in the very rare occasions that there is
no ticket for the work submitted.

To indicate the change as a breaking change you **must** include a line in the
footer that says `BREAKING CHANGE`.

---

Here is an example commit message that covers all of the above:

```
fix(renderer)!: change encoding back to sRGB

There was a regression introduced to the renderer class in a
prior change (<link>).

This change restores desired functionality
by changing encoding back from linear->sRGB. This also removes the
encoder prop from the renderer class constructor so it doesn't
happen again -- introducing a BREAKING CHANGE.

BREAKING CHANGE
resolves FG-420
```
