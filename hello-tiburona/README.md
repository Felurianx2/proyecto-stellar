# 🦈 Hello Tiburona - Contrato Inteligente Soroban

Un contrato inteligente profesional desarrollado en Rust usando el SDK de Soroban para la blockchain de Stellar. Este proyecto implementa un sistema de saludos con gestión de usuarios, administración y estadísticas.

## Características

### ✅ Implementadas
- **Sistema de Autenticación**: Cada usuario debe autenticarse para realizar acciones
- **Gestión de Administrador**: Sistema de admin con control de acceso
- **Validaciones Robustas**: Validación de nombres vacíos y longitud excesiva
- **Storage Organizado**: Uso eficiente de Instance y Persistent storage
- **TTL Management**: Gestión automática del tiempo de vida de los datos
- **Tests Comprehensivos**: 8 tests que cubren todos los casos de uso
- **Manejo de Errores**: Errores personalizados con códigos específicos

### 🚀 Funcionalidades Principales
- Inicialización única del contrato
- Sistema de saludos con contador global
- Almacenamiento de último saludo por usuario
- Reset del contador (solo admin)
- Consultas de información (sin autenticación requerida)

## 🏛️ Arquitectura

```
HelloContract
├── Storage (Instance)
│   ├── Admin (Address)
│   └── ContadorSaludos (u32)
├── Storage (Persistent)
│   └── UltimoSaludo(Address) → String
└── Funciones
    ├── initialize(admin)
    ├── hello(usuario, nombre)
    ├── get_contador()
    ├── get_ultimo_saludo(usuario)
    └── reset_contador(caller)
```

## 📚 Funcionalidades

### 🔧 Funciones Administrativas

#### `initialize(admin: Address) -> Result<(), Error>`
- **Propósito**: Inicializa el contrato una sola vez
- **Parámetros**: `admin` - Dirección que será el administrador
- **Validaciones**: Verifica que no haya sido inicializado previamente
- **Storage**: Guarda el admin y inicializa contador en 0

#### `reset_contador(caller: Address) -> Result<(), Error>`
- **Propósito**: Resetea el contador global a 0
- **Requisitos**: Solo el admin puede ejecutar esta función
- **Validaciones**: Verifica autenticación y permisos de admin

### 👋 Funciones de Usuario

#### `hello(usuario: Address, nombre: String) -> Result<Symbol, Error>`
- **Propósito**: Función principal del contrato - saluda a un usuario
- **Parámetros**: 
  - `usuario`: Dirección del usuario que saluda
  - `nombre`: Nombre a validar y almacenar
- **Validaciones**:
  - Nombre no puede estar vacío
  - Nombre no puede superar 32 caracteres
  - Usuario debe estar autenticado
- **Efectos**:
  - Incrementa contador global
  - Almacena último saludo del usuario
  - Extiende TTL de ambos storages

### 📊 Funciones de Consulta

#### `get_contador() -> u32`
- **Propósito**: Obtiene el contador total de saludos
- **Sin autenticación**: Función pública de solo lectura

#### `get_ultimo_saludo(usuario: Address) -> Option<String>`
- **Propósito**: Obtiene el último nombre que usó un usuario
- **Retorna**: `Some(String)` si existe, `None` si nunca ha saludado
- **Sin autenticación**: Función pública de solo lectura

## ⚙️ Prerequisitos

### Herramientas Necesarias
- **Rust**: Última versión estable ([instalar](https://rustup.rs/))
- **Soroban CLI**: Para compilar y desplegar
- **Target WebAssembly**: `wasm32v1-none` específico para Soroban

### Verificar Instalación
```bash
# Verificar Rust
rustc --version
cargo --version

# Verificar target WebAssembly
rustup target list | grep wasm32v1-none

# Si no está instalado:
rustup target add wasm32v1-none
```

## 🚀 Instalación

### 1. Clonar y Navegar
```bash
cd hello-tiburona
```

### 2. Verificar Dependencias
```bash
# Verificar que las dependencias están correctas
cargo check --target wasm32v1-none
```

### 3. Target WebAssembly (si no está instalado)
```bash
rustup target add wasm32v1-none
```

## 🔨 Compilación

### Compilación Básica
```bash
# Desde la raíz del proyecto hello-tiburona
stellar contract build
```

### Compilación con Verificación
```bash
# Verificar que compila sin errores
cargo build --target wasm32v1-none --release

# Build optimizado con Soroban CLI
stellar contract build
```

### Verificar Resultado
```bash
# Verificar que se generó el archivo WASM
ls -lh target/wasm32v1-none/release/*.wasm

# Optimizar el WASM (opcional)
stellar contract optimize --wasm target/wasm32v1-none/release/hello_tiburona.wasm
```

## 🧪 Tests

### Ejecutar Todos los Tests
```bash
cd contracts/hello-tiburona
cargo test
```

### Tests Individuales
```bash
# Test específico de inicialización
cargo test test_initialize

# Test de validaciones
cargo test test_nombre

# Test de control de acceso
cargo test test_reset
```

### Resultado Esperado
```
running 8 tests
test test::test_hello_exitoso ... ok
test test::test_initialize ... ok
test test::test_multiples_saludos ... ok
test test::test_no_reinicializar ... ok
test test::test_nombre_muy_largo ... ok
test test::test_nombre_vacio ... ok
test test::test_reset_no_autorizado ... ok
test test::test_reset_solo_admin ... ok

test result: ok. 8 passed; 0 failed
```

## 💻 Uso del Contrato

### Flujo de Uso Típico

1. **Inicialización** (una sola vez)
```rust
// Solo el deployer puede inicializar
client.initialize(&admin_address);
```

2. **Saludos de Usuarios**
```rust
// Usuario autenticado saluda
let nombre = String::from_str(&env, "Ana");
let resultado = client.hello(&usuario_address, &nombre);
// Retorna: Symbol("Hola")
```

3. **Consultas**
```rust
// Ver contador total
let total = client.get_contador();

// Ver último saludo de usuario
let ultimo = client.get_ultimo_saludo(&usuario_address);
```

4. **Administración**
```rust
// Admin resetea contador
client.reset_contador(&admin_address);
```

## 🏗️ Estructura del Código

### Archivos Principales
```
hello-tiburona/
├── Cargo.toml                 # Configuración del workspace
├── contracts/
│   └── hello-tiburona/
│       ├── Cargo.toml        # Dependencias del contrato
│       └── src/
│           └── lib.rs        # Código principal del contrato
└── target/                   # Archivos compilados
```

### Elementos Clave en `lib.rs`

#### Enums de Error y Storage
```rust
#[contracterror]
pub enum Error {
    NombreVacio = 1,
    NombreMuyLargo = 2,
    NoAutorizado = 3,
    NoInicializado = 4,
}

#[contracttype]
pub enum DataKey {
    Admin,
    ContadorSaludos,
    UltimoSaludo(Address),
}
```

#### Estructura del Contrato
```rust
#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    // Implementación de todas las funciones públicas
}
```

## ⚠️ Manejo de Errores

### Tipos de Error
| Código | Error | Descripción |
|--------|-------|-------------|
| 1 | `NombreVacio` | El nombre proporcionado está vacío |
| 2 | `NombreMuyLargo` | El nombre supera los 32 caracteres |
| 3 | `NoAutorizado` | Operación solo disponible para admin |
| 4 | `NoInicializado` | Contrato no ha sido inicializado |

### Patrón de Manejo
```rust
// Las funciones que pueden fallar retornan Result<T, Error>
pub fn hello(env: Env, usuario: Address, nombre: String) -> Result<Symbol, Error> {
    if nombre.len() == 0 {
        return Err(Error::NombreVacio);
    }
    // ... resto de la lógica
    Ok(Symbol::new(&env, "Hola"))
}
```

## 💾 Storage y TTL

### Instance Storage
- **`Admin`**: Dirección del administrador (vital para el contrato)
- **`ContadorSaludos`**: Contador global (alta frecuencia de acceso)

### Persistent Storage
- **`UltimoSaludo(Address)`**: Último nombre por usuario (datos específicos)

### Gestión de TTL
```rust
// Extender TTL de Instance Storage (100 ledgers)
env.storage().instance().extend_ttl(100, 100);

// Extender TTL de Persistent Storage por clave
env.storage().persistent()
    .extend_ttl(&DataKey::UltimoSaludo(usuario), 100, 100);
```

## 📚 Recursos Adicionales

- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Stellar Developers](https://developers.stellar.org/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Soroban Examples](https://github.com/stellar/soroban-examples)

## 🤝 Contribución

Este contrato fue desarrollado como parte del curso "Código Futura" y implementa las mejores prácticas de desarrollo de contratos inteligentes en Soroban.

### Para Desarrolladores
- El código está documentado en español
- Cada función tiene comentarios explicativos
- Los tests cubren casos de éxito y error
- Sigue las convenciones de Soroban SDK

---

**Desarrollado por**: Isamar Suarez - Tiburona de Código Futura 🦈  
**Curso**: Clase 4 - Construcción de Contratos Profesionales  
**Blockchain**: Stellar/Soroban