#![doc(
    html_logo_url = "https://raw.githubusercontent.com/igni-engine/igni_input_core/main/assets/vector_color.svg",
    html_favicon_url = "https://raw.githubusercontent.com/igni-engine/igni_input_core/main/assets/vector_color.svg"
)]
//! # IGNI INPUT CORE
//!
//! **IGNI INPUT CORE** define los *contratos mínimos* del sistema de entrada
//! del futuro **IGNI ENGINE**.  
//!
//! Este crate **no implementa ningún backend ni lógica interna**.  
//! Su propósito es proporcionar **interfaces (`traits`) estables,
//! extensibles y deterministas** para que:
//!
//! - cada plataforma o backend pueda implementar su propio sistema de entrada,  
//! - el runtime del motor pueda leer el estado sin mutarlo,
//! - el editor pueda modificar configuraciones de forma segura,
//! - futuras herramientas externas sigan un contrato común,
//! - múltiples backends (teclado, mouse, gamepad, VR, sensores, etc.) sean compatibles por diseño.
//!
//! ---
//!
//! ## Estructura general del sistema
//!
//! El modelo de entrada se basa en **tres capas**, cada una más abstracta que la anterior:
//!
//! 1. **Raw Layer (Capa 0)**  
//!    Representa eventos crudos provenientes del sistema operativo o backend real.
//!
//! 2. **Processing Layer (Capa 1)**  
//!    Procesa y transforma eventos en un estado utilizable (transiciones, historial, etc.).
//!
//! 3. **Mapping Layer (Capa 2)**  
//!    Gestiona acciones, contextos, asignaciones y lógica semántica de control.
//!
//! Cada capa define *solo los traits necesarios* para garantizar separación estricta de
//! responsabilidades, máxima compatibilidad y rendimiento óptimo.
//!
//! ---
//!
//! ## Filosofía
//!
//! Este crate se rige por tres principios fundamentales:
//!
//! - **Separación absoluta entre lectura y mutación**  
//!   Cada capa expone un `State` (solo lectura) y un `Control` (mutación) donde corresponda.
//!
//! - **Determinismo y claridad de contratos**  
//!   Las implementaciones deben ser 100% explícitas en comportamiento y efectos.
//!
//! - **Extensibilidad total sin modificaciones al core**  
//!   Cualquier backend o futura tecnología puede acoplarse sin tocar este crate.
//!
//! ---
//!
//! ## ¿Qué NO es este crate?
//!
//! - No contiene implementaciones reales de input.  
//! - No depende de ninguna API de sistema operativo.  
//! - No incluye lógica de engine ni editor.  
//! - No gestiona hilos, buffers ni sincronización.  
//!
//! Su función es ser **la base estable y genérica** del ecosistema IGNI.
//!
//! ---
//!
//! ## Módulos
//!
//! Este crate expone un único módulo público principal:
//!
//! - [`layers`](./layers/index.html): contiene todos los traits de las capas Raw, Processing y Mapping.
//!
//! Estas definiciones sirven como contrato para cualquier backend o framework que desee integrarse con IGNI ENGINE.
//!
//! ---

pub mod layers;




///---------------------------------------------------------------------------
/// ## TRAITS PRINCIPALES
///Re-exportar los contratos principales para facilitar su uso externo.
/// Estos contratos son el corazon del input del engine y deben estar
/// disponibles para cualquier crate que implemente un backend o interactue
/// con el sistema de entrada.
/// ---------------------------------------------------------------------------
pub mod contracts;
pub use contracts::{game_contract, runtime_contract};