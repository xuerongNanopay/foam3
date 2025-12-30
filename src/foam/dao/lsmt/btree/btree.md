BTree

- Immutable: update will create new root
- less memory: no NODE wrapper. Entire tree is organized in array.
    - Internal Node:
        - array length must be even
        - eg: [ key1, key2, ... keyN, child1, child2, ...., childN, childN+1, presum]
    - LEAF NODE:
        - array length must be odd
        - eg: [ key1, key2, ..... , keyN ] where N is odd
              [ key1, key2, ..... , keyN, null ] where N id even.
- reduce memory lookup.
- better lookup performance.