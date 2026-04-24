use crate::utilities::hook;

pub fn init() -> Result<(), hook::HookError> {
    hook::init()?;

    // hook some game funcs here

    Ok(())
}

pub fn eject() {
    
}