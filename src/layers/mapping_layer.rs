use crate::layers::raw_layer::KeyCodeExt;


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
    fn key_for_action_in(&self,ctx: &Self::Ctx,action: &str,) -> Option<Self::KeyCode>;

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
    fn actions_for_key_in(&self,ctx: &Self::Ctx,key: &Self::KeyCode,) -> &[String];

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
    fn is_context_enabled(&self,ctx : &Self::Ctx) -> bool;


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


    /// Cambia el contexto activo.
    ///
    /// Retorna:
    /// - `true` si el contexto existe y fue activado,
    /// - `false` si no existe.
    fn set_current_context(&mut self, ctx: Self::Ctx) -> bool;


    /// Asigna una tecla a una acción dentro del contexto activo.
    ///
    /// Retorna false si la acción no existe.
    fn map_action(&mut self, action: &str, key: Self::KeyCode) -> bool;

    /// Elimina la asignación de una acción dentro del contexto activo.
    fn unmap_action(&mut self, action: &str) -> bool;

    /// Asigna una tecla a una acción dentro de un contexto concreto.
    fn map_action_in(&mut self,ctx: &Self::Ctx,action: &str,key: Self::KeyCode,) -> bool;

    /// Desasigna una acción dentro de un contexto específico.
    fn unmap_action_in(&mut self, ctx: &Self::Ctx, action: &str) -> bool;

    /// Asigna una tecla a una acción en todos los contextos en los que exista.
    fn map_action_all(&mut self, action: &str, key: Self::KeyCode) -> bool;

    /// Quita la asignación de una acción en todos los contextos en los que exista.
    fn unmap_action_all(&mut self, action: &str) -> bool;


    /// Elimina un contexto completo.
    fn remove_context(&mut self, ctx: &Self::Ctx) -> bool;

    /// Crea un nuevo contexto vacío.
    fn add_context(&mut self, ctx: Self::Ctx) -> bool;


    /// Renombra una acción dentro del contexto activo.
    fn rename_action(&mut self, old_action: &str, new_action: &str) -> bool;

    /// Renombra una acción dentro de un contexto específico.
    fn rename_action_in(&mut self,ctx: &Self::Ctx,old_action: &str,new_action: &str,) -> bool;

    /// Renombra una acción en todos los contextos donde exista.
    fn rename_action_all(&mut self,old_action: &str,new_action: &str,) -> bool;


    /// Crea una acción nueva sin tecla asignada dentro del contexto activo.
    fn add_action(&mut self, action: &str) -> bool;

    /// Crea una acción nueva dentro de un contexto específico.
    fn add_action_in(&mut self, ctx: &Self::Ctx, action: &str) -> bool;

    /// Crea una acción nueva en todos los contextos existentes.
    fn add_action_all(&mut self, action: &str) -> bool;


    /// Elimina una acción dentro del contexto activo.
    fn delete_action(&mut self, action: &str) -> bool;

    /// Elimina una acción dentro de un contexto específico.
    fn delete_action_in(&mut self, ctx: &Self::Ctx, action: &str) -> bool;

    /// Elimina una acción en todos los contextos donde exista.
    fn delete_action_all(&mut self, action: &str) -> bool;


    /// Elimina todas las acciones dentro del contexto activo.
    fn delete_all_actions(&mut self) -> bool;

    /// Elimina todas las acciones dentro de un contexto específico.
    fn delete_all_actions_in(&mut self, ctx: &Self::Ctx) -> bool;


    /// Clona un contexto existente hacia uno nuevo.
    fn clone_context(&mut self, to: &Self::Ctx, from: Self::Ctx) -> bool;




    /// resetea el contexto actiov a estado vacio.
    /// esto desasigna todas las acciones de sus respectivas teclas.
    /// no elimina las acciones, las deja en esta "sin asignar".
    fn reset_context(&mut self) -> bool;

    /// resetea un contexto especifico a estado vacio.
    fn reset_context_in(&mut self, ctx: &Self::Ctx) -> bool;

    /// resetea todos los contextos a estado vacio.
    fn reset_all_contexts(&mut self);


    /// Habilita un contexto específico.
    /// Retorna `true` si el contexto fue agregado o ya existía.
    /// Retorna `false` si no existe ese contexto,
    fn enable_context(&mut self, ctx: &Self::Ctx) -> bool;


    /// Deshabilita un contexto específico, inhabilitandolo del sistema pero sin eliminarlo.
    /// Retorna `true` si el contexto existía y fue eliminado.
    fn disable_context(&mut self, ctx: &Self::Ctx) -> bool;



    #[cfg(feature = "IE_maping")]
    /// Importa una configuración serializable en el sistema de mapeo.
    fn import_key_mappings<T>(&mut self, data: T);






}
