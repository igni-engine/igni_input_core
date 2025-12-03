use crate::layers::{history::HistoryStateExt, processing_layer::ProcessingLayerState, raw_layer::KeyCodeExt};


// -----------------------------------------------------------------------------
// ContextId
// -----------------------------------------------------------------------------

/// Representa un identificador de contexto dentro del sistema de mapeo.
/// 
/// Los contextos permiten definir múltiples “capas” de control independientes.
/// Ejemplos:
/// - gameplay,
/// - UI,
/// - cinemáticas,
/// - vehículos,
/// - editores internos.
///
/// Requisito: debe ser clonable y comparable para poder gestionarlo en estructuras.
pub trait ContextId: Clone + Eq {}


// -----------------------------------------------------------------------------
// MappingLayerState — SOLO LECTURA, CERO ALOCACIONES EXTRAS
// -----------------------------------------------------------------------------

/// Consultas inmutables del sistema de mapeo.
/// 
/// Esta capa:
/// - **no modifica estado interno**,  
/// - **no reasigna teclas**,  
/// - **no crea ni elimina contextos**,  
/// - **no muta estructuras**.
///
/// Exposición principal: *slices* y referencias para máximo rendimiento.
pub trait MappingLayerState {
    type KeyCode: KeyCodeExt;
    type Ctx: ContextId;


    // -------------------------------------------------------------------------
    // CONTEXT
    // -------------------------------------------------------------------------

    /// Devuelve el contexto actualmente activo del sistema.
    ///
    /// Garantiza referencia estable sin copias ni asignaciones.
    fn current_context(&self) -> &Self::Ctx;

    /// Devuelve un slice con todos los contextos registrados.
    ///
    /// No crea nuevo vector; expone la memoria interna directamente
    /// para evitar overhead de clonados o asignaciones.
    fn contexts(&self) -> &[Self::Ctx];

    /// Indica si un contexto existe dentro del sistema.
    ///
    /// Útil para validar operaciones antes de ejecutarlas.
    fn has_context(&self, ctx: &Self::Ctx) -> bool;


    // -------------------------------------------------------------------------
    // ACTION → KEY
    // -------------------------------------------------------------------------

    /// Devuelve la tecla asociada a una acción dentro del contexto activo.
    ///
    /// Retorna `None` si:
    /// - la acción no existe,
    /// - o no tiene una tecla asignada.
    fn key_for_action(&self, action: &str) -> Option<Self::KeyCode>;

    /// Igual que `key_for_action`, pero restringido a un contexto específico.
    fn key_for_action_in(&self,ctx: &Self::Ctx,action: &str) -> Option<Self::KeyCode>;

    /// Indica si una acción existe en el contexto activo.
    fn has_action(&self, action: &str) -> bool;

    /// Indica si una acción existe en un contexto específico.
    fn has_action_in(&self, ctx: &Self::Ctx, action: &str) -> bool;

    /// Indica si la acción tiene una tecla asignada en el contexto activo.
    fn is_action_mapped(&self, action: &str) -> bool;

    /// Igual que `is_action_mapped`, pero para un contexto específico.
    fn is_action_mapped_in(&self, ctx: &Self::Ctx, action: &str) -> bool;

    /// Devuelve todas las acciones definidas en el contexto activo.
    ///
    /// Se expone un slice para evitar clonados.
    fn actions(&self) -> &[String];

    /// Devuelve todas las acciones definidas dentro de un contexto específico.
    fn actions_in(&self, ctx: &Self::Ctx) -> &[String];


    // -------------------------------------------------------------------------
    // KEY → ACTION
    // -------------------------------------------------------------------------

    /// Devuelve todas las acciones que se activan con una tecla en el contexto activo.
    ///
    /// Una tecla puede activar varias acciones a la vez.
    /// Ejemplo típico: accesibilidad o bindings de UI.
    fn actions_for_key(&self, key: &Self::KeyCode) -> &[String];

    /// Igual que `actions_for_key`, pero en un contexto específico.
    fn actions_for_key_in(&self,ctx: &Self::Ctx,key: &Self::KeyCode) -> &[String];

    /// Indica si una tecla está asignada a una o más acciones en el contexto activo.
    fn is_key_mapped(&self, key: &Self::KeyCode) -> bool;

    /// Igual que `is_key_mapped`, pero para un contexto específico.
    fn is_key_mapped_in(&self, ctx: &Self::Ctx, key: &Self::KeyCode) -> bool;

    /// Devuelve todos los pares (acción, tecla) dentro del contexto activo.
    ///
    /// Se devuelve un slice de tuplas internas para evitar copias.
    fn bindings(&self) -> &[(String, Self::KeyCode)];

    /// Devuelve todos los pares (acción, tecla) dentro de un contexto específico.
    fn bindings_in(&self, ctx: &Self::Ctx) -> &[(String, Self::KeyCode)];

    /// Indica si un contexto específico está habilitado en el sistema.
    /// retorna `true` si el contexto está activo y puede ser usado.
    /// retorna `false` si el contexto está deshabilitado o no existe.
    fn is_context_enabled(&self, ctx : &Self::Ctx) -> bool;


    // -------------------------------------------------------------------------
    // EXPORT (opcional)
    // -------------------------------------------------------------------------

    /// Exporta toda la configuración en formato serializable.
    ///
    /// Disponible solo con la feature `IE_maping`.
    #[cfg(feature = "IE_maping")]
    fn export_key_mappings<T>(&self) -> T;
}



// -----------------------------------------------------------------------------
// MappingLayerControl — CONTROL / MUTACIÓN DEL SISTEMA
// -----------------------------------------------------------------------------

/// Define todas las operaciones de modificación del sistema:
/// - crear contextos,
/// - borrar contextos,
/// - añadir acciones,
/// - renombrar,
/// - reasignar teclas,
/// - desasignar,
/// - operaciones globales.
///
/// Esta capa es mutante y debe ser usada por el editor y por scripts de tooling.
/// El runtime del juego normalmente usará solo `MappingLayerState`.
pub trait MappingLayerControl {
    type KeyCode: KeyCodeExt;
    type Ctx: ContextId;

    // -------------------------------------------------------------------------
    // CONTEXT MANAGEMENT
    // -------------------------------------------------------------------------

    /// Establece el contexto activo del sistema.
    ///
    /// Retorna:
    /// - `true` si el contexto existe y fue establecido con éxito.
    /// - `false` si el contexto no existe.
    ///
    /// Cambiar el contexto afecta qué acciones y bindings estarán vigentes
    /// durante la resolución del frame.
    fn set_current_context(&mut self, ctx: Self::Ctx) -> bool;


    // -------------------------------------------------------------------------
    // ACTION → KEY (MAPEO BÁSICO)
    // -------------------------------------------------------------------------

    /// Asigna una tecla a una acción dentro del **contexto activo**.
    ///
    /// Si la acción no existe en el contexto actual, retorna `false`.
    ///
    /// Si ya existía una asignación previa, se sobrescribe.
    fn map_action(&mut self, action: &str, key: Self::KeyCode) -> bool;

    /// Elimina la asignación de una acción dentro del **contexto activo**.
    ///
    /// Retorna:
    /// - `true` si la acción existía y fue desasignada.
    /// - `false` si la acción no existe.
    fn unmap_action(&mut self, action: &str) -> bool;


    // -------------------------------------------------------------------------
    // ACTION → KEY (MAPEO EN CONTEXTO ESPECÍFICO)
    // -------------------------------------------------------------------------

    /// Asigna una tecla a una acción dentro de un contexto específico.
    ///
    /// Retorna `true` solo si:
    /// - el contexto existe,
    /// - la acción existe en dicho contexto.
    fn map_action_in(&mut self,ctx: &Self::Ctx,action: &str,key: Self::KeyCode) -> bool;

    /// Elimina la asignación de una acción dentro de un contexto específico.
    ///
    /// Retorna `true` si la acción existía y fue desasignada.
    fn unmap_action_in(&mut self, ctx: &Self::Ctx, action: &str) -> bool;


    // -------------------------------------------------------------------------
    // ACTION → KEY (OPERACIONES GLOBALES)
    // -------------------------------------------------------------------------

    /// Asigna una tecla a una acción en **todos los contextos donde exista**.
    ///
    /// Útil para accesibilidad, UI global, o configuraciones compartidas.
    fn map_action_all(&mut self, action: &str, key: Self::KeyCode) -> bool;

    /// Elimina la asignación de una acción en **todos los contextos donde exista**.
    fn unmap_action_all(&mut self, action: &str) -> bool;


    // -------------------------------------------------------------------------
    // CONTEXT CREATION / DELETION
    // -------------------------------------------------------------------------

    /// Elimina un contexto completo del sistema.
    ///
    /// Retorna:
    /// - `true` si el contexto existía y fue eliminado.
    /// - `false` si no existía.
    ///
    /// NOTA: El contexto activo debe cambiarse previamente si coincide
    /// con el eliminado.
    fn remove_context(&mut self, ctx: &Self::Ctx) -> bool;

    /// Crea un nuevo contexto vacío.
    ///
    /// Retorna:
    /// - `true` si el contexto fue creado,
    /// - `false` si ya existía.
    fn add_context(&mut self, ctx: Self::Ctx) -> bool;


    // -------------------------------------------------------------------------
    // ACTION RENAMING
    // -------------------------------------------------------------------------

    /// Renombra una acción dentro del contexto activo.
    ///
    /// Retorna `true` si:
    /// - la acción antigua existe,
    /// - el nuevo nombre no está en uso.
    fn rename_action(&mut self, old_action: &str, new_action: &str) -> bool;

    /// Renombra una acción dentro de un contexto específico.
    fn rename_action_in(&mut self,ctx: &Self::Ctx,old_action: &str,new_action: &str) -> bool;

    /// Renombra una acción en **todos los contextos donde exista**.
    fn rename_action_all(&mut self,old_action: &str,new_action: &str) -> bool;


    // -------------------------------------------------------------------------
    // ACTION CREATION
    // -------------------------------------------------------------------------

    /// Crea una acción nueva sin tecla asignada dentro del **contexto activo**.
    ///
    /// Retorna:
    /// - `true` si fue creada,
    /// - `false` si ya existía.
    fn add_action(&mut self, action: &str) -> bool;

    /// Crea una acción dentro de un contexto específico.
    fn add_action_in(&mut self, ctx: &Self::Ctx, action: &str) -> bool;

    /// Crea una acción dentro de **todos los contextos existentes**.
    fn add_action_all(&mut self, action: &str) -> bool;


    // -------------------------------------------------------------------------
    // ACTION DELETION
    // -------------------------------------------------------------------------

    /// Elimina una acción del contexto activo.
    ///
    /// Retorna `true` solo si la acción existía.
    fn delete_action(&mut self, action: &str) -> bool;

    /// Elimina una acción dentro de un contexto específico.
    fn delete_action_in(&mut self, ctx: &Self::Ctx, action: &str) -> bool;

    /// Elimina una acción en **todos los contextos donde exista**.
    fn delete_action_all(&mut self, action: &str) -> bool;


    // -------------------------------------------------------------------------
    // BULK ACTION OPERATIONS
    // -------------------------------------------------------------------------

    /// Elimina **todas** las acciones del contexto activo.
    ///
    /// Retorna `true` si el contexto tenía acciones.
    fn delete_all_actions(&mut self) -> bool;

    /// Elimina todas las acciones dentro de un contexto específico.
    fn delete_all_actions_in(&mut self, ctx: &Self::Ctx) -> bool;


    // -------------------------------------------------------------------------
    // CONTEXT DUPLICATION
    // -------------------------------------------------------------------------

    /// Clona completamente un contexto existente en otro.
    ///
    /// - `to`: nuevo contexto destino (debe no existir o ser sobreescrito).
    /// - `from`: contexto origen.
    ///
    /// Retorna `true` si la operación fue exitosa.
    fn clone_context(&mut self, to: &Self::Ctx, from: Self::Ctx) -> bool;


    // -------------------------------------------------------------------------
    // CONTEXT RESET
    // -------------------------------------------------------------------------

    /// Resetea el contexto activo a estado vacío:
    /// - elimina **todas** las asignaciones,
    /// - conserva las acciones pero sin teclas asociadas.
    fn reset_context(&mut self) -> bool;

    /// Resetea un contexto específico a estado vacío.
    fn reset_context_in(&mut self, ctx: &Self::Ctx) -> bool;

    /// Resetea **todos** los contextos del sistema.
    fn reset_all_contexts(&mut self);


    // -------------------------------------------------------------------------
    // CONTEXT ENABLE / DISABLE
    // -------------------------------------------------------------------------

    /// Habilita un contexto específico.
    ///
    /// Un contexto habilitado puede ser utilizado para mapeo
    /// y para resolución de acciones.
    fn enable_context(&mut self, ctx: &Self::Ctx) -> bool;

    /// Deshabilita un contexto sin eliminarlo.
    ///
    /// Un contexto deshabilitado:
    /// - no participa en resolución,
    /// - no puede ser activado como actual.
    fn disable_context(&mut self, ctx: &Self::Ctx) -> bool;


    // -------------------------------------------------------------------------
    // IMPORT (opcional)
    // -------------------------------------------------------------------------

    /// Importa una configuración serializable de mapeo.
    ///
    /// Disponible solo bajo la feature `IE_maping`.
    #[cfg(feature = "IE_maping")]
    fn import_key_mappings<T>(&mut self, data: T);

    // -------------------------------------------------------------------------
    // FRAME CYCLE — Agregado según solicitaste
    // -------------------------------------------------------------------------

    /// Preparación del frame de mapeo.
    ///
    /// - Limpia el estado final del frame anterior.
    /// - Reinicia buffers internos.
    /// - No se resuelven acciones todavía.
    fn begin_frame(&mut self);

    /// Resolución de acciones del frame.
    ///
    /// Combina información proveniente de:
    /// - ProcessingLayerState (inmediato)
    /// - HistoryStateExt (temporal)
    ///
    /// Para producir:
    /// - pressed(action)
    /// - released(action)
    /// - held(action)
    /// - action_value(action)
    fn resolve_actions(&mut self,processing: &impl ProcessingLayerState<KeyCode = Self::KeyCode>,history: &impl HistoryStateExt<KeyCode = Self::KeyCode>);

    /// Finalización del frame:
    /// - Sella los resultados para lectura (`MappingLayerState`)
    /// - Limpia buffers temporales
    fn end_frame(&mut self);
}
