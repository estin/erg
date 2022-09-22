use erg_compiler::context::Context;
use erg_compiler::mod_cache::SharedModuleCache;

#[test]
fn test_subtyping() -> Result<(), ()> {
    let context = Context::new_module("<module>", SharedModuleCache::new());
    context.test_refinement_subtyping()?;
    Ok(())
}

#[test]
fn test_instantiation_and_generalization() -> Result<(), ()> {
    let context = Context::new_module("<module>", SharedModuleCache::new());
    context.test_instantiation_and_generalization()?;
    Ok(())
}

/*
#[test]
fn test_resolve_trait() -> Result<(), ()> {
    let context = Context::new_main_module();
    context.test_resolve_trait()?;
    Ok(())
}

#[test]
fn test_resolve_trait_inner1() -> Result<(), ()> {
    let context = Context::new_main_module();
    context.test_resolve_trait_inner1()?;
    Ok(())
}
*/
