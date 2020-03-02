pub struct Pair<A, B> {
    pub left: A,
    pub right: B,
}

pub struct GlitchType<T> {
    pub left: T,
    pub right: Option<List<char>>,
}

pub enum  List<T> {
    Emp,
    NEmp(T, Box<List<T>>)
}

