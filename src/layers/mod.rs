//! # IGNI Input Core — Organización de las capas
//!
//! El sistema de entrada de IGNI se estructura en **tres capas principales**, cada una
//! con responsabilidades bien definidas y completamente desacopladas.  
//!
//! Estas capas forman una arquitectura jerárquica:
//!
//! ```text
//! Raw Layer (Capa 0)  →  Processing Layer (Capa 1)  →  Mapping Layer (Capa 2)
//! ```
//!
//! Cada nivel se apoya únicamente en el nivel inmediatamente inferior, evitando
//! dependencias circulares y permitiendo reemplazar o extender partes del sistema
//! sin modificar el núcleo del motor.
//!
//! Este módulo expone los traits que representan las interfaces formales de cada capa.
//! Las implementaciones concretas se desarrollarán en crates independientes.
//!
//! ---
//!
//! # 1. Raw Layer (Capa 0)
//!
//! La capa más cercana al sistema operativo o backend físico de entrada.
//!
//! Su función es **describir eventos crudos** provenientes de:
//! - sistemas de ventanas,
//! - APIs nativas de teclado o mouse,
//! - gamepads,
//! - VR / dispositivos especiales,
//! - crates externos (winit, gilrs, evdev, etc.).
//!
//! Características clave:
//! - No procesa ni interpreta eventos.
//! - No mantiene estado.
//! - No calcula transiciones ni historial.
//! - No realiza mapeos ni lógica semántica.
//!
//! Su propósito es entregar datos “tal cual llegaron”, de forma estandarizada,
//! para las capas superiores.
//!
//! Traits principales (conceptuales):
//! - [`KeyCodeExt`]
//! - [`KeyEventExt`]
//! - [`KeyStateExt`]
//!
//! Estos traits modelan la mínima información necesaria sobre códigos, estados y eventos.
//!
//! ---
//!
//! # 2. Processing Layer (Capa 1)
//!
//! La encargada de **convertir eventos crudos en un estado útil y consultable**.
//!
//! Sus responsabilidades incluyen:
//! - reconocimiento de teclas *presionadas*, *mantenidas* y *liberadas*,
//! - cálculo de diferencias entre frames,
//! - manejo del estado previo y actual,
//! - registro opcional de historial reciente (si la feature `history` está habilitada).
//!
//! Esta capa define dos tipos de traits para garantizar separación estricta:
//!
//! ## a) `ProcessingLayerState`
//! - Solo lectura (<em>read-only</em>).
//! - Expone el estado ya procesado y listo para consultar en tiempo de juego.
//!
//! ## b) `ProcessingLayerControl`
//! - Mutación controlada.
//! - Recibe eventos crudos de la Raw Layer.
//! - Avanza el frame, limpia transiciones y reinicia el estado cuando corresponde.
//!
//! El runtime del motor utiliza principalmente `ProcessingLayerState`, mientras que el
//! backend del sistema operativo emplea `ProcessingLayerControl`.
//!
//! Si la característica `history` está habilitada, se expone un módulo adicional
//! con herramientas para análisis temporal del input.
//!
//! ---
//!
//! # 3. Mapping Layer (Capa 2)
//!
//! La capa de mayor nivel, orientada a la lógica de juego, accesibilidad y perfiles
//! controlados por el usuario.
//!
//! Aquí se definen conceptos como:
//! - **acciones** (“Jump”, “Fire”, “Interact”…),
//! - **contextos** (“Gameplay”, “UI”, “Vehicle”…),
//! - **mapeos dinámicos** entre teclas y acciones,
//! - **habilitación o deshabilitación** de contextos,
//! - **renombramiento, clonación y reseteo** de configuraciones.
//!
//! Como en la capa procesada, esta capa también distingue entre:
//!
//! ## a) `MappingLayerState`
//! - Solo lectura.
//! - Ideal para consultas durante gameplay, scripting o el runtime del motor.
//! - Retorna siempre slices y referencias para evitar asignaciones.
//!
//! ## b) `MappingLayerControl`
//! - Permite mutar configuraciones, redefinir controles, gestionar contextos y
//!   manipular acciones.
//! - Usado por el editor visual, herramientas externas y sistemas de configuración.
//!
//! Gracias a esta capa, el motor puede operar con conceptos semánticos de alto nivel,
//! independientemente del hardware o backend subyacente.
//!
//! ---
//!
//! # Módulos disponibles
//!
//! - [`raw_layer`] — Traits de la capa cruda (códigos, estados, eventos).
//! - [`processing_layer`] — Estado procesado del frame y control mutante del procesamiento.
//! - [`mapping_layer`] — Acciones, contextos y mapeos.
//!
//! Si está habilitada la feature:
//! - [`history`] — Herramientas opcionales de historial temporal para la capa de procesamiento.
//!
//! Cada módulo contiene exclusivamente **definiciones de traits**.  
//! No se incluye ninguna implementación real dentro de este crate.

pub mod raw_layer;
pub mod mapping_layer;
pub mod processing_layer;

#[cfg(feature = "history")]
pub mod history;
