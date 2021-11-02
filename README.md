# Apollo Federation Router

Rust implementation of Federated GraphQL router.

## Status

🚧 Apollo Federation Router is experimental software, not yet ready for production use.
It is not yet feature complete nor fully compliant to the GraphQL specification, we are
working on it.
It can already perform queries though, so we'd welcome experiments and feedback on it.

## Usage

Apollo Federation Router requires `configuration.yaml` and `supergraph.graphql`
to be supplied.  These are either located in the current directory or explicitly
specified via flag, either by an absolute path, or a path relative to the current
directory.

```
OPTIONS:
    -c, --config <configuration-path>    Configuration file location [env:
                                         CONFIGURATION_PATH=]
    -s, --schema <schema-path>           Schema location [env: SCHEMA_PATH=]
```


This CLI is not meant to be a long term thing, as users will likely use Rover
to start the server in future.

## Design principles

The development of Apollo Federation Router is driven by those principles that inform
architecture decisions and implementation.
**Correctness:** the router strives to be the most correct implementation of GraphQL and Federation, we care about testing and documenting everything implied by the specification, up to failure cases. The router’s behavior should follow the principle of least surprise for developers.

**Reliability:** the router is a critical part of GraphQL APIs, so it must be one of the strongest parts of the infrastructure. This implies stability in its behavior (no crashes, infinite loops, leaks, etc), in its availability (predictable latency, RAM and CPU usage, scalability) and observability (metrics, alerts). It should give strong confidence to infrastructure people that they can learn its limits and operate it safely.

**Safe experimentation:** the router will support all the future work around Federation, so it must allow new ideas and explorations without disturbing existing features. The project is still in movement, we cannot allow it to crystallize too early, while still following the principles of correctness and reliability.

**Usability:** the router must be simple to operate. Prefer extensibility over configuration options, and ensure that the user has enough information to help themselves when things go wrong. For example:
* Common environmental misconfiguration should detected and surfaced to the user in the form of mitigation steps.
* User supplied extensions should be observable and flagged when they cause performance issues. Tell the users how much time an extension is consuming per request and why.

### Architecture

The design principles are guiding these architecture areas:
** Unit testability:** All new code should be unit testable, or have a good reason why it is not. This may mean spending a little extra time to ensure code is testable in isolation. Do not rely solely on integration testing.

**Integration test suite:** we will integrate with the gateway’s test suite and help improve it to test all aspects of the specifications. In particular, this test suite will verify failure cases like invalid queries or network problems. Integration tests must be bullet proof, and must not fail in the case of slow test execution or race conditions.

**Measurement and learning:** reliability has to be tested and measured, through benchmarks, profiling, and through exploration of the router’s limits. We want to learn how to operate the router and what is its nominal point. To that end, the router shall be instrumented in detail, allowing us to measure how code changes affect it. We especially take care of measuring the overhead of new features, to keep bounded latency and resource usage.

**Extensibility:** by allowing extensions and directives to modify the router’s behavior, we will be able to run experiments, test new features, while limiting its impact to specific queries or endpoints, and always in a way that is easy to deactivate at runtime (feature flags, canaries, etc). This will be especially important to keep development velocity once the router begins running in production.

## Project maintainers

Apollo Graph, Inc.
