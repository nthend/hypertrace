use crate::{base::*, math::*, Config};
use std::io::{self, Read, Write};

pub trait CntRead: Read {
    fn position(&self) -> usize;
    fn skip(&mut self, count: usize) -> io::Result<()> {
        let mut buf = [0];
        for _ in 0..count {
            self.read_exact(&mut buf)?;
        }
        Ok(())
    }
    fn align(&mut self, align: usize) -> io::Result<()> {
        self.skip((align - (self.position() % align)) % align)
    }
    fn as_dyn_ref(&self) -> &dyn CntRead;
    fn as_dyn_mut(&mut self) -> &mut dyn CntRead;
}

pub trait CntWrite: Write {
    fn position(&self) -> usize;
    fn skip(&mut self, count: usize) -> io::Result<()> {
        for _ in 0..count {
            self.write_all(&[0])?;
        }
        Ok(())
    }
    fn align(&mut self, align: usize) -> io::Result<()> {
        self.skip((align - (self.position() % align)) % align)
    }
    fn as_dyn_ref(&self) -> &dyn CntWrite;
    fn as_dyn_mut(&mut self) -> &mut dyn CntWrite;
}

pub struct CountingWrapper<T> {
    inner: T,
    pos: usize,
}
impl<T> CountingWrapper<T> {
    pub fn new(read: T) -> Self {
        Self {
            inner: read,
            pos: 0,
        }
    }
    pub fn inner(&self) -> &T {
        &self.inner
    }
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
    pub fn into_inner(self) -> T {
        self.inner
    }
}
impl<R: Read> Read for CountingWrapper<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.pos += n;
        Ok(n)
    }
}
impl<R: Read> CntRead for CountingWrapper<R> {
    fn position(&self) -> usize {
        self.pos
    }
    fn as_dyn_ref(&self) -> &dyn CntRead {
        self
    }
    fn as_dyn_mut(&mut self) -> &mut dyn CntRead {
        self
    }
}
impl<W: Write> Write for CountingWrapper<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.inner.write(buf)?;
        self.pos += n;
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
impl<W: Write> CntWrite for CountingWrapper<W> {
    fn position(&self) -> usize {
        self.pos
    }
    fn as_dyn_ref(&self) -> &dyn CntWrite {
        self
    }
    fn as_dyn_mut(&mut self) -> &mut dyn CntWrite {
        self
    }
}

pub trait ValueReader {
    fn read_value<T: Type>(&mut self, cfg: &Config, type_: &T) -> io::Result<T::Value>;
}
impl<R: CntRead> ValueReader for R {
    fn read_value<T: Type>(&mut self, cfg: &Config, type_: &T) -> io::Result<T::Value> {
        let align = type_.align(cfg);
        assert!(align != 0, "Align of type {:?} ({}) is zero", type_, align);
        assert!(
            is_pow2(type_.align(cfg)),
            "Align of type {:?} ({}) is not a power of 2",
            type_,
            align,
        );

        let pos = self.position();
        assert!(
            pos % align == 0,
            "Stream is not properly aligned (position: {}) for type {:?} (align: {})",
            pos,
            type_,
            align,
        );

        let value = type_.load(cfg, self)?;
        let shift = self.position() - pos;
        let size = value.size(cfg);
        assert!(
            size % align == 0,
            "Align of type {:?} ({}) is not a multiple of its value size ({})",
            type_,
            align,
            size,
        );
        assert_eq!(
            size, shift,
            "Size of a value of type {:?} ({}) differs from a count of actually read bytes ({})",
            type_, size, shift,
        );
        Ok(value)
    }
}
pub trait EntityReader {
    fn read_entity<E: Entity>(&mut self, cfg: &Config) -> io::Result<E>;
}
impl<R: ValueReader> EntityReader for R {
    fn read_entity<E: Entity>(&mut self, cfg: &Config) -> io::Result<E> {
        self.read_value(cfg, &EntityType::<E>::new())
            .map(|v| v.into_entity())
    }
}

pub trait ValueWriter {
    fn write_value<V: Value>(&mut self, cfg: &Config, value: &V) -> io::Result<()>;
}
impl<W: CntWrite> ValueWriter for W {
    fn write_value<V: Value>(&mut self, cfg: &Config, value: &V) -> io::Result<()> {
        let type_ = value.type_of();
        let align = type_.align(cfg);
        let size = value.size(cfg);
        assert!(align != 0, "Align of type {:?} ({}) is zero", type_, align);
        assert!(
            is_pow2(type_.align(cfg)),
            "Align of type {:?} ({}) is not a power of 2",
            type_,
            align
        );
        assert!(
            size % align == 0,
            "Align of type {:?} ({}) is not a multiple of its value size ({})",
            type_,
            align,
            size
        );

        let pos = self.position();
        assert!(
            pos % align == 0,
            "Stream is not properly aligned (position: {}) for type {:?} (align: {})",
            pos,
            type_,
            align
        );

        value.store(cfg, self)?;
        let shift = self.position() - pos;
        assert_eq!(
            size, shift,
            "Size of a value of type {:?} ({}) differs from a count of actually written bytes ({})",
            type_, size, shift
        );
        Ok(())
    }
}
pub trait EntityWriter {
    fn write_entity<E: Entity>(&mut self, cfg: &Config, entity: &E) -> io::Result<()>;
}
impl<R: ValueWriter> EntityWriter for R {
    fn write_entity<E: Entity>(&mut self, cfg: &Config, entity: &E) -> io::Result<()> {
        self.write_value(cfg, EntityValue::new_ref(entity))
    }
}

#[cfg(test)]
#[derive(Clone, Default, Debug)]
pub struct TestBuffer {
    vec: Vec<u8>,
}

#[cfg(test)]
impl TestBuffer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn reader(&self) -> CountingWrapper<&[u8]> {
        CountingWrapper::new(self.vec.as_ref())
    }
    pub fn writer(&mut self) -> CountingWrapper<&mut Vec<u8>> {
        CountingWrapper::new(&mut self.vec)
    }
}
