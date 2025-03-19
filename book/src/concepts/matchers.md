# Matchers

`Matcher` is a trait used to implement logic for matching a request to a mock. A mock consists of a set of matchers that must all evaluate `true` for a given request to be considered a match.

```rust
pub trait Matcher: std::fmt::Debug + Send + Sync + 'static {
    /// Matcher name.
    fn name(&self) -> &str;
    /// Evaluates a match condition.
    fn matches(&self, req: &Request) -> bool;
}
```
Several matchers are provided out of the box for common use cases:
- MethodMatcher
- PathMatcher
- PathPrefixMatcher
- BodyMatcher
- HeadersMatcher
- HeadersExactMatcher
- HeaderMatcher
- HeaderExistsMatcher
- QueryParamsMatcher
- QueryParamMatcher
- AnyMatcher

Matcher types are not used directly; `When` has methods corresponding to all matchers plus additional convenience methods for body type variants, method variants, etc. 

We are still expanding the list of matchers and welcome PRs to implement matchers for common use cases.

Custom matchers can be implemented with the `Matcher` trait. `When::matcher()` can be used to plug custom `Matcher` implementations.