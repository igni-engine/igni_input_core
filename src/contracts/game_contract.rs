use crate::layers::{
    history::HistoryStateExt,
    mapping_layer::MappingLayerState,
    processing_layer::ProcessingLayerState,
};

/// ---------------------------------------------------------------------------
/// **GameContract**
///
/// Interfaz de alto nivel expuesta al motor de videojuegos (`Igni Engine`)
/// para consultar el estado final del sistema de entrada.
///
/// Este contrato representa la **capa de juego**, es decir, la vista más
/// semántica del input. No expone dispositivos (teclado, mouse, gamepad, etc.),
/// sino **acciones ya procesadas**, listas para usarse en gameplay.
///
/// Todo backend de entrada —sea el oficial o uno personalizado— debe
/// implementar este trait para integrarse con el motor.
///
/// ---------------------------------------------------------------------------
/// ## Responsabilidades
///
/// `GameContract` proporciona:
///
/// - Lectura de acciones: presionadas, liberadas y sostenidas.
/// - Lectura de valores analógicos normalizados (ejes, triggers, intensidades).
/// - Lectura de la duración temporal de una acción.
/// - Acceso **solo lectura** a las capas internas del sistema de entrada:
///   - MappingLayer (asociación acción → inputs)
///   - ProcessingLayer (transiciones y estados derivados)
///   - HistoryLayer (registro temporal de eventos)
///
/// No permite mutación. Toda la modificación del sistema de entrada ocurre en
/// `RuntimeInputExt`.
///
/// ---------------------------------------------------------------------------
/// ## Motivación del diseño
///
/// Esta interfaz es universal:
///
/// - No depende de dispositivos.
/// - Es estable ante cualquier backend.
/// - El motor puede consultar el input sin conocer su implementación.
/// - Permite agregar nuevos dispositivos sin modificar esta API.
///
/// ---------------------------------------------------------------------------
/// ## Ejemplo de uso (desde el motor)
///
/// ```ignore
/// let input = runtime.game_layer();
///
/// if input.action_pressed("jump") {
///     player.jump();
/// }
///
/// let move_x = input.action_value("move_x");
/// let time_holding_shoot = input.action_duration("shoot");
/// ```
///
/// ---------------------------------------------------------------------------
pub trait GameContract {
    // -----------------------------------------------------------------------
    // Tipos asociados: capas internas en modo solo lectura
    // -----------------------------------------------------------------------

    /// Estado actual del mapeo acción → inputs.
    ///
    /// Esta capa permite inspeccionar qué inputs activan cada acción,
    /// ya sea para depuración, UI o herramientas avanzadas.
    type MappingLayer: MappingLayerState;

    /// Estado derivado del procesamiento de entradas crudas.
    ///
    /// Aquí se calculan transiciones (`pressed`, `released`), deltas,
    /// patrones simples y estados intermedios que luego usa el sistema de acciones.
    type ProcessingLayer: ProcessingLayerState;

    /// Registro temporal de eventos, duraciones y estados históricos.
    ///
    /// Permite saber cuánto tiempo lleva activa una acción o consultar eventos pasados.
    type HistoryLayer: HistoryStateExt;

    // -----------------------------------------------------------------------
    // API universal de acciones (consumida por gameplay)
    // -----------------------------------------------------------------------

    /// Devuelve `true` si la acción fue **presionada en este frame**.
    ///
    /// Equivalente al tradicional `just_pressed`.
    fn action_pressed(&self, action: &str) -> bool;

    /// Devuelve `true` si la acción fue **liberada en este frame**.
    ///
    /// Equivalente a `just_released`.
    fn action_released(&self, action: &str) -> bool;

    /// Devuelve `true` mientras la acción permanezca activa.
    ///
    /// Esta función no distingue entre frames; refleja el estado actual inmediato.
    fn action_held(&self, action: &str) -> bool;

    /// Retorna un **valor analógico** normalizado asociado a la acción.
    ///
    /// - Para teclas digitales: típicamente `0.0` o `1.0`.
    /// - Para sticks/triggers: rango esperado `[-1.0, 1.0]` o `[0.0, 1.0]`.
    /// - Para acciones compuestas: valor calculado por la capa de mapeo.
    fn action_value(&self, action: &str) -> f32;

    /// Duración en segundos desde que la acción entró en estado `held`.
    ///
    /// Muy útil para:
    /// - cargar disparos
    /// - mantener botones para acciones largas
    /// - medir interacción prolongada
    fn action_duration(&self, action: &str) -> f32;

    // -----------------------------------------------------------------------
    // Acceso de solo lectura a las capas internas
    // -----------------------------------------------------------------------

    /// Referencia de solo lectura a la capa de mapeo.
    fn mapping_layer(&self) -> &Self::MappingLayer;

    /// Referencia de solo lectura a la capa de procesamiento.
    fn processing_layer(&self) -> &Self::ProcessingLayer;

    /// Referencia de solo lectura a la capa histórica.
    fn history_layer(&self) -> &Self::HistoryLayer;
}
