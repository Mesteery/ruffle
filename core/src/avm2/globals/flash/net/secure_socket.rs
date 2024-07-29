use crate::avm2::parameters::ParametersExt;
use crate::avm2::{Activation, Error, Object, TObject, Value};
use crate::context::UpdateContext;
use crate::socket::invalid_port_number;

pub fn connect<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let socket = match this.as_socket() {
        Some(socket) => socket,
        None => return Ok(Value::Undefined),
    };

    let host = args.get_string(activation, 0)?;
    let port = args.get_u32(activation, 1)?;
    let port: u16 = port
        .try_into()
        .map_err(|_| invalid_port_number(activation))?;

    let UpdateContext {
        sockets, navigator, ..
    } = &mut activation.context;

    sockets.connect_avm2(
        *navigator,
        socket,
        host.to_utf8_lossy().into_owned(),
        port,
        true,
    );

    Ok(Value::Undefined)
}
