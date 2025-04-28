
use soroban_sdk::{Env, Address};

/// Estructura que maneja la participación de Bryan o de otros usuarios.
pub struct Participacion;

impl Participacion {
    /// Registra la participación de un usuario actualizando su acción y contador.
    pub fn registrar(env: Env, usuario: Address, accion: String) {
        let clave_accion = (usuario.clone(), "ultima_accion_bryan");
        let clave_contador = (usuario.clone(), "contador_bryan");

        env.storage().set(&clave_accion, &accion);

        let contador: u32 = env
            .storage()
            .get(&clave_contador)
            .unwrap_or(Ok(0))
            .unwrap();

        env.storage().set(&clave_contador, &(contador + 1));
    }

    /// Devuelve la última acción y el contador de un usuario.
    pub fn leer(env: Env, usuario: Address) -> (String, u32) {
        let clave_accion = (usuario.clone(), "ultima_accion_bryan");
        let clave_contador = (usuario.clone(), "contador_bryan");

        let accion: String = env
            .storage()
            .get(&clave_accion)
            .unwrap_or(Ok("Sin registro Bryan".into()))
            .unwrap();

        let contador: u32 = env
            .storage()
            .get(&clave_contador)
            .unwrap_or(Ok(0))
            .unwrap();

        (accion, contador)
    }

    /// Verifica si un usuario ya tiene registros en el sistema.
    pub fn existe(env: Env, usuario: Address) -> bool {
        let clave_accion = (usuario.clone(), "ultima_accion_bryan");
        let clave_contador = (usuario.clone(), "contador_bryan");

        env.storage().has(&clave_accion) || env.storage().has(&clave_contador)
    }

    /// Devuelve los datos de participación en formato JSON.
    pub fn datos_json(env: Env, usuario: Address) -> String {
        let (accion, contador) = Self::leer(env, usuario);
        format!("{{ \"accion\": \"{}\", \"contador\": {} }}", accion, contador)
    }
}