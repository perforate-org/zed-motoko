; Brackets, braces, and parentheses
(_ "[" "]" @end) @indent
(_ "<" ">" @end) @indent
(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent

; Block-like constructs
[
  "actor"
  "class"
  "module"
  "object"
  "func"
  "if"
  "else"
  "switch"
  "case"
  "try"
  "catch"
  "for"
  "loop"
  "while"
] @indent

; Continuation indent after certain keywords
[
  "return"
  "async"
  "await"
  "throw"
  "assert"
] @indent.continuation

; Dedent after these tokens
[
  "}"
  ")"
  "]"
  ">"
  "else"
  "catch"
] @dedent
