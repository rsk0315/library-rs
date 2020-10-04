pub trait SetValue<I> {
    type Input;
    fn set_value(&mut self, i: I, x: Self::Input);
}
