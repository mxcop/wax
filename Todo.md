## Todo List

<br>

- [x] Lexer
  - [x] Generic
    - [x] Tokens
    - [x] Whitespace
    - [x] Keywords
  - [x] Speed
    - [x] Iter from Fn (std::iter::from_fn)
- [ ] Parser
  - [x] Error handling
  - [ ] Templates
    - [x] Tags
    - [x] Attributes
    - [x] Self-closing Tags
    - [x] Void Elements (area, base, br...)
    - [x] IDs (#id)
    - [x] Tag contents
    - [ ] Unquoted attribute values
    - [ ] *Unicode character checks
  - [ ] Implementations
    - [x] Plain text
    - [ ] Links (#id)
  - [x] Stylesheets
    - [x] Plain text
- [ ] Codegen
  - [ ] Templates (HTML)
    - [x] Basics
    - [ ] IDs (#id)
    - [ ] Slots
  - [ ] Stylesheets (CSS)
    - [x] Basics
    - [ ] Import in HTML
  - [ ] Implementations (JS)
    - [ ] Links (#id)