use crate::avm2::parameters::ParametersExt;
use crate::avm2::{Activation, Error, Object, TObject, Value};
use crate::string::AvmString;
use chrono::format::Locale;

fn pattern_to_chrono(letter: char, count: usize) -> &'static str {
    (match (letter, count) {
        ('E', 4) => "%A",         // Full weekday name
        ('E', _) => "%a",         // Abbreviated weekday name
        ('y', 2) => "%y",         // Last 2 digits of the year
        ('y', _) => "%Y",         // Full year
        ('M', 1) => "%-m",        // Numeric month without padding
        ('M', 2) => "%m",         // Zero-padded month
        ('M', 4) => "%B",         // Full month name
        ('M', _) => "%b",         // Abbreviated month name
        ('d', 1) => "%-d",        // Numeric day without padding
        ('d', _) => "%d",         // Zero-padded day
        ('H', 1) => "%-H",        // 24-hour format without padding
        ('H', _) => "%H",         // 24-hour format with padding
        ('h', 1) => "%-I",        // 12-hour format without padding
        ('h', _) => "%I",         // 12-hour format with padding
        ('m', 1) => "%-M",        // Numeric minutes without padding
        ('m', _) => "%M",         // Zero-padded minutes
        ('s', 1) => "%-S",        // Numeric seconds without padding
        ('s', _) => "%S",         // Zero-padded seconds
        ('S', _) => "%6f",        // Fractional seconds
        ('a', _) => "%p",         // AM/PM indicator
        ('w', 1) => "%-U",        // Week number of the year (0-52/53)
        ('w', _) => "%U",         // Week number of the year (0-52/53) with padding
        ('D', 1) => "%-j",        // Day of the year without padding
        ('D', _) => "%j",         // Day of the year with padding
        ('Z', 1 | 2 | 3) => "%z", // Timezone offset (±hhmm)
        ('Z', _) => "%:z",        // Timezone offset (±hh:mm)
        ('v', _) => "%Z",         // Timezone abbreviation
        _ => "",                  // Unknown/unsupported pattern
    })
    .into()
}

fn convert_to_chrono_pattern(flash_pattern: &str) -> String {
    let mut result = String::with_capacity(flash_pattern.len());
    let mut chars = flash_pattern.chars().peekable();

    while let Some(&current_char) = chars.peek() {
        if current_char.is_alphabetic() {
            let mut count = 0;
            while chars.peek() == Some(&current_char) {
                count += 1;
                chars.next();
            }
            result.push_str(pattern_to_chrono(current_char, count));
        } else if current_char == '\'' {
            let mut literal = String::new();
            chars.next();
            while let Some(c) = chars.next() {
                if c == '\'' {
                    if chars.peek() == Some(&'\'') {
                        literal.push(chars.next().unwrap());
                        continue;
                    }
                    if literal.is_empty() {
                        literal.push('\'');
                    }
                    break;
                } else {
                    literal.push(c);
                }
            }
            result.push_str(&literal);
        } else {
            while let Some(&c) = chars.peek() {
                if c == '\'' || c.is_alphabetic() {
                    break;
                } else {
                    result.push(c);
                    chars.next();
                }
            }
        }
    }
    result
}

pub fn convert_pattern_internal<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let flash_pattern = args.get_string(activation, 0)?.to_string();
    let chrono_pattern = convert_to_chrono_pattern(&flash_pattern);
    Ok(AvmString::new_utf8(activation.context.gc_context, chrono_pattern).into())
}

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
