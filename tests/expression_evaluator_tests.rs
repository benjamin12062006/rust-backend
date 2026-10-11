use expression_evaluator::{ExpresionError, calcular};

#[test]
fn backend_puede_calcular_con_la_dependencia_local() {
    assert_eq!(calcular("(10 + 5) * 2"), Ok(30.0));
}

#[test]
fn backend_puede_distinguir_los_errores_del_evaluador() {
    assert_eq!(calcular("1 / 0"), Err(ExpresionError::DivisionPorCero));
}

#[test]
fn error_del_evaluador_es_compatible_con_el_trait_estandar() {
    let error = calcular(" ").unwrap_err();
    let error: &dyn std::error::Error = &error;
    assert_eq!(error.to_string(), "la entrada está vacía");
}
