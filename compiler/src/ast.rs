#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,       // +
    Sub,       // -
    Equal,     // ==
    NotEqual,  // !=
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(u8),
    Identifier(String),
    BinaryOp {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },
    Peek(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let {
        name: String,
        value: Expr,
    },

    Assign {
        name: String,
        value: Expr,
    },

    Poke {
        address: Expr,
        value: Expr,
    },

    If {
        condition: Expr,
        body: Vec<Stmt>,
    },

    Loop {
        body: Vec<Stmt>,
    },

    InlineFn {
        name: String,
        body: Vec<Stmt>,
    },

    FnCall(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
}