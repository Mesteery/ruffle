//! Loader-info object

use crate::avm2::activation::Activation;
use crate::avm2::object::script_object::ScriptObjectData;
use crate::avm2::object::{ClassObject, Object, ObjectPtr, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::avm2::bytearray::ObjectEncoding;
use crate::backend::filesystem::File;
use core::fmt;
use gc_arena::{Collect, GcCell, GcWeakCell, MutationContext};
use std::cell::{Ref, RefMut};

/// A class instance allocator that allocates FileStream objects.
pub fn file_stream_allocator<'gc>(
    class: ClassObject<'gc>,
    activation: &mut Activation<'_, 'gc>,
) -> Result<Object<'gc>, Error<'gc>> {
    let base = ScriptObjectData::new(class);

    Ok(FileStreamObject(GcCell::allocate(
        activation.context.gc_context,
        FileStreamObjectData {
            base,
            handle: None,
            object_encoding: ObjectEncoding::Amf3,
        },
    ))
    .into())
}

#[derive(Collect, Clone, Copy)]
#[collect(no_drop)]
pub struct FileStreamObject<'gc>(pub GcCell<'gc, FileStreamObjectData<'gc>>);

#[derive(Collect, Clone, Copy, Debug)]
#[collect(no_drop)]
pub struct FileStreamObjectWeak<'gc>(pub GcWeakCell<'gc, FileStreamObjectData<'gc>>);

impl fmt::Debug for FileStreamObject<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileStreamObject")
            .field("ptr", &self.0.as_ptr())
            .finish()
    }
}

impl<'gc> FileStreamObject<'gc> {
    pub fn object_encoding(self) -> ObjectEncoding {
        self.0.read().object_encoding
    }

    pub fn set_object_encoding(self, mc: MutationContext<'gc, '_>, encoding: ObjectEncoding) {
        self.0.write(mc).object_encoding = encoding;
    }
}

#[derive(Collect)]
#[collect(no_drop)]
pub struct FileStreamObjectData<'gc> {
    base: ScriptObjectData<'gc>,
    object_encoding: ObjectEncoding,
    #[collect(require_static)]
    pub handle: Option<Box<dyn File>>,
}

impl<'gc> TObject<'gc> for FileStreamObject<'gc> {
    fn base(&self) -> Ref<ScriptObjectData<'gc>> {
        Ref::map(self.0.read(), |read| &read.base)
    }

    fn base_mut(&self, mc: MutationContext<'gc, '_>) -> RefMut<ScriptObjectData<'gc>> {
        RefMut::map(self.0.write(mc), |write| &mut write.base)
    }

    fn as_ptr(&self) -> *const ObjectPtr {
        self.0.as_ptr() as *const ObjectPtr
    }

    fn value_of(&self, _mc: MutationContext<'gc, '_>) -> Result<Value<'gc>, Error<'gc>> {
        Ok(Value::Object((*self).into()))
    }

    fn as_file_stream_object(&self) -> Option<FileStreamObject<'gc>> {
        Some(*self)
    }
}
