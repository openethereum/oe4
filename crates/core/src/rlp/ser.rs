// Copyright 2021 The OpenEthereum Authors.
// Licensed under the Apache License, Version 2.0.

pub(crate) struct EthereumRlpSerializer {
  stream: rlp::RlpStream,
}

impl EthereumRlpSerializer {
  pub fn new() -> Self {
      EthereumRlpSerializer {
          stream: rlp::RlpStream::new(),
      }
  }

  pub fn finalize(self) -> Vec<u8> {
      self.stream.as_raw().into()
  }
}

impl<'a> serde::Serializer for &'a mut EthereumRlpSerializer {
  type Ok = ();
  type Error = super::err::ErrorKind;

  type SerializeSeq = Self;
  type SerializeTuple = Self;
  type SerializeTupleStruct = Self;
  type SerializeTupleVariant = Self;
  type SerializeMap = Self;
  type SerializeStruct = Self;
  type SerializeStructVariant = Self;

  fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
      self.stream.append(&v);
      Ok(())
  }

  fn serialize_i8(self, _: i8) -> Result<Self::Ok, Self::Error> {
      unimplemented!("ethereum rlp does not support signed integers");
  }

  fn serialize_i16(self, _: i16) -> Result<Self::Ok, Self::Error> {
      unimplemented!("ethereum rlp does not support signed integers");
  }

  fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
      unimplemented!("ethereum rlp does not support signed integers");
  }

  fn serialize_i64(self, _: i64) -> Result<Self::Ok, Self::Error> {
      unimplemented!("ethereum rlp does not support signed integers");
  }

  fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
      self.stream.append(&v);
      Ok(())
  }

  fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
      todo!()
  }

  fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
      self.stream.append(&v);
      Ok(())
  }

  fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
      self.stream.append(&v);
      Ok(())
  }

  fn serialize_f32(self, _: f32) -> Result<Self::Ok, Self::Error> {
      unimplemented!("ethereum rlp does not support floating points");
  }

  fn serialize_f64(self, _: f64) -> Result<Self::Ok, Self::Error> {
      unimplemented!("ethereum rlp does not support floating points");
  }

  fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
      self.serialize_u8(v as u8)
  }

  fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
      self.stream.append(&v);
      Ok(())
  }

  fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
      self.stream.append_raw(&v, 1);
      Ok(())
  }

  fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
      //self.stream.begin_list(0);
      self.stream.append_empty_data();
      Ok(())
  }

  fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
  where
      T: serde::Serialize,
  {
      self.stream.begin_list(1);
      value.serialize(self)
  }

  fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
      self.stream.append_empty_data();
      Ok(())
  }

  fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
      todo!()
  }

  fn serialize_unit_variant(
      self,
      _: &'static str,
      _: u32,
      _: &'static str,
  ) -> Result<Self::Ok, Self::Error> {
      todo!()
  }

  fn serialize_newtype_struct<T: ?Sized>(
      self,
      _: &'static str,
      _: &T,
  ) -> Result<Self::Ok, Self::Error>
  where
      T: serde::Serialize,
  {
      todo!()
  }

  fn serialize_newtype_variant<T: ?Sized>(
      self,
      _: &'static str,
      _: u32,
      _: &'static str,
      _: &T,
  ) -> Result<Self::Ok, Self::Error>
  where
      T: serde::Serialize,
  {
      todo!()
  }

  fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
      self.stream.begin_list(len.unwrap_or(0));
      Ok(self)
  }

  fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
      self.serialize_seq(Some(len))
  }

  fn serialize_tuple_struct(
      self,
      _: &'static str,
      _: usize,
  ) -> Result<Self::SerializeTupleStruct, Self::Error> {
      unimplemented!()
  }

  fn serialize_tuple_variant(
      self,
      _: &'static str,
      _: u32,
      _: &'static str,
      _: usize,
  ) -> Result<Self::SerializeTupleVariant, Self::Error> {
      unimplemented!()
  }

  fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
      unimplemented!()
  }

  fn serialize_struct(
      self,
      _: &'static str,
      len: usize,
  ) -> Result<Self::SerializeStruct, Self::Error> {
      self.stream.begin_list(len);
      Ok(self)
  }

  fn serialize_struct_variant(
      self,
      _: &'static str,
      _: u32,
      _: &'static str,
      _: usize,
  ) -> Result<Self::SerializeStructVariant, Self::Error> {
      unimplemented!()
  }
}

impl<'a> serde::ser::SerializeTuple for &'a mut EthereumRlpSerializer {
  type Ok = ();

  type Error = super::err::ErrorKind;

  fn serialize_element<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
  where
      T: serde::Serialize,
  {
      unimplemented!()
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
      unimplemented!()
  }
}
impl<'a> serde::ser::SerializeSeq for &'a mut EthereumRlpSerializer {
  type Ok = ();
  type Error = super::err::ErrorKind;

  fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
  where
      T: ?Sized + serde::Serialize,
  {
      value.serialize(&mut **self)?;
      Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
      Ok(())
  }
}

impl<'a> serde::ser::SerializeStruct for &'a mut EthereumRlpSerializer {
  type Ok = ();

  type Error = super::err::ErrorKind;

  fn serialize_field<T: ?Sized>(
      &mut self,
      _: &'static str,
      value: &T,
  ) -> Result<(), Self::Error>
  where
      T: serde::Serialize,
  {
      value.serialize(&mut **self)?;
      Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
      Ok(())
  }
}
impl<'a> serde::ser::SerializeStructVariant for &'a mut EthereumRlpSerializer {
  type Ok = ();

  type Error = super::err::ErrorKind;

  fn serialize_field<T: ?Sized>(
      &mut self,
      _: &'static str,
      _: &T,
  ) -> Result<(), Self::Error>
  where
      T: serde::Serialize,
  {
      todo!()
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
      todo!()
  }
}

impl<'a> serde::ser::SerializeTupleStruct for &'a mut EthereumRlpSerializer {
  type Ok = ();

  type Error = super::err::ErrorKind;

  fn serialize_field<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
  where
      T: serde::Serialize,
  {
      todo!()
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
      todo!()
  }
}
impl<'a> serde::ser::SerializeTupleVariant for &'a mut EthereumRlpSerializer {
  type Ok = ();

  type Error = super::err::ErrorKind;

  fn serialize_field<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
  where
      T: serde::Serialize,
  {
      todo!()
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
      todo!()
  }
}

impl<'a> serde::ser::SerializeMap for &'a mut EthereumRlpSerializer {
  type Ok = ();

  type Error = super::err::ErrorKind;

  fn serialize_key<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
  where
      T: serde::Serialize,
  {
      todo!()
  }

  fn serialize_value<T: ?Sized>(&mut self, _: &T) -> Result<(), Self::Error>
  where
      T: serde::Serialize,
  {
      todo!()
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
      todo!()
  }
}