use neon::prelude::*;

fn get_text(mut cx: FunctionContext) -> JsResult<JsString> {
    let arg_value = cx.argument::<JsString>(0)?.value(&mut cx);
    let re = regex::Regex::new(r"[a-z0-9]+").unwrap();

    match re.captures(&arg_value) {
        Some(caps) => Ok(cx.string(caps.get(0).unwrap().as_str())),
        None => Ok(cx.string("")),
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("getText", get_text)?;
    Ok(())
}
