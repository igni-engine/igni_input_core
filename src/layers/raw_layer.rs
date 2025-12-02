//! ---------------------------------------------------------------------------
//! Interfaces de bajo nivel para representar códigos de entrada, eventos
//! individuales y estados lógicos dentro del sistema de input de **Igni Engine**.
//!
//! Este módulo define únicamente **contratos abstractos**, sin imponer
//! implementaciones concretas. Cada backend (teclado, mouse, gamepad, sensores,
//! VR, HID personalizados, etc.) es libre de proporcionar sus propios tipos
//! para KeyCode, KeyState y Event, siempre que cumplan con estos traits.
//!
//! Estos contratos permiten que Igni-Core sea:
//! - totalmente agnóstico al hardware,
//! - modular,
//! - extensible,
//! - multiplataforma,
//! - y compatible con cualquier dispositivo presente o futuro.
//!
//! ---------------------------------------------------------------------------

use std::time::{Duration, Instant};


/// ---------------------------------------------------------------------------
/// Representa la abstracción de un **código de tecla o botón**.
///
/// No todos los dispositivos comparten el mismo tipo de código, y cada backend
/// puede usar su propia representación (scancodes, keycodes nativos, índices de
/// botones, identificadores HID, etc.).
///
/// Este trait define la relación entre:
///
/// - **BackendKey:** el código que proviene del backend (OS, librería o driver)
/// - **NativeKey:** el código normalizado que el motor usará internamente
///
/// Igni no impone qué tipo deben tener estos códigos. Cada backend elige.
/// ---------------------------------------------------------------------------
pub trait KeyCodeExt: Sized {
    type NativeKey: Clone + Eq;
    type BackendKey: Clone;



    
    // Convierte un código nativo del motor a la representación del backend.
    fn from_native(key: &Self::NativeKey) -> Self;

    // Convierte un código del backend a la representación nativa del motor.
    fn from_backend(key: &Self::BackendKey) -> Self;

    // Convierte a la representación nativa del motor.
    fn to_native(&self) -> &Self::NativeKey;

    // Convierte a la representación del backend.
    fn to_backend(&self) -> Self::BackendKey;

    // Verifica si un código nativo es equivalente a uno del backend.
    fn is_equivalent(native : &Self::NativeKey, backend: &Self::BackendKey) -> bool {
        let key_from_backend = Self::from_backend(backend);
        key_from_backend.to_native() == native
    }
}

/// ---------------------------------------------------------------------------
/// Representa un **evento de entrada individual**.
///
/// Un evento corresponde a una única acción atómica:
/// - presionar una tecla
/// - soltar un botón
/// - mover un stick
/// - mover el mouse
/// - tocar la pantalla
///
/// Cada backend define su propio tipo de evento tanto para:
/// - `KeyCode`
/// - `KeyState`
/// - `DeviceKind`
///
/// Este trait describe la interfaz mínima que EL MOTOR necesita para trabajar
/// con cualquier evento de forma genérica.
/// ---------------------------------------------------------------------------
pub trait KeyEventExt : Clone {
    /// Tipo que identifica qué tecla/botón se activó.
    type KeyCode;

    /// Tipo que indica el estado asociado al evento (presionado, liberado, etc.).
    type KeyState;

    /// Tipo que identifica la clase de dispositivo (teclado, mouse, VR, HID...).
    type DeviceKind;

    /// Devuelve el código asociado al evento.
    fn keycode(&self) -> Self::KeyCode;

    /// Devuelve el estado lógico del evento.
    fn state(&self) -> Self::KeyState;

    /// Devuelve el instante en el que ocurrió el evento.
    fn timestamp(&self) -> Instant;

    /// Devuelve el tipo de dispositivo que generó el evento.
    fn device_kind(&self) -> Self::DeviceKind;

    /// Tiempo transcurrido desde que ocurrió el evento.
    ///
    /// Útil para detección de repeticiones rápidas, análisis temporal y
    /// características avanzadas del sistema de input.
    fn time_pressed(&self) -> Duration {
        self.timestamp().elapsed()
    }
}


/// ---------------------------------------------------------------------------
/// Interfaz que marca que un tipo representa un **estado de entrada**.
///
/// No define métodos porque cada backend puede decidir qué información
/// debe contener un estado. Ejemplos posibles:
///
/// - Teclado: `Pressed`, `Released`
/// - Mouse: `Down`, `Up`, `DoubleClick`
/// - Gamepad: `Pressed`, `Held`, `Released`
/// - Touch: `Start`, `Move(x, y)`, `End`
///
/// El único requisito es que los estados sean clonables.
/// ---------------------------------------------------------------------------
pub trait KeyStateExt: Clone {}


/// ---------------------------------------------------------------------------
/// Representa una **capa de entrada cruda** (Raw Input Layer).
///
/// Esta es la **capa más baja** del sistema de entrada de **Igni Engine**.
/// Su responsabilidad es *únicamente*:
///
/// **Recolectar eventos crudos del backend real**  
///    (SO, driver, librería, hardware, protocolo, etc.)
///
/// **Transformarlos al tipo genérico `KeyEventExt`**  
///    definido por el desarrollador.
///
/// **Entregar todos los eventos nuevos ocurridos desde la última llamada**.
///
/// ---
///
/// #Qué NO debe hacer esta capa
///
/// La capa RAW **no realiza ningún tipo de procesamiento**, por lo que:
///
/// - No mantiene estados (`pressed`, `held`, `released`)
/// - No normaliza valores
/// - No detecta gestos (double tap, chords...)
/// - No aplica deadzones
/// - No mapea acciones del juego
/// - No almacena historial completo
/// - No interpreta semántica del input
///
/// Su función es estrictamente recolectar **eventos crudos**, sin lógica añadida.
///
/// ---
///
/// # Funcionamiento esperado
///
/// Cada backend (evdev, Wayland, Win32 RawInput, HID, Web, etc.) implementa
/// este trait y utiliza `poll_events` para obtener los eventos pendientes del
/// buffer del sistema operativo o de la API correspondiente.
///
/// El método debe:
///
/// - leer los eventos “en cola” del backend,
/// - convertirlos a `KeyEventExt`,
/// - devolverlos en el orden en que ocurrieron,
/// - y **terminar rápidamente**, sin bloquear.
///
/// La capa de procesamiento superior (Processed Input Layer) será la encargada
/// de interpretar estos eventos y generar estados lógicos.
///
/// ---
///
/// #Ejemplo conceptual de uso
///
/// ```ignore
/// let raw_events = raw_layer.poll_events();      // eventos crudos
/// processed_layer.update(&raw_events);           // estado procesado
/// action_map.resolve(&processed_layer);          // acciones de juego
/// ```
///
/// La arquitectura se mantiene limpia, modular y multiplataforma.
/// ---------------------------------------------------------------------------
pub trait RawInputLayer {
    /// Tipo de evento crudo que este backend produce.
    ///
    /// Debe implementar [`KeyEventExt`] para que el resto del sistema
    /// pueda manejarlo de forma genérica.
    type KeyEvent: KeyEventExt;

    /// Recolecta **todos los eventos crudos** disponibles desde el backend.
    ///
    /// Este método:
    ///
    /// - lee el buffer del sistema operativo o driver,
    /// - recolecta los eventos nuevos desde la última llamada,
    /// - los convierte al tipo `KeyEvent`,
    /// - y devuelve un vector con ellos en orden cronológico.
    ///
    /// Debe **terminar rápidamente** y **no bloquear**.
    fn poll_events(&mut self) -> Vec<Self::KeyEvent>;
}
