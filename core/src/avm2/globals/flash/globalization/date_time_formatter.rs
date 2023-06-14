use crate::avm2::parameters::ParametersExt;
use crate::avm2::{Activation, Error, Object, TObject, Value};
use crate::string::AvmString;
use chrono::format::Locale;

pub fn format_utc_internal<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let mut locale_id = args.get_string(activation, 1)?.to_string();
    if let Some(idx) = locale_id.find('-') {
        locale_id.replace_range(idx.., &locale_id[idx + 1..].to_uppercase());
        locale_id.insert(idx, '_');
    }
    let locale = Locale::try_from(locale_id.as_str()).unwrap_or(Locale::en_US);
    let pattern = args.get_string(activation, 2)?.to_string();
    if let Some(date_object) = args.get_object(activation, 0, "Date")?.as_date_object() {
        if let Some(dt) = date_object.date_time() {
            return Ok(AvmString::new_utf8(
                activation.context.gc_context,
                dt.format_localized(&pattern, locale).to_string(),
            )
            .into());
        }
    }
    Ok(Value::Undefined)
}
