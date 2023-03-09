use crate_b::Inner;

fn main() {
    ()
}

fn do_something(n: &Inner) -> u8 {
    n.field.reverse_bits()
}

#[cfg(test)]
mod test {
    use crate_c::Outer;

    use crate::do_something;

    #[test]
    fn test() {
        let Outer { inner: ctx } = Outer::new();

        // ctx.field is unknown right now
        println!("{}", ctx.field);

        // Then we make rust-analyzer infer the type of ctx as Inner
        println!("{}", do_something(&ctx));

        // And now rust-analyzer knows the type of ctx
        println!("{}", ctx.field);
    }
}
