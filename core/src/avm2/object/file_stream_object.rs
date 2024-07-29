use crate::avm2::activation::Activation;
use crate::avm2::bytearray::{Endian, ObjectEncoding};
use crate::avm2::object::script_object::ScriptObjectData;
use crate::avm2::object::{ClassObject, Object, ObjectPtr, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::backend::filesystem::File;
use core::fmt;
use gc_arena::{Collect, Gc};
use gc_arena::{GcWeak, Mutation};
use std::cell::{Cell, RefCell};

/// A class instance allocator that allocates FileStream objects.
pub fn file_stream_allocator<'gc>(
    class: ClassObject<'gc>,
    activation: &mut Activation<'_, 'gc>,
) -> Result<Object<'gc>, Error<'gc>> {
    let base = ScriptObjectData::new(class);

    Ok(FileStreamObject(Gc::new(
        activation.context.gc(),
        FileStreamObjectData {
            base,
            handle: RefCell::new(None),
            object_encoding: Cell::new(ObjectEncoding::Amf3),
            endian: Cell::new(Endian::Big),
        },
    ))
    .into())
}

#[derive(Collect)]
#[collect(no_drop)]
#[repr(C, align(8))]
pub struct FileStreamObjectData<'gc> {
    base: ScriptObjectData<'gc>,
    //#[collect(require_static)]
    handle: RefCell<Option<Box<dyn File>>>,

    endian: Cell<Endian>,
    object_encoding: Cell<ObjectEncoding>,
}

#[derive(Collect, Clone, Copy)]
#[collect(no_drop)]
pub struct FileStreamObject<'gc>(pub Gc<'gc, FileStreamObjectData<'gc>>);

#[derive(Collect, Clone, Copy, Debug)]
#[collect(no_drop)]
pub struct FileStreamObjectWeak<'gc>(pub GcWeak<'gc, FileStreamObjectData<'gc>>);

impl<'gc> TObject<'gc> for FileStreamObject<'gc> {
    fn gc_base(&self) -> Gc<'gc, ScriptObjectData<'gc>> {
        // SAFETY: Object data is repr(C), and a compile-time assert ensures
        // that the ScriptObjectData stays at offset 0 of the struct- so the
        // layouts are compatible

        unsafe { Gc::cast(self.0) }
    }

    fn as_ptr(&self) -> *const ObjectPtr {
        Gc::as_ptr(self.0) as *const ObjectPtr
    }

    fn value_of(&self, _mc: &Mutation<'gc>) -> Result<Value<'gc>, Error<'gc>> {
        Ok(Value::Object(Object::from(*self)))
    }

    fn as_file_stream_object(&self) -> Option<FileStreamObject<'gc>> {
        Some(*self)
    }
}

impl<'gc> FileStreamObject<'gc> {
    pub fn object_encoding(self) -> ObjectEncoding {
        self.0.object_encoding.get()
    }

    pub fn set_object_encoding(self, encoding: ObjectEncoding) {
        self.0.object_encoding.set(encoding);
    }

    pub fn endian(self) -> Endian {
        self.0.endian.get()
    }

    pub fn set_endian(self, endian: Endian) {
        self.0.endian.set(endian);
    }

    pub fn handle(&self) -> &RefCell<Option<Box<dyn File>>> {
        &self.0.handle
    }

    pub fn set_handle(&self, handle: Option<Box<dyn File>>) -> Option<Box<dyn File>> {
        self.0.handle.replace(handle)
    }
}

impl fmt::Debug for FileStreamObject<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileStreamObject")
    }
}
