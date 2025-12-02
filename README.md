# IGNI INPUT CORE

IGNI INPUT CORE define los contratos mínimos necesarios para la máxima compatibilidad con el futuro motor de videojuegos IGNI ENGINE.

Este crate **no implementa ningún comportamiento concreto**.
Su propósito exclusivo es ofrecer **interfaces (traits)** que garantizan:

* compatibilidad entre múltiples backends de entrada,
* separación estricta entre lectura y mutación,
* comportamiento determinista para el runtime,
* consistencia y seguridad en el editor,
* flexibilidad para futuras extensiones.

IGNI INPUT CORE es la base sobre la cual se construirán los sistemas de entrada del motor, del editor visual, de las herramientas externas y de los backends de cada plataforma.

---

## Estructura general

El sistema se divide en **tres capas fundamentales**, ordenadas desde el nivel más bajo (sin procesar) hasta el nivel lógico más complejo (acciones y contextos).

Cada capa superior depende únicamente de la inferior, nunca al revés.

---

## 1. Raw Layer (Capa 0)

La capa más cercana al sistema operativo o backend real de entrada.

### Responsabilidades

* Recibir eventos crudos provenientes del sistema operativo, drivers o crates externos.
* Representar teclas, códigos, estados y eventos en su forma más simple y sin procesar.
* Estandarizar el input de diversas plataformas en un formato común para el motor.

### Restricciones

* No procesa ni interpreta eventos.
* No mantiene estado.
* No agrupa, no asigna acciones, no calcula transiciones.
* Su responsabilidad termina en entregar eventos crudos hacia capas superiores.

### Traits de la capa

Debido a su naturaleza puramente descriptiva, esta capa define solo **traits de estado**, sin control mutante.

Ejemplos conceptuales:

* `KeyCodeExt`
* `KeyStateExt`
* `KeyEventExt`

Estos traits exponen metadatos y propiedades estáticas sobre los elementos crudos, pero nunca mutan estado ni establecen relaciones.

---

## 2. Processing Layer (Capa 1)

Esta capa recibe los eventos de la Raw Layer y construye a partir de ellos un **estado procesado** del input.

Ejemplos:

* teclas recién presionadas,
* teclas mantenidas,
* teclas liberadas,
* historial inmediato,
* cálculos de deltas temporales.

### Responsabilidades

* Transformar eventos crudos en información utilizable.
* Mantener el estado actual y previo del frame.
* Registrar transiciones.
* Preparar la información para sistemas superiores (acciones y mapeos).

### Estructura interna

La capa se divide en dos traits:

#### a) ProcessingLayerState

Solo lectura.
Permite consultar:

* estados de las teclas,
* transiciones,
* historial,
* valores derivados.

No muta nada.

#### b) ProcessingLayerControl

Permite:

* actualizar con nuevos eventos,
* avanzar o reiniciar el estado del frame,
* limpiar transiciones,
* ejecutar resets completos.

Esta separación garantiza seguridad en tiempo de compilación y claridad en la intención de uso.

---

## 3. Mapping Layer (Capa 2)

La capa más abstracta y orientada a lógica de juego.
Permite construir esquemas de control basados en **acciones**, **contextos** y **asignaciones dinámicas**.

A diferencia de la Processing Layer, esta capa opera sobre conceptos semánticos y configurables.

Ejemplos:

* Acción "Saltar" asignada a Space.
* Acción "Disparar" asignada a Mouse1.
* Contexto "Gameplay" separado del contexto "UI".
* Mapeos distintos para vehículos o cinemáticas.
* Contextos habilitados y deshabilitados dinámicamente.

### Responsabilidades

* Resolver Action → Key.
* Resolver Key → Action.
* Registrar, modificar o eliminar acciones.
* Crear, renombrar o eliminar contextos.
* Gestionar habilitación o deshabilitación de contextos.
* Aplicar resets totales o parciales.
* Clonar contextos.
* Permitir introspección profunda mediante traits de estado.

### Estructura interna

La capa define dos traits principales:

#### a) MappingLayerState

Solo lectura.
Permite consultar:

* contexto activo,
* lista de contextos,
* lista de acciones,
* bindings,
* acciones por tecla,
* estado de habilitación,
* consistencia del sistema.

Retorna slices y referencias para evitar asignaciones innecesarias.

#### b) MappingLayerControl

Mutación completa del sistema.
Permite:

* cambiar de contexto activo,
* añadir o eliminar contextos,
* habilitar o deshabilitar contextos,
* añadir, renombrar o eliminar acciones,
* asignar y desasignar teclas,
* resetear contextos total o parcialmente,
* clonar contextos completos,
* aplicar operaciones globales en todos los contextos.

Esta capa es la base de:

* el editor de IGNI ENGINE,
* las configuraciones de usuario,
* perfiles de control,
* sistemas de accesibilidad,
* mapeo dinámico en runtime, si está permitido.

---

## Filosofía del diseño

IGNI INPUT CORE está diseñado bajo cuatro principios:

1. Separación estricta de responsabilidades
   Cada capa hace solo lo que debe hacer, sin conocimiento innecesario de otras capas.

2. Máximo rendimiento
   La lectura del estado debe ser extremadamente rápida, evitando asignaciones y copias.

3. Extensibilidad total
   Cualquier backend (Windows, Linux evdev, XInput, gamepad, VR) puede cumplir los contratos sin restricciones artificiales.

4. Determinismo y claridad
   Los traits definen explícitamente qué se puede hacer y qué no, para evitar ambigüedades en plataformas y backends.

---

## Estado del proyecto

Este crate define exclusivamente los **contratos** necesarios para construir:

* backends,
* sistemas de procesamiento,
* el editor,
* el motor en sí.

No contiene implementación alguna.
Las implementaciones se desarrollarán en crates separados, manteniendo el núcleo completamente genérico y desacoplado.

---

