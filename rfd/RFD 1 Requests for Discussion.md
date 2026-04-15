---
date: 2026-04-16
authors: |
  @liamkinne
labels: meta
state: published
---

# Requests for Discussion

RFDs are a tool for thinking. By writing down the intent behind a design
decision, it allows them to be more rigorously evalutated. For the kind of
applications we expect Nodal to be useful for, rigor is a core value.

This is by no means a new idea. Some examples of prior art:

- [Oxide RFDs](https://oxide.computer/blog/rfd-1-requests-for-discussion)
- [NATS ADRs](https://github.com/nats-io/nats-architecture-and-design)
- [ITEF RFCs](https://www.ietf.org/process/rfcs/)

## RFD Metadata

At the top of each document shall be at least the following metadata fields (excluding the comments):

```yaml
date: <YYYY-MM-DD> # date of RFD creation
authors: | # who worked on the document
  @<github username>
  @<etc..>
labels: comma, separated, labels
state: draft # one of: draft, published, superseded(number)
```

## Workflow

### Starting an RFD

Reserve an RFD number by creating a branch in the form `rfd/<number>`. Find the
next available number looking at the existing RFD branches. Don't include any
leading zeroes for the RFD number.

Then commit your draft RFD with even just the metadata. The filename should be
in the form `RFD <number> <title>.md`.

### Submitting for approval

Open a pull-request to merge the RFD branch into `main` and select a number of reviewers.

### Publishing

Once the RFD has been approved by all reviewers, change the `state` to
`published` in the metadata and merge the PR.

### After publishing

RFDs can continue to be updated once published with new commits to the RFD
branch and a PR with approval.

If an RFD needs to be superseded, the new state needs to be moved to
`superseded(<number>)` which references the number of the superseding RFD.
