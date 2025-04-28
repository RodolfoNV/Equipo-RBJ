                                                                                                                                                                                                 #![no_std]

use soroban_sdk::{contractimpl, symbol_short, Address, Env, Map, Symbol};

/// Contrato de participación de Bryan
pub struct ParticipacionBryan;

#[contractimpl]
impl ParticipacionBryan {
    /// Inicializa el registro de un usuario con mapa vacío.
    pub fn iniciar(env: Env, usuario: Address) {
        let registro = Map::<Symbol, u32>::new(&env);
        env.storage().instance().set(&usuario, &registro);
    }

    /// Incrementa el contador de lecturas del usuario.
    pub fn registrar_lectura(env: Env, usuario: Address) {
        Self::incrementar(env, usuario, symbol_short!("lectura_bryan"));
    }

    /// Incrementa el contador de escrituras del usuario.
    pub fn registrar_escritura(env: Env, usuario: Address) {
        Self::incrementar(env, usuario, symbol_short!("escritura_bryan"));
    }

    /// Obtiene todo el registro de lecturas y escrituras de un usuario.
    pub fn obtener_registro(env: Env, usuario: Address) -> Map<Symbol, u32> {
        env.storage()
            .instance()
            .get::<Address, Map<Symbol, u32>>(&usuario)
            .unwrap_or_else(|| Map::new(&env))
    }

    /// Elimina todos los registros del usuario.
    pub fn eliminar(env: Env, usuario: Address) {
        env.storage().instance().remove(&usuario);
    }

    /// Reinicia los contadores de lectura y escritura a cero.
    pub fn resetear(env: Env, usuario: Address) {
        let mut registro = Self::obtener_registro(env.clone(), usuario.clone());
        registro.set(symbol_short!("lectura_bryan"), 0);
        registro.set(symbol_short!("escritura_bryan"), 0);
        env.storage().instance().set(&usuario, &registro);
    }

    /// Guarda la última acción realizada por el usuario.
    pub fn registrar_ultima_accion(env: Env, usuario: Address, accion: Symbol) {
        env.storage()
            .instance()
            .set(&(usuario, symbol_short!("ultima_accion_bryan")), &accion);
    }

    /// Obtiene la última acción realizada por el usuario.
    pub fn obtener_ultima_accion(env: Env, usuario: Address) -> Option<Symbol> {
        env.storage()
            .instance()
            .get::<(Address, Symbol), Symbol>(&(usuario, symbol_short!("ultima_accion_bryan")))
    }

    /// Función auxiliar para incrementar contadores.
    fn incrementar(env: Env, usuario: Address, campo: Symbol) {
        let mut registro = Self::obtener_registro(env.clone(), usuario.clone());
        let valor = registro.get(campo.clone()).unwrap_or(0);
        registro.set(campo, valor + 1);
        env.storage().instance().set(&usuario, &registro);
    }
}