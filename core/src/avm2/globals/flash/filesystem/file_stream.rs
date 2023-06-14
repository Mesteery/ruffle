use std::rc::Rc;
use crate::avm2::amf::{deserialize_value, serialize_value};
use crate::avm2::bytearray::{ByteArrayStorage, Endian, ObjectEncoding};
use crate::avm2::error::io_error;
pub use crate::avm2::object::file_stream_allocator;
use crate::avm2::parameters::ParametersExt;
use crate::avm2::{Activation, Error, Object, TObject, Value};
use crate::backend::filesystem::File;
use crate::ecma_conversions::{
    f64_to_wrapping_f32, f64_to_wrapping_i16, f64_to_wrapping_i32, f64_to_wrapping_u16,
    f64_to_wrapping_u32,
};
use crate::string::AvmString;
use encoding_rs::{Encoding, UTF_8};
use flash_lso::amf0::read::AMF0Decoder;
use flash_lso::amf3::read::AMF3Decoder;
use flash_lso::types::{AMFVersion, Element, Lso};
use fnv::FnvHashMap;
use ruffle_wstr::WString;
use std::io::{Read, SeekFrom};

pub fn get_bytes_available<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        if let Some(ref mut handle) = *file_stream_object.handle().borrow_mut() {
            if let (Ok(len), Ok(pos)) = (handle.stream_len(), handle.stream_position()) {
                return Ok((len - pos).into());
            }
        }
    }

    Ok(0.into())
}

pub fn get_position<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        if let Some(ref mut handle) = *file_stream_object.handle().borrow_mut() {
            return Ok(handle.stream_position().unwrap_or(0).into());
        }
    }

    Ok(0.into())
}

pub fn set_position<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let pos = args
        .get(0)
        .unwrap_or(&Value::Undefined)
        .coerce_to_u32(activation)? as u64;

    if let Some(file_stream_object) = this.as_file_stream_object() {
        if let Some(ref mut handle) = *file_stream_object.handle().borrow_mut() {
            if let Err(_) = handle.seek(SeekFrom::Start(pos)) {
                return Err(Error::AvmError(io_error(activation, "TODO", 1000)?));
            }
        }
    }

    Ok(Value::Null)
}

pub fn get_endian<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        return Ok(AvmString::new_utf8(
            activation.context.gc_context,
            match file_stream_object.endian() {
                Endian::Big => "bigEndian",
                Endian::Little => "littleEndian",
            },
        )
        .into());
    }

    Ok(Value::Undefined)
}

pub fn set_endian<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        let endian = args
            .get(0)
            .unwrap_or(&Value::Undefined)
            .coerce_to_string(activation)?;
        file_stream_object.set_endian(match endian.to_string().as_str() {
            "bigEndian" => Endian::Big,
            "littleEndian" => Endian::Little,
            _ => return Ok(Value::Undefined),
        });
    }

    Ok(Value::Undefined)
}

pub fn get_object_encoding<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        return Ok((file_stream_object.object_encoding() as u32).into());
    }

    Ok(Value::Undefined)
}

pub fn set_object_encoding<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        let encoding = args
            .get(0)
            .unwrap_or(&Value::Undefined)
            .coerce_to_u32(activation)?;
        file_stream_object.set_object_encoding(match encoding {
            0 => ObjectEncoding::Amf0,
            3 => ObjectEncoding::Amf3,
            _ => return Ok(Value::Undefined),
        });
    }

    Ok(Value::Undefined)
}

pub fn open<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        if file_stream_object.handle().borrow().is_none() {
            if let Some(file_object) = args.get_object(activation, 0, "File")?.as_file_object() {
                if let Some(path) = file_object.native_path() {
                    let mode = args.get_string(activation, 1)?.to_string();
                    let handle = match activation
                        .context
                        .filesystem
                        .open(&path, mode.as_str().into())
                    {
                        Ok(handle) => handle,
                        Err(_) => return Err(Error::AvmError(io_error(activation, "TODO", 1000)?)),
                    };
                    file_stream_object.set_handle(Some(handle));
                }
            }
        }
    }
    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}

pub fn close<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        file_stream_object.set_handle(None);
    }

    Ok(Value::Undefined)
}

pub fn truncate<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        if let Some(ref mut handle) = *file_stream_object.handle().borrow_mut() {
            if handle.truncate().is_ok() {
                return Ok(Value::Undefined);
            }
        }
    }

    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}

pub fn read_boolean<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        if let Some(ref mut handle) = *file_stream_object.handle().borrow_mut() {
            let mut buf = [0; 1];
            handle
                .read_exact(&mut buf)
                .map_err(|_| match io_error(activation, "TODO", 1000) {
                    Ok(e) => Error::AvmError(e),
                    Err(e) => e,
                })?;
            return Ok((buf == [1]).into());
        }
    }

    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}

macro_rules! impl_write {
    ($($method_name:ident $as_method_name:ident $($coerce:ident)?; $data_type:ty), *)
    =>
    {
        $( pub fn $method_name<'gc>(activation: &mut Activation<'_, 'gc>, handle: &mut Box<dyn File>, endian: Endian, val: $data_type) -> Result<(), Error<'gc>> {
            let val_bytes = match endian {
                Endian::Big => val.to_be_bytes(),
                Endian::Little => val.to_le_bytes(),
            };
            match handle.write(&val_bytes) {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::AvmError(io_error(activation, "TODO", 1000)?)),
            }
        } )*

        $( pub fn $as_method_name<'gc>(
            activation: &mut Activation<'_, 'gc>,
            this: Object<'gc>,
            args: &[Value<'gc>],
        ) -> Result<Value<'gc>, Error<'gc>> {
            let n = $($coerce)?(args.get(0).unwrap_or(&Value::Undefined).coerce_to_number(activation)?);
            if let Some(file_stream_object) = this.as_file_stream_object() {
                let endian = file_stream_object.endian();
                if let Some(ref mut handle) = *file_stream_object.handle().borrow_mut() {
                    if let Ok(_) = $method_name(activation, handle, endian, n) {
                        return Ok(Value::Undefined);
                    }
                }
            }
            Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
        } )*
    }
}

macro_rules! impl_read {
    ($($method_name:ident $as_method_name:ident $size:expr; $data_type:ty ), *)
    =>
    {
        $( pub fn $method_name<'gc>(activation: &mut Activation<'_, 'gc>, handle: &mut Box<dyn File>, endian: Endian) -> Result<$data_type, Error<'gc>> {
            let mut buf = [0; $size];
            handle.read_exact(&mut buf).map_err(|_| match io_error(activation, "TODO", 1000) {
                Ok(e) => Error::AvmError(e),
                Err(e) => e,
            })?;
            Ok(match endian {
                Endian::Big => <$data_type>::from_be_bytes(buf),
                Endian::Little => <$data_type>::from_le_bytes(buf),
            })
        } )*

        $( pub fn $as_method_name<'gc>(
            activation: &mut Activation<'_, 'gc>,
            this: Object<'gc>,
            _args: &[Value<'gc>],
        ) -> Result<Value<'gc>, Error<'gc>> {
            if let Some(file_stream_object) = this.as_file_stream_object() {
                let endian = file_stream_object.endian();
                if let Some(ref mut handle) = *file_stream_object.handle().borrow_mut() {
                    if let Ok(value) = $method_name(activation, handle, endian) {
                        return Ok(value.into());
                    }
                };
            }

            Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
        } )*
    }
}

impl_write!(write_f32 write_float f64_to_wrapping_f32; f32, write_f64 write_double; f64, write_i32 write_int f64_to_wrapping_i32; i32,
            write_u32 write_unsigned_int f64_to_wrapping_u32; u32, write_i16 write_short f64_to_wrapping_i16; i16, write_u16 write_unsigned_short f64_to_wrapping_u16; u16);
impl_read!(read_f32 read_float 4; f32, read_f64 read_double 8; f64, read_i32 read_int 4; i32, read_u32 read_unsigned_int 4; u32,
           read_i16 read_short 2; i16, read_u16 read_unsigned_short 2; u16, read_i8 read_byte 1; i8, read_u8 read_unsigned_byte 1; u8);

pub fn write_byte<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        if let Some(ref mut handle) = *file_stream_object.handle().borrow_mut() {
            let n = args
                .get(0)
                .unwrap_or(&Value::Undefined)
                .coerce_to_i32(activation)?;
            return match handle.write(&[n as u8]) {
                Ok(_) => Ok(Value::Undefined),
                Err(_) => Err(Error::AvmError(io_error(activation, "TODO", 1000)?)),
            };
        }
    }

    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}

pub fn write_boolean<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        if let Some(ref mut handle) = *file_stream_object.handle().borrow_mut() {
            let n = args.get(0).unwrap_or(&Value::Undefined).coerce_to_boolean() as u8;
            return match handle.write(&[n]) {
                Ok(_) => Ok(Value::Undefined),
                Err(_) => Err(Error::AvmError(io_error(activation, "TODO", 1000)?)),
            };
        }
    }

    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}

pub fn write_bytes<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let offset = args
        .get(1)
        .unwrap_or(&Value::Undefined)
        .coerce_to_u32(activation)? as usize;
    let length = args
        .get(2)
        .unwrap_or(&Value::Undefined)
        .coerce_to_u32(activation)? as usize;

    if let Some(buffer) = args.get_object(activation, 0, "ByteArray")?.as_bytearray() {
        if let Some(file_stream_object) = this.as_file_stream_object() {
            if let Some(ref mut handle) = *file_stream_object.handle().borrow_mut() {
                let end = if length == 0 {
                    buffer.len()
                } else {
                    offset
                        .checked_add(length)
                        .ok_or("RangeError: Cannot overflow usize")?
                };
                if handle
                    .write_all(&<ByteArrayStorage>::bytes(&buffer)[offset..end])
                    .is_ok()
                {
                    return Ok(Value::Undefined);
                }
            }
        }
    }

    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}

pub fn write_multi_byte<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        if let Some(ref mut handle) = *file_stream_object.handle().borrow_mut() {
            let string = args
                .get(0)
                .unwrap_or(&Value::Undefined)
                .coerce_to_string(activation)?;
            let charset_label = args
                .get(1)
                .unwrap_or(&"UTF-8".into())
                .coerce_to_string(activation)?;
            let encoder =
                Encoding::for_label(charset_label.to_utf8_lossy().as_bytes()).unwrap_or(UTF_8);
            let utf8 = string.to_utf8_lossy();
            let encoded_bytes = encoder.encode(&utf8).0;
            if handle.write_all(&encoded_bytes).is_ok() {
                return Ok(Value::Undefined);
            }
        }
    }

    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}

pub fn write_utf_bytes_inner<'gc>(
    activation: &mut Activation<'_, 'gc>,
    handle: &mut Box<dyn File>,
    value: AvmString<'gc>,
) -> Result<Value<'gc>, Error<'gc>> {
    if handle.write_all(&value.to_utf8_lossy().as_bytes()).is_ok() {
        return Ok(Value::Undefined);
    }
    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}

pub fn write_utf_bytes<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        if let Some(ref mut handle) = *file_stream_object.handle().borrow_mut() {
            let value = args
                .get(0)
                .unwrap_or(&Value::Undefined)
                .coerce_to_string(activation)?;
            return write_utf_bytes_inner(activation, handle, value);
        }
    }

    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}

pub fn write_utf<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        let endian = file_stream_object.endian();
        if let Some(ref mut handle) = *file_stream_object.handle().borrow_mut() {
            let value = args
                .get(0)
                .unwrap_or(&Value::Undefined)
                .coerce_to_string(activation)?;
            if let Ok(length) = u16::try_from(value.len()) {
                return match write_u16(activation, handle, endian, length) {
                    Ok(_) => write_utf_bytes_inner(activation, handle, value),
                    Err(_) => Err(Error::AvmError(io_error(activation, "TODO", 1000)?)),
                };
            } else {
                return Err("RangeError: UTF String length must fit into a short".into());
            }
        }
    }

    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}

pub fn read_bytes<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let offset = args
        .get(1)
        .unwrap_or(&Value::Integer(0))
        .coerce_to_u32(activation)? as usize;
    let length = args
        .get(2)
        .unwrap_or(&Value::Integer(0))
        .coerce_to_u32(activation)? as usize;

    if let Some(file_stream_object) = this.as_file_stream_object() {
        if let Some(ref mut handle) = *file_stream_object.handle().borrow_mut() {
            if let Some(mut buffer) = args
                .get_object(activation, 0, "ByteArray")?
                .as_bytearray_mut(activation.context.gc_context)
            {
                let res = if length == 0 {
                    if offset == 0 {
                        handle.read_to_end(buffer.vec_mut()).is_ok()
                    } else {
                        let mut tmp = Vec::new();
                        handle
                            .read_to_end(&mut tmp)
                            .map(|_| buffer.write_at(&tmp, offset))
                            .is_ok()
                    }
                } else {
                    let end = offset
                        .checked_add(length)
                        .ok_or("RangeError: Cannot overflow usize")?;
                    if buffer.len() < end {
                        buffer.set_length(end);
                    }
                    handle
                        .read_exact(&mut buffer.bytes_mut()[offset..end])
                        .is_ok()
                };
                if res {
                    return Ok(Value::Undefined);
                }
            }
        }
    }

    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}

pub fn read_multi_byte<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        if let Some(ref mut handle) = *file_stream_object.handle().borrow_mut() {
            let len = args
                .get(0)
                .unwrap_or(&Value::Undefined)
                .coerce_to_u32(activation)? as usize;
            let charset_label = args
                .get(1)
                .unwrap_or(&"UTF-8".into())
                .coerce_to_string(activation)?;

            let mut bytes = vec![0; len];
            match handle.read_exact(&mut bytes) {
                Ok(_) => {}
                Err(_) => return Err(Error::AvmError(io_error(activation, "TODO", 1000)?)),
            };
            let encoder =
                Encoding::for_label(charset_label.to_utf8_lossy().as_bytes()).unwrap_or(UTF_8);
            let decoded_str = encoder.decode(&bytes).0;
            return Ok(AvmString::new_utf8(activation.context.gc_context, decoded_str).into());
        }
    }

    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}

fn read_utf_bytes_inner<'gc>(
    activation: &mut Activation<'_, 'gc>,
    handle: &mut Box<dyn File>,
    length: usize,
) -> Result<Value<'gc>, Error<'gc>> {
    let mut bytes = Vec::with_capacity(length);
    unsafe { bytes.set_len(length) };
    if handle.read_exact(&mut bytes).is_ok() {
        //let start = if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) { 3 } else { 0 };
        //let end = if let Some(null) = bytes[start..].iter().position(|b| *b == b'\0') { null } else { length };
        //bytes.drain(start..end);
        return Ok(AvmString::new(
            activation.context.gc_context,
            WString::from_utf8_bytes(bytes),
        )
        .into());
    }
    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}

pub fn read_utf<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        let endian = file_stream_object.endian();
        if let Some(ref mut handle) = *file_stream_object.handle().borrow_mut() {
            let length = read_u16(activation, handle, endian)?;
            return read_utf_bytes_inner(activation, handle, length as usize);
        }
    }

    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}

pub fn read_utf_bytes<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        if let Some(ref mut handle) = *file_stream_object.handle().borrow_mut() {
            let length = args
                .get(0)
                .unwrap_or(&Value::Undefined)
                .coerce_to_u32(activation)? as usize;
            return read_utf_bytes_inner(activation, handle, length);
        }
    }

    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}

pub fn read_object<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        let object_encoding = file_stream_object.object_encoding();
        if let Some(ref mut handle) = *file_stream_object.handle().borrow_mut() {
            let mut bytes = Vec::new();
            if handle.read_to_end(&mut bytes).is_ok() {
                let (extra, amf) = match object_encoding {
                    ObjectEncoding::Amf0 => {
                        let mut decoder = AMF0Decoder::default();
                        decoder
                            .parse_single_element(&bytes)
                            .map_err(|_| "Error: Invalid object")?
                    }
                    ObjectEncoding::Amf3 => {
                        let mut decoder = AMF3Decoder::default();
                        decoder
                            .parse_single_element(&bytes)
                            .map_err(|_| "Error: Invalid object")?
                    }
                };

                let _ = handle.seek(SeekFrom::Current(-(extra.len() as i64)));
                let value = deserialize_value(activation, &amf)?;

                return Ok(value);
            }
        }
    }

    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}

pub fn write_object<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_stream_object) = this.as_file_stream_object() {
        let amf_version: AMFVersion = file_stream_object.object_encoding().into();
        if let Some(ref mut handle) = *file_stream_object.handle().borrow_mut() {
            let obj = args.get(0).cloned().unwrap_or(Value::Undefined);
            let mut object_table = FnvHashMap::default();
            if let Some(amf) = serialize_value(activation, obj, amf_version, &mut object_table) {
                let element = Element::new("", Rc::new(amf));
                let mut lso = Lso::new(vec![element], "", amf_version);
                let bytes = flash_lso::write::write_to_bytes(&mut lso)
                    .map_err(|_| "Failed to serialize object")?;
                // This is kind of hacky: We need to strip out the header and any padding so that we only write
                // the value. In the future, there should be a method to do this in the flash_lso crate.
                let element_padding = match amf_version {
                    AMFVersion::AMF0 => 8,
                    AMFVersion::AMF3 => 7,
                };

                if handle
                    .write_all(
                        &bytes[flash_lso::write::header_length(&lso.header) + element_padding
                            ..bytes.len() - 1],
                    )
                    .is_ok()
                {
                    return Ok(Value::Undefined);
                }
            }
        }
    }

    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}
