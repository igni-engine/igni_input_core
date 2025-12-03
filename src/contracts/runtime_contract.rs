use crate::layers::{
    history::HistoryControlExt,
    mapping_layer::MappingLayerControl,
    processing_layer::ProcessingLayerControl,
    raw_layer::KeyEventExt,
};

use super::game_contract::GameContract;

/// ---------------------------------------------------------------------------
/// **RuntimeInputExt**
///
/// Contrato de alto nivel para el *runtime* del sistema de entrada.
/// 
/// Esta interfaz:
/// - Orquesta el ciclo de vida del input por frame.
/// - Recibe eventos crudos desde la plataforma (teclado, mouse, gamepad, etc.).
/// - Actualiza las capas internas (History, Processing, Mapping).
/// - Expone una vista final (`GameContract`) lista para que el motor la consuma.
///
/// No define *cómo* están implementadas las capas, solo **qué** debe poder hacer
/// un runtime de entrada para integrarse con Igni Engine.
/// ---------------------------------------------------------------------------
/// ## Flujo típico por frame
///
/// Un motor que use este trait suele hacer algo como:
///
/// ```ignore
/// // 1) Inicio del frame de entrada
/// runtime.begin_frame();
///
/// // 2) Inyectar eventos crudos provenientes del backend/plataforma
/// for event in backend.poll_events() {
///     runtime.push_raw_event(event);
/// }
///
/// // 3) Cerrar el frame de entrada (procesar capas y resolver acciones)
/// runtime.end_frame();
///
/// // 4) Consultar estado final desde la capa de juego
/// let input = runtime.game_layer();
/// if input.action_pressed("jump") {
///     player.jump();
/// }
/// ```
///
/// `RuntimeInputExt` no decide cómo se implementan las capas internas, solo
/// garantiza que el motor pueda:
/// - marcar el inicio del frame,
/// - alimentar el sistema con eventos crudos,
/// - cerrar el frame,
/// - acceder al estado final del input.
/// ---------------------------------------------------------------------------
pub trait RuntimeInputExt {
    /// Evento crudo que el runtime sabe manejar.
    ///
    /// Normalmente será algo que implemente `KeyEventExt` y provenga de la Raw Layer
    /// (tecla, botón de mouse, gamepad, etc.).
    type Event: KeyEventExt;

    /// Capa de juego que expone el estado final del sistema de entrada.
    ///
    /// Debe implementar [`GameContract`] y usar internamente las capas:
    /// - MappingLayerState
    /// - ProcessingLayerState
    /// - HistoryStateExt
    type GameLayer: GameContract;

    /// Implementación concreta de la capa de historial (control/mutación).
    type HistoryControl: HistoryControlExt;

    /// Implementación concreta de la capa procesada (control/mutación).
    type ProcessingControl: ProcessingLayerControl;

    /// Implementación concreta de la capa de mapeo (control/mutación).
    type MappingControl: MappingLayerControl;

    // -----------------------------------------------------------------------
    // CICLO DE VIDA DEL FRAME
    // -----------------------------------------------------------------------

    /// Marca el inicio del frame de entrada.
    ///
    /// Este método debe:
    /// - Preparar las capas internas para el nuevo frame.
    /// - Delegar en `begin_frame` de History/Processing/Mapping si aplica.
    /// - Limpiar estados transitorios del frame anterior.
    ///
    /// No debe procesar ningún evento todavía.
    fn begin_frame(&mut self);

    /// Inyecta un evento crudo en el runtime.
    ///
    /// Normalmente:
    /// - se registra el evento en `HistoryControl`,
    /// - se pasa a `ProcessingControl` para actualizar estados inmediatos.
    ///
    /// El orden exacto queda a criterio de la implementación del runtime.
    fn push_raw_event(&mut self, event: Self::Event);

    /// Completa el procesamiento del frame de entrada.
    ///
    /// Este método debe:
    /// - finalizar el procesamiento de `ProcessingControl` (transiciones, etc.),
    /// - actualizar el historial en `HistoryControl`,
    /// - invocar a `MappingControl` para resolver acciones,
    /// - dejar `GameLayer` listo para ser consultado por el motor.
    fn end_frame(&mut self);

    // -----------------------------------------------------------------------
    // ACCESO A CAPAS INTERNAS (MUTACIÓN)
    // -----------------------------------------------------------------------

    /// Acceso mutable a la capa de historial.
    ///
    /// Útil para:
    /// - limpiar historial,
    /// - resetear duraciones,
    /// - tooling/editor avanzado.
    fn history_mut(&mut self) -> &mut Self::HistoryControl;

    /// Acceso mutable a la capa procesada.
    ///
    /// Útil para:
    /// - limpiar estados,
    /// - ajustar lógica interna,
    /// - debug avanzado.
    fn processing_mut(&mut self) -> &mut Self::ProcessingControl;

    /// Acceso mutable a la capa de mapeo.
    ///
    /// Útil para:
    /// - rebinding en tiempo de ejecución,
    /// - cambiar contextos,
    /// - cargar perfiles de control, etc.
    fn mapping_mut(&mut self) -> &mut Self::MappingControl;

    // -----------------------------------------------------------------------
    // CAPA FINAL DE JUEGO (SOLO LECTURA)
    // -----------------------------------------------------------------------

    /// Devuelve una referencia a la capa de juego (`GameContract`),
    /// que representa el estado final del input en el frame actual.
    ///
    /// Esta es la interfaz que el motor y el gameplay deberían usar
    /// para leer acciones, valores y duraciones.
    fn game_layer(&self) -> &Self::GameLayer;
}
