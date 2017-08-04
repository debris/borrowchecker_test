pub struct Foo {
	pub data: [u8; 64],
}

pub struct FooRef<'a> {
	pub data: &'a [u8; 64],
}

impl<'a> From<&'a Foo> for FooRef<'a> {
	fn from(foo: &'a Foo) -> Self {
		FooRef {
			data: &foo.data,
		}
	}
}

impl Foo {
	pub fn do_something<'a, F>(&self, _foo: F) where FooRef<'a>: From<F> {
		let _test_foo = FooRef::from(self);
	}
}
