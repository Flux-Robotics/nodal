🚧 Work in progress! 🚧

# Nodal

Nodal is a general-purpose framework for creating RPC-like APIs in Rust using
NATS messaging. It also happens to be good for building robot software.

A key feature is the separation of the API from the implementation, allowing multiple implementations for a single API.

```mermaid
flowchart LR
  api[API Trait] --> base
  real_lib --> api
  mock_lib --> api
  test_lib --> api
  test_lib --> base
  mock_lib --> base
  stateful --> base
  base[Base Types]

  subgraph real_impl [Real Implementation]
  real_binary([Real Binary]) --> real_lib
  real_lib[Real Library] --> stateful
  stateful[Stateful logic]
  end

  subgraph mock_impl [Mock Implementation]
  mock_binary([Mock Binary]) --> mock_lib
  mock_lib[Mock Library]
  end

  subgraph test_impl [Test Implementation]
  test_binary([Test Binary]) --> test_lib
  test_lib[Test Library]
  end
```
