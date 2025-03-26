# Mock Set

A mock set is simply a set of mocks for a mock server. It is implemented as a newtype wrapping `Vec<Mock>`. 

It keeps mocks sorted by priority and ensures that there are no duplicates. It has shorthand `MockSet::mock()` and `MockSet::mock_with_options()` methods to build and insert mocks directly into it. 

The server calls it's `MockSet::match_by_request()` method to match incoming requests to mock responses.