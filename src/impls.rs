use std::any::Any;
use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::{fmt, hash, io};

use crate::Trait;

unsafe impl Trait for dyn Send { }
unsafe impl Trait for dyn Sync { }
unsafe impl Trait for dyn Send + Sync { }
unsafe impl Trait for dyn Any + Send { }
unsafe impl Trait for dyn Any + Sync { }
unsafe impl Trait for dyn Any + Send + Sync { }
unsafe impl<T: ?Sized> Trait for dyn Borrow<T> + Send { }
unsafe impl<T: ?Sized> Trait for dyn Borrow<T> + Sync { }
unsafe impl<T: ?Sized> Trait for dyn Borrow<T> + Send + Sync { }
unsafe impl<T: ?Sized> Trait for dyn BorrowMut<T> + Send { }
unsafe impl<T: ?Sized> Trait for dyn BorrowMut<T> + Sync { }
unsafe impl<T: ?Sized> Trait for dyn BorrowMut<T> + Send + Sync { }
unsafe impl<T: ?Sized> Trait for dyn AsMut<T> + Send { }
unsafe impl<T: ?Sized> Trait for dyn AsMut<T> + Sync { }
unsafe impl<T: ?Sized> Trait for dyn AsMut<T> + Send + Sync { }
unsafe impl<T: ?Sized> Trait for dyn AsRef<T> + Send { }
unsafe impl<T: ?Sized> Trait for dyn AsRef<T> + Sync { }
unsafe impl<T: ?Sized> Trait for dyn AsRef<T> + Send + Sync { }
unsafe impl Trait for dyn Error + Send { }
unsafe impl Trait for dyn Error + Sync { }
unsafe impl Trait for dyn Error + Send + Sync { }
unsafe impl Trait for dyn fmt::Binary + Send { }
unsafe impl Trait for dyn fmt::Binary + Sync { }
unsafe impl Trait for dyn fmt::Binary + Send + Sync { }
unsafe impl Trait for dyn fmt::Debug + Send { }
unsafe impl Trait for dyn fmt::Debug + Sync { }
unsafe impl Trait for dyn fmt::Debug + Send + Sync { }
unsafe impl Trait for dyn fmt::Display + Send { }
unsafe impl Trait for dyn fmt::Display + Sync { }
unsafe impl Trait for dyn fmt::Display + Send + Sync { }
unsafe impl Trait for dyn fmt::LowerExp + Send { }
unsafe impl Trait for dyn fmt::LowerExp + Sync { }
unsafe impl Trait for dyn fmt::LowerExp + Send + Sync { }
unsafe impl Trait for dyn fmt::LowerHex + Send { }
unsafe impl Trait for dyn fmt::LowerHex + Sync { }
unsafe impl Trait for dyn fmt::LowerHex + Send + Sync { }
unsafe impl Trait for dyn fmt::Octal + Send { }
unsafe impl Trait for dyn fmt::Octal + Sync { }
unsafe impl Trait for dyn fmt::Octal + Send + Sync { }
unsafe impl Trait for dyn fmt::Pointer + Send { }
unsafe impl Trait for dyn fmt::Pointer + Sync { }
unsafe impl Trait for dyn fmt::Pointer + Send + Sync { }
unsafe impl Trait for dyn fmt::UpperExp + Send { }
unsafe impl Trait for dyn fmt::UpperExp + Sync { }
unsafe impl Trait for dyn fmt::UpperExp + Send + Sync { }
unsafe impl Trait for dyn fmt::UpperHex + Send { }
unsafe impl Trait for dyn fmt::UpperHex + Sync { }
unsafe impl Trait for dyn fmt::UpperHex + Send + Sync { }
unsafe impl Trait for dyn fmt::Write + Send { }
unsafe impl Trait for dyn fmt::Write + Sync { }
unsafe impl Trait for dyn fmt::Write + Send + Sync { }
unsafe impl Trait for dyn hash::Hasher + Send { }
unsafe impl Trait for dyn hash::Hasher + Sync { }
unsafe impl Trait for dyn hash::Hasher + Send + Sync { }
unsafe impl Trait for dyn io::BufRead + Send { }
unsafe impl Trait for dyn io::BufRead + Sync { }
unsafe impl Trait for dyn io::BufRead + Send + Sync { }
unsafe impl Trait for dyn io::Read + Send { }
unsafe impl Trait for dyn io::Read + Sync { }
unsafe impl Trait for dyn io::Read + Send + Sync { }
unsafe impl Trait for dyn io::Seek + Send { }
unsafe impl Trait for dyn io::Seek + Sync { }
unsafe impl Trait for dyn io::Seek + Send + Sync { }
unsafe impl Trait for dyn io::Write + Send { }
unsafe impl Trait for dyn io::Write + Sync { }
unsafe impl Trait for dyn io::Write + Send + Sync { }
unsafe impl<T, I> Trait for dyn IntoIterator<IntoIter = I, Item = T> { }
unsafe impl<T> Trait for dyn Iterator<Item = T> + Send { }
unsafe impl<T> Trait for dyn Iterator<Item = T> + Sync { }
unsafe impl<T> Trait for dyn Iterator<Item = T> + Send + Sync { }
unsafe impl Trait for dyn Drop + Send { }
unsafe impl Trait for dyn Drop + Sync { }
unsafe impl Trait for dyn Drop + Send + Sync { }
unsafe impl Trait for dyn ToString + Send { }
unsafe impl Trait for dyn ToString + Sync { }
unsafe impl Trait for dyn ToString + Send + Sync { }
