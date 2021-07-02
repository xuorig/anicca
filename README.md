# Heraclitus

> Everything changes and nothing stands still.

Get the difference between two OpenAPI descriptions.

## CLI

```shell
$ cargo run --bin cli diff --base-file fixtures/pet-store.json --head-file fixtures/pet-store-changed.json --format json
```

## Library

## Diffs

Here is the list of OpenAPI objects and properties oaidiff currently compares.

- [x] OpenAPI Version
- [ ] Info Object
  - [ ] Title
  - [ ] Description
  - [ ] Terms of Service
  - [ ] Contact Object
  - [ ] License Object
  - [ ] Version
  - [ ] Extensions
- [ ] Servers Object
- [ ] Paths
  - [x] Path Added
  - [x] Path Removed
  - [ ] Path Changed
    - [x] Operation Added
    - [x] Operation Removed
    - [ ] Operation Changed
      - [x] Operation Id Changed 
      - [x] Summary Changed 
      - [x] Description Changed 
      - [x] Tags Changed 
      - [ ] External Documentation
      - [ ] Parameters
        - [x] Added
        - [x] Removed
        - [ ] Changed
          - [x] Required
          - [x] In
          - [ ] Schema
          - [ ] Deprecated
          - [ ] Format
          - [ ] Example
          - [ ] Examples
          - [ ] Extegnsions
      - [ ] Request Body
      - [ ] Responses
      - [ ] Deprecated
      - [ ] Security
      - [ ] Servers
      - [ ] Extensions
- [ ] Components
- [ ] Security
- [ ] Tags
- [ ] External Docs
- [ ] Extensions

Contributions are welcome to improve the completeness of the diff.
