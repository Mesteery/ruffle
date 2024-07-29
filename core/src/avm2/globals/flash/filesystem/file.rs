use crate::avm2::error::io_error;
pub use crate::avm2::object::file_allocator;
use crate::avm2::parameters::ParametersExt;
use crate::avm2::{Activation, ArrayObject, ArrayStorage, Error, Object, TObject, Value};
use crate::string::AvmString;
use path_clean::PathClean;
use std::path::PathBuf;
use url::Url;

pub fn get_separator<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    Ok(AvmString::new_utf8(activation.context.gc_context, std::path::MAIN_SEPARATOR_STR).into())
}

pub fn set_url<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_object) = this.as_file_object() {
        let url = args.get_string(activation, 0)?.to_string();
        file_object.set_url(activation, url);
    }

    Ok(Value::Undefined)
}

pub fn get_url<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_object) = this.as_file_object() {
        if let Some(url) = file_object.url() {
            return Ok(AvmString::new_utf8(activation.context.gc_context, url).into());
        }
    }

    Ok(Value::Undefined)
}

pub fn get_native_path<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_object) = this.as_file_object() {
        if let Some(native_path) = file_object.native_path() {
            return Ok(AvmString::new_utf8(
                activation.context.gc_context,
                native_path.to_string_lossy(),
            )
            .into());
        }
    }

    Ok(Value::Undefined)
}

pub fn set_native_path<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_object) = this.as_file_object() {
        let path = args.get_string(activation, 0)?;
        file_object.set_native_path(Some(PathBuf::from(path.to_string()).clean()));
        file_object.set_formatted_url(None);
    }

    Ok(Value::Undefined)
}

pub fn get_user_directory_path<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    Ok(AvmString::new_utf8(
        activation.context.gc_context,
        activation
            .context
            .filesystem
            .known_directories()
            .user
            .to_string_lossy(),
    )
    .into())
}

pub fn get_desktop_directory_path<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    Ok(AvmString::new_utf8(
        activation.context.gc_context,
        activation
            .context
            .filesystem
            .known_directories()
            .desktop
            .to_string_lossy(),
    )
    .into())
}

pub fn get_documents_directory_path<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    Ok(AvmString::new_utf8(
        activation.context.gc_context,
        activation
            .context
            .filesystem
            .known_directories()
            .documents
            .to_string_lossy(),
    )
    .into())
}

pub fn get_size<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_object) = this.as_file_object() {
        if let Some(path) = file_object.native_path() {
            return Ok(activation.context.filesystem.size(&path).into());
        }
    }

    Ok(0.into())
}

pub fn get_exists<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_object) = this.as_file_object() {
        if let Some(path) = file_object.native_path() {
            return Ok(activation.context.filesystem.exists(&path).into());
        }
    }

    Ok(false.into())
}

pub fn get_is_directory<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_object) = this.as_file_object() {
        if let Some(path) = file_object.native_path() {
            return Ok(activation.context.filesystem.is_directory(&path).into());
        }
    }

    Ok(false.into())
}

pub fn get_is_hidden<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_object) = this.as_file_object() {
        if let Some(path) = file_object.native_path() {
            return Ok(activation.context.filesystem.is_hidden(&path).into());
        }
    }

    Ok(false.into())
}

pub fn get_space_available<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_object) = this.as_file_object() {
        if let Some(path) = file_object.native_path() {
            return Ok((activation.context.filesystem.available_space(&path) as f64).into());
        }
    }

    Ok(Value::Undefined)
}

pub fn create_directory<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_object) = this.as_file_object() {
        if let Some(path) = file_object.native_path() {
            if let Err(_) = activation.context.filesystem.create_directory(&path) {
                return Err(Error::AvmError(io_error(activation, "TODO", 1000)?));
            }
        }
    }

    Ok(Value::Undefined)
}

pub fn delete_directory<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_object) = this.as_file_object() {
        if let Some(path) = file_object.native_path() {
            if let Err(_) = activation
                .context
                .filesystem
                .delete_directory(&path, args.get_bool(0))
            {
                return Err(Error::AvmError(io_error(activation, "TODO", 1000)?));
            }
        }
    }

    Ok(Value::Undefined)
}

pub fn delete_file<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_object) = this.as_file_object() {
        if let Some(path) = file_object.native_path() {
            if let Err(_) = activation.context.filesystem.delete_file(&path) {
                return Err(Error::AvmError(io_error(activation, "TODO", 1000)?));
            }
        }
    }

    Ok(Value::Undefined)
}

pub fn get_directory_listing<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_object) = this.as_file_object() {
        if let Some(path) = file_object.native_path() {
            return match activation.context.filesystem.read_directory(&path) {
                Ok(listing) => {
                    let values = listing
                        .into_iter()
                        .filter_map(|entry| {
                            let file_entry_object = activation
                                .avm2()
                                .classes()
                                .file
                                .construct(activation, &[])
                                .ok()?
                                .as_file_object()?;
                            file_entry_object.set_native_path(Some(entry));
                            Some(Some(file_entry_object.into()))
                        })
                        .collect::<Vec<Option<Value<'gc>>>>();
                    let storage = ArrayStorage::from_storage(values);
                    Ok(ArrayObject::from_storage(activation, storage)?.into())
                }
                Err(_) => Err(Error::AvmError(io_error(activation, "TODO", 1000)?)),
            };
        }
    }

    Ok(ArrayObject::from_storage(activation, ArrayStorage::new(0))?.into())
}

pub fn resolve_path<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let path = PathBuf::from(args.get_string(activation, 0)?.to_string());

    if let Some(file_object) = this.as_file_object() {
        if let Some(new_file_object) = activation
            .avm2()
            .classes()
            .file
            .construct(activation, &[])?
            .as_file_object()
        {
            new_file_object.set_native_path(
                file_object
                    .native_path()
                    .and_then(|p| Some(p.join(&path).clean())),
            );

            if let Some(mut formatted_url) = file_object.formatted_url() {
                if !formatted_url.ends_with('/') {
                    formatted_url.push('/');
                }

                new_file_object.set_formatted_url(
                    Url::parse(&formatted_url)
                        .unwrap()
                        .join(&path.to_string_lossy())
                        .ok()
                        .map(|u| u.into()),
                );
            }

            return Ok(new_file_object.into());
        }
    }

    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}

pub fn move_to<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_object) = this.as_file_object() {
        if let Some(dest_file_object) = args.get_object(activation, 0, "File")?.as_file_object() {
            if let (Some(path), Some(dest_path)) =
                (file_object.native_path(), dest_file_object.native_path())
            {
                if activation
                    .context
                    .filesystem
                    .rename(&path, &dest_path, args.get_bool(1))
                    .is_ok()
                {
                    return Ok(Value::Undefined);
                }
            }
        }
    }

    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}

pub fn copy_to<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_object) = this.as_file_object() {
        if let Some(dest_file_object) = args.get_object(activation, 0, "File")?.as_file_object() {
            if let (Some(path), Some(dest_path)) =
                (file_object.native_path(), dest_file_object.native_path())
            {
                if activation
                    .context
                    .filesystem
                    .copy(&path, &dest_path, args.get_bool(1))
                    .is_ok()
                {
                    return Ok(Value::Undefined);
                }
            }
        }
    }

    Err(Error::AvmError(io_error(activation, "TODO", 1000)?))
}

pub fn get_name<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_object) = this.as_file_object() {
        if let Some(path) = file_object.native_path() {
            if let Some(file_name) = path.file_name() {
                return Ok(AvmString::new_utf8(
                    activation.context.gc_context,
                    file_name.to_string_lossy(),
                )
                .into());
            }
        }
    }

    Ok(Value::Null)
}

pub fn get_type<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_object) = this.as_file_object() {
        if let Some(path) = file_object.native_path() {
            if let Some(extension) = path.extension() {
                return Ok(AvmString::new_utf8(
                    activation.context.gc_context,
                    format!(".{}", extension.to_string_lossy()),
                )
                .into());
            }
        }
    }

    Ok(Value::Null)
}

pub fn get_parent<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(file_object) = this.as_file_object() {
        if let Some(new_file_object) = activation
            .avm2()
            .classes()
            .file
            .construct(activation, &[])?
            .as_file_object()
        {
            if let Some(parent) = file_object
                .native_path()
                .and_then(|p| Some(p.parent()?.to_owned()))
            {
                new_file_object.set_native_path(Some(parent.clean()));

                if let Some(mut formatted_url) = file_object.formatted_url() {
                    if !formatted_url.ends_with('/') {
                        formatted_url.push('/');
                    }

                    new_file_object.set_formatted_url(
                        Url::parse(&formatted_url)
                            .unwrap()
                            .join("..")
                            .ok()
                            .map(|u| u.into()),
                    )
                }

                return Ok(new_file_object.into());
            }
        }
    }

    Ok(Value::Null)
}
