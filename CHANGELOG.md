# Moka &mdash; Change Log

## Version 0.3.1

### Changed

- Stop skeptic from having to be compiled by all downstream users. ([#16][gh-pull-0016])

## Version 0.3.0

### Added

- Add an unsync cache (`moka::unsync::Cache`) and its builder for single-thread
  applications. ([#9][gh-pull-0009])
- Add `invalidate_all` method to `sync`, `future` and `unsync` caches.
  ([#11][gh-pull-0011])

### Fixed

- Fix problems including segfault caused by race conditions between the sync/eviction
  thread and client writes. (Addressed as a part of [#11][gh-pull-0011]).


## Version 0.2.0

### Added

- Add an asynchronous, futures aware cache (`moka::future::Cache`) and its builder.
  ([#7][gh-pull-0007])


## Version 0.1.0

### Added

- Add thread-safe, highly concurrent in-memory cache implementations
  (`moka::sync::{Cache, SegmentedCache}`) with the following features:
    - Bounded by the maximum number of elements.
    - Maintains good hit rate by using entry replacement algorithms inspired by
      [Caffeine][caffeine-git]:
        - Admission to a cache is controlled by the Least Frequently Used (LFU) policy.
        - Eviction from a cache is controlled by the Least Recently Used (LRU) policy.
    - Expiration policies:
        - Time to live
        - Time to idle


<!-- Links -->

[caffeine-git]: https://github.com/ben-manes/caffeine

[gh-pull-0016]: https://github.com/moka-rs/moka/pull/16/
[gh-pull-0011]: https://github.com/moka-rs/moka/pull/11/
[gh-pull-0009]: https://github.com/moka-rs/moka/pull/9/
[gh-pull-0007]: https://github.com/moka-rs/moka/pull/7/
