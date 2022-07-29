enum SyntaxTree<T> {
    Empty,
    Terminal(Token<T>),
    Node(T, Vec<SyntaxTree<T>>)
}