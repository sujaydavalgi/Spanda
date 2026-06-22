# spanda-ast

Spanda compiler frontend types: AST nodes (`Program`, `Expr`, `Stmt`), foundation declarations, comm declaration types, and `RegexPattern`.

Phase 4 extraction unit — breaks the internal `ast` ↔ `foundations` cycle by co-locating mutual types in one crate. Includes `nodes`, `foundations`, `comm_decl`, `robotics_decl`, and `regex`. Parser, typechecker, and runtime remain in `spanda-core` for now.
