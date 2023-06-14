//! Loader-info object

use crate::avm2::activation::Activation;
use crate::avm2::object::script_object::ScriptObjectData;
use crate::avm2::object::{ClassObject, Object, ObjectPtr, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use core::fmt;
use gc_arena::{Collect, GcCell, GcWeakCell, MutationContext};
use std::cell::{Ref, RefMut};
use std::path::PathBuf;
use url::{Position, Url};
use path_clean::PathClean;

/// A class instance allocator that allocates File objects.
pub fn file_allocator<'gc>(
    class: ClassObject<'gc>,
    activation: &mut Activation<'_, 'gc>,
) -> Result<Object<'gc>, Error<'gc>> {
    let base = ScriptObjectData::new(class);

    Ok(FileObject(GcCell::allocate(
        activation.context.gc_context,
        FileObjectData {
            base,
            path: None,
            formatted_url: None,
        },
    ))
    .into())
}

#[derive(Collect, Clone, Copy)]
#[collect(no_drop)]
pub struct FileObject<'gc>(pub GcCell<'gc, FileObjectData<'gc>>);

#[derive(Collect, Clone, Copy, Debug)]
#[collect(no_drop)]
pub struct FileObjectWeak<'gc>(pub GcWeakCell<'gc, FileObjectData<'gc>>);

impl fmt::Debug for FileObject<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileObject")
            .field("ptr", &self.0.as_ptr())
            .finish()
    }
}

#[derive(Collect, Clone)]
#[collect(no_drop)]
pub struct FileObjectData<'gc> {
    base: ScriptObjectData<'gc>,
    pub path: Option<PathBuf>,
    pub formatted_url: Option<String>,
}

impl<'gc> FileObject<'gc> {
    pub fn native_path(self) -> Option<PathBuf> {
        self.0.read().path.clone()
    }

    pub fn url(self) -> Option<String> {
        match self.0.read().formatted_url.clone() {
            Some(url) => Some(url),
            None => Some(Url::from_file_path(self.native_path()?).ok()?.into()),
        }
    }

    pub fn set_native_path(self, mc: MutationContext<'gc, '_>, path: PathBuf) {
        self.0.write(mc).path.replace(path.clean());
        self.0.write(mc).formatted_url = None;
    }

    pub fn set_url(self, activation: &mut Activation<'_, 'gc>, mut url: String) {
        if let Some(idx) = url.find("://") {
            if url.chars().nth(idx + 3) != Some('/') {
                url.insert(idx + 3, '/');
            }
        }
        if let Ok(mut url) = Url::parse(&url) {
            if url[Position::AfterScheme..Position::BeforeUsername].ends_with(":/") {
                url =
                    Url::parse(&[url.scheme(), ":///", &url[Position::BeforeUsername..]].concat())
                        .unwrap();
            }

            let scheme = url.scheme();
            self.0
                .write(activation.context.gc_context)
                .formatted_url
                .replace(url.clone().into());

            self.0.write(activation.context.gc_context).path = match scheme {
                "app" | "app-storage" => Url::from_directory_path(match scheme {
                    "app" => &activation.context.filesystem.known_directories().app,
                    "app-storage" => {
                        &activation
                            .context
                            .filesystem
                            .known_directories()
                            .app_storage
                    }
                    _ => unreachable!(),
                })
                .and_then(|base| base.join(&url[Position::BeforeUsername..][1..]).or(Err(())))
                .and_then(|url| url.to_file_path())
                .and_then(|p| Ok(p.clean()))
                .ok(),
                "file" => url.to_file_path().and_then(|p| Ok(p.clean())).ok(),
                _ => None,
            }
        }
    }
}

impl<'gc> TObject<'gc> for FileObject<'gc> {
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

    fn as_file_object(&self) -> Option<FileObject<'gc>> {
        Some(*self)
    }
}
