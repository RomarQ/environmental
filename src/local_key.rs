// Copyright 2017-2022 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

// This code is a simplified version of [`LocalKey`].
// [`LocalKey`]: https://github.com/rust-lang/rust/blob/c53af1ccd0305906c2c0aa7c561df99dbc9a4a35/library/std/src/thread/local.rs#L106

pub struct LocalKey<T: 'static> {
	pub init: fn() -> T,
	pub inner: RefCell<Option<T>>,
}

// This is safe as long there is no threads in wasm32.
unsafe impl<T: 'static> ::core::marker::Sync for LocalKey<T> {}

impl<T: 'static> LocalKey<T> {
	pub fn with<F, R>(&'static self, f: F) -> R
	where F: FnOnce(&T) -> R
	{
		if self.inner.borrow().is_none() {
			let v = (self.init)();
			*self.inner.borrow_mut() = Some(v);
		}
		// This code can't panic because:
		// 1. `inner` can be borrowed mutably only once at the initialization time.
		// 2. After the initialization `inner` is always `Some`.
		f(&*self.inner.borrow().as_ref().unwrap())
	}
}

/// Initialize [`LocalKey`].
#[macro_export]
macro_rules! local_key_init {
	(
		$init:ident
	) => {
		$crate::LocalKey {
			init: $init,
			inner: $crate::RefCell::new(None),
		}
	}
}
