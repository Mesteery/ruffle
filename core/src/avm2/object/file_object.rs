//! Loader-info object

use crate::avm2::activation::Activation;
use crate::avm2::object::script_object::ScriptObjectData;
use crate::avm2::object::{ClassObject, Object, ObjectPtr, TObject};
use crate::avm2::value::Value;
use crate::avm2::Error;
use core::fmt;
use gc_arena::barrier::unlock;
use gc_arena::{lock::RefLock, Collect, Gc};
use gc_arena::{GcWeak, Mutation};
use path_clean::PathClean;
use std::cell::{Ref, RefCell, RefMut};
use std::path::PathBuf;
use url::{Position, Url};

/// A class instance allocator that allocates File objects.
pub fn file_allocator<'gc>(
    class: ClassObject<'gc>,
    activation: &mut Activation<'_, 'gc>,
) -> Result<Object<'gc>, Error<'gc>> {
    let base = ScriptObjectData::new(class).into();

    Ok(FileObject(Gc::new(
        activation.context.gc(),
        FileObjectData {
            base,
            path: RefCell::new(None),
            formatted_url: RefCell::new(None),
        },
    ))
    .into())
}

#[derive(Collect, Clone, Copy)]
#[collect(no_drop)]
pub struct FileObject<'gc>(pub Gc<'gc, FileObjectData<'gc>>);

#[derive(Collect, Clone, Copy, Debug)]
#[collect(no_drop)]
pub struct FileObjectWeak<'gc>(pub GcWeak<'gc, FileObjectData<'gc>>);

impl<'gc> TObject<'gc> for FileObject<'gc> {
    fn base(&self) -> Ref<ScriptObjectData<'gc>> {
        self.0.base.borrow()
    }

    fn base_mut(&self, mc: &Mutation<'gc>) -> RefMut<ScriptObjectData<'gc>> {
        unlock!(Gc::write(mc, self.0), FileObjectData, base).borrow_mut()
    }

    fn as_ptr(&self) -> *const ObjectPtr {
        Gc::as_ptr(self.0) as *const ObjectPtr
    }

    fn value_of(&self, _mc: &Mutation<'gc>) -> Result<Value<'gc>, Error<'gc>> {
        Ok(Value::Object(Object::from(*self)))
    }

    fn as_file_object(&self) -> Option<FileObject<'gc>> {
        Some(*self)
    }
}

impl<'gc> FileObject<'gc> {
    pub fn native_path(self) -> Option<PathBuf> {
        self.0.path.borrow().clone()
    }

    pub fn formatted_url(self) -> Option<String> {
        self.0.formatted_url.borrow().clone()
    }

    pub fn url(self) -> Option<String> {
        match *self.0.formatted_url.borrow() {
            Some(ref url) => Some(url.clone()),
            None => Some(Url::from_file_path(self.native_path()?).ok()?.into()),
        }
    }

    pub fn set_native_path(self, path: Option<PathBuf>) {
        self.0.path.replace(path);
    }

    pub fn set_formatted_url(self, url: Option<String>) {
        self.0.formatted_url.replace(url);
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
            self.0.formatted_url.replace(Some(url.clone().into()));

            self.0.path.replace(match scheme {
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
            });
        }
    }
}

#[derive(Collect, Clone)]
#[collect(no_drop)]
pub struct FileObjectData<'gc> {
    base: RefLock<ScriptObjectData<'gc>>,
    path: RefCell<Option<PathBuf>>,
    formatted_url: RefCell<Option<String>>,
}

impl fmt::Debug for FileObject<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileObject")
    }
}
