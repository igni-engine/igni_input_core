use std::time::Duration;

use crate::layers::raw_layer::{KeyCodeExt, KeyStateExt};


/// Control del historial de entrada.
///
/// Esta capa registra eventos crudos directamente desde la Raw Layer.
/// NO interpreta estados (eso lo hace Processing).
/// NO resuelve acciones (eso lo hace Mapping).
///
/// Su función principal es almacenar información temporal relevante:
/// - timestamps de presión y liberación
/// - duración acumulada de una tecla en estado `Pressed`
/// - secuencia cronológica de eventos (útil para combos, replay, debug)
pub trait HistoryControlExt {

    type KeyCode: KeyCodeExt;
    type KeyState: KeyStateExt;

    /// Llamado al inicio del frame.
    ///
    /// Se recomienda:
    /// - mover buffers temporales (`now → prev`)
    /// - limpiar marcadores históricos del frame anterior
    /// - preparar acumuladores de duración
    fn begin_frame(&mut self);

    /// Registra un evento crudo proveniente de la Raw Layer.
    ///
    /// Parámetros:
    /// - `key`: código de tecla o input
    /// - `state`: nuevo estado (`Pressed`, `Released`, etc.)
    /// - `timestamp`: instante del evento
    ///
    /// Este método:
    /// - almacena el evento en el historial
    /// - actualiza el tiempo de `held` si aplica
    /// - actualiza timestamps de actividad
    fn add_event(&mut self,key: impl Into<Self::KeyCode>, state: impl Into<Self::KeyState>,timestamp: Duration,);

    /// Llamado al final del frame.
    ///
    /// Este método debe:
    /// - consolidar duraciones (`held_duration`)
    /// - limpiar eventos transitorios si aplica
    /// - cerrar el registro del frame
    fn end_frame(&mut self);

    /// Elimina todo el historial registrado.
    ///
    /// Deja la capa en estado completamente vacío.
    fn clear(&mut self);
}

/// Consultas de estado y funciones de análisis temporal sobre el historial
/// de entrada.
/// 
/// Esta interfaz permite inspeccionar eventos previos, analizar combinaciones
/// no limitadas a un solo frame, validar secuencias en ventanas de tiempo,
/// y detectar patrones ordenados.
///
/// El historial es una estructura orientada a almacenar eventos crudos o 
/// procesados con marca de tiempo, dependiendo del backend que lo implemente.
pub trait HistoryStateExt {
    type KeyCode: KeyCodeExt;
    type KeyState: KeyStateExt;

    /// Indica si el historial está vacío.
    fn is_empty(&self) -> bool;

    /// Devuelve la cantidad total de eventos almacenados.
    fn len(&self) -> usize;

    /// Comprueba si todas las teclas especificadas en `combo` han sido 
    /// registradas en su estado más reciente, sin límites de tiempo ni frames.
    ///
    /// El orden no importa; solo la presencia simultánea en el último snapshot 
    /// representado por el historial.
    fn match_combo(&self, combo: &[Self::KeyCode]) -> bool;

    /// Comprueba si todas las teclas en `combo` ocurrieron dentro de una 
    /// ventana de `prev_frames` frames hacia atrás.
    ///
    /// Esta función es adecuada para engines que trabajan con buffers 
    /// discretos por frame y desean validar combinaciones recientes.
    fn match_combo_in_frames(&self,combo: &[Self::KeyCode],prev_frames: usize) -> bool;

    /// Comprueba si una clave específica con un estado dado fue registrada 
    /// en los últimos `prev_frames` frames.
    ///
    /// Esto permite verificar secuencias simples como pulsaciones rápidas,
    /// releases recientes o cualquier transición basada en frames.
    fn match_key_in_frames(&self,key: &Self::KeyCode,state: &Self::KeyState,prev_frames: usize) -> bool;

    /// Comprueba si todas las teclas en `combo` ocurrieron dentro de un
    /// intervalo de tiempo absoluto.
    ///
    /// A diferencia de las variantes basadas en frames, esta función utiliza
    /// tiempo real (`Duration`), lo que permite detectar:
    /// - combos dependientes de timing,
    /// - secuencias rápidas,
    /// - inputs estilo "rhythm".
    fn match_combo_in_time_window(&self,combo: &[Self::KeyCode],max_window: Duration) -> bool;

    /// Comprueba si una secuencia ORDENADA de teclas ocurrió respetando un 
    /// intervalo máximo entre cada par consecutivo.
    ///
    /// Por ejemplo, para la secuencia `[Shift, A]` con `max_interval = 5 ms`,
    /// el combo es válido si:
    ///     timestamp(A) - timestamp(Shift) <= 5 ms
    ///
    /// Esta función detecta secuencias rápidas dependientes del ritmo del 
    /// jugador, común en juegos de pelea, plataformas y shooters.
    fn match_ordered_sequence(&self,sequence: &[Self::KeyCode],max_interval: Duration) -> bool;

    /// Versión alternativa que inspecciona únicamente la secuencia más reciente
    /// hacia atrás en el historial.
    ///
    /// La diferencia con `match_ordered_sequence` es que esta función busca
    /// la coincidencia partiendo del evento más reciente y avanzando hacia el
    /// pasado, útil para bufers grandes donde la secuencia válida está cerca
    /// del final del historial.
    fn match_recent_ordered_sequence(&self,sequence: &[Self::KeyCode],max_interval: Duration) -> bool;



    /// Proporciona acceso directo al historial completo de eventos.
    /// Cada evento es una tupla que contiene:
    /// - El código de la tecla (`KeyCode`),
    /// - El estado de la tecla (`KeyState`),
    /// - La marca de tiempo (`Duration`) del evento.
    fn history(&self) -> &Vec<(Self::KeyCode, Self::KeyState, Duration)>;
}
