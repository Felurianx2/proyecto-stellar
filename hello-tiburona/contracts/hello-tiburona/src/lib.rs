#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracterror, contracttype,
    Env, Symbol, Address, String
};

// Definir errores
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NombreVacio = 1,
    NombreMuyLargo = 2,
    NoAutorizado = 3,
    NoInicializado = 4,
}

// Definir las DataKeys
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    ContadorSaludos,
    UltimoSaludo(Address),
}

// Definir el contrato
#[contract]
pub struct HelloContract;

// Implementar el contrato
#[contractimpl]
impl HelloContract {
    /// Inicializa el contrato con un administrador
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        // Verificar si ya fue inicializado
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::NoInicializado);
        }
        
        // Guardar el admin
        env.storage()
            .instance()
            .set(&DataKey::Admin, &admin);
        
        // Inicializar contador en 0
        env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32);
        
        // Extender TTL
        env.storage()
            .instance()
            .extend_ttl(100, 100);
        
        Ok(())
    }

    /// Función principal: saluda a un usuario
    pub fn hello(
        env: Env,
        usuario: Address,
        nombre: String
    ) -> Result<Symbol, Error> {
        // 1. Autenticar el usuario
        usuario.require_auth();

        // 2. Validar: nombre no vacío
        if nombre.len() == 0 {
            return Err(Error::NombreVacio);
        }

        // 3. Validar: nombre no muy largo (máximo 32 caracteres)
        if nombre.len() > 32 {
            return Err(Error::NombreMuyLargo);
        }

        // 4. Incrementar contador de saludos (Instance Storage)
        let key_contador = DataKey::ContadorSaludos;
        let contador: u32 = env.storage()
            .instance()
            .get(&key_contador)
            .unwrap_or(0);
        
        env.storage()
            .instance()
            .set(&key_contador, &(contador + 1));

        // 5. Guardar el último saludo del usuario (Persistent Storage)
        env.storage()
            .persistent()
            .set(&DataKey::UltimoSaludo(usuario.clone()), &nombre);

        // 6. Extender TTL del Persistent Storage (100 ledgers)
        env.storage()
            .persistent()
            .extend_ttl(&DataKey::UltimoSaludo(usuario), 100, 100);

        // 7. Extender TTL del Instance Storage (100 ledgers)
        env.storage()
            .instance()
            .extend_ttl(100, 100);

        // 8. Retornar el saludo
        Ok(Symbol::new(&env, "Hola"))
    }

    /// Obtener el contador total de saludos
    pub fn get_contador(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::ContadorSaludos)
            .unwrap_or(0)
    }

    /// Obtener el último saludo de un usuario específico
    pub fn get_ultimo_saludo(env: Env, usuario: Address) -> Option<String> {
        env.storage()
            .persistent()
            .get(&DataKey::UltimoSaludo(usuario))
    }

    /// Reset del contador (solo admin)
    pub fn reset_contador(env: Env, caller: Address) -> Result<(), Error> {
        // Verificar que el caller sea el admin
        let admin: Address = env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NoInicializado)?;
        
        caller.require_auth();
        
        if caller != admin {
            return Err(Error::NoAutorizado);
        }
        
        // Resetear el contador
        env.storage()
            .instance()
            .set(&DataKey::ContadorSaludos, &0u32);
        
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        
        client.initialize(&admin);
        
        // Verificar que el contador está en 0
        assert_eq!(client.get_contador(), 0);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #4)")]
    fn test_no_reinicializar() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        
        client.initialize(&admin);
        // Intentar inicializar de nuevo debe fallar
        client.initialize(&admin);
    }

    #[test]
    fn test_hello_exitoso() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        let nombre = String::from_str(&env, "Ana");
        let resultado = client.hello(&usuario, &nombre);
        
        assert_eq!(resultado, Symbol::new(&env, "Hola"));
        assert_eq!(client.get_contador(), 1);
        assert_eq!(client.get_ultimo_saludo(&usuario), Some(nombre));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_nombre_vacio() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        let vacio = String::from_str(&env, "");
        client.hello(&usuario, &vacio);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #2)")]
    fn test_nombre_muy_largo() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        // Nombre con más de 32 caracteres
        let largo = String::from_str(&env, "Este_es_un_nombre_extremadamente_largo_que_supera_los_32_caracteres");
        client.hello(&usuario, &largo);
    }

    #[test]
    fn test_reset_solo_admin() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario = Address::generate(&env);
        
        client.initialize(&admin);
        
        // Hacer un saludo
        client.hello(&usuario, &String::from_str(&env, "Test"));
        assert_eq!(client.get_contador(), 1);
        
        // Admin resetea el contador
        client.reset_contador(&admin);
        assert_eq!(client.get_contador(), 0);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #3)")]
    fn test_reset_no_autorizado() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let otro = Address::generate(&env);
        
        client.initialize(&admin);
        
        // Otro usuario intenta resetear (debe fallar)
        client.reset_contador(&otro);
    }

    #[test]
    fn test_multiples_saludos() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        let usuario1 = Address::generate(&env);
        let usuario2 = Address::generate(&env);
        
        client.initialize(&admin);
        
        // Primer saludo
        client.hello(&usuario1, &String::from_str(&env, "Alice"));
        assert_eq!(client.get_contador(), 1);
        
        // Segundo saludo
        client.hello(&usuario2, &String::from_str(&env, "Bob"));
        assert_eq!(client.get_contador(), 2);
        
        // Tercer saludo del mismo usuario
        client.hello(&usuario1, &String::from_str(&env, "Alicia"));
        assert_eq!(client.get_contador(), 3);
        
        // Verificar último saludo actualizado
        assert_eq!(
            client.get_ultimo_saludo(&usuario1),
            Some(String::from_str(&env, "Alicia"))
        );
    }
}