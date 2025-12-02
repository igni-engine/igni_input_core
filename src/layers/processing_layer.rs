//! ---------------------------------------------------------------------------
//! Módulo de contratos de la **Capa de Procesamiento** (Processed Layer)
//! del sistema de entrada de **Igni Engine**.
//!
//! Esta capa recibe eventos crudos provenientes de la capa RAW y genera
//! estados lógicos inmediatos del frame:
//!
//! - teclas presionadas
//! - teclas liberadas
//! - transiciones (`just_pressed`, `just_released`)
//! - duración de presión (`pressed_duration`)
//!
//! **No** almacena historial completo, **no** detecta combos temporales grandes,
//! **no** interpreta gestos complejos. Es una capa *inmediata*
//! (frame actual + frame previo).
//!
//! Se divide en dos traits:
//!
//! - [`ProcessingLayerControl`] → Mutación / ciclo de vida
//! - [`ProcessingLayerState`]   → Consultas / información del frame
//!
//!
//! ---------------------------------------------------------------------------

use std::time::Duration;
use crate::layers::raw_layer::{KeyCodeExt, KeyEventExt, KeyStateExt};


/// ---------------------------------------------------------------------------
/// Trait de **control interno** de la capa procesada.
///
/// Este trait es utilizado por el motor (o InputManager) para administrar
/// el ciclo de vida del estado procesado.
///
/// Los métodos de este trait:
/// - **mutan** el estado interno,
/// - NO deben ser usados por el gameplay directamente,
/// - deben ejecutarse una vez por frame.
/// ---------------------------------------------------------------------------
pub trait ProcessingLayerControl {
    type Event: KeyEventExt;

    /// Procesa los eventos crudos acumulados del frame.
    ///
    /// Este método debe:
    /// - actualizar el estado actual (`now`)
    /// - registrar timestamps de presión y liberación
    /// - preparar información para las consultas en `ProcessingLayerState`
    ///
    /// *Importante:* no debe mover `now → prev`, eso se hace en `begin_frame`.
    fn update(&mut self, events: &[Self::Event]);

    /// Restablece completamente el estado procesado.
    ///
    /// Borra:
    /// - estados actuales y anteriores
    /// - transiciones
    /// - timers de presión/liberación
    /// - cualquier estructura intermedia
    ///
    /// Útil al cambiar de escena, reiniciar niveles, o restaurar foco de ventana.
    fn reset(&mut self);

    /// Debe ser llamado al inicio del frame.
    ///
    /// Realiza:
    /// - copia del estado actual en el estado previo
    /// - limpieza de transiciones (`just_pressed`, `just_released`)
    ///
    /// Este método prepara el sistema para recibir nuevos eventos.
    fn begin_frame(&mut self);

    /// Debe ser llamado al final del frame.
    ///
    /// Calcula:
    /// - transiciones
    /// - estados derivados
    /// - flags de actividad (si alguna tecla fue presionada, etc.)
    ///
    /// Este método completa el procesamiento del frame.
    fn end_frame(&mut self);

    /// Limpia únicamente las transiciones (`just_pressed`, `just_released`).
    ///
    /// No modifica:
    /// - `now`
    /// - `prev`
    /// - timers
    ///
    /// Se usa típicamente en sistemas de UI donde se desea "consumir" entradas.
    fn clear_transitions(&mut self);

    /// Limpia completamente el estado procesado.
    ///
    /// Es equivalente a `reset()`, pero se deja explícito por claridad semántica.
    fn clear(&mut self);
}


/// ---------------------------------------------------------------------------
/// Trait de **consulta de estado** de la capa procesada.
///
/// NO muta estado.
/// NO puede fallar.
/// NO depende de ningún hardware.
///
/// Esta capa representa el estado lógico **del frame actual**, comparado con
/// el estado del frame anterior, lo cual permite detectar transiciones.
///
/// ---------------------------------------------------------------------------
pub trait ProcessingLayerState {
    type KeyCode: KeyCodeExt;
    type KeyState: KeyStateExt;

    // -----------------------------------------------------------------------
    // ESTADO INMEDIATO
    // -----------------------------------------------------------------------

    /// Devuelve `true` si la tecla está presionada actualmente.
    fn is_pressed(&self, key: &Self::KeyCode) -> bool;

    /// Devuelve `true` si la tecla está liberada actualmente.
    fn is_released(&self, key: &Self::KeyCode) -> bool;

    /// Devuelve `true` si la tecla está siendo mantenida (held).
    ///
    /// Equivale a `is_pressed` pero semánticamente más claro para game logic.
    fn is_held(&self, key: &Self::KeyCode) -> bool;

    /// Devuelve el estado lógico actual de la tecla.
    fn get_key_state(&self, key: &Self::KeyCode) -> Self::KeyState;

    // -----------------------------------------------------------------------
    // TRANSICIONES
    // -----------------------------------------------------------------------

    /// `true` si la tecla fue presionada *en este frame*.
    fn just_pressed(&self, key: &Self::KeyCode) -> bool;

    /// `true` si la tecla fue liberada *en este frame*.
    fn just_released(&self, key: &Self::KeyCode) -> bool;

    /// Devuelve `true` si *alguna* tecla fue presionada en este frame.
    fn any_key_just_pressed(&self) -> bool;

    /// Devuelve `true` si *alguna* tecla fue liberada en este frame.
    fn any_key_just_released(&self) -> bool;

    // -----------------------------------------------------------------------
    // COMBOS INMEDIATOS
    // -----------------------------------------------------------------------

    /// Devuelve `true` si **todas** las teclas del slice están presionadas.
    fn combo_pressed(&self, keys: &[Self::KeyCode]) -> bool;

    /// Devuelve `true` si **todas** las teclas fueron presionadas en este frame.
    ///
    /// No requiere historial largo → pertenece correctamente a este trait.
    fn just_pressed_combo(&self, keys: &[Self::KeyCode]) -> bool;

    // -----------------------------------------------------------------------
    // TIEMPOS / DURACIONES
    // -----------------------------------------------------------------------

    /// Tiempo que la tecla lleva presionada.
    ///
    /// Retorna `None` si la tecla no está presionada.
    ///
    /// Útil para:
    /// - cargar un ataque
    /// - "mantener para abrir"
    /// - repetir acciones graduales
    fn pressed_duration(&self, key: &Self::KeyCode) -> Option<Duration>;

    /// Tiempo desde la última liberación de la tecla.
    ///
    /// Útil para:
    /// - double tap (si no se usa HistoryLayer)
    /// - "cooldown" de entradas
    fn time_since_release(&self, key: &Self::KeyCode) -> Option<Duration>;

    // -----------------------------------------------------------------------
    // QUERIES DE DEBBUG Y REFLEXIÓN
    // -----------------------------------------------------------------------

    /// Devuelve una lista con todas las teclas presionadas actualmente.
    fn all_pressed_keys(&self) -> Vec<Self::KeyCode>;

    /// Devuelve un snapshot del estado actual de todas las teclas.
    ///
    /// Cada entrada contiene:
    /// - KeyCode
    /// - KeyState lógico actual
    ///
    /// Útil para logging, debug o herramientas de editor.
    fn current_state_snapshot(&self) -> Vec<(Self::KeyCode, Self::KeyState)>;
}
