use std::ptr;

use runtime::{Class, Messageable, Object};
use foundation::INSObject;

#[unsafe_no_drop_flag]
pub struct Id {
	ptr: *Object,
}

impl Id {
	pub fn nil() -> Id {
		Id { ptr: ptr::null() }
	}

	unsafe fn retain(&self) {
		msg_send![self.as_ptr() retain];
	}

	unsafe fn release(&self) {
		msg_send![self.as_ptr() release];
	}

	pub unsafe fn from_ptr(ptr: *Object) -> Id {
		let id = Id { ptr: ptr };
		id.retain();
		id
	}

	pub unsafe fn from_retained_ptr(ptr: *Object) -> Id {
		Id { ptr: ptr }
	}
}

impl Messageable for Id {
	unsafe fn as_ptr(&self) -> *Object {
		self.ptr
	}
}

impl Clone for Id {
	fn clone(&self) -> Id {
		unsafe {
			Id::from_ptr(self.ptr)
		}
	}
}

impl Drop for Id {
	fn drop(&mut self) {
		if !self.ptr.is_null() {
			unsafe {
				self.release();
			}
			self.ptr = ptr::null();
		}
	}
}

pub trait FromId {
	unsafe fn from_id(id: Id) -> Self;

	unsafe fn from_ptr(ptr: *Object) -> Self {
		FromId::from_id(Id::from_ptr(ptr))
	}

	unsafe fn from_retained_ptr(ptr: *Object) -> Self {
		FromId::from_id(Id::from_retained_ptr(ptr))
	}

	unsafe fn maybe_from_ptr(ptr: *Object) -> Option<Self> {
		if ptr.is_null() {
			None
		} else {
			Some(FromId::from_ptr(ptr))
		}
	}

	unsafe fn maybe_from_retained_ptr(ptr: *Object) -> Option<Self> {
		if ptr.is_null() {
			None
		} else {
			Some(FromId::from_retained_ptr(ptr))
		}
	}
}

pub struct ClassName<T> {
	name: &'static str,
}

impl<T> ClassName<T> {
	pub fn from_str(name: &'static str) -> ClassName<T> {
		ClassName { name: name }
	}

	pub fn as_str(&self) -> &'static str {
		self.name
	}
}

pub fn class<T: INSObject>() -> Class {
	let name: ClassName<T> = INSObject::class_name();
	Class::get(name.as_str())
}
